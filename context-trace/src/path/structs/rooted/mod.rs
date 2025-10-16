pub(crate) mod index_range;
pub(crate) mod pattern_range;
pub(crate) mod role_path;
pub(crate) mod root;
pub(crate) mod split_path;

use root::{
    PathRoot,
    RootedPath,
};
use split_path::RootedSplitPathRef;

use crate::path::{
    accessors::role::{
        End,
        Start,
    },
    structs::{
        role_path::RolePath,
        rooted::{
            pattern_range::PatternRangePath,
            role_path::{
                PatternStartPath,
                RootedEndPath,
            },
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootedRangePath<Root: PathRoot> {
    pub(crate) root: Root,
    pub(crate) start: RolePath<Start>,
    pub(crate) end: RolePath<End>,
}
impl<R: PathRoot> RootedPath for RootedRangePath<R> {
    type Root = R;
    fn path_root(&self) -> Self::Root {
        self.root.clone()
    }
}
impl<R: PathRoot> From<RootedEndPath<R>> for RootedRangePath<R> {
    fn from(value: RootedEndPath<R>) -> Self {
        Self {
            root: value.root,
            start: Default::default(),
            end: value.role_path,
        }
    }
}
impl From<PatternStartPath> for PatternRangePath {
    fn from(value: PatternStartPath) -> Self {
        Self {
            start: value.role_path,
            end: RolePath::new(value.root.len()),
            root: value.root,
        }
    }
}

impl<R: PathRoot> RootedRangePath<R> {
    pub fn start_path(&self) -> RootedSplitPathRef<'_, R> {
        RootedSplitPathRef {
            root: &self.root,
            sub_path: &self.start.sub_path,
        }
    }
    pub fn end_path(&self) -> RootedSplitPathRef<'_, R> {
        RootedSplitPathRef {
            root: &self.root,
            sub_path: &self.end.sub_path,
        }
    }
}
