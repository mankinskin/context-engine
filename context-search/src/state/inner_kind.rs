use std::cmp::Ordering;

use context_trace::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InnerKind {
    Parent(ParentState),
    Child(ChildState),
}
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
