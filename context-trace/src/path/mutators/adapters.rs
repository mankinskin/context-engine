use crate::{
    PathAppend,
    direction::Right,
    path::{
        BasePath,
        RolePathUtils,
        accessors::{
            child::{
                LeafChild,
                RootChildIndexMut,
                root::{
                    GraphRootChild,
                    RootChild,
                },
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
    },
    trace::has_graph::HasGraph,
};
use std::fmt::Debug;

pub(crate) trait NodePath<R: PathRole>:
    RootChild<R> + Send + Clone + Eq + Debug
{
}

impl<R: PathRole, T: RootChild<R> + Send + Clone + Eq + Debug> NodePath<R>
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
    + LeafChild<Start>
    + LeafChild<End>
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
        + LeafChild<Start>
        + LeafChild<End>
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
pub trait IntoAdvanced: Sized + Clone {
    type Next;
    fn into_advanced<G: HasGraph>(
        self,
        trav: &G,
    ) -> Result<Self::Next, Self>;
}
