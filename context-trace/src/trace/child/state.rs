use crate::{
    HasPath,
    PathRole,
    RootChild,
    RootChildIndex,
    graph::{
        getters::vertex::VertexSet,
        vertex::{
            location::child::ChildLocation,
            token::{
                SubToken,
                Token,
            },
        },
    },
    path::{
        RolePathUtils,
        accessors::{
            child::{
                LeafToken,
                RootedLeafToken,
            },
            has_path::IntoRootedRolePath,
            role::End,
            root::GraphRoot,
        },
        mutators::{
            adapters::IntoAdvanced,
            append::PathAppend,
            move_path::advance::Advance,
        },
        structs::rooted::index_range::IndexRangePath,
    },
    trace::{
        cache::key::{
            directed::up::UpKey,
            props::{
                LeafKey,
                RootKey,
            },
        },
        has_graph::HasGraph,
        state::{
            BaseState,
            parent::ParentState,
        },
    },
};
use derive_more::{
    DerefMut,
    derive::Deref,
};
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::VecDeque,
    fmt::Debug,
};

//impl_cursor_pos! {
//    CursorPosition for ChildState, self => self.cursor.relative_pos
//}
//impl TargetKey for ChildState {
//    fn target_key(&self) -> DirectedKey {
//        self.target.clone().into()
//    }
//}

#[derive(Clone, Debug, PartialEq, Eq, Deref, DerefMut)]
pub struct RootChildState {
    #[deref]
    #[deref_mut]
    pub child_state: ChildState,
    pub root_parent: ParentState,
}

#[derive(Clone, Debug, PartialEq, Eq, Deref, DerefMut)]
pub struct ChildState {
    #[deref]
    #[deref_mut]
    pub base: BaseState<IndexRangePath>,
}
impl ChildState {
    pub fn parent_state(&self) -> ParentState {
        ParentState {
            path: self.base.path.get_rooted_role_path(),
            ..self.base.clone()
        }
    }
}
impl PathAppend for ChildState {
    fn path_append(
        &mut self,
        parent_entry: ChildLocation,
    ) {
        self.base.path.path_append(parent_entry);
    }
}
impl RootChildIndex<End> for ChildState {
    fn root_child_index(&self) -> usize {
        self.base.path.role_root_child_index::<End>()
    }
}
impl RootChild<End> for ChildState {
    fn root_child<G: HasGraph>(
        &self,
        trav: &G,
    ) -> Token {
        RootChild::<End>::root_child(&self.base.path, trav)
    }
}
impl LeafToken<End> for ChildState {
    fn leaf_token<G: HasGraph>(
        &self,
        trav: &G,
    ) -> Option<Token> {
        Some(self.base.path.role_leaf_token::<End, G>(trav))
    }
    fn leaf_token_location(&self) -> Option<ChildLocation> {
        self.base.path.role_leaf_token_location::<End>()
    }
}
impl<R: PathRole> HasPath<R> for ChildState
where
    IndexRangePath: HasPath<R>,
{
    fn path(&self) -> &Vec<ChildLocation> {
        HasPath::<R>::path(&self.base.path)
    }
    fn path_mut(&mut self) -> &mut Vec<ChildLocation> {
        HasPath::<R>::path_mut(&mut self.base.path)
    }
}
pub trait PrefixStates: Sized + Clone {
    fn prefix_states<G: HasGraph>(
        &self,
        trav: &G,
    ) -> VecDeque<(SubToken, Self)>;
}
impl<T: RootedLeafToken<End> + PathAppend + Clone + Sized> PrefixStates for T {
    fn prefix_states<G: HasGraph>(
        &self,
        trav: &G,
    ) -> VecDeque<(SubToken, Self)> {
        let leaf = self.role_leaf_token::<End, _>(trav);
        trav.graph()
            .expect_vertex(leaf)
            .prefix_children::<G>()
            .iter()
            .sorted_unstable_by(|a, b| b.token.width.cmp(&a.token.width))
            .map(|sub| {
                let mut next = self.clone();
                next.path_append(leaf.to_child_location(sub.location));
                (sub.clone(), next)
            })
            .collect()
    }
}
//impl From<ChildState> for EditKind {
//    fn from(state: ChildState) -> Self {
//        match state.path.role_leaf_token_location::<End>() {
//            Some(entry) => DownEdit {
//                target: state.target,
//                entry,
//            }
//            .into(),
//            None => RootEdit {
//                entry_key: state.target,
//                entry_location: entry,
//            }
//            .into(),
//        }
//    }
//}

impl IntoAdvanced for ChildState {
    type Next = Self;
    fn into_advanced<G: HasGraph>(
        mut self,
        trav: &G,
    ) -> Result<Self, Self> {
        if self.base.path.advance(trav).is_continue() {
            // gen next token
            //Ok(Self {
            //    target: DownKey::new(
            //        self.base.path.role_leaf_token::<End, _>(&trav),
            //        (*self.cursor_pos()).into(),
            //    ),
            //    ..self
            //})
            Ok(self)
        } else {
            Err(self)
        }
    }
}

impl Ord for ChildState {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        self.path.root_parent().cmp(&other.path.root_parent())
    }
}

impl PartialOrd for ChildState {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl RootKey for ChildState {
    fn root_key(&self) -> UpKey {
        UpKey::new(self.path.root_parent(), self.root_pos.into())
    }
}

impl LeafKey for ChildState {
    fn leaf_location(&self) -> ChildLocation {
        self.path.leaf_location()
    }
}
