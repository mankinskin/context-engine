//! Workspace registry: maps workspace names to `TicketStore` instances.

use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    storage::store::TicketStore,
    workspace::WorkspaceConfig,
};

/// A map from workspace name → lazily-opened `TicketStore`.
pub struct WorkspaceRegistry {
    /// name → filesystem path to the `.ticket/` index root.
    paths: HashMap<String, PathBuf>,
    /// Lazy-opened stores, keyed by name.
    stores: Mutex<HashMap<String, Arc<TicketStore>>>,
}

impl WorkspaceRegistry {
    /// Build from a `WorkspaceConfig` (reads `~/.ticket-workspaces.toml`).
    pub fn from_config(config: &WorkspaceConfig) -> Self {
        let paths = config
            .workspaces
            .iter()
            .map(|(name, path_str)| (name.clone(), PathBuf::from(path_str)))
            .collect();
        Self {
            paths,
            stores: Mutex::new(HashMap::new()),
        }
    }

    /// Build with a single pre-loaded workspace named `"default"`.
    pub fn single(path: PathBuf) -> Self {
        let mut paths = HashMap::new();
        paths.insert("default".into(), path);
        Self {
            paths,
            stores: Mutex::new(HashMap::new()),
        }
    }

    /// List workspace names.
    pub fn workspace_names(&self) -> Vec<String> {
        let mut names: Vec<_> = self.paths.keys().cloned().collect();
        names.sort();
        names
    }

    /// Return `true` if a workspace with the given name is registered.
    pub fn contains(&self, name: &str) -> bool {
        self.paths.contains_key(name)
    }

    /// Get or lazily open the `TicketStore` for `workspace`.
    ///
    /// Returns `None` if the workspace name is not registered.
    pub fn get(&self, workspace: &str) -> Option<Arc<TicketStore>> {
        let path = self.paths.get(workspace)?.clone();

        let mut stores = self.stores.lock().unwrap();
        if let Some(store) = stores.get(workspace) {
            return Some(Arc::clone(store));
        }

        // Lazy open
        match TicketStore::open(&path) {
            Ok(store) => {
                let arc = Arc::new(store);
                stores.insert(workspace.to_string(), Arc::clone(&arc));
                Some(arc)
            }
            Err(e) => {
                tracing::warn!(workspace, error = %e, "failed to open workspace store");
                None
            }
        }
    }
}
