//! Sandboxed integration tests — validation and release protocol chain.
//!
//! These tests drive the full `validate_start` → `validate_result` →
//! `release_candidate_create` → `release_gate_check` → `release_promote`
//! command sequence through the real `ticket exec` CLI endpoint, asserting
//! the state machine transitions described in `VALIDATION_RELEASE_GOVERNANCE.md`.
//!
//! Every test gets a fresh isolated `Sandbox`; no shared state between runs.

mod common;

use common::{
    Sandbox, advance_to_review, create_ticket, run_validate_pass, run_validate_start,
};

// ---------------------------------------------------------------------------
// Happy path — full validate → release chain
// ---------------------------------------------------------------------------

#[test]
fn validate_release_full_chain_via_exec() {
    let s = Sandbox::new();

    // ── Setup ──────────────────────────────────────────────────────────────
    let id = create_ticket(&s, "Feature: secure API tokens");
    advance_to_review(&s, &id); // open → in-progress → review

    // ── validate_start: review → validating ───────────────────────────────
    let vs = run_validate_start(&s, &id, "assign-001", "validator-agent");
    assert_eq!(vs["status"], "ok");
    assert_eq!(
        vs["ticket"]["state"],
        "validating",
        "validate_start must move ticket to 'validating'"
    );
    assert_eq!(vs["ticket"]["validation_status"], "in-progress");
    assert_eq!(vs["ticket"]["validator_id"], "validator-agent");

    // ── validate_result (pass): validating → validated ────────────────────
    let vr = run_validate_pass(&s, &id, "assign-001", "validator-agent");
    assert_eq!(vr["status"], "ok");
    assert_eq!(vr["state"], "validated");
    assert_eq!(vr["validation_status"], "passed");
    assert!(vr["passed"].as_bool().unwrap());

    // ── release_candidate_create: validated → release-candidate ───────────
    let rc = s.ticket_exec(&format!(
        r#"{{"command":"release_candidate_create","ticket_id":"{id}","release_target":"v2.0","assignment_chain":["assign-001"]}}"#
    ));
    assert_eq!(rc["status"], "ok");
    assert_eq!(rc["ticket"]["state"], "release-candidate");
    assert_eq!(rc["ticket"]["release_target"], "v2.0");

    // ── release_gate_check: all gates must pass ────────────────────────────
    let gk = s.ticket_exec(
        r#"{"command":"release_gate_check","release_target":"v2.0","required_gates":["R1","R2","R3","R4"]}"#,
    );
    assert_eq!(gk["status"], "ok");
    assert!(
        gk["all_gates_pass"].as_bool().unwrap(),
        "all gates should pass: {gk}"
    );

    // ── release_promote: release-candidate → released ─────────────────────
    let rp = s.ticket_exec(
        r#"{"command":"release_promote","release_target":"v2.0","release_version":"2.0.0","merge_commit":"deadbeef123","required_gates":["R1","R2","R3","R4"]}"#,
    );
    assert_eq!(rp["status"], "ok");
    assert_eq!(rp["release_version"], "2.0.0");
    assert_eq!(rp["promoted_ticket_count"].as_u64().unwrap(), 1);
    assert_eq!(rp["monitoring_state"], "active");

    // ── Verify final persisted state via CLI get ──────────────────────────
    let got = s.ticket_json(&["get", "--id", &id]);
    assert_eq!(got["ticket"]["fields"]["state"], "released");
    assert_eq!(got["ticket"]["fields"]["release_version"], "2.0.0");
    assert_eq!(got["ticket"]["fields"]["merge_commit"], "deadbeef123");
}

// ---------------------------------------------------------------------------
// validate_start — guard failures
// ---------------------------------------------------------------------------

#[test]
fn validate_start_fails_when_ticket_not_in_review() {
    let s = Sandbox::new();
    let id = create_ticket(&s, "Feature: not yet in review");
    // Ticket is in 'open' state — validate_start requires 'review'.

    let stderr = s.ticket_exec_fail(&format!(
        r#"{{"command":"validate_start","ticket_id":"{id}","assignment_id":"assign-1","validator_id":"validator-agent"}}"#
    ));
    assert!(
        stderr.contains("validate.invalid_state")
            || stderr.contains("review")
            || stderr.contains("open"),
        "expected a state-mismatch error, got: {stderr}"
    );
}

#[test]
fn validate_start_fails_when_validator_equals_worker() {
    let s = Sandbox::new();
    let id = create_ticket(&s, "Feature: separation of duties check");

    // Record working_by = "same-agent-id" on the ticket, then advance to review.
    s.ticket_json(&[
        "update",
        "--id",
        &id,
        "--field",
        "working_by=same-agent-id",
        "--to-state",
        "in-progress",
    ]);
    s.ticket_json(&["update", "--id", &id, "--to-state", "review"]);

    // The validator has the same identity as the worker — must be rejected.
    let stderr = s.ticket_exec_fail(&format!(
        r#"{{"command":"validate_start","ticket_id":"{id}","assignment_id":"assign-1","validator_id":"same-agent-id"}}"#
    ));
    assert!(
        stderr.contains("validate.same_identity")
            || stderr.contains("same")
            || stderr.contains("identity"),
        "expected same-identity separation error, got: {stderr}"
    );
}

// ---------------------------------------------------------------------------
// validate_result — guard failures
// ---------------------------------------------------------------------------

#[test]
fn validate_result_fails_for_empty_evidence_refs() {
    let s = Sandbox::new();
    let id = create_ticket(&s, "Feature: requires evidence");
    advance_to_review(&s, &id);
    run_validate_start(&s, &id, "assign-1", "validator-agent");
    // Ticket is now in 'validating' state.

    let stderr = s.ticket_exec_fail(&format!(
        r#"{{"command":"validate_result","ticket_id":"{id}","assignment_id":"assign-1","validator_id":"validator-agent","result":"passed","evidence_refs":[]}}"#
    ));
    assert!(
        stderr.contains("validate.missing_evidence") || stderr.contains("evidence"),
        "expected missing-evidence error, got: {stderr}"
    );
}

#[test]
fn validate_result_fails_when_wrong_validator_submits() {
    let s = Sandbox::new();
    let id = create_ticket(&s, "Feature: validator mismatch check");
    advance_to_review(&s, &id);
    run_validate_start(&s, &id, "assign-1", "validator-agent");
    // 'validator-agent' was recorded; submit result as a different agent.

    let stderr = s.ticket_exec_fail(&format!(
        r#"{{"command":"validate_result","ticket_id":"{id}","assignment_id":"assign-1","validator_id":"different-agent","result":"passed","evidence_refs":["test-001"]}}"#
    ));
    assert!(
        stderr.contains("validate.assignment_mismatch") || stderr.contains("mismatch"),
        "expected assignment-mismatch error, got: {stderr}"
    );
}

#[test]
fn validate_result_fail_returns_ticket_to_review() {
    let s = Sandbox::new();
    let id = create_ticket(&s, "Feature: intentionally fails validation");
    advance_to_review(&s, &id);
    run_validate_start(&s, &id, "assign-1", "validator-agent");

    // Submit a failing result.
    let vr = s.ticket_exec(&format!(
        r#"{{"command":"validate_result","ticket_id":"{id}","assignment_id":"assign-1","validator_id":"validator-agent","result":"failed","evidence_refs":["test-failure-log-001"]}}"#
    ));
    assert_eq!(vr["status"], "ok");
    assert_eq!(
        vr["state"], "review",
        "a failed validation must return the ticket to 'review'"
    );
    assert!(!vr["passed"].as_bool().unwrap());
    assert_eq!(vr["validation_status"], "failed");

    // Persist check: ticket must really be back in 'review'.
    let got = s.ticket_json(&["get", "--id", &id]);
    assert_eq!(got["ticket"]["fields"]["state"], "review");
    assert_eq!(got["ticket"]["fields"]["validation_status"], "failed");
}

// ---------------------------------------------------------------------------
// release_candidate_create — guard failures
// ---------------------------------------------------------------------------

#[test]
fn release_candidate_create_fails_unless_validated() {
    let s = Sandbox::new();
    let id = create_ticket(&s, "Feature: not yet validated");
    advance_to_review(&s, &id);
    // Ticket is in 'review' — NOT yet 'validated'.

    let stderr = s.ticket_exec_fail(&format!(
        r#"{{"command":"release_candidate_create","ticket_id":"{id}","release_target":"v1.0","assignment_chain":["assign-1"]}}"#
    ));
    assert!(
        stderr.contains("release.invalid_state")
            || stderr.contains("validated")
            || stderr.contains("review"),
        "expected invalid-state error for release candidate, got: {stderr}"
    );
}

#[test]
fn release_candidate_create_fails_with_empty_assignment_chain() {
    let s = Sandbox::new();
    let id = create_ticket(&s, "Feature: missing assignment chain");
    advance_to_review(&s, &id);
    run_validate_start(&s, &id, "assign-1", "validator-agent");
    run_validate_pass(&s, &id, "assign-1", "validator-agent");
    // Ticket is now 'validated'.

    let stderr = s.ticket_exec_fail(&format!(
        r#"{{"command":"release_candidate_create","ticket_id":"{id}","release_target":"v1.0","assignment_chain":[]}}"#
    ));
    assert!(
        stderr.contains("release.assignment_chain_missing")
            || stderr.contains("assignment_chain")
            || stderr.contains("empty"),
        "expected assignment-chain-missing error, got: {stderr}"
    );
}

// ---------------------------------------------------------------------------
// release_promote — guard failures
// ---------------------------------------------------------------------------

#[test]
fn release_promote_fails_without_merge_commit() {
    let s = Sandbox::new();
    let id = create_ticket(&s, "Feature: forgotten merge commit");
    advance_to_review(&s, &id);
    run_validate_start(&s, &id, "assign-1", "validator-agent");
    run_validate_pass(&s, &id, "assign-1", "validator-agent");
    s.ticket_exec(&format!(
        r#"{{"command":"release_candidate_create","ticket_id":"{id}","release_target":"v1.0","assignment_chain":["assign-1"]}}"#
    ));

    // merge_commit is empty — must be rejected before gate evaluation.
    let stderr = s.ticket_exec_fail(
        r#"{"command":"release_promote","release_target":"v1.0","release_version":"1.0.0","merge_commit":""}"#,
    );
    assert!(
        stderr.contains("release.merge_metadata_missing")
            || stderr.contains("merge_commit")
            || stderr.contains("merge"),
        "expected merge-metadata-missing error, got: {stderr}"
    );
}
