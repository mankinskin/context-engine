pub(crate) mod complete;
pub(crate) mod end;
pub(crate) mod inner_kind;
pub(crate) mod result;
pub(crate) mod start;

use std::cmp::Ordering;

use context_trace::*;

use inner_kind::InnerKind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraversalState {
    pub(crate) prev: DirectedKey,
    pub(crate) kind: InnerKind,
}
impl From<(DirectedKey, ParentState)> for TraversalState {
    fn from((prev, ps): (DirectedKey, ParentState)) -> Self {
        Self {
            prev,
            kind: InnerKind::Parent(ps),
        }
    }
}
impl From<(DirectedKey, ChildState)> for TraversalState {
    fn from((prev, cs): (DirectedKey, ChildState)) -> Self {
        Self {
            prev,
            kind: InnerKind::Child(cs),
        }
    }
}
impl HasRootPos for TraversalState {
    fn root_pos(&self) -> &AtomPosition {
        match &self.kind {
            InnerKind::Parent(state) => state.root_pos(),
            InnerKind::Child(state) => state.root_pos(),
        }
    }
    fn root_pos_mut(&mut self) -> &mut AtomPosition {
        match &mut self.kind {
            InnerKind::Parent(state) => state.root_pos_mut(),
            InnerKind::Child(state) => state.root_pos_mut(),
        }
    }
}
impl TargetKey for TraversalState {
    fn target_key(&self) -> DirectedKey {
        match &self.kind {
            InnerKind::Parent(state) => state.target_key(),
            InnerKind::Child(state) => state.target_key(),
        }
    }
}
impl RootKey for TraversalState {
    fn root_key(&self) -> UpKey {
        match &self.kind {
            InnerKind::Parent(state) => state.root_key(),
            InnerKind::Child(state) => state.root_key(),
        }
    }
}
impl Ord for TraversalState {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        self.kind.cmp(&other.kind)
    }
}

impl PartialOrd for TraversalState {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TraversalState {
    pub(crate) fn entry_location(&self) -> Option<ChildLocation> {
        match &self.kind {
            InnerKind::Parent(state) =>
                Some(state.rooted_path().graph_root_child_location()),
            InnerKind::Child(state) =>
                state.rooted_path().role_leaf_token_location::<End>(),
        }
    }
    pub(crate) fn prev_key(&self) -> DirectedKey {
        self.prev.clone()
    }

    pub(crate) fn state_direction(&self) -> StateDirection {
        match &self.kind {
            InnerKind::Parent(_) => StateDirection::BottomUp,
            InnerKind::Child(_) => StateDirection::TopDown,
        }
    }
}
