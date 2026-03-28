/// Application-level actions that combine store mutations with async API calls.
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::{api, store::Store};

/// Load the list of log files from the server and populate the store.
pub fn load_log_files(store: Store) {
    spawn_local(async move {
        store.is_loading.set(true);
        store.error.set(None);
        match api::list_log_files().await {
            Ok(files) => {
                store.status_message.set(format!("{} files found", files.len()));
                store.log_files.set(files);
            }
            Err(e) => {
                store.error.set(Some(e.clone()));
                store.status_message.set(format!("Error: {e}"));
            }
        }
        store.is_loading.set(false);
    });
}

/// Select a file and load its log entries.
pub fn select_file(filename: String) {
    let store = leptos::prelude::expect_context::<Store>();
    let name = filename.clone();
    store.current_file.set(Some(filename.clone()));
    store.is_loading.set(true);
    store.status_message.set(format!("Loading {name}…"));
    spawn_local(async move {
        match api::load_log_file(&filename).await {
            Ok(entries) => {
                store.status_message.set(format!("{} entries", entries.len()));
                store.set_entries(filename, entries);
            }
            Err(e) => {
                store.error.set(Some(e.clone()));
                store.status_message.set(format!("Error: {e}"));
            }
        }
        store.is_loading.set(false);
    });
}
