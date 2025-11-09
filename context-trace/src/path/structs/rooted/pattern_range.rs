use std::{
    borrow::Borrow,
    ops::ControlFlow,
};

use crate::{
    direction::pattern::PatternDirection,
    path::structs::rooted::{
        RangePath,
        role_path::PatternRolePath,
    },
    trace::has_graph::TravDir,
    *,
};

pub type PatternRangePath = RootedRangePath<Pattern>;
pub type PatternPostfixPath = RootedRolePath<Start, Pattern>;
pub type PatternPrefixPath = RootedRolePath<End, Pattern>;

impl RangePath for PatternRangePath {
    fn new_range(
        root: Self::Root,
        entry: usize,
        exit: usize,
    ) -> Self {
        Self {
            root,
            start: SubPath::new(entry, vec![]).into(),
            end: SubPath::new(exit, vec![]).into(),
        }
    }
}
impl RootChildIndexMut<End> for PatternRangePath {
    fn root_child_index_mut(&mut self) -> &mut usize {
        &mut self.end.sub_path.root_entry
    }
}

impl<P: IntoPattern> From<P> for PatternRangePath {
    fn from(p: P) -> Self {
        let p = p.into_pattern();
        let entry =
            <<BaseGraphKind as GraphKind>::Direction as PatternDirection>::head_index(p.borrow());
        RootedRangePath {
            root: p,
            start: SubPath::new(entry, vec![]).into(),
            end: SubPath::new(entry, vec![]).into(),
        }
    }
}
impl_root! { RootPattern for PatternRangePath, self, _trav => PatternRoot::pattern_root_pattern(self) }
impl_root! { PatternRoot for PatternRangePath, self => self.root.borrow() }
impl_root! { <Role: PathRole> PatternRoot for PatternRolePath<Role>, self => self.root.borrow() }

impl RootChildIndex<Start> for PatternRangePath {
    fn root_child_index(&self) -> usize {
        self.start.root_entry
    }
}
impl RootChildIndex<End> for PatternRangePath {
    fn root_child_index(&self) -> usize {
        self.end.root_entry
    }
}

impl MoveRootIndex<Right, End> for PatternRangePath {
    fn move_root_index<G: HasGraph>(
        &mut self,
        _trav: &G,
    ) -> ControlFlow<()> {
        if let Some(next) = TravDir::<G>::index_next(
            RootChildIndex::<End>::root_child_index(self),
        ) {
            *self.root_child_index_mut() = next;
            ControlFlow::Continue(())
        } else {
            ControlFlow::Break(())
        }
    }
}

impl<R: PathRole> LeafToken<R> for PatternRolePath<R> where
    Self: HasPath<R> + PatternRootChild<R>
{
}
impl<R: PathRole> LeafToken<R> for PatternRangePath where
    Self: HasPath<R> + PatternRootChild<R>
{
}

impl<R: PathRole> HasPath<R> for PatternRangePath
where
    Self: HasRolePath<R>,
{
    fn path(&self) -> &Vec<ChildLocation> {
        HasRolePath::<R>::role_path(self).path()
    }
    fn path_mut(&mut self) -> &mut Vec<ChildLocation> {
        HasRolePath::<R>::role_path_mut(self).path_mut()
    }
}
impl<Role: PathRole, Root: PathRoot + Clone> IntoRootedRolePath<Role>
    for RootedRangePath<Root>
where
    Self: IntoRolePath<Role> + RootedPath<Root = Root>,
{
    fn into_rooted_role_path(self) -> RootedRolePath<Role, Self::Root> {
        let root = self.path_root();
        self.into_role_path().into_rooted(root)
    }
}
impl IntoRolePath<Start> for PatternRangePath {
    fn into_role_path(self) -> RolePath<Start> {
        self.start
    }
}
impl IntoRolePath<End> for PatternRangePath {
    fn into_role_path(self) -> RolePath<End> {
        self.end
    }
}

impl<R: PathRole> PatternRootChild<R> for PatternRolePath<R> where
    Self: RootChildIndex<R>
{
}
impl<R: PathRole> PatternRootChild<R> for PatternRangePath where
    Self: RootChildIndex<R>
{
}

impl_root_child_token! { RootChildToken for PatternRangePath, self, _trav =>
       *self.root.get(self.role_root_child_index::<R>()).unwrap()
}
impl<Root: PathRoot> CalcOffset for RootedRangePath<Root> {
    fn calc_offset<G: HasGraph>(
        &self,
        trav: G,
    ) -> usize {
        let outer_offsets =
            self.start.calc_offset(&trav) + self.end.calc_offset(&trav);
        let graph = trav.graph();
        let pattern = self.root.root_pattern::<G>(&graph);
        let entry = self.start.sub_path.root_entry;
        let exit = self.end.sub_path.root_entry;
        let inner_offset = if entry < exit {
            pattern_width(&pattern[entry + 1..exit])
        } else {
            0
        };
        inner_offset + outer_offsets
    }
}

impl<R: PathRoot> PathPop for RootedRangePath<R> {
    fn path_pop(&mut self) -> Option<ChildLocation> {
        self.end.path_pop()
    }
}
