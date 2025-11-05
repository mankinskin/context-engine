use context_trace::*;
use core::fmt;
use std::fmt::Debug;

use crate::{
    state::end::EndKind,
    Response,
};

pub trait UnwrapToken: Sized + Debug {
    fn as_token(&self) -> Option<Token>;

    fn is_token(&self) -> bool {
        self.as_token().is_some()
    }
    #[track_caller]
    fn unwrap_token(self) -> Token {
        self.as_token().unwrap_or_else(|| {
            panic!("Unable to unwrap {:?} as complete.", self)
        })
    }
    #[track_caller]
    fn expect_token(
        self,
        msg: &str,
    ) -> Token {
        self.as_token().unwrap_or_else(|| {
            panic!("Unable to unwrap {:?} as complete: {}", self, msg)
        })
    }
}
//impl<T: UnwrapComplete> UnwrapToken for T {
//    fn as_token(&self) -> Option<Token> {
//        self.as_complete().map(|c| c.path.root_parent())
//    }
//}
//pub trait UnwrapComplete: Sized + Debug {
//    fn as_complete(&self) -> Option<&CompleteState>;
//    fn to_complete(self) -> Option<CompleteState>;
//
//    fn is_complete(&self) -> bool {
//        self.as_complete().is_some()
//    }
//    #[track_caller]
//    fn unwrap_complete(self) -> CompleteState {
//        let fself = format!("{:?}", self);
//        self.to_complete().unwrap_or_else(|| {
//            panic!("Unable to unwrap {} as complete.", fself)
//        })
//    }
//    #[track_caller]
//    fn expect_complete(
//        self,
//        msg: &str,
//    ) -> CompleteState {
//        let fself = format!("{:?}", self);
//        self.to_complete().unwrap_or_else(|| {
//            panic!("Unable to unwrap {} as complete: {}", fself, msg)
//        })
//    }
//}

//impl UnwrapComplete for EndKind {
//    fn as_complete(&self) -> Option<&CompleteState> {
//        match self {
//            Self::Complete(c) => Some(c),
//            _ => None,
//        }
//    }
//    fn to_complete(self) -> Option<CompleteState> {
//        match self {
//            Self::Complete(c) => Some(c),
//            _ => None,
//        }
//    }
//}

//impl UnwrapComplete for Response {
//    /// returns token if reduced to single token
//    fn as_complete(&self) -> Option<&CompleteState> {
//        match self {
//            Self::Complete(c) => Some(c),
//            _ => None,
//        }
//    }
//    fn to_complete(self) -> Option<CompleteState> {
//        match self {
//            Self::Complete(c) => Some(c),
//            _ => None,
//        }
//    }
//    fn unwrap_complete(self) -> CompleteState {
//        self.expect_complete("Unable to unwrap complete Response")
//    }
//
//    fn expect_complete(
//        self,
//        msg: &str,
//    ) -> CompleteState {
//        match self {
//            Self::Complete(c) => c,
//            _ => panic!("{}", msg),
//        }
//    }
//}
