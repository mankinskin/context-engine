use context_trace::*;

use crate::{
    cursor::{
        PathCursor,
        PatternCursor,
    },
    state::end::{
        EndKind,
        EndReason,
        EndState,
    },
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BaseResponse {
    pub cache: TraceCache,
    pub(crate) start: Token,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Response {
    Complete(CompleteState),
    Incomplete(IncompleteState),
}

//impl From<EndState> for Response {
//    fn from(state: EndState) -> Self {
//        if let EndKind::Complete(c) = &state.kind {
//            Response::Complete(*c) // cursor.path
//        } else {
//            Response::Incomplete(Box::new(state))
//        }
//    }
//}
impl Response {
    pub(crate) fn new(
        base: BaseResponse,
        end: EndState,
    ) -> Self {
        match end.kind {
            EndKind::Complete(c) => Response::Complete(c),
            _ => Response::Incomplete(IncompleteState { base, end }),
        }
    }

    pub(crate) fn unwrap_incomplete(self) -> IncompleteState {
        self.expect_incomplete("Unable to unwrap incomplete FoundRange")
    }
    pub(crate) fn expect_incomplete(
        self,
        msg: &str,
    ) -> IncompleteState {
        match self {
            Self::Incomplete(s) => s,
            _ => panic!("{}", msg),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteState {
    pub(crate) base: BaseResponse,
    pub path: IndexRangePath,
}

impl TryFrom<Response> for CompleteState {
    type Error = IncompleteState;
    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::Complete(complete) => Ok(complete),
            Response::Incomplete(incomplete_state) => Err(incomplete_state),
        }
    }
}
impl CompleteState {
    pub fn new_token<Trav: HasGraph>(
        token: Token,
        trav: Trav,
    ) -> Self {
        let graph = trav.graph();
        let (pattern_id, _) =
            graph.expect_vertex(token).expect_any_child_pattern();
        Self::new_path(IndexRangePath::new_empty(IndexRoot::from(
            PatternLocation::new(token, *pattern_id),
        )))
    }
    pub fn new_root(
        value: PatternLocation,
        root: Pattern,
    ) -> Self {
        Self::new_pattern_path(value, PatternRangePath::from(root.clone()))
    }
    pub fn new_pattern_path(
        value: PatternLocation,
        path: PatternRangePath,
    ) -> Self {
        Self::new_path(IndexRangePath::new_path(value, path))
    }
    pub fn new_path(path: impl Into<IndexRangePath>) -> Self {
        let path = path.into();
        Self {
            base: BaseResponse {
                cache: TraceCache::default(),
                start: path.root_parent(),
            },
            path,
        }
    }
}
impl TargetKey for CompleteState {
    fn target_key(&self) -> DirectedKey {
        DirectedKey::up(
            self.path.root_parent(),
            self.path.root_parent().width(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncompleteState {
    pub(crate) base: BaseResponse,
    pub end: EndState,
}
impl TryFrom<Response> for IncompleteState {
    type Error = CompleteState;
    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match CompleteState::try_from(value) {
            Ok(x) => Err(x),
            Err(x) => Ok(x),
        }
    }
}
