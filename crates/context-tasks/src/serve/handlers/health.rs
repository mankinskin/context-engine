use axum::{extract::State, response::Json};
use serde::Serialize;

use crate::serve::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub auth_generation: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_last_reload_ts: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn healthz(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "ticket-serve",
        auth_generation: state.auth.generation(),
        auth_last_reload_ts: state.auth.last_reload_ts(),
    })
}
