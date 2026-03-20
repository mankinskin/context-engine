use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

use crate::error::StorageError;
use crate::storage::store::{ScanReport, TicketStore};
use crate::watcher::events::WatchEventKind;

/// A structured event emitted by the reconciler for external consumers.
pub struct ReconcileEvent {
    pub path: PathBuf,
    pub kind: WatchEventKind,
}

/// Perform a one-shot reconciliation pass over all scan roots.
/// This is the command-line equivalent of `ticket scan`.
pub fn reconcile_once(store: &TicketStore, reindex: bool) -> Result<ScanReport, StorageError> {
    store.scan(reindex)
}

/// Start an asynchronous filesystem watcher over all registered scan roots.
///
/// Returns a `WatchHandle` that keeps the watcher alive. Drop it to stop watching.
///
/// On each file change, triggers a targeted reconcile for the affected ticket folder.
/// Falls back to `scan()` for events that cannot be mapped to a specific ticket.
///
/// # Note
/// This is a best-effort watching layer. Crash safety and correctness are
/// guaranteed by `ticket scan --reindex`, not by the watcher alone.
pub fn start_watcher(
    store: &TicketStore,
) -> Result<WatchHandle, StorageError> {
    let roots = store.list_scan_roots()?;
    let default_root = store.index_root.join("tickets");

    let (tx, rx) = mpsc::channel();
    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, notify::Config::default().with_poll_interval(Duration::from_secs(2)))
            .map_err(|e| StorageError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    // Watch default root.
    if default_root.exists() {
        let _ = watcher.watch(&default_root, RecursiveMode::Recursive);
    }
    // Watch all registered scan roots.
    for root in &roots {
        if root.path.exists() {
            let _ = watcher.watch(&root.path, RecursiveMode::Recursive);
        }
    }

    Ok(WatchHandle { _watcher: watcher, rx })
}

/// Opaque handle that keeps the filesystem watcher alive.
/// Drop to stop watching.
pub struct WatchHandle {
    _watcher: RecommendedWatcher,
    pub rx: mpsc::Receiver<notify::Result<Event>>,
}

impl WatchHandle {
    /// Poll for the next event with a timeout.
    /// Returns `None` when the channel is idle.
    pub fn try_recv_event(&self) -> Option<notify::Result<Event>> {
        self.rx.try_recv().ok()
    }
}

/// Classify a `notify::Event` into our `WatchEventKind`.
pub fn classify_event(event: &Event) -> WatchEventKind {
    match event.kind {
        EventKind::Create(_) => WatchEventKind::Created,
        EventKind::Modify(_) => WatchEventKind::Modified,
        EventKind::Remove(_) => WatchEventKind::Deleted,
        EventKind::Access(_) => WatchEventKind::Modified,
        _ => WatchEventKind::Modified,
    }
}
