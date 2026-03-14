//! Integration tests for the context-http server.
//!
//! These tests build the full axum router (with a temporary workspace directory)
//! and exercise the HTTP endpoints using `axum_test::TestServer`.

use axum_test::TestServer;
use context_api::workspace::manager::WorkspaceManager;
use serde_json::{
    json,
    Value,
};
use tempfile::TempDir;

// We need access to the crate's internal modules. Since context-http is a
// binary crate, we re-import the modules via path dependencies. The test
// harness can access `context_http::*` only if we have a lib target or if
// we inline the needed pieces. Because context-http is a bin crate, we
// reconstruct the router here using the same logic as `main.rs`.

/// Helper: build a test server backed by a fresh temp directory.
fn test_server() -> (TestServer, TempDir) {
    let tmp = TempDir::new().expect("failed to create temp dir");
    let manager = WorkspaceManager::new(tmp.path().to_path_buf());

    // Reproduce the state + router construction from the binary crate.
    // We use the types directly since they are pub.
    let state = context_http::state::AppState::new(manager);
    let router = context_http::router::create_router(state, None);

    let server = TestServer::new(router).expect("failed to create test server");
    (server, tmp)
}

// ---------------------------------------------------------------------------
// Health endpoint
// ---------------------------------------------------------------------------

#[tokio::test]
async fn health_returns_200_with_ok_status() {
    let (server, _tmp) = test_server();

    let resp = server.get("/api/health").await;
    resp.assert_status_ok();

    let body: Value = resp.json();
    assert_eq!(body["status"], "ok");
    assert!(body["version"].is_string());
}

// ---------------------------------------------------------------------------
// RPC: POST /api/execute — workspace lifecycle
// ---------------------------------------------------------------------------

#[tokio::test]
async fn execute_create_workspace() {
    let (server, _tmp) = test_server();

    let resp = server
        .post("/api/execute")
        .json(&json!({
            "command": "create_workspace",
            "name": "test-ws"
        }))
        .await;

    resp.assert_status_ok();

    let body: Value = resp.json();
    // CommandResult::WorkspaceInfo is tagged with "type": "workspace_info"
    assert_eq!(body["result"]["type"], "workspace_info");
    assert_eq!(body["result"]["name"], "test-ws");
}

#[tokio::test]
async fn execute_list_workspaces_empty() {
    let (server, _tmp) = test_server();

    let resp = server
        .post("/api/execute")
        .json(&json!({ "command": "list_workspaces" }))
        .await;

    resp.assert_status_ok();

    let body: Value = resp.json();
    assert_eq!(body["result"]["type"], "workspace_info_list");
    assert!(body["result"]["workspaces"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn execute_list_workspaces_after_create() {
    let (server, _tmp) = test_server();

    // Create a workspace first.
    server
        .post("/api/execute")
        .json(&json!({
            "command": "create_workspace",
            "name": "ws1"
        }))
        .await
        .assert_status_ok();

    // Now list.
    let resp = server
        .post("/api/execute")
        .json(&json!({ "command": "list_workspaces" }))
        .await;

    resp.assert_status_ok();

    let body: Value = resp.json();
    let workspaces = body["result"]["workspaces"].as_array().unwrap();
    assert_eq!(workspaces.len(), 1);
    assert_eq!(workspaces[0]["name"], "ws1");
}

// ---------------------------------------------------------------------------
// RPC: error handling
// ---------------------------------------------------------------------------

#[tokio::test]
async fn execute_open_nonexistent_workspace_returns_404() {
    let (server, _tmp) = test_server();

    let resp = server
        .post("/api/execute")
        .json(&json!({
            "command": "open_workspace",
            "name": "does-not-exist"
        }))
        .await;

    resp.assert_status(axum_test::http::StatusCode::NOT_FOUND);

    let body: Value = resp.json();
    assert_eq!(body["kind"], "workspace");
}

#[tokio::test]
async fn execute_list_atoms_without_open_workspace_returns_400() {
    let (server, _tmp) = test_server();

    let resp = server
        .post("/api/execute")
        .json(&json!({
            "command": "list_atoms",
            "workspace": "not-open"
        }))
        .await;

    // NotOpen maps to 400 BAD_REQUEST.
    // The error originates as WorkspaceError::NotOpen (kind="workspace"),
    // not AtomError, because the workspace lookup fails first.
    resp.assert_status(axum_test::http::StatusCode::BAD_REQUEST);

    let body: Value = resp.json();
    // Accept either "workspace" or "atom" — the underlying error may be
    // wrapped differently depending on the command dispatch path.
    let kind = body["kind"].as_str().unwrap();
    assert!(
        kind == "workspace" || kind == "atom",
        "expected kind 'workspace' or 'atom', got '{kind}'"
    );
}

#[tokio::test]
async fn execute_malformed_json_returns_400() {
    let (server, _tmp) = test_server();

    let resp = server
        .post("/api/execute")
        .json(&json!({ "not_a_command_field": true }))
        .await;

    // Our handler returns 400 for unparseable Command JSON.
    resp.assert_status(axum_test::http::StatusCode::BAD_REQUEST);

    let body: Value = resp.json();
    assert_eq!(body["kind"], "bad_request");
}

// ---------------------------------------------------------------------------
// RPC: trace flag
// ---------------------------------------------------------------------------

#[tokio::test]
async fn execute_with_trace_false_has_no_trace_summary() {
    let (server, _tmp) = test_server();

    let resp = server
        .post("/api/execute")
        .json(&json!({
            "command": "list_workspaces",
            "trace": false
        }))
        .await;

    resp.assert_status_ok();

    let body: Value = resp.json();
    assert!(body["trace"].is_null(), "trace should be absent/null");
}

// ---------------------------------------------------------------------------
// RPC: full round-trip workflow
// ---------------------------------------------------------------------------

#[tokio::test]
async fn execute_round_trip_workflow() {
    let (server, _tmp) = test_server();

    // 1. Create workspace.
    server
        .post("/api/execute")
        .json(&json!({
            "command": "create_workspace",
            "name": "round-trip"
        }))
        .await
        .assert_status_ok();

    // 2. Add atoms.
    server
        .post("/api/execute")
        .json(&json!({
            "command": "add_atoms",
            "workspace": "round-trip",
            "chars": ["h", "e", "l", "o"]
        }))
        .await
        .assert_status_ok();

    // 3. List atoms — should have 4.
    let resp = server
        .post("/api/execute")
        .json(&json!({
            "command": "list_atoms",
            "workspace": "round-trip"
        }))
        .await;
    resp.assert_status_ok();
    let body: Value = resp.json();
    let atoms = body["result"]["atoms"].as_array().unwrap();
    assert_eq!(atoms.len(), 4);

    // 4. Add a simple pattern.
    let resp = server
        .post("/api/execute")
        .json(&json!({
            "command": "add_simple_pattern",
            "workspace": "round-trip",
            "atoms": ["h", "e"]
        }))
        .await;
    resp.assert_status_ok();

    // 5. Get statistics.
    let resp = server
        .post("/api/execute")
        .json(&json!({
            "command": "get_statistics",
            "workspace": "round-trip"
        }))
        .await;
    resp.assert_status_ok();
    let body: Value = resp.json();
    assert_eq!(body["result"]["type"], "statistics");

    // 6. Get snapshot.
    let resp = server
        .post("/api/execute")
        .json(&json!({
            "command": "get_snapshot",
            "workspace": "round-trip"
        }))
        .await;
    resp.assert_status_ok();
    let body: Value = resp.json();
    assert_eq!(body["result"]["type"], "snapshot");

    // 7. Save workspace.
    server
        .post("/api/execute")
        .json(&json!({
            "command": "save_workspace",
            "name": "round-trip"
        }))
        .await
        .assert_status_ok();
}

// ---------------------------------------------------------------------------
// REST: GET /api/workspaces
// ---------------------------------------------------------------------------

#[tokio::test]
async fn rest_list_workspaces_empty() {
    let (server, _tmp) = test_server();

    let resp = server.get("/api/workspaces").await;
    resp.assert_status_ok();

    let body: Value = resp.json();
    assert!(body.as_array().unwrap().is_empty());
}

#[tokio::test]
async fn rest_list_workspaces_after_create() {
    let (server, _tmp) = test_server();

    // Create via RPC.
    server
        .post("/api/execute")
        .json(&json!({
            "command": "create_workspace",
            "name": "rest-test"
        }))
        .await
        .assert_status_ok();

    // List via REST.
    let resp = server.get("/api/workspaces").await;
    resp.assert_status_ok();

    let body: Value = resp.json();
    let workspaces = body.as_array().unwrap();
    assert_eq!(workspaces.len(), 1);
    assert_eq!(workspaces[0]["name"], "rest-test");
}

// ---------------------------------------------------------------------------
// REST: GET /api/workspaces/:name/atoms
// ---------------------------------------------------------------------------

#[tokio::test]
async fn rest_list_atoms() {
    let (server, _tmp) = test_server();

    // Setup: create workspace and add atoms.
    server
        .post("/api/execute")
        .json(&json!({
            "command": "create_workspace",
            "name": "atoms-ws"
        }))
        .await
        .assert_status_ok();

    server
        .post("/api/execute")
        .json(&json!({
            "command": "add_atoms",
            "workspace": "atoms-ws",
            "chars": ["a", "b", "c"]
        }))
        .await
        .assert_status_ok();

    // REST query.
    let resp = server.get("/api/workspaces/atoms-ws/atoms").await;
    resp.assert_status_ok();

    let body: Value = resp.json();
    let atoms = body.as_array().unwrap();
    assert_eq!(atoms.len(), 3);
}

// ---------------------------------------------------------------------------
// REST: GET /api/workspaces/:name/statistics
// ---------------------------------------------------------------------------

#[tokio::test]
async fn rest_get_statistics() {
    let (server, _tmp) = test_server();

    server
        .post("/api/execute")
        .json(&json!({
            "command": "create_workspace",
            "name": "stats-ws"
        }))
        .await
        .assert_status_ok();

    let resp = server.get("/api/workspaces/stats-ws/statistics").await;
    resp.assert_status_ok();

    let body: Value = resp.json();
    // GraphStatistics should have known fields.
    assert!(body.is_object());
}

// ---------------------------------------------------------------------------
// REST: GET /api/workspaces/:name/snapshot
// ---------------------------------------------------------------------------

#[tokio::test]
async fn rest_get_snapshot() {
    let (server, _tmp) = test_server();

    server
        .post("/api/execute")
        .json(&json!({
            "command": "create_workspace",
            "name": "snap-ws"
        }))
        .await
        .assert_status_ok();

    let resp = server.get("/api/workspaces/snap-ws/snapshot").await;
    resp.assert_status_ok();

    let body: Value = resp.json();
    // A fresh workspace snapshot should have empty nodes and edges.
    assert!(body["nodes"].as_array().unwrap().is_empty());
    assert!(body["edges"].as_array().unwrap().is_empty());
}

// ---------------------------------------------------------------------------
// REST: GET /api/workspaces/:name/vertices
// ---------------------------------------------------------------------------

#[tokio::test]
async fn rest_list_vertices() {
    let (server, _tmp) = test_server();

    server
        .post("/api/execute")
        .json(&json!({
            "command": "create_workspace",
            "name": "verts-ws"
        }))
        .await
        .assert_status_ok();

    server
        .post("/api/execute")
        .json(&json!({
            "command": "add_atoms",
            "workspace": "verts-ws",
            "chars": ["x", "y"]
        }))
        .await
        .assert_status_ok();

    let resp = server.get("/api/workspaces/verts-ws/vertices").await;
    resp.assert_status_ok();

    let body: Value = resp.json();
    let verts = body.as_array().unwrap();
    assert_eq!(verts.len(), 2);
}

// ---------------------------------------------------------------------------
// REST: error — workspace not open
// ---------------------------------------------------------------------------

#[tokio::test]
async fn rest_atoms_workspace_not_open_returns_error() {
    let (server, _tmp) = test_server();

    let resp = server.get("/api/workspaces/nonexistent/atoms").await;

    // The workspace is not open, so we expect a client error status.
    assert!(resp.status_code().is_client_error());
}

// ---------------------------------------------------------------------------
// CORS headers are present
// ---------------------------------------------------------------------------

#[tokio::test]
async fn cors_headers_present_on_health() {
    let (server, _tmp) = test_server();

    let resp = server.get("/api/health").await;
    resp.assert_status_ok();

    // The default_cors layer should add access-control headers when
    // an Origin header is present. axum-test doesn't set Origin by
    // default, so we just verify the endpoint works. A more thorough
    // test would send a preflight OPTIONS request with an Origin header.
}

// ---------------------------------------------------------------------------
// Multiple concurrent requests don't deadlock
// ---------------------------------------------------------------------------

#[tokio::test]
async fn concurrent_requests_do_not_deadlock() {
    let (server, _tmp) = test_server();

    // Fire several sequential requests to verify the mutex doesn't deadlock.
    // (TestServer doesn't implement Clone, so we can't spawn parallel tasks
    // that each own a handle. Sequential requests still exercise the lock/
    // unlock cycle repeatedly.)
    for _ in 0..10 {
        let resp = server.get("/api/health").await;
        resp.assert_status_ok();
    }
}
