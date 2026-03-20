use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use chrono::Utc;
use serde_json::Value;
use uuid::Uuid;

use crate::error::StorageError;
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
}

pub struct ScanReport {
    pub integrated: usize,
    pub diagnostics: Vec<crate::model::filesystem::ParseDiagnostic>,
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
