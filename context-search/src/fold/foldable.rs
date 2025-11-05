use context_trace::*;
use derive_new::new;
use tracing::debug;

use crate::{
    cursor::{
        PatternCursor,
        PatternPrefixCursor,
        ToCursor,
    },
    fold::{
        FoldCtx,
        StartFoldPath,
    },
    r#match::iterator::MatchIterator,
    state::{
        end::EndState,
        result::Response,
        start::StartCtx,
    },
    traversal::TraversalKind,
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
impl StartFold for PatternCursor {
    fn start_fold<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        let location = self.path.start_location(&trav);
        let start = StartCtx {
            location,
            cursor: self,
        };

        match start.get_parent_batch::<K>(&trav) {
            Ok(p) => {
                debug!("First ParentBatch {:?}", p);
                Ok(FoldCtx {
                    start_index: start.location.parent,
                    last_match: EndState::init_fold(start),
                    matches: MatchIterator::start_parent(
                        trav,
                        start.location.parent,
                        p,
                    ),
                })
            },
            Err(err) => Err(err),
        }
    }
}

impl StartFold for PatternPrefixCursor {
    fn start_fold<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        PatternCursor::from(self).start_fold(trav)
    }
}
