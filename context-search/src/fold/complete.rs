use context_trace::*;
use std::fmt::Debug;

use crate::{
    state::end::EndKind,
    FinishedKind,
    FinishedState,
};

pub trait UnwrapComplete: Sized + Debug {
    fn as_complete(&self) -> Option<Token>;

    fn is_complete(&self) -> bool {
        self.as_complete().is_some()
    }
    #[track_caller]
    fn unwrap_complete(self) -> Token {
        self.as_complete().unwrap_or_else(|| {
            panic!("Unable to unwrap {:?} as complete.", self)
        })
    }
    #[track_caller]
    fn expect_complete(
        self,
        msg: &str,
    ) -> Token {
        self.as_complete().unwrap_or_else(|| {
            panic!("Unable to unwrap {:?} as complete: {}", self, msg)
        })
    }
}

impl UnwrapComplete for EndKind {
    fn as_complete(&self) -> Option<Token> {
        match self {
            Self::Complete(c) => Some(*c),
            _ => None,
        }
    }
}

impl UnwrapComplete for FinishedKind {
    /// returns token if reduced to single token
    fn as_complete(&self) -> Option<Token> {
        match self {
            Self::Complete(c) => Some(*c),
            _ => None,
        }
    }
    fn unwrap_complete(self) -> Token {
        self.expect_complete("Unable to unwrap complete FoundRange")
    }

    fn expect_complete(
        self,
        msg: &str,
    ) -> Token {
        match self {
            FinishedKind::Complete(c) => c,
            _ => panic!("{}", msg),
        }
    }
}

impl UnwrapComplete for FinishedState {
    /// returns token if reduced to single token
    fn as_complete(&self) -> Option<Token> {
        self.kind.as_complete()
    }
    #[track_caller]
    fn unwrap_complete(self) -> Token {
        self.kind.unwrap_complete()
    }

    #[track_caller]
    fn expect_complete(
        self,
        msg: &str,
    ) -> Token {
        self.kind.expect_complete(msg)
    }
}
