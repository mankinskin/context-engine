//! Source file viewing and resolution.

use tracing::{
    debug,
    error,
    info,
    instrument,
    warn,
};
use viewer_api::{
    axum::{
        extract::{
            Path,
            Query,
            State,
        },
        http::{
            HeaderMap,
            StatusCode,
        },
        response::Json,
    },
    source::{
        extract_snippet,
    },
};

// Re-export shared utilities so crate-level code can access them
pub use viewer_api::source::{detect_language, resolve_source_path};

use crate::{
    handlers::to_unix_path,
    state::{
        get_session_config,
        increment_source_count,
        AppState,
    },
    types::{
        ErrorResponse,
        SourceQuery,
    },
};

/// Get full source file content or snippet around a line
#[instrument(skip(state, headers), fields(workspace_root = %to_unix_path(&state.workspace_root)))]
pub async fn get_source(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(path): Path<String>,
    Query(query): Query<SourceQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // Get session config for conditional logging
    let session = get_session_config(&state.sessions, &headers);
    let verbose = session.as_ref().map(|s| s.verbose).unwrap_or(false);

    // Track request count for this session
    let request_count = session
        .as_ref()
        .map(|s| increment_source_count(&state.sessions, &s.session_id));

    debug!(path = %path, line = ?query.line, context = query.context, "Getting source file");

    let full_path =
        resolve_source_path(&state.workspace_root, &path).map_err(|e| {
            warn!(error = %e, path = %path, "Invalid source path");
            (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: e }))
        })?;

    debug!(full_path = %to_unix_path(&full_path), "Resolved source path");

    let content = std::fs::read_to_string(&full_path).map_err(|e| {
        error!(error = %e, path = %to_unix_path(&full_path), "Failed to read source file");
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Failed to read source file: {}", e),
            }),
        )
    })?;

    let language = detect_language(&path);

    // If line is specified, return a snippet
    if let Some(line) = query.line {
        let (snippet_content, start_line, end_line) =
            extract_snippet(&content, line, query.context);
        let line = line.min(content.lines().count()).max(1);

        // Only log if verbose or first request in session
        if verbose || request_count == Some(1) {
            info!(
                path = %path,
                line = line,
                start = start_line,
                end = end_line,
                language = %language,
                session_request = ?request_count,
                "Returning source snippet"
            );
        } else {
            debug!(
                path = %path,
                line = line,
                start = start_line,
                end = end_line,
                language = %language,
                session_request = ?request_count,
                "Returning source snippet"
            );
        }

        Ok(Json(serde_json::json!({
            "path": path,
            "content": snippet_content,
            "start_line": start_line,
            "end_line": end_line,
            "highlight_line": line,
            "language": language
        })))
    } else {
        // Return full file
        let total_lines = content.lines().count();

        // Only log if verbose or first request in session
        if verbose || request_count == Some(1) {
            info!(
                path = %path,
                total_lines = total_lines,
                language = %language,
                session_request = ?request_count,
                "Returning full source file"
            );
        } else {
            debug!(
                path = %path,
                total_lines = total_lines,
                language = %language,
                session_request = ?request_count,
                "Returning source file"
            );
        }

        Ok(Json(serde_json::json!({
            "path": path,
            "content": content,
            "language": language,
            "total_lines": total_lines
        })))
    }
}
