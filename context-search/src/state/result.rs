use std::{
    ops::Index,
    path::Path,
};

use context_trace::*;
use petgraph::Direction::Incoming;

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
pub struct Response {
    pub cache: TraceCache,
    pub(crate) start: Token,
    pub end: EndState,
    pub path: IndexRangePath,
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
        start: Token,
        cache: TraceCache,
        end: EndState,
        path: IndexRangePath,
    ) -> Self {
        Self {
            cache,
            start,
            end,
            path,
        }
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
    //            kind: EndKind::Complete(path),
    //        },
    //    }
    //}
}
impl TargetKey for Response {
    fn target_key(&self) -> DirectedKey {
        DirectedKey::up(
            self.path.root_parent(),
            self.path.root_parent().width(),
        )
    }
}
//impl TryInto<IncompleteState> for Response {
//    type Error = &'static str;
//    fn try_into(self) -> Result<IncompleteState, Self::Error> {
//        match self.end.kind {
//            EndKind::Complete(_) => Err("Response is Complete, not Incomplete"),
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
