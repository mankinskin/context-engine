use root::RootChild;

pub(crate) mod root;

use crate::{
    graph::vertex::{
        location::child::ChildLocation,
        token::Token,
    },
    path::{
        accessors::{
            has_path::HasPath,
            role::PathRole,
        },
        structs::rooted::role_path::{
            RootChildIndex,
            RootChildIndexMut,
        },
    },
    trace::has_graph::HasGraph,
};

pub trait LeafToken<R: PathRole>: RootChildIndex<R> + HasPath<R> {
    fn leaf_token_location_mut(&mut self) -> Option<&mut ChildLocation> {
        R::bottom_up_iter(self.path_mut().iter_mut()).next()
    }
    fn leaf_token_location(&self) -> Option<ChildLocation> {
        R::bottom_up_iter(self.path().iter()).next().cloned() as Option<_>
    }
    fn leaf_token<G: HasGraph>(
        &self,
        trav: &G,
    ) -> Option<Token> {
        self.leaf_token_location()
            .map(|loc| *trav.graph().expect_child_at(loc))
    }
}
pub trait RootedLeafToken<R: PathRole>: LeafToken<R> + RootChild<R> {
    fn rooted_leaf_token<G: HasGraph>(
        &self,
        trav: &G,
    ) -> Token {
        self.leaf_token(trav)
            .unwrap_or_else(|| self.root_child(trav))
    }
}
impl<R: PathRole, T: LeafToken<R> + RootChild<R>> RootedLeafToken<R> for T {}

pub(crate) trait LeafTokenPosMut<R>: RootChildIndexMut<R> {
    fn leaf_token_pos_mut(&mut self) -> &mut usize;
}
