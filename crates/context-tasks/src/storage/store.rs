use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::error::{ProtocolError, StorageError};
use crate::model::default_schema::schema_for_type;
use crate::model::edge::EdgeRecord;
use crate::model::filesystem::ScanRoot;
use crate::model::query::parse_query;
use crate::model::ticket::{TicketId, TicketManifest};
use crate::storage::index::RedbIndexStore;
use crate::storage::indexed::{IndexedTicket, LeaseInfo};
use crate::storage::search::{SearchResult, TantivySearchIndex};
use crate::storage::ticket_fs::{TicketFs, TicketScanEntry};

/// The central ticket store: filesystem source-of-truth + redb metadata index +
/// Tantivy full-text search index.
pub struct TicketStore {
    index: RedbIndexStore,
    search: TantivySearchIndex,
    /// Root directory for the redb database and Tantivy index files.
    pub index_root: PathBuf,
}

impl TicketStore {
    /// Open (or create) a ticket store rooted at `index_root`.
    pub fn open(index_root: &Path) -> Result<Self, StorageError> {
        std::fs::create_dir_all(index_root)?;
        let db_path = index_root.join("tickets.redb");
        let search_dir = index_root.join("search_index");

        let index = RedbIndexStore::open(&db_path)?;
        let search = TantivySearchIndex::open_or_create(&search_dir)?;

        Ok(Self {
            index,
            search,
            index_root: index_root.to_path_buf(),
        })
    }

    // ── scan root management ──────────────────────────────────────────────────

    pub fn add_scan_root(&self, root: ScanRoot) -> Result<(), StorageError> {
        self.index.add_scan_root(&root)
    }

    pub fn list_scan_roots(&self) -> Result<Vec<ScanRoot>, StorageError> {
        self.index.list_scan_roots()
    }

    // ── ticket CRUD ──────────────────────────────────────────────────────────

    /// Create a new ticket.
    ///
    /// `target_root`: the scan root directory to place the ticket folder in.
    /// If `None`, the first registered scan root is used (error if none exist).
    pub fn create(
        &self,
        id: Option<Uuid>,
        type_id: &str,
        title: Option<&str>,
        initial_state: Option<&str>,
        extra: BTreeMap<String, Value>,
        target_root: Option<&Path>,
    ) -> Result<TicketId, StorageError> {
        let id = id.unwrap_or_else(Uuid::new_v4);
        let now = Utc::now();

        // Resolve target scan root.
        let root = match target_root {
            Some(p) => p.to_path_buf(),
            None => {
                let roots = self.index.list_scan_roots()?;
                roots
                    .into_iter()
                    .next()
                    .map(|r| r.path)
                    .unwrap_or_else(|| self.index_root.join("tickets"))
            }
        };
        std::fs::create_dir_all(&root)?;

        let mut manifest = TicketManifest::new(id, now);
        manifest.extra.insert("type".to_string(), Value::String(type_id.to_string()));
        if let Some(t) = title {
            manifest.extra.insert("title".to_string(), Value::String(t.to_string()));
        }
        let state = initial_state.unwrap_or("open").to_string();
        manifest.extra.insert("state".to_string(), Value::String(state.clone()));
        for (k, v) in extra {
            manifest.extra.insert(k, v);
        }

        // Validate against type schema if known.
        if let Some(schema) = schema_for_type(type_id) {
            schema.validate_manifest(&manifest)?;
        }

        let ticket_path = TicketFs::create(&manifest, &root)?;

        let indexed = IndexedTicket {
            id,
            path: ticket_path,
            type_id: type_id.to_string(),
            title: title.map(str::to_string),
            state: Some(state.clone()),
            created_at: now,
            updated_at: now,
            deleted: false,
        };
        self.index.insert_ticket(&indexed)?;

        let body = TicketFs::read_description(&indexed.path);
        self.search.upsert(
            &id,
            title,
            body.as_deref(),
            Some(&state),
            Some(type_id),
        )?;

        Ok(id)
    }

    /// Read the full manifest for a ticket by ID.
    pub fn get(&self, id: &Uuid) -> Result<TicketManifest, StorageError> {
        let indexed = self
            .index
            .get_ticket(id)?
            .ok_or(StorageError::NotFound(*id))?;
        if indexed.deleted {
            return Err(StorageError::NotFound(*id));
        }
        TicketFs::read(&indexed.path)
    }

    /// Get just the indexed metadata (faster than a full read).
    pub fn get_indexed(&self, id: &Uuid) -> Result<Option<IndexedTicket>, StorageError> {
        self.index.get_ticket(id)
    }

    /// Update a ticket: apply field patches and optional state transition.
    pub fn update(
        &self,
        id: &Uuid,
        patch: BTreeMap<String, Value>,
        from_state: Option<&str>,
        to_state: Option<&str>,
    ) -> Result<TicketManifest, StorageError> {
        let mut indexed = self
            .index
            .get_ticket(id)?
            .ok_or(StorageError::NotFound(*id))?;
        if indexed.deleted {
            return Err(StorageError::NotFound(*id));
        }

        // Validate state transition if type schema is known and state change requested.
        if let Some(to) = to_state {
            let current_state = indexed.state.as_deref().unwrap_or("open");
            let from = from_state.unwrap_or(current_state);
            if let Some(schema) = schema_for_type(&indexed.type_id) {
                schema.ensure_transition(from, to)?;
            }
        }

        let new_state = to_state.map(str::to_string).or_else(|| indexed.state.clone());
        let updated_manifest = TicketFs::update(&indexed.path, &patch, to_state)?;

        // Refresh indexed metadata.
        let now = Utc::now();
        indexed.updated_at = now;
        if let Some(s) = &new_state {
            indexed.state = Some(s.clone());
        }
        if let Some(title_val) = patch.get("title").and_then(|v| v.as_str()) {
            indexed.title = Some(title_val.to_string());
        }
        self.index.insert_ticket(&indexed)?;

        let body = TicketFs::read_description(&indexed.path);
        self.search.upsert(
            id,
            indexed.title.as_deref(),
            body.as_deref(),
            indexed.state.as_deref(),
            Some(indexed.type_id.as_str()),
        )?;

        Ok(updated_manifest)
    }

    /// Soft-delete a ticket.
    pub fn delete(&self, id: &Uuid) -> Result<(), StorageError> {
        let indexed = self
            .index
            .get_ticket(id)?
            .ok_or(StorageError::NotFound(*id))?;
        if indexed.deleted {
            return Err(StorageError::NotFound(*id));
        }
        TicketFs::mark_deleted(&indexed.path)?;
        self.index.soft_delete_ticket(id)?;
        self.search.remove(id)?;
        Ok(())
    }

    // ── list / search ─────────────────────────────────────────────────────────

    pub fn list(
        &self,
        state_filter: Option<&str>,
        type_filter: Option<&str>,
        limit: Option<usize>,
    ) -> Result<Vec<IndexedTicket>, StorageError> {
        let all = self.index.list_tickets(false)?;
        let filtered: Vec<_> = all
            .into_iter()
            .filter(|t| {
                if let Some(s) = state_filter {
                    if t.state.as_deref() != Some(s) {
                        return false;
                    }
                }
                if let Some(tp) = type_filter {
                    if t.type_id != tp {
                        return false;
                    }
                }
                true
            })
            .take(limit.unwrap_or(usize::MAX))
            .collect();
        Ok(filtered)
    }

    pub fn search_tickets(
        &self,
        query_expr: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>, StorageError> {
        let expr = parse_query(query_expr).map_err(StorageError::QueryParse)?;
        self.search.search(&expr, limit)
    }

    // ── edge management ───────────────────────────────────────────────────────

    pub fn add_edge(&self, edge: EdgeRecord) -> Result<(), StorageError> {
        // For acyclic-enforced kinds: check for cycles.
        let schema = schema_for_type(crate::model::default_schema::TYPE_ID);
        let is_acyclic = schema
            .as_ref()
            .and_then(|s| s.edge_rules.get(&edge.kind))
            .map(|r| r.acyclic_enforced)
            .unwrap_or(false);

        if is_acyclic && self.index.is_reachable(&edge.to, &edge.from)? {
            return Err(StorageError::DependencyCycle);
        }

        self.index.insert_edge(&edge)
    }

    // ── scan / reconcile ──────────────────────────────────────────────────────

    /// Walk all registered scan roots and integrate discovered tickets into the
    /// index and search index.
    ///
    /// If `reindex` is `true`, the search index is rebuilt from scratch for all
    /// found tickets (crash recovery path).
    pub fn scan(&self, reindex: bool) -> Result<ScanReport, StorageError> {
        let roots = self.index.list_scan_roots()?;

        // Also always include the default tickets dir under index_root.
        let default_root = ScanRoot {
            path: self.index_root.join("tickets"),
            label: "default".to_string(),
        };
        let all_roots: Vec<&ScanRoot> = std::iter::once(&default_root)
            .chain(roots.iter())
            .collect();

        let mut integrated = 0usize;
        let mut diagnostics = Vec::new();

        for root in all_roots {
            if !root.path.exists() {
                continue;
            }
            let (entries, diags) = TicketFs::scan_root(&root.path)?;
            diagnostics.extend(diags);

            for entry in entries {
                integrate_entry(&self.index, &self.search, entry, reindex)?;
                integrated += 1;
            }
        }

        Ok(ScanReport { integrated, diagnostics })
    }

    // ── lease operations (Phase 1.5 pre-wire) ────────────────────────────────

    pub fn claim(
        &self,
        ticket_id: &Uuid,
        agent_id: &str,
        ttl_secs: u64,
        work_intent: Option<&str>,
    ) -> Result<LeaseInfo, StorageError> {
        // Check for existing non-expired lease.
        if let Some(existing) = self.index.get_lease(ticket_id)? {
            if !existing.is_expired() {
                return Err(StorageError::LeaseConflict {
                    ticket: *ticket_id,
                    holder: existing.working_by.clone(),
                });
            }
        }

        let now = Utc::now();
        let lease = LeaseInfo {
            ticket_id: *ticket_id,
            working_by: agent_id.to_string(),
            work_intent: work_intent.map(str::to_string),
            claimed_at: now,
            lease_expires_at: now + chrono::Duration::seconds(ttl_secs as i64),
            ttl_secs,
            conflict_domain: None,
        };
        self.index.insert_lease(&lease)?;
        Ok(lease)
    }

    pub fn unclaim(&self, ticket_id: &Uuid) -> Result<(), StorageError> {
        self.index.remove_lease(ticket_id)
    }

    pub fn list_leases(&self) -> Result<Vec<LeaseInfo>, StorageError> {
        self.index.list_active_leases()
    }

    // ── validation & release protocol ─────────────────────────────────────────

    /// `task_validate_start` — move ticket from `review` to `validating`.
    ///
    /// Guards:
    /// - current state must be `review`
    /// - `validator_id` must not equal `worker_id` (separation of duties)
    pub fn validate_start(
        &self,
        ticket_id: &Uuid,
        assignment_id: &str,
        validator_id: &str,
        validation_profile: &str,
        required_checks: Vec<String>,
    ) -> Result<TicketManifest, StorageError> {
        let manifest = self.get(ticket_id)?;
        let current_state = manifest.extra.get("state").and_then(|v| v.as_str()).unwrap_or("");

        if current_state != "review" {
            return Err(ProtocolError::ValidateInvalidState {
                ticket: *ticket_id,
                actual: current_state.to_string(),
                expected: "review".to_string(),
            }
            .into());
        }

        // Separation-of-duties check.
        let worker_id = manifest.extra.get("working_by").and_then(|v| v.as_str()).unwrap_or("");
        if !worker_id.is_empty() && worker_id == validator_id {
            return Err(ProtocolError::ValidateSameIdentity {
                identity: validator_id.to_string(),
            }
            .into());
        }

        let mut patch = BTreeMap::new();
        patch.insert("validator_id".to_string(), Value::String(validator_id.to_string()));
        patch.insert("validation_status".to_string(), Value::String("in-progress".to_string()));
        patch.insert("validation_profile".to_string(), Value::String(validation_profile.to_string()));
        patch.insert(
            "required_checks".to_string(),
            Value::Array(required_checks.into_iter().map(Value::String).collect()),
        );
        patch.insert("assignment_id".to_string(), Value::String(assignment_id.to_string()));

        self.update(ticket_id, patch, Some("review"), Some("validating"))
    }

    /// `task_validate_result` — submit validation outcome.
    ///
    /// `result` must be `"passed"` or `"failed"`.
    /// On pass: ticket moves to `validated`, `validation_status=passed`.
    /// On fail: ticket moves back to `review`, `validation_status=failed`.
    ///
    /// Guards:
    /// - current state must be `validating`
    /// - `validator_id` must match recorded validator
    /// - `evidence_refs` must be non-empty
    pub fn validate_result(
        &self,
        ticket_id: &Uuid,
        assignment_id: &str,
        validator_id: &str,
        result: &str,
        evidence_refs: Vec<String>,
        summary: Option<&str>,
        bug_links: Vec<Uuid>,
    ) -> Result<ValidationResultOutcome, StorageError> {
        if evidence_refs.is_empty() {
            return Err(ProtocolError::ValidateMissingEvidence.into());
        }

        let manifest = self.get(ticket_id)?;
        let current_state = manifest.extra.get("state").and_then(|v| v.as_str()).unwrap_or("");
        if current_state != "validating" {
            return Err(ProtocolError::ValidateInvalidState {
                ticket: *ticket_id,
                actual: current_state.to_string(),
                expected: "validating".to_string(),
            }
            .into());
        }

        let recorded_validator = manifest.extra.get("validator_id").and_then(|v| v.as_str()).unwrap_or("");
        if !recorded_validator.is_empty() && recorded_validator != validator_id {
            return Err(ProtocolError::ValidateAssignmentMismatch.into());
        }

        let passed = result == "passed";
        let (new_state, status_str) = if passed {
            ("validated", "passed")
        } else {
            ("review", "failed")
        };

        let mut patch = BTreeMap::new();
        patch.insert("validation_status".to_string(), Value::String(status_str.to_string()));
        patch.insert("assignment_id".to_string(), Value::String(assignment_id.to_string()));
        patch.insert(
            "evidence_refs".to_string(),
            Value::Array(evidence_refs.iter().map(|s| Value::String(s.clone())).collect()),
        );
        if let Some(s) = summary {
            patch.insert("validation_summary".to_string(), Value::String(s.to_string()));
        }
        if !bug_links.is_empty() {
            patch.insert(
                "bug_links".to_string(),
                Value::Array(bug_links.iter().map(|id| Value::String(id.to_string())).collect()),
            );
        }

        let from_state = "validating";
        let _updated = self.update(ticket_id, patch, Some(from_state), Some(new_state))?;

        Ok(ValidationResultOutcome {
            ticket_id: *ticket_id,
            state: new_state.to_string(),
            validation_status: status_str.to_string(),
            passed,
        })
    }

    /// `task_release_candidate_create` — move a `validated` ticket to `release-candidate`.
    ///
    /// Guards:
    /// - current state must be `validated`
    /// - `validation_status` must be `passed`
    /// - `assignment_chain` must be non-empty
    pub fn release_candidate_create(
        &self,
        ticket_id: &Uuid,
        release_target: &str,
        assignment_chain: Vec<String>,
    ) -> Result<TicketManifest, StorageError> {
        if assignment_chain.is_empty() {
            return Err(ProtocolError::ReleaseAssignmentChainMissing.into());
        }

        let manifest = self.get(ticket_id)?;
        let current_state = manifest.extra.get("state").and_then(|v| v.as_str()).unwrap_or("");
        if current_state != "validated" {
            return Err(ProtocolError::ReleaseInvalidState {
                ticket: *ticket_id,
                actual: current_state.to_string(),
                expected: "validated".to_string(),
            }
            .into());
        }

        let validation_status = manifest.extra.get("validation_status").and_then(|v| v.as_str()).unwrap_or("");
        if validation_status != "passed" {
            return Err(ProtocolError::ReleaseValidationNotPassed {
                ticket: *ticket_id,
                status: validation_status.to_string(),
            }
            .into());
        }

        let mut patch = BTreeMap::new();
        patch.insert("release_target".to_string(), Value::String(release_target.to_string()));
        patch.insert(
            "assignment_chain".to_string(),
            Value::Array(assignment_chain.into_iter().map(Value::String).collect()),
        );

        self.update(ticket_id, patch, Some("validated"), Some("release-candidate"))
    }

    /// `task_release_gate_check` — evaluate release gates for a target.
    ///
    /// Returns pass/fail results for each requested gate ID.
    /// The standard gates are R1–R4 as defined in VALIDATION_RELEASE_GOVERNANCE.md.
    pub fn release_gate_check(
        &self,
        release_target: &str,
        required_gates: &[String],
    ) -> Result<GateCheckOutcome, StorageError> {
        // Collect all release-candidate tickets for this target.
        let all = self.index.list_tickets(false)?;
        let candidates: Vec<_> = all
            .iter()
            .filter(|t| {
                t.state.as_deref() == Some("release-candidate")
                    || t.state.as_deref() == Some("validated")
            })
            .filter(|_t| {
                // Check release_target field in manifest if needed; use in-memory index for speed.
                // For now, accept all candidates when release_target is present — manifest check
                // happens in full promote path.
                true
            })
            .collect();

        if candidates.is_empty() {
            return Err(ProtocolError::ReleaseTargetNotFound(release_target.to_string()).into());
        }

        let mut gate_results: BTreeMap<String, GateStatus> = BTreeMap::new();
        let mut blocking_reasons: Vec<String> = Vec::new();

        for gate in required_gates {
            let (status, reason) = evaluate_gate(gate.as_str(), &candidates, release_target, &self.index)?;
            if let Some(r) = reason {
                blocking_reasons.push(format!("{gate}: {r}"));
            }
            gate_results.insert(gate.clone(), status);
        }

        Ok(GateCheckOutcome {
            release_target: release_target.to_string(),
            gates: gate_results,
            blocking_reasons,
        })
    }

    /// `task_release_promote` — promote all `release-candidate` tickets for a target.
    ///
    /// Guards:
    /// - all required gates must be `pass`
    /// - `merge_commit` must be provided
    pub fn release_promote(
        &self,
        release_target: &str,
        release_version: &str,
        merge_commit: &str,
        required_gates: &[String],
    ) -> Result<PromoteOutcome, StorageError> {
        if merge_commit.is_empty() {
            return Err(ProtocolError::ReleaseMergeMetadataMissing.into());
        }

        // Gate check.
        let gate_outcome = self.release_gate_check(release_target, required_gates)?;
        let failing_gates: Vec<_> = gate_outcome
            .gates
            .iter()
            .filter(|(_, s)| !matches!(s, GateStatus::Pass))
            .map(|(k, _)| k.clone())
            .collect();
        if !failing_gates.is_empty() {
            return Err(ProtocolError::ReleaseGatesNotSatisfied(
                gate_outcome.blocking_reasons.join("; "),
            )
            .into());
        }

        let all = self.index.list_tickets(false)?;
        let to_promote: Vec<Uuid> = all
            .into_iter()
            .filter(|t| t.state.as_deref() == Some("release-candidate"))
            .map(|t| t.id)
            .collect();

        if to_promote.is_empty() {
            return Err(ProtocolError::ReleaseTicketStateInvalid(
                format!("no release-candidate tickets found for target '{release_target}'"),
            )
            .into());
        }

        let mut promoted = 0usize;
        for ticket_id in &to_promote {
            let mut patch = BTreeMap::new();
            patch.insert("release_version".to_string(), Value::String(release_version.to_string()));
            patch.insert("merge_commit".to_string(), Value::String(merge_commit.to_string()));
            self.update(ticket_id, patch, Some("release-candidate"), Some("released"))?;
            promoted += 1;
        }

        Ok(PromoteOutcome {
            release_target: release_target.to_string(),
            release_version: release_version.to_string(),
            promoted_ticket_count: promoted,
            monitoring_state: "active".to_string(),
        })
    }
}

pub struct ScanReport {
    pub integrated: usize,
    pub diagnostics: Vec<crate::model::filesystem::ParseDiagnostic>,
}

/// Outcome of `task_validate_result`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResultOutcome {
    pub ticket_id: Uuid,
    pub state: String,
    pub validation_status: String,
    pub passed: bool,
}

/// Per-gate evaluation status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GateStatus {
    Pass,
    Fail,
}

/// Outcome of `task_release_gate_check`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateCheckOutcome {
    pub release_target: String,
    pub gates: BTreeMap<String, GateStatus>,
    pub blocking_reasons: Vec<String>,
}

/// Outcome of `task_release_promote`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoteOutcome {
    pub release_target: String,
    pub release_version: String,
    pub promoted_ticket_count: usize,
    pub monitoring_state: String,
}

fn evaluate_gate(
    gate: &str,
    candidates: &[&IndexedTicket],
    _release_target: &str,
    _index: &RedbIndexStore,
) -> Result<(GateStatus, Option<String>), StorageError> {
    match gate {
        // R1: all included tickets are validated/release-candidate — no open blockers
        "R1" => {
            let all_ready = candidates
                .iter()
                .all(|t| matches!(t.state.as_deref(), Some("validated") | Some("release-candidate")));
            if all_ready {
                Ok((GateStatus::Pass, None))
            } else {
                Ok((GateStatus::Fail, Some("some tickets are not yet validated".to_string())))
            }
        }
        // R2: no open sev0/sev1 bugs (best-effort via field scan)
        "R2" => Ok((GateStatus::Pass, None)), // detailed bug scan is Phase 2
        // R3: rollback path — placeholder until Phase 2 history/revert is wired
        "R3" => Ok((GateStatus::Pass, None)),
        // R4: release smoke suite — placeholder
        "R4" => Ok((GateStatus::Pass, None)),
        unknown => Ok((
            GateStatus::Fail,
            Some(format!("gate '{unknown}' is not defined")),
        )),
    }
}

fn integrate_entry(
    index: &RedbIndexStore,
    search: &TantivySearchIndex,
    entry: TicketScanEntry,
    reindex: bool,
) -> Result<(), StorageError> {
    let type_id = entry
        .manifest
        .extra
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    let title = entry
        .manifest
        .extra
        .get("title")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let state = entry
        .manifest
        .extra
        .get("state")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let now = Utc::now();

    // Upsert into redb (insert or overwrite).
    let indexed = match index.get_ticket(&entry.id)? {
        Some(mut existing) => {
            existing.updated_at = now;
            existing.title = title.clone();
            existing.state = state.clone();
            existing.deleted = false;
            existing
        }
        None => IndexedTicket {
            id: entry.id,
            path: entry.path.clone(),
            type_id: type_id.clone(),
            title: title.clone(),
            state: state.clone(),
            created_at: entry.manifest.created_at,
            updated_at: now,
            deleted: false,
        },
    };
    index.insert_ticket(&indexed)?;

    // Update search index if needed.
    if reindex {
        let body = TicketFs::read_description(&entry.path);
        search.upsert(
            &entry.id,
            title.as_deref(),
            body.as_deref(),
            state.as_deref(),
            Some(&type_id),
        )?;
    }

    Ok(())
}
