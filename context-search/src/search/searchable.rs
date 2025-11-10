use context_trace::{
    logging::format_utils::pretty,
    *,
};
use derive_new::new;

use crate::{
    search::FoldCtx,
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

pub trait Searchable: Sized {
    fn to_fold_ctx<K: TraversalKind>(self) -> FoldCtx<K>;

    #[instrument(skip(self))]
    fn search<K: TraversalKind>(self) -> Response {
        debug!("Searchable::search - creating fold context");
        let fold_ctx = self.to_fold_ctx::<K>();
        debug!("Fold context created, starting search");
        fold_ctx.search()
    }
}
