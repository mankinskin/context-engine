use context_trace::*;
use derive_new::new;

use crate::{
    search::SearchState,
    state::result::Response,
    traversal::TraversalKind,
};
use std::fmt::Debug;
use tracing::{
    debug,
    instrument,
};

pub(crate) type FoldResult = Result<Response, ErrorState>;

#[derive(Debug, new)]
pub struct ErrorState {
    pub reason: ErrorReason,
    pub found: Option<Response>,
}
impl From<ErrorReason> for ErrorState {
    fn from(reason: ErrorReason) -> Self {
        Self {
            reason,
            found: None,
        }
    }
}
impl From<IndexWithPath> for ErrorState {
    fn from(value: IndexWithPath) -> Self {
        ErrorReason::SingleIndex(Box::new(value)).into()
    }
}
