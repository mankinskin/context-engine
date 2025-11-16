use crate::{
    PathAppend,
    direction::Right,
    path::{
        BasePath,
        RolePathUtils,
        accessors::{
            child::{
                LeafToken,
                root::GraphRootChild,
            },
            has_path::HasRolePath,
            role::{
                End,
                PathRole,
                Start,
            },
            root::GraphRoot,
        },
        mutators::move_path::root::MoveRootIndex,
        structs::rooted::role_path::{
            RootChildIndexMut,
            RootChildToken,
        },
    },
    trace::has_graph::HasGraph,
};
use std::fmt::Debug;

pub(crate) trait NodePath<R: PathRole>:
    RootChildToken<R> + Send + Clone + Eq + Debug
{
}

impl<R: PathRole, T: RootChildToken<R> + Send + Clone + Eq + Debug> NodePath<R>
    for T
{
}

pub(crate) trait Advanced:
    RolePathUtils
    + NodePath<Start>
    + BasePath
    + HasRolePath<Start>
    + HasRolePath<End>
    + GraphRootChild<Start>
    + GraphRootChild<End>
    + LeafToken<Start>
    + LeafToken<End>
    + MoveRootIndex<Right, End>
    + RootChildIndexMut<End>
    + GraphRoot
    + PathAppend
{
}

impl<
    T: RolePathUtils
        + NodePath<Start>
        + BasePath
        + HasRolePath<Start>
        + HasRolePath<End>
        + GraphRootChild<Start>
        + GraphRootChild<End>
        + LeafToken<Start>
        + LeafToken<End>
        + MoveRootIndex<Right, End>
        + RootChildIndexMut<End>
        + PathAppend,
> Advanced for T
{
}
pub(crate) trait FromAdvanced<A: Advanced> {
    fn from_advanced<G: HasGraph>(
        path: A,
        trav: &G,
    ) -> Self;
}
pub trait StateAdvance: Sized + Clone {
    type Next;
    fn advance_state<G: HasGraph>(
        self,
        trav: &G,
    ) -> Result<Self::Next, Self>;
}
