use crate::{
    direction::{
        Right,
        pattern::PatternDirection,
    },
    graph::{
        getters::ErrorReason,
        vertex::{
            pattern::{
                IntoPattern,
                Pattern,
            },
            token::Token,
        },
    },
    path::{
        BaseQuery,
        RolePathUtils,
        accessors::{
            child::{
                LeafToken,
                RootedLeafToken,
            },
            has_path::IntoRootedRolePath,
            role::{
                End,
                Start,
            },
            root::RootPattern,
        },
        mutators::{
            append::PathAppend,
            move_path::root::MoveRootIndex,
            pop::PathPop,
        },
        structs::rooted::{
            role_path::calc::CalcWidth,
            root::RootedPath,
        },
    },
    trace::has_graph::HasGraph,
};

use crate::path::structs::rooted::pattern_range::PatternRangePath;

pub trait FoldablePath:
BaseQuery
//+ LeafTokenPosMut<End>
+ PathAppend
+ PathPop
+ MoveRootIndex<Right, End>
+ RootedLeafToken<End>
+ RootPattern
+ CalcWidth
{
    fn to_range_path(self) -> PatternRangePath;
    fn complete(pattern: impl IntoPattern) -> Self;
    fn new_directed<
        D: PatternDirection,
    >(query: Pattern) -> Result<Self, (ErrorReason, Self)>;
    fn start_index<G: HasGraph>(
        &self,
        trav: G,
    ) -> Token {
        self.role_leaf_token(&trav)
    }
}
pub(crate) trait RangePath:
    RootedPath + IntoRootedRolePath<Start> + IntoRootedRolePath<End>
{
    fn new_range(
        root: Self::Root,
        entry: usize,
        exit: usize,
    ) -> Self;
}

//impl PatternStart for PatternRangePath {}
//impl PatternEnd for PatternRangePath {}
//impl TraversalPath for PatternRangePath {
//    fn prev_exit_pos<
//        'a: 'g,
//        'g,
//        T: Atomize,
//        D: ,
//        G: HasGraph<T>,
//    >(&self, trav: G) -> Option<usize> {
//        if self.end.is_empty() {
//            D::pattern_index_prev(self.query.borrow(), self.exit)
//        } else {
//            let location = *self.end.last().unwrap();
//            let pattern = trav.graph().expect_pattern_at(&location);
//            D::pattern_index_prev(pattern, location.sub_index)
//        }
//    }
//}
