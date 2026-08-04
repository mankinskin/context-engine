use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Registry {
    #[serde(rename = "artifact", default)]
    pub artifacts: Vec<Artifact>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Artifact {
    pub id: String,
    pub category: String,
    pub kind: ArtifactKind,
    pub path: String,
    #[serde(default)]
    pub bin: Option<String>,
    #[serde(default)]
    pub npm_script: Option<String>,
    // Not read yet: installation for vscode-extension artifacts is not wired up in this unit.
    #[serde(default)]
    #[allow(dead_code)]
    pub extension_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ArtifactKind {
    RustBinary,
    VscodeExtension,
}

const REGISTRY_RELATIVE_PATH: &str = "tools/install/artifacts.toml";
const REPO_ROOT_ENV_VAR: &str = "INSTALL_CTL_REPO_ROOT";

/// Resolve the repo root without relying on the process cwd: an explicit env
/// override wins, otherwise walk up from the running executable's own path.
///
/// This is the single repo-root resolver for install-ctl: `resolve_repo_root`
/// below derives the root from this path, and `config::Config::load` (ported
/// from viewer-ctl, which used to search for `viewer-ctl.toml` independently)
/// reuses the same root to find `<root>/viewer-ctl.toml`.
pub fn resolve_registry_path() -> Result<PathBuf, String> {
    if let Ok(root) = std::env::var(REPO_ROOT_ENV_VAR) {
        let candidate = PathBuf::from(&root).join(REGISTRY_RELATIVE_PATH);
        if candidate.is_file() {
            return Ok(candidate);
        }
        return Err(format!(
            "{REPO_ROOT_ENV_VAR}={root} does not contain {REGISTRY_RELATIVE_PATH}"
        ));
    }

    if let Ok(exe) = std::env::current_exe()
        && let Some(found) = find_upwards(&exe)
    {
        return Ok(found);
    }

    if let Ok(cwd) = std::env::current_dir()
        && let Some(found) = find_upwards(&cwd)
    {
        return Ok(found);
    }

    Err(format!(
        "could not locate {REGISTRY_RELATIVE_PATH} from the executable path, the current \
         directory, or {REPO_ROOT_ENV_VAR}"
    ))
}

fn find_upwards(start: &Path) -> Option<PathBuf> {
    let mut dir = if start.is_file() {
        start.parent()?
    } else {
        start
    };
    loop {
        let candidate = dir.join(REGISTRY_RELATIVE_PATH);
        if candidate.is_file() {
            return Some(candidate);
        }
        dir = dir.parent()?;
    }
}

pub fn load_registry() -> Result<Registry, String> {
    let path = resolve_registry_path()?;
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("failed to read {}: {e}", path.display()))?;
    toml::from_str(&text).map_err(|e| format!("failed to parse {}: {e}", path.display()))
}

/// Resolve the repo root as the parent of `tools/install/artifacts.toml`.
pub fn resolve_repo_root() -> Result<PathBuf, String> {
    let registry_path = resolve_registry_path()?;
    registry_path
        .ancestors()
        .nth(3)
        .map(Path::to_path_buf)
        .ok_or_else(|| format!("could not derive repo root from {}", registry_path.display()))
}
