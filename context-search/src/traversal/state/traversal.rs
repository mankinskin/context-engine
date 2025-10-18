use std::cmp::Ordering;

use context_trace::*;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraversalState {
    pub(crate) prev: DirectedKey,
    pub(crate) kind: InnerKind,
}
impl HasRootPos for TraversalState {
    fn root_pos(&self) -> &AtomPosition {
        match &self.kind {
            InnerKind::Parent(state) => state.root_pos(),
            InnerKind::Token(state) => state.root_pos(),
        }
    }
    fn root_pos_mut(&mut self) -> &mut AtomPosition {
        match &mut self.kind {
            InnerKind::Parent(state) => state.root_pos_mut(),
            InnerKind::Token(state) => state.root_pos_mut(),
        }
    }
}
impl TraversalState {
    pub(crate) fn entry_location(&self) -> Option<ChildLocation> {
        match &self.kind {
            InnerKind::Parent(state) =>
                Some(state.rooted_path().root_child_location()),
            InnerKind::Token(state) =>
                state.rooted_path().role_leaf_token_location::<End>(),
        }
    }
    pub(crate) fn prev_key(&self) -> DirectedKey {
        self.prev.clone()
    }

    pub(crate) fn state_direction(&self) -> StateDirection {
        match &self.kind {
            InnerKind::Parent(_) => StateDirection::BottomUp,
            InnerKind::Token(_) => StateDirection::TopDown,
        }
    }
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
            kind: InnerKind::Token(cs),
        }
    }
}
impl TargetKey for TraversalState {
    fn target_key(&self) -> DirectedKey {
        match &self.kind {
            InnerKind::Parent(state) => state.target_key(),
            InnerKind::Token(state) => state.target_key(),
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
impl RootKey for TraversalState {
    fn root_key(&self) -> UpKey {
        match &self.kind {
            InnerKind::Parent(state) => state.root_key(),
            InnerKind::Token(state) => state.root_key(),
        }
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
