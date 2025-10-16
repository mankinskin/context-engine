use std::{
    borrow::Borrow,
    ops::Deref,
};

use crate::{
    graph::vertex::location::child::ChildLocation,
    path::{
        accessors::{
            border::PathBorder,
            child::{
                LeafChildPosMut,
                PathChild,
                RootChildIndex,
                RootChildIndexMut,
            },
            has_path::{
                HasPath,
                HasRolePath,
                HasSinglePath,
            },
            role::{
                End,
                PathRole,
                Start,
            },
        },
        mutators::{
            adapters::FromAdvanced,
            simplify::PathSimplify,
        },
        structs::{
            rooted::{
                role_path::RootedRolePath,
                root::PathRoot,
            },
            sub_path::SubPath,
        },
    },
    trace::has_graph::HasGraph,
};

use crate::path::{
    RolePathUtils,
    structs::rooted::index_range::IndexRangePath,
};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct RolePath<R: PathRole> {
    pub(crate) sub_path: SubPath,
    pub(crate) _ty: std::marker::PhantomData<R>,
}

impl<R: PathRole> RolePath<R> {
    pub(crate) fn new(entry: usize) -> Self {
        Self {
            sub_path: SubPath::new(entry),
            _ty: Default::default(),
        }
    }
    pub(crate) fn path(&self) -> &Vec<ChildLocation> {
        &self.sub_path.path
    }
    pub(crate) fn path_mut(&mut self) -> &mut Vec<ChildLocation> {
        &mut self.sub_path.path
    }
    pub(crate) fn into_rooted<Root: PathRoot>(
        self,
        root: Root,
    ) -> RootedRolePath<R, Root> {
        RootedRolePath::from((root, self))
    }
}
pub trait CalcOffset {
    // TODO: Make offset side relative
    fn calc_offset<G: HasGraph>(
        &self,
        trav: G,
    ) -> usize;
}
impl<R: PathRole> CalcOffset for RolePath<R> {
    fn calc_offset<G: HasGraph>(
        &self,
        trav: G,
    ) -> usize {
        let graph = trav.graph();
        self.sub_path
            .path
            .iter()
            .fold(0, |acc, loc| acc + loc.role_inner_width::<_, R>(&graph))
    }
}
impl<R: PathRole> RootChildIndex<R> for RolePath<R> {
    fn root_child_index(&self) -> usize {
        self.sub_path.root_entry
    }
}
impl LeafChildPosMut<End> for RolePath<End> {
    fn leaf_child_pos_mut(&mut self) -> &mut usize {
        if !self.path().is_empty() {
            &mut self.path_child_location_mut().unwrap().sub_index
        } else {
            self.root_child_index_mut()
        }
    }
}
impl<R: PathRole> RootChildIndexMut<R> for RolePath<R> {
    fn root_child_index_mut(&mut self) -> &mut usize {
        &mut self.sub_path.root_entry
    }
}

impl<R: PathRole> HasSinglePath for RolePath<R> {
    fn single_path(&self) -> &[ChildLocation] {
        self.path().borrow()
    }
}

impl<R: PathRole> HasPath<R> for RolePath<R> {
    fn path(&self) -> &Vec<ChildLocation> {
        &self.path
    }
    fn path_mut(&mut self) -> &mut Vec<ChildLocation> {
        &mut self.sub_path.path
    }
}

impl<R: PathRole> HasRolePath<R> for RolePath<R> {
    fn role_path(&self) -> &RolePath<R> {
        self
    }
    fn role_path_mut(&mut self) -> &mut RolePath<R> {
        self
    }
}

impl<R: PathRole> PathSimplify for RolePath<R> {
    fn into_simplified<G: HasGraph>(
        mut self,
        trav: &G,
    ) -> Self {
        let graph = trav.graph();
        while let Some(loc) = self.path_mut().pop() {
            if !<R as PathBorder>::is_at_border(graph.graph(), loc) {
                self.path_mut().push(loc);
                break;
            }
        }
        self
    }
}

impl<R: PathRole> Deref for RolePath<R> {
    type Target = SubPath;
    fn deref(&self) -> &Self::Target {
        &self.sub_path
    }
}

impl From<IndexRangePath> for RolePath<Start> {
    fn from(p: IndexRangePath) -> Self {
        p.start
    }
}

impl<R: PathRole> From<SubPath> for RolePath<R> {
    fn from(sub_path: SubPath) -> Self {
        Self {
            sub_path,
            _ty: Default::default(),
        }
    }
}

impl From<IndexRangePath> for RolePath<End> {
    fn from(p: IndexRangePath) -> Self {
        p.end
    }
}
//impl<R> WideMut for RolePath<R> {
//    fn width_mut(&mut self) -> &mut usize {
//        &mut self.width
//    }
//}

impl FromAdvanced<IndexRangePath> for RolePath<Start> {
    fn from_advanced<G: HasGraph>(
        path: IndexRangePath,
        _trav: &G,
    ) -> Self {
        path.start
    }
}
