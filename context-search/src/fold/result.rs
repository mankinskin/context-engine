use context_trace::*;

use crate::traversal::state::end::{
    EndKind,
    EndState,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FinishedKind {
    Complete(Token),
    Incomplete(Box<EndState>),
}

impl From<EndState> for FinishedKind {
    fn from(state: EndState) -> Self {
        if let EndKind::Complete(c) = &state.kind {
            FinishedKind::Complete(*c) // cursor.path
        } else {
            FinishedKind::Incomplete(Box::new(state))
        }
    }
}
impl FinishedKind {
    pub(crate) fn unwrap_incomplete(self) -> Box<EndState> {
        self.expect_incomplete("Unable to unwrap incomplete FoundRange")
    }
    pub(crate) fn expect_incomplete(
        self,
        msg: &str,
    ) -> Box<EndState> {
        match self {
            Self::Incomplete(s) => s,
            _ => panic!("{}", msg),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteState {
    pub(crate) cache: TraceCache,
    pub root: IndexWithPath,
    pub(crate) start: Token,
}
impl TryFrom<FinishedState> for CompleteState {
    type Error = IncompleteState;
    fn try_from(value: FinishedState) -> Result<Self, Self::Error> {
        match value {
            FinishedState {
                kind: FinishedKind::Incomplete(end_state),
                cache,
                root,
                start,
            } => Err(IncompleteState {
                end_state: *end_state,
                cache,
                root,
                start,
            }),
            FinishedState {
                kind: FinishedKind::Complete(_),
                cache,
                root,
                start,
            } => Ok(CompleteState { cache, root, start }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncompleteState {
    pub end_state: EndState,
    pub cache: TraceCache,
    pub root: IndexWithPath,
    pub(crate) start: Token,
}
impl TryFrom<FinishedState> for IncompleteState {
    type Error = CompleteState;
    fn try_from(value: FinishedState) -> Result<Self, Self::Error> {
        match CompleteState::try_from(value) {
            Ok(x) => Err(x),
            Err(x) => Ok(x),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinishedState {
    pub kind: FinishedKind,
    pub(crate) cache: TraceCache,
    pub(crate) root: IndexWithPath,
    pub(crate) start: Token,
}
