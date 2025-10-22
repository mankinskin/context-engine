use crate::{
    state::result::{
        BaseResponse,
        Response,
    },
    traversal::{
        IntoTraversalCtx,
        TraversalCtx,
        TraversalKind,
    },
    IncompleteState,
};
use context_trace::*;
use foldable::ErrorState;
use std::fmt::Debug;
use tracing::debug;

pub(crate) mod final_state;
pub(crate) mod foldable;

pub(crate) trait IntoFoldCtx<K: TraversalKind> {
    fn instart_fold(self) -> FoldCtx<K>;
}

impl<K: TraversalKind, S: IntoTraversalCtx<K> + ToToken> IntoFoldCtx<K> for S {
    fn instart_fold(self) -> FoldCtx<K> {
        let start_index = self.to_child();
        FoldCtx {
            tctx: self.into_traversal_context(),
            //max_width: start_index.width(),
            start_index,
        }
    }
}
/// context for running fold traversal
#[derive(Debug)]
pub struct FoldCtx<K: TraversalKind> {
    pub(crate) tctx: TraversalCtx<K>,
    pub(crate) start_index: Token,
}

impl<K: TraversalKind> Iterator for FoldCtx<K> {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        self.tctx.next()
    }
}

impl<K: TraversalKind> FoldCtx<K> {
    fn fold(mut self) -> Response {
        debug!("Starting fold {:#?}", self);

        (&mut self).for_each(|_| ());
        let end = self.tctx.last_match;
        let trace_ctx = &mut self.tctx.match_iter.0;
        end.trace(trace_ctx);
        let base = BaseResponse {
            cache: self.tctx.match_iter.0.cache,
            start: self.start_index,
        };

        Response::new(base, end)
    }
}
