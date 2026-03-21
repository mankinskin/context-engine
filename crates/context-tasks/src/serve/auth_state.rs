//! Hot-reloadable bearer token auth state for `ticket serve`.
//!
//! Implements atomic token-set swapping without interrupting active connections.
//! Security: raw tokens are never retained after validation; the `TokenSet` from
//! `viewer_api::auth` holds them only during the lifetime of the in-memory arc.
//!
//! **Audit log** entries are printed to tracing (not to SSE yet — SSE `DiagnosticWarning`
//! integration is wired by ticket `5e68c2e1`).

use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
};

use arc_swap::ArcSwap;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use viewer_api::auth::TokenSet;

/// Live-reloadable bearer token registry.
pub struct AuthState {
    current: ArcSwap<TokenSet>,
    generation: AtomicU64,
    last_reload_ts: Mutex<Option<DateTime<Utc>>>,
}

/// Where a token is sourced from (in precedence order).
#[derive(Debug, Clone)]
pub enum TokenSource {
    /// A literal token string supplied programmatically (e.g. tests).
    Literal(String),
    /// `TICKET_SERVE_TOKEN` environment variable.
    Env,
    /// A plain-text file where each non-empty line is a token.
    File(PathBuf),
}

#[derive(Debug, thiserror::Error)]
pub enum TokenLoadError {
    #[error("no token source configured")]
    NoSource,
    #[error("environment variable TICKET_SERVE_TOKEN not set")]
    EnvMissing,
    #[error("token file not found: {0}")]
    FileMissing(PathBuf),
    #[error("token file read error: {0}")]
    FileIo(#[from] std::io::Error),
    #[error("token is too short — minimum 16 characters required")]
    TooShort,
}

impl AuthState {
    /// Initialise from a source.  Fails if the source yields no valid tokens.
    pub fn from_source(source: TokenSource) -> Result<Self, TokenLoadError> {
        let set = load_token_set(&source)?;
        Ok(Self {
            current: ArcSwap::new(Arc::new(set)),
            generation: AtomicU64::new(1),
            last_reload_ts: Mutex::new(Some(Utc::now())),
        })
    }

    /// Convenience: load from `TICKET_SERVE_TOKEN` env var.
    pub fn from_env() -> Result<Self, TokenLoadError> {
        Self::from_source(TokenSource::Env)
    }

    /// Convenience: build from a literal token string (testing / CLI flag).
    pub fn from_literal(token: impl Into<String>) -> Result<Self, TokenLoadError> {
        Self::from_source(TokenSource::Literal(token.into()))
    }

    /// Return an `Arc<TokenSet>` snapshot of the current token set.
    ///
    /// This is used by `routes.rs` to provide a stable reference to the
    /// bearer-auth middleware via `from_fn_with_state`.  For hot-reload,
    /// regenerate the router or use a middleware that reads from the `AuthState`
    /// directly via `AppState`.
    pub fn token_set_arc(&self) -> Arc<TokenSet> {
        Arc::clone(&self.current.load_full())
    }

    /// Validate a raw bearer token against the current token set.
    pub fn validate(&self, raw: &str) -> bool {
        self.current.load().contains(raw)
    }

    /// Atomically replace the token set from `source`.
    ///
    /// On validation failure the previous set is retained.
    /// Returns the new generation number on success.
    pub fn reload(&self, source: &TokenSource) -> Result<u64, TokenLoadError> {
        match load_token_set(source) {
            Ok(new_set) => {
                self.current.store(Arc::new(new_set));
                let generation = self.generation.fetch_add(1, Ordering::SeqCst) + 1;
                *self.last_reload_ts.lock().unwrap() = Some(Utc::now());
                tracing::info!(generation, "auth.reload.success");
                Ok(generation)
            }
            Err(e) => {
                tracing::warn!(error = %e, "auth.reload.failed — retaining previous token set");
                Err(e)
            }
        }
    }

    /// Current generation counter (bumped on every successful reload).
    pub fn generation(&self) -> u64 {
        self.generation.load(Ordering::SeqCst)
    }

    /// Timestamp of the last successful reload (or initial load).
    pub fn last_reload_ts(&self) -> Option<DateTime<Utc>> {
        *self.last_reload_ts.lock().unwrap()
    }
}

fn load_token_set(source: &TokenSource) -> Result<TokenSet, TokenLoadError> {
    let raw: Vec<String> = match source {
        TokenSource::Literal(t) => {
            validate_token_str(t)?;
            vec![t.clone()]
        }
        TokenSource::Env => {
            let t = std::env::var("TICKET_SERVE_TOKEN")
                .map_err(|_| TokenLoadError::EnvMissing)?;
            validate_token_str(&t)?;
            vec![t]
        }
        TokenSource::File(path) => {
            if !path.exists() {
                return Err(TokenLoadError::FileMissing(path.clone()));
            }
            let contents = std::fs::read_to_string(path)?;
            let tokens: Vec<String> = contents
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty() && !l.starts_with('#'))
                .map(String::from)
                .collect();
            for t in &tokens {
                validate_token_str(t)?;
            }
            if tokens.is_empty() {
                return Err(TokenLoadError::NoSource);
            }
            tokens
        }
    };

    Ok(TokenSet::new(raw))
}

fn validate_token_str(token: &str) -> Result<(), TokenLoadError> {
    if token.len() < 16 {
        return Err(TokenLoadError::TooShort);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_accepts_correct_token() {
        let state = AuthState::from_literal("a-valid-token-here").unwrap();
        assert!(state.validate("a-valid-token-here"));
        assert!(!state.validate("wrong-token"));
    }

    #[test]
    fn short_token_is_rejected() {
        assert!(matches!(
            AuthState::from_literal("short"),
            Err(TokenLoadError::TooShort)
        ));
    }

    #[test]
    fn reload_bumps_generation() {
        let state = AuthState::from_literal("first-valid-token-ok").unwrap();
        assert_eq!(state.generation(), 1);
        // Reload with the same literal source
        let source = TokenSource::Literal("second-valid-token-ok".into());
        let new_gen = state.reload(&source).unwrap();
        assert_eq!(new_gen, 2);
        assert!(state.validate("second-valid-token-ok"));
        assert!(!state.validate("first-valid-token-ok"));
    }
}
