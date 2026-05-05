use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::error::AuditError;

pub const CONFIG_FILE_NAME: &str = ".repo-qa.toml";

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RepoQaFileConfig {
    #[serde(default)]
    pub exclude_paths: Vec<String>,
}

impl RepoQaFileConfig {
    pub fn load(repo_root: &Path) -> Result<Self, AuditError> {
        let config_path = repo_root.join(CONFIG_FILE_NAME);
        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(config_path)?;
        let mut config: Self = toml::from_str(&content)?;
        config.exclude_paths = config
            .exclude_paths
            .into_iter()
            .map(|path| normalize_config_path(&path))
            .filter(|path| !path.is_empty())
            .collect();
        Ok(config)
    }
}

pub fn is_repo_relative_path_excluded(
    relative_path: &Path,
    exclude_paths: &[String],
) -> bool {
    let normalized = normalize_repo_relative_path(relative_path);
    if normalized.is_empty() {
        return false;
    }

    exclude_paths.iter().any(|excluded| {
        normalized == *excluded || normalized.starts_with(&format!("{excluded}/"))
    })
}

pub fn normalize_repo_relative_path(path: &Path) -> String {
    normalize_config_path(&path.to_string_lossy())
}

fn normalize_config_path(path: &str) -> String {
    path.replace('\\', "/")
        .trim()
        .trim_start_matches("./")
        .trim_matches('/')
        .to_string()
}