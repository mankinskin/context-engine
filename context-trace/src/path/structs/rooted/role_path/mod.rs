pub(crate) mod calc;
pub(crate) mod range;

use auto_impl::auto_impl;
use derive_more::{
    Deref,
    DerefMut,
};
use std::borrow::Borrow;

use crate::{
    EndPath,
    HasEndPath,
    HasStartPath,
    RolePathUtils,
    StartPath,
    graph::{
        getters::ErrorReason,
        vertex::{
            child::Child,
            location::{
                child::ChildLocation,
                pattern::{
                    IntoPatternLocation,
                    PatternLocation,
                },
            },
            pattern::{
                IntoPattern,
                Pattern,
            },
            wide::Wide,
        },
    },
    impl_child,
    path::{
        accessors::{
            child::{
                LeafChild,
                PathChild,
                root::{
                    GraphRootChild,
                    PatternRootChild,
                },
            },
            has_path::{
                HasPath,
                HasRolePath,
                HasSinglePath,
                IntoRolePath,
            },
            role::{
                End,
                PathRole,
                Start,
            },
            root::{
                GraphRoot,
                GraphRootPattern,
                RootPattern,
            },
        },
        structs::{
            query_range_path::FoldablePath,
            role_path::{
                CalcOffset,
                RolePath,
            },
            rooted::{
                RootedRangePath,
                pattern_range::PatternRangePath,
                role_path::calc::CalcWidth,
                root::{
                    IndexRoot,
                    PathRoot,
                    RootedPath,
                },
            },
            sub_path::SubPath,
        },
    },
    trace::has_graph::HasGraph,
};

#[derive(Debug, Clone, PartialEq, Eq, Default, Deref, DerefMut)]
pub struct RootedRolePath<R: PathRole, Root: PathRoot> {
    pub(crate) root: Root,
    #[deref]
    #[deref_mut]
    pub(crate) role_path: RolePath<R>,
}
impl<Root: PathRoot, R: PathRole> RootedRolePath<R, Root> {
    pub fn new(
        root: impl Into<Root>,
        role_path: RolePath<R>,
    ) -> Self {
        Self {
            root: root.into(),
            role_path,
        }
    }
}
impl<Root: PathRoot> HasStartPath for RootedStartPath<Root> {
    fn start_path(&self) -> &StartPath {
        &self.role_path
    }
    fn start_path_mut(&mut self) -> &mut StartPath {
        &mut self.role_path
    }
}
impl<Root: PathRoot> HasEndPath for RootedEndPath<Root> {
    fn end_path(&self) -> &EndPath {
        &self.role_path
    }
    fn end_path_mut(&mut self) -> &mut EndPath {
        &mut self.role_path
    }
}

impl<R: PathRole, Root: PathRoot> HasRolePath<R> for RootedRolePath<R, Root> {
    fn role_path(&self) -> &RolePath<R> {
        &self.role_path
    }
    fn role_path_mut(&mut self) -> &mut RolePath<R> {
        &mut self.role_path
    }
}
impl<R: PathRole, Root: PathRoot> IntoRolePath<R> for RootedRolePath<R, Root> {
    fn into_role_path(self) -> RolePath<R> {
        self.role_path
    }
}
impl<Root: PathRoot> CalcWidth for RootedRangePath<Root>
where
    Self: LeafChild<Start> + LeafChild<End>,
{
    fn calc_width<G: HasGraph>(
        &self,
        trav: G,
    ) -> usize {
        self.calc_offset(&trav)
            + self.role_leaf_child::<Start, _>(&trav).width()
            + if self.role_root_child_index::<Start>()
                != self.role_root_child_index::<End>()
            {
                self.role_leaf_child::<End, _>(&trav).width()
            } else {
                0
            }
    }
}

pub(crate) type IndexRolePath<R> = RootedRolePath<R, IndexRoot>;

pub(crate) type PatternRolePath<R> = RootedRolePath<R, Pattern>;

pub(crate) type RootedStartPath<R> = RootedRolePath<Start, R>;
pub(crate) type RootedEndPath<R> = RootedRolePath<End, R>;
pub type IndexStartPath = IndexRolePath<Start>;
pub type IndexEndPath = IndexRolePath<End>;
pub(crate) type PatternStartPath = PatternRolePath<Start>;
pub type PatternEndPath = PatternRolePath<End>;

impl<R: PathRole> IndexRolePath<R> {
    pub fn new_location(first: ChildLocation) -> Self {
        Self::from(first)
    }
}
impl<R: PathRole> PathChild<R> for IndexRolePath<R>
where
    Self: HasRolePath<R>,
{
    fn path_child_location(&self) -> Option<ChildLocation> {
        Some(
            R::bottom_up_iter(self.path().iter())
                .next()
                .cloned()
                .unwrap_or(
                    self.root
                        .location
                        .to_child_location(self.role_path().root_entry),
                ),
        )
    }
    fn path_child<G: HasGraph>(
        &self,
        trav: &G,
    ) -> Option<Child> {
        PathChild::<R>::path_child(self.role_path(), trav)
    }
}

impl<R: PathRole> From<ChildLocation> for IndexRolePath<R> {
    fn from(first: ChildLocation) -> Self {
        Self {
            role_path: RolePath::new_empty(first.sub_index),
            root: IndexRoot::from(first.into_pattern_location()),
        }
    }
}
impl<Root: PathRoot, R: PathRole> From<(Root, RolePath<R>)>
    for RootedRolePath<R, Root>
{
    fn from((root, role_path): (Root, RolePath<R>)) -> Self {
        Self { root, role_path }
    }
}
impl<R: PathRoot> RootedStartPath<R> {
    pub(crate) fn into_range(
        self,
        exit: usize,
    ) -> RootedRangePath<R> {
        RootedRangePath {
            root: self.root,
            start: self.role_path,
            end: RolePath {
                sub_path: SubPath {
                    root_entry: exit,
                    path: vec![],
                },
                _ty: Default::default(),
            },
        }
    }
}
impl<R: PathRoot> RootedEndPath<R> {
    pub(crate) fn into_range(
        self,
        entry: usize,
    ) -> RootedRangePath<R> {
        RootedRangePath {
            root: self.root,
            start: RolePath {
                sub_path: SubPath {
                    root_entry: entry,
                    path: vec![],
                },
                _ty: Default::default(),
            },
            end: self.role_path,
        }
    }
}

/// access to the position of a child
#[auto_impl(&, & mut)]
pub trait RootChildIndex<R> {
    fn root_child_index(&self) -> usize;
}

pub(crate) trait RootChildIndexMut<R>: RootChildIndex<R> {
    fn root_child_index_mut(&mut self) -> &mut usize;
}

impl<R: PathRole, Root: PathRoot> RootChildIndexMut<R>
    for RootedRolePath<R, Root>
{
    fn root_child_index_mut(&mut self) -> &mut usize {
        self.role_path.root_child_index_mut()
    }
}

impl<R: PathRoot> From<RootedRangePath<R>> for RootedRolePath<Start, R> {
    fn from(path: RootedRangePath<R>) -> Self {
        Self {
            root: path.root,
            role_path: path.start,
        }
    }
}
impl<R: PathRoot> From<RootedRangePath<R>> for RootedRolePath<End, R> {
    fn from(path: RootedRangePath<R>) -> Self {
        Self {
            root: path.root,
            role_path: path.end,
        }
    }
}

impl<R: PathRole, Root: PathRoot> RootedPath for RootedRolePath<R, Root> {
    type Root = Root;
    fn path_root(&self) -> Self::Root {
        self.root.clone()
    }
}

impl<R: PathRole> PathChild<R> for RolePath<R> {}

impl_child! {
    RootChild for IndexRolePath<R>, self,
    trav => *trav.graph().expect_child_at(
            self.path_root().location.to_child_location(
                RootChildIndex::<R>::root_child_index(&self.role_path)
            )
        )
}
impl<R: PathRole> GraphRootChild<R> for RootedRolePath<R, IndexRoot> {
    fn root_child_location(&self) -> ChildLocation {
        self.path_root()
            .location
            .to_child_location(self.role_path.sub_path.root_entry)
    }
}
impl<R: PathRole, Root: PathRoot> RootChildIndex<R>
    for RootedRolePath<R, Root>
{
    fn root_child_index(&self) -> usize {
        RootChildIndex::<R>::root_child_index(&self.role_path)
    }
}

impl<R: PathRole> GraphRoot for RootedRolePath<R, IndexRoot> {
    fn root_parent(&self) -> Child {
        self.root.location.parent
    }
}

impl<R: PathRole> GraphRootPattern for RootedRolePath<R, IndexRoot> {
    fn root_pattern_location(&self) -> PatternLocation {
        self.root.location.clone()
    }
}

impl<R: PathRole, Root: PathRoot> HasSinglePath for RootedRolePath<R, Root> {
    fn single_path(&self) -> &[ChildLocation] {
        self.role_path.sub_path.path.borrow()
    }
}

impl<Role: PathRole, Root: PathRoot> RootPattern
    for RootedRolePath<Role, Root>
{
    fn root_pattern<'a: 'g, 'b: 'g, 'g, G: HasGraph + 'a>(
        &'b self,
        trav: &'g G::Guard<'a>,
    ) -> &'g Pattern {
        self.root.root_pattern::<G>(trav)
    }
}

//impl_root! { <R: PathRole> PatternRoot for PatternRolePath<R>, self => self.root.borrow() }

impl RootChildIndex<Start> for PatternEndPath {
    fn root_child_index(&self) -> usize {
        0
    }
}
//impl<R: PathRole> PathChild<R> for PatternRolePath<R> where
//    Self: HasPath<R> + PatternRootChild<R>
//{
//}
//
//impl<R: PathRole> PatternRootChild<R> for PatternRolePath<R> where
//    PatternEndPath: RootChildIndex<R>
//{
//}

impl HasPath<End> for PatternEndPath {
    fn path(&self) -> &Vec<ChildLocation> {
        self.role_path.path()
    }
    fn path_mut(&mut self) -> &mut Vec<ChildLocation> {
        self.role_path.path_mut()
    }
}
//impl PathChild<Start> for PatternEndPath {
//    fn path_child_location(&self) -> Option<ChildLocation> {
//        None
//    }
//    fn path_child<G: HasGraph>(
//        &self,
//        trav: &G,
//    ) -> Option<Child> {
//        Some(self.root_child())
//    }
//}
//impl_child! { RootChild for PatternRolePath<R>, self, _trav => self.pattern_root_child() }

impl_child! { RootChild for PatternRolePath<R>, self, _trav => self.pattern_root_child() }

impl FoldablePath for PatternEndPath {
    fn to_range_path(self) -> PatternRangePath {
        self.into_range(0)
    }
    fn complete(query: impl IntoPattern) -> Self {
        let pattern = query.into_pattern();
        Self {
            role_path: RolePath::new_empty(pattern.len() - 1),
            root: pattern,
        }
    }
    fn new_directed<D>(query: Pattern) -> Result<Self, (ErrorReason, Self)> {
        let pattern = query.into_pattern();
        let len = pattern.len();
        let p = Self {
            role_path: RolePath::new_empty(0),
            root: pattern,
        };
        match len {
            0 => Err((ErrorReason::EmptyPatterns, p)),
            1 => Err((
                ErrorReason::SingleIndex(Box::new(
                    PatternRangePath::from(p.clone()).into(),
                )),
                p,
            )),
            _ => Ok(p),
        }
    }
}
