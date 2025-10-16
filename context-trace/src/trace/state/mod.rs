pub(crate) mod parent;

use crate::{
    graph::vertex::location::pattern::IntoPatternLocation as _,
    path::{
        accessors::has_path::IntoRootedPath,
        structs::rooted::root::RootedPath,
    },
    *,
};
use parent::ParentState;
use std::cmp::Ordering;

//pub(crate) trait SearchSpace {
//    fn expand(&mut self) -> Vec<TraceState<Self>>;
//}
// TODO:
// - param K: SearchSpace
// - connect input and internal path with Into<InternalState>
// AVAILABLE DATA FIELDS
// - PathBagMember<K> for each path for pos or id
// - PathBagProps<K> for whole bag for global pos
// AVAILABLE OPERATIONS
// - expand - (compare match, expand)
//

//
// TraceState<K> is either {ChildState<K>, ParentState<K>}
//
pub trait HasPrevPos {
    fn prev_pos(&self) -> &TokenPosition;
    fn prev_pos_mut(&mut self) -> &mut TokenPosition;
}
impl<P: RootedPath> HasPrevPos for BaseState<P> {
    fn prev_pos(&self) -> &TokenPosition {
        &self.prev_pos
    }
    fn prev_pos_mut(&mut self) -> &mut TokenPosition {
        &mut self.prev_pos
    }
}
pub trait HasRootPos {
    fn root_pos(&self) -> &TokenPosition;
    fn root_pos_mut(&mut self) -> &mut TokenPosition;
}
impl<P: RootedPath> HasRootPos for BaseState<P> {
    fn root_pos(&self) -> &TokenPosition {
        &self.root_pos
    }
    fn root_pos_mut(&mut self) -> &mut TokenPosition {
        &mut self.root_pos
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BaseState<P: RootedPath> {
    pub prev_pos: TokenPosition,
    pub root_pos: TokenPosition,
    pub path: P,
}
impl<P: RootedPath> IntoRootedPath<P> for BaseState<P> {
    fn into_rooted_path(self) -> P {
        self.path
    }
}
impl<P: RootedPath> HasRootedPath<P> for BaseState<P> {
    fn rooted_path(&self) -> &P {
        &self.path
    }
    fn rooted_path_mut(&mut self) -> &mut P {
        &mut self.path
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InnerKind {
    Parent(ParentState),
    Child(ChildState),
}
impl InnerKind {
    pub fn unwrap_parent(self) -> ParentState {
        if let Self::Parent(p) = self {
            p
        } else {
            panic!();
        }
    }
    pub(crate) fn unwrap_child(self) -> ChildState {
        if let Self::Child(c) = self {
            c
        } else {
            panic!();
        }
    }
}

//impl From<InnerKind> for EditKind {
//    fn from(state: InnerKind) -> Self {
//        match state {
//            InnerKind::Parent(state) => Self::Parent(state.into()),
//            InnerKind::Child(state) => Self::Child(state.into()),
//        }
//    }
//}

impl Ord for InnerKind {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        match (self, other) {
            (InnerKind::Child(a), InnerKind::Child(b)) => a.cmp(b),
            (InnerKind::Parent(a), InnerKind::Parent(b)) => a.cmp(b),
            (InnerKind::Child(_), _) => Ordering::Less,
            (_, InnerKind::Child(_)) => Ordering::Greater,
        }
    }
}

impl PartialOrd for InnerKind {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub trait IntoParentState: Sized {
    fn into_parent_state<G: HasGraph>(
        self,
        trav: &G,
        parent_entry: ChildLocation,
    ) -> ParentState;
}
impl IntoParentState for Child {
    fn into_parent_state<G: HasGraph>(
        self,
        _trav: &G,
        parent_entry: ChildLocation,
    ) -> ParentState {
        let width = self.width().into();
        ParentState {
            prev_pos: width,
            root_pos: width,
            path: RootedRolePath {
                root: IndexRoot {
                    location: parent_entry.into_pattern_location(),
                },
                role_path: RolePath {
                    sub_path: SubPath {
                        root_entry: parent_entry.sub_index,
                        path: vec![],
                    },
                    _ty: Default::default(),
                },
            },
        }
    }
}
