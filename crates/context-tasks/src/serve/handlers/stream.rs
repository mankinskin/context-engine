//! SSE stream stub — wired by ticket `5e68c2e1`.
//!
//! Returns a minimal keep-alive SSE stream. The real implementation
//! (HookEmitter → StreamBroker → SSE fan-out) is implemented in the SSE
//! pipeline ticket.

use axum::{
    extract::State,
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse,
    },
};
use futures_util::stream;
use std::convert::Infallible;

use crate::serve::AppState;

pub async fn stream_stub(
    State(_state): State<AppState>,
) -> impl IntoResponse {
    // Stub: single snapshot.ready event then keep-alive.
    // The full implementation is in ticket 5e68c2e1.
    let events = stream::once(async {
        Ok::<Event, Infallible>(
            Event::default()
                .event("snapshot.ready")
                .data(r#"{"workspace":"default","node_count":0,"edge_count":0}"#),
        )
    });

    Sse::new(events).keep_alive(KeepAlive::default())
}
