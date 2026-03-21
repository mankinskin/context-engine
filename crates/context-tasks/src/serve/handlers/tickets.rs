use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use uuid::Uuid;

use viewer_api::error::RequestIdExt;
use crate::serve::{error::storage_err, AppState};
use crate::storage::ticket_fs::TicketFs;

#[derive(Deserialize)]
pub struct WorkspaceParam {
    pub workspace: String,
    pub state: Option<String>,
    pub query: Option<String>,
    pub limit: Option<usize>,
    /// Pagination cursor — not yet implemented, accepted to keep the API forward-compatible.
    #[allow(dead_code)]
    pub cursor: Option<String>,
}

#[derive(Deserialize)]
pub struct TicketIdParam {
    pub workspace: String,
}

#[derive(Serialize)]
pub struct TicketSummary {
    pub id: String,
    #[serde(rename = "type")]
    pub type_id: String,
    pub title: Option<String>,
    pub state: Option<String>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub fields: BTreeMap<String, Value>,
}

#[derive(Serialize)]
pub struct TicketsResponse {
    pub request_id: String,
    pub workspace: String,
    pub items: Vec<TicketSummary>,
    pub next_cursor: Option<String>,
}

#[derive(Serialize)]
pub struct TicketDetailResponse {
    pub request_id: String,
    pub workspace: String,
    pub ticket: TicketDetail,
}

#[derive(Serialize)]
pub struct TicketDetail {
    pub id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub fields: BTreeMap<String, Value>,
}

pub async fn list_tickets(
    State(state): State<AppState>,
    Extension(rid): Extension<RequestIdExt>,
    Query(params): Query<WorkspaceParam>,
) -> Response {
    let store = match state.registry.get(&params.workspace) {
        Some(s) => s,
        None => {
            return viewer_api::error::ApiError::not_found("workspace", &rid.0)
                .into_response_with_status(StatusCode::NOT_FOUND);
        }
    };

    // Use query search if provided, otherwise plain list
    let tickets = if let Some(q) = &params.query {
        let limit = params.limit.unwrap_or(100).min(1000);
        match store.search_tickets(q, limit) {
            Ok(results) => results
                .into_iter()
                .map(|r| TicketSummary {
                    id: r.id.to_string(),
                    type_id: r.ticket_type.unwrap_or_default(),
                    title: r.title,
                    state: r.state,
                    updated_at: chrono::Utc::now(), // SearchResult has no updated_at
                    fields: BTreeMap::new(),
                })
                .collect(),
            Err(e) => return storage_err(e, &rid.0),
        }
    } else {
        let limit = params.limit.map(|l| l.min(1000));
        match store.list(params.state.as_deref(), None, limit) {
            Ok(items) => items
                .into_iter()
                .map(|t| TicketSummary {
                    id: t.id.to_string(),
                    type_id: t.type_id,
                    title: t.title,
                    state: t.state,
                    updated_at: t.updated_at,
                    fields: BTreeMap::new(),
                })
                .collect(),
            Err(e) => return storage_err(e, &rid.0),
        }
    };

    Json(TicketsResponse {
        request_id: rid.0,
        workspace: params.workspace,
        items: tickets,
        next_cursor: None, // cursor pagination deferred to later iteration
    })
    .into_response()
}

pub async fn get_ticket(
    State(state): State<AppState>,
    Extension(rid): Extension<RequestIdExt>,
    Path(id): Path<Uuid>,
    Query(params): Query<TicketIdParam>,
) -> Response {
    let store = match state.registry.get(&params.workspace) {
        Some(s) => s,
        None => {
            return viewer_api::error::ApiError::not_found("workspace", &rid.0)
                .into_response_with_status(StatusCode::NOT_FOUND);
        }
    };

    match store.get(&id) {
        Ok(manifest) => Json(TicketDetailResponse {
            request_id: rid.0,
            workspace: params.workspace,
            ticket: TicketDetail {
                id: manifest.id.to_string(),
                created_at: manifest.created_at,
                fields: manifest.extra.into_iter().map(|(k, v)| (k, v)).collect(),
            },
        })
        .into_response(),
        Err(e) => storage_err(e, &rid.0),
    }
}

#[derive(Serialize)]
pub struct TicketDescriptionResponse {
    pub request_id: String,
    pub workspace: String,
    pub id: String,
    pub description: Option<String>,
}

/// `GET /api/tickets/{id}/description?workspace=<name>`
///
/// Returns the raw Markdown content of `description.md` for a ticket, if it
/// exists.  Returns `{ "description": null }` when no description has been
/// written, rather than 404, so the UI can show a placeholder without special-
/// casing the status code.
pub async fn get_ticket_description(
    State(state): State<AppState>,
    Extension(rid): Extension<RequestIdExt>,
    Path(id): Path<Uuid>,
    Query(params): Query<TicketIdParam>,
) -> Response {
    let store = match state.registry.get(&params.workspace) {
        Some(s) => s,
        None => {
            return viewer_api::error::ApiError::not_found("workspace", &rid.0)
                .into_response_with_status(StatusCode::NOT_FOUND);
        }
    };

    let indexed = match store.get_indexed(&id) {
        Ok(Some(t)) => t,
        Ok(None) => {
            return viewer_api::error::ApiError::not_found("ticket", &rid.0)
                .into_response_with_status(StatusCode::NOT_FOUND);
        }
        Err(e) => return storage_err(e, &rid.0),
    };

    if indexed.deleted {
        return viewer_api::error::ApiError::not_found("ticket", &rid.0)
            .into_response_with_status(StatusCode::NOT_FOUND);
    }

    let description = TicketFs::read_description(&indexed.path);

    Json(TicketDescriptionResponse {
        request_id: rid.0,
        workspace: params.workspace,
        id: id.to_string(),
        description,
    })
    .into_response()
}
