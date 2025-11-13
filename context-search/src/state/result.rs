use context_trace::*;

use crate::state::end::{
    EndState,
    PathEnum,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Response {
    pub cache: TraceCache,
    pub end: EndState,
}
//impl From<EndState> for Response {
//    fn from(state: EndState) -> Self {
//        if let PathEnum::Complete(c) = &state.kind {
//            Response::Complete(*c) // cursor.path
//        } else {
//            Response::Incomplete(Box::new(state))
//        }
//    }
//}

impl Response {
    pub(crate) fn new(
        //start: Token,
        cache: TraceCache,
        end: EndState,
    ) -> Self {
        Self {
            cache,
            //start,
            end,
        }
    }

    /// Check if the response is complete (search fully matched)
    pub fn is_complete(&self) -> bool {
        matches!(self.end.path, PathEnum::Complete(_))
    }

    /// Unwrap a complete response, panicking if incomplete
    pub fn unwrap_complete(self) -> IndexRangePath {
        self.expect_complete("Called unwrap_complete on incomplete Response")
    }

    /// Unwrap a complete response with a custom error message
    pub fn expect_complete(
        self,
        msg: &str,
    ) -> IndexRangePath {
        match self.end.path {
            PathEnum::Complete(path) => path,
            _ => panic!("{}", msg),
        }
    }

    /// Try to get the complete path if the response is complete
    pub fn as_complete(&self) -> Option<&IndexRangePath> {
        match &self.end.path {
            PathEnum::Complete(path) => Some(path),
            _ => None,
        }
    }

    /// Get the query pattern cursor from the response
    pub fn query_cursor(&self) -> &crate::cursor::PatternCursor {
        &self.end.cursor
    }

    /// Get the query pattern path from the response
    pub fn query_pattern(&self) -> &PatternRangePath {
        &self.end.cursor.path
    }

    /// Get the root token from the located path
    pub fn root_token(&self) -> Token {
        self.end.path.root_parent()
    }

    /// Get the cursor atom position
    pub fn cursor_position(&self) -> AtomPosition {
        self.end.cursor.atom_position
    }

    //pub(crate) fn unwrap_incomplete(self) -> Self {
    //    self.expect_incomplete("Unable to unwrap incomplete FoundRange")
    //}
    //pub(crate) fn expect_incomplete(
    //    self,
    //    msg: &str,
    //) -> IncompleteState {
    //    self.try_into().unwrap()
    //}

    //pub fn new_token<Trav: HasGraph>(
    //    token: Token,
    //    trav: Trav,
    //) -> Self {
    //    let graph = trav.graph();
    //    let (pattern_id, _) =
    //        graph.expect_vertex(token).expect_any_child_pattern();
    //    Self::new_path(IndexRangePath::new_empty(IndexRoot::from(
    //        PatternLocation::new(token, *pattern_id),
    //    )))
    //}
    //pub fn new_root(
    //    value: PatternLocation,
    //    root: Pattern,
    //) -> Self {
    //    let path = PatternRangePath::from(root.clone());
    //    let path = IndexRangePath::new_path(value, path);
    //    Self {
    //        cache: TraceCache::default(),
    //        start: path.root_parent(),
    //        path,
    //        end: EndState {
    //            reason: EndReason::QueryEnd,
    //            kind: PathEnum::Complete(path),
    //        },
    //    }
    //}
}
impl TargetKey for Response {
    fn target_key(&self) -> DirectedKey {
        DirectedKey::up(
            self.end.path.root_parent(),
            self.end.path.root_parent().width(),
        )
    }
}
//impl TryInto<IncompleteState> for Response {
//    type Error = &'static str;
//    fn try_into(self) -> Result<IncompleteState, Self::Error> {
//        match self.end.kind {
//            PathEnum::Complete(_) => Err("Response is Complete, not Incomplete"),
//            _ => Ok(IncompleteState { end: self.end, }),
//        }
//    }
//}
//
//#[derive(Debug, Clone, Eq, PartialEq)]
//pub struct CompleteState {
//    pub path: IndexRangePath,
//    //pub end_token: Token,
//    //pub pattern_cursor: PatternCursor,
//}
//#[derive(Debug, Clone, Eq, PartialEq)]
//pub struct IncompleteState {
//    pub end: EndState,
//    pub path: IndexRangePath,
//    //pub pattern_cursor: PatternCursor,
//}
