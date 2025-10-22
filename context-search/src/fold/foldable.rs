use context_trace::*;
use derive_new::new;

use crate::{
    cursor::{
        PatternCursor,
        PatternRangeCursor,
        ToCursor,
    },
    fold::{
        FoldCtx,
        IntoFoldCtx,
    },
    state::{
        result::Response,
        start::StartCtx,
    },
    traversal::{
        TraversalKind,
        TryIntoTraversalCtx,
    },
};
use std::fmt::Debug;

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

pub trait Foldable: Sized {
    fn to_fold_ctx<K: TraversalKind>(self) -> FoldCtx<K>;
    fn fold<K: TraversalKind>(self) -> Response {
        self.to_fold_ctx::<K>().fold()
    }
}
pub trait StartFold: Sized {
    fn start_fold<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState>;
    fn fold<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<Response, ErrorState> {
        self.start_fold::<K>(trav).map(|ctx| ctx.fold())
    }
}

impl<T: StartFold + Clone> StartFold for &T {
    fn start_fold<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        self.clone().start_fold(trav)
    }
}

impl<const N: usize> StartFold for &'_ [Token; N] {
    fn start_fold<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        PatternRangePath::from(self).start_fold::<K>(trav)
    }
}
impl StartFold for &'_ [Token] {
    fn start_fold<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        PatternRangePath::from(self).start_fold::<K>(trav)
    }
}
impl StartFold for Pattern {
    fn start_fold<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        PatternRangePath::from(self).start_fold::<K>(trav)
    }
}

impl StartFold for PatternEndPath {
    fn start_fold<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        self.to_range_path().to_cursor(&trav).start_fold::<K>(trav)
    }
}
impl StartFold for PatternRangePath {
    fn start_fold<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        self.to_range_path().to_cursor(&trav).start_fold::<K>(trav)
    }
}
impl StartFold for PatternRangeCursor {
    fn start_fold<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        PatternCursor::from(self).start_fold(trav)
    }
}

impl StartFold for PatternCursor {
    fn start_fold<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        let start_index = self.path.start_index(&trav);
        let tctx = StartCtx {
            index: start_index,
            cursor: self,
            trav,
        }
        .try_into_traversal_context()?;
        Ok(FoldCtx { start_index, tctx })
    }
}
