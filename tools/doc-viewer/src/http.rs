//! HTTP server module for doc-viewer frontend.
//!
//! Provides REST API endpoints for browsing and reading documentation.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

use crate::schema::{DocType, ModuleTreeNode};
use crate::tools::{CrateDocsManager, DetailLevel, DocsManager, ListFilter};

/// Application state shared across HTTP handlers.
#[derive(Clone)]
pub struct HttpState {
    pub docs_manager: Arc<DocsManager>,
    pub crate_manager: Arc<CrateDocsManager>,
}

// === API Response Types ===

#[derive(Serialize)]
struct ApiError {
    error: String,
}

#[derive(Serialize)]
struct DocListResponse {
    total: usize,
    categories: Vec<CategoryResponse>,
}

#[derive(Serialize)]
struct CategoryResponse {
    category: String,
    count: usize,
    docs: Vec<DocSummaryResponse>,
}

#[derive(Serialize)]
struct DocSummaryResponse {
    filename: String,
    title: String,
    date: String,
    summary: String,
    tags: Vec<String>,
    status: Option<String>,
}

#[derive(Serialize)]
struct DocContentResponse {
    filename: String,
    doc_type: String,
    title: String,
    date: String,
    summary: String,
    tags: Vec<String>,
    status: Option<String>,
    body: Option<String>,
}

#[derive(Serialize)]
struct CrateListResponse {
    crates: Vec<CrateSummaryResponse>,
}

#[derive(Serialize)]
struct CrateSummaryResponse {
    name: String,
    version: Option<String>,
    description: String,
    module_count: usize,
    has_readme: bool,
}

#[derive(Serialize)]
struct CrateTreeResponse {
    name: String,
    description: String,
    children: Vec<ModuleNodeResponse>,
}

#[derive(Serialize)]
struct ModuleNodeResponse {
    name: String,
    path: String,
    description: String,
    has_readme: bool,
    children: Vec<ModuleNodeResponse>,
}

// === Query Parameters ===

#[derive(Deserialize)]
struct ListDocsQuery {
    doc_type: Option<String>,
    tag: Option<String>,
}

#[derive(Deserialize)]
struct ReadDocQuery {
    detail: Option<String>,
}

// === Router Creation ===

/// Create the HTTP router with all API endpoints.
pub fn create_router(state: HttpState, static_dir: Option<PathBuf>) -> Router {
    // CORS for development
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // API routes
    let api_routes = Router::new()
        .route("/docs", get(list_docs))
        .route("/docs/{filename}", get(read_doc))
        .route("/crates", get(list_crates))
        .route("/crates/{name}", get(browse_crate));

    // Main app
    let app = Router::new()
        .nest("/api", api_routes)
        .layer(cors)
        .with_state(state);

    // Add static file serving if directory provided
    if let Some(dir) = static_dir {
        app.fallback_service(ServeDir::new(dir))
    } else {
        app
    }
}

// === Handlers ===

/// GET /api/docs - List all documentation
async fn list_docs(
    State(state): State<HttpState>,
    Query(params): Query<ListDocsQuery>,
) -> Result<Json<DocListResponse>, (StatusCode, Json<ApiError>)> {
    let doc_types = match params.doc_type.as_deref() {
        Some(dt) => match parse_doc_type(dt) {
            Some(t) => vec![t],
            None => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ApiError {
                        error: format!("Invalid doc_type: {}", dt),
                    }),
                ))
            }
        },
        None => vec![
            DocType::Guide,
            DocType::Plan,
            DocType::Implemented,
            DocType::BugReport,
            DocType::Analysis,
        ],
    };

    let filter = ListFilter {
        tag: params.tag,
        status: None,
    };

    let mut categories = Vec::new();
    let mut total = 0;

    for dt in doc_types {
        match state.docs_manager.list_documents_filtered(dt, &filter) {
            Ok(docs) => {
                let count = docs.len();
                total += count;

                let category = CategoryResponse {
                    category: dt.directory().to_string(),
                    count,
                    docs: docs
                        .into_iter()
                        .map(|d| DocSummaryResponse {
                            filename: d.filename,
                            title: d.title,
                            date: d.date,
                            summary: d.summary,
                            tags: d.tags,
                            status: d.status.map(|s| s.to_string()),
                        })
                        .collect(),
                };
                categories.push(category);
            }
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiError {
                        error: e.to_string(),
                    }),
                ))
            }
        }
    }

    Ok(Json(DocListResponse { total, categories }))
}

/// GET /api/docs/:filename - Read a specific document
async fn read_doc(
    State(state): State<HttpState>,
    Path(filename): Path<String>,
    Query(params): Query<ReadDocQuery>,
) -> Result<Json<DocContentResponse>, (StatusCode, Json<ApiError>)> {
    let detail = match params.detail.as_deref() {
        Some("outline") => DetailLevel::Outline,
        Some("full") => DetailLevel::Full,
        _ => DetailLevel::Full, // Default to full for viewing
    };

    match state.docs_manager.read_document(&filename, detail) {
        Ok(result) => Ok(Json(DocContentResponse {
            filename: result.filename,
            doc_type: result.doc_type,
            title: result.title,
            date: result.date,
            summary: result.summary,
            tags: result.tags,
            status: result.status.map(|s| s.to_string()),
            body: result.body,
        })),
        Err(e) => {
            let status = if e.to_string().contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            Err((
                status,
                Json(ApiError {
                    error: e.to_string(),
                }),
            ))
        }
    }
}

/// GET /api/crates - List all documented crates
async fn list_crates(
    State(state): State<HttpState>,
) -> Result<Json<CrateListResponse>, (StatusCode, Json<ApiError>)> {
    match state.crate_manager.discover_crates_with_diagnostics() {
        Ok(result) => Ok(Json(CrateListResponse {
            crates: result
                .crates
                .into_iter()
                .map(|c| CrateSummaryResponse {
                    name: c.name,
                    version: c.version,
                    description: c.description,
                    module_count: c.module_count,
                    has_readme: c.has_readme,
                })
                .collect(),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError {
                error: e.to_string(),
            }),
        )),
    }
}

/// GET /api/crates/:name - Browse a specific crate's module tree
async fn browse_crate(
    State(state): State<HttpState>,
    Path(name): Path<String>,
) -> Result<Json<CrateTreeResponse>, (StatusCode, Json<ApiError>)> {
    match state.crate_manager.browse_crate(&name) {
        Ok(tree) => Ok(Json(CrateTreeResponse {
            name: tree.name.clone(),
            description: tree.description.clone(),
            children: tree.children.iter().map(convert_module_node).collect(),
        })),
        Err(e) => {
            let status = if e.to_string().contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            Err((
                status,
                Json(ApiError {
                    error: e.to_string(),
                }),
            ))
        }
    }
}

fn convert_module_node(node: &ModuleTreeNode) -> ModuleNodeResponse {
    ModuleNodeResponse {
        name: node.name.clone(),
        path: node.path.clone(),
        description: node.description.clone(),
        has_readme: node.has_readme,
        children: node.children.iter().map(convert_module_node).collect(),
    }
}

// === Helpers ===

fn parse_doc_type(s: &str) -> Option<DocType> {
    match s.to_lowercase().as_str() {
        "guide" | "guides" => Some(DocType::Guide),
        "plan" | "plans" => Some(DocType::Plan),
        "implemented" => Some(DocType::Implemented),
        "bug-report" | "bug-reports" | "bug_report" | "bugreport" => Some(DocType::BugReport),
        "analysis" => Some(DocType::Analysis),
        _ => None,
    }
}
