use context_trace::{
    path::{
        accessors::has_path::HasRolePath,
        RolePathUtils,
    },
    *,
};
use postfix::PostfixEnd;
use prefix::PrefixEnd;
use range::RangeEnd;

use crate::{
    compare::parent::ParentCompareState,
    cursor::PatternCursor,
};

pub(crate) mod postfix;
pub(crate) mod prefix;
pub(crate) mod range;

/// Represents the state of matching during search.
/// Distinguishes between "haven't found anything yet" (query state)
/// and "found something" (located in graph).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum MatchState {
    /// Initial state: searching for the query pattern, no graph location yet
    Query(PatternRangePath),
    /// Found state: matched something and located it in the graph
    Located(EndState),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum PathEnum {
    Range(RangeEnd),
    Postfix(PostfixEnd),
    Prefix(PrefixEnd),
    Complete(IndexRangePath),
}
impl GraphRoot for PathEnum {
    fn root_parent(&self) -> Token {
        match self {
            PathEnum::Range(p) => p.path.root_parent(),
            PathEnum::Postfix(p) => p.path.root_parent(),
            PathEnum::Prefix(p) => p.path.root_parent(),
            PathEnum::Complete(c) => c.root_parent(),
        }
    }
}
impl PathEnum {
    pub(crate) fn from_range_path<G: HasGraph>(
        mut path: IndexRangePath,
        root_pos: AtomPosition,
        target: DownKey,
        trav: &G,
    ) -> Self {
        path.child_path_mut::<Start>().simplify(trav);
        path.child_path_mut::<End>().simplify(trav);

        match (
            path.is_at_border::<_, Start>(trav.graph()),
            path.raw_child_path::<Start>().is_empty(),
            path.is_at_border::<_, End>(trav.graph()),
            path.raw_child_path::<End>().is_empty(),
        ) {
            (true, true, true, true) => PathEnum::Complete(path),
            (true, true, false, _) | (true, true, true, false) =>
                PathEnum::Prefix(PrefixEnd {
                    path: path.into(),
                    target,
                }),
            (false, _, true, true) | (true, false, true, true) => {
                let path: IndexStartPath = path.into();
                PathEnum::Postfix(PostfixEnd { path, root_pos })
            },
            _ => PathEnum::Range(RangeEnd {
                path,
                root_pos,
                target,
            }),
        }
    }
    pub(crate) fn from_start_path<G: HasGraph>(
        mut path: IndexStartPath,
        root_pos: AtomPosition,
        trav: &G,
    ) -> Self {
        path.role_path_mut().simplify(trav);
        match (
            path.is_at_border::<_, Start>(trav.graph()),
            path.raw_child_path().is_empty(),
        ) {
            (true, true) => PathEnum::Complete(path.into()),
            _ => PathEnum::Postfix(PostfixEnd { path, root_pos }),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum EndReason {
    QueryEnd,
    Mismatch,
}
// End types:
// - top down match-mismatch
// - top down match-query end
// - bottom up-no matching parents

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EndState {
    pub(crate) reason: EndReason,
    pub(crate) path: PathEnum,
    pub(crate) cursor: PatternCursor,
}
// impl_cursor_pos! {
//     CursorPosition for EndState, self => self.cursor.atom_position
// }

impl Traceable for &EndState {
    fn trace<G: HasGraph>(
        self,
        ctx: &mut TraceCtx<G>,
    ) {
        match &self.path {
            PathEnum::Range(p) => p.trace(ctx),
            PathEnum::Prefix(p) => p.trace(ctx),
            PathEnum::Postfix(p) => p.trace(ctx),
            _ => {},
        }
    }
}
impl EndState {
    pub(crate) fn init_fold(cursor: PatternCursor) -> MatchState {
        // Initially, we have a query pattern that hasn't been located in the graph yet
        MatchState::Query(cursor.path)
    }
    pub(crate) fn with_reason<G: HasGraph>(
        trav: G,
        reason: EndReason,
        parent: ParentCompareState,
    ) -> Self {
        let root_pos = *parent.parent_state.root_pos();
        Self {
            reason,
            path: PathEnum::from_start_path(
                parent.parent_state.into_rooted_path(),
                root_pos,
                &trav,
            ),
            cursor: parent.cursor,
        }
    }
    pub(crate) fn query_end<G: HasGraph>(
        trav: G,
        parent: ParentCompareState,
    ) -> Self {
        Self::with_reason(trav, EndReason::QueryEnd, parent)
    }
    //pub(crate) fn complete(
    //    trav: G,
    //    parent: ParentCompareState,
    //) -> Self {
    //    Self::with_reason(trav, EndReason::QueryEnd, parent)
    //}
    pub(crate) fn mismatch<G: HasGraph>(
        trav: G,
        parent: ParentCompareState,
    ) -> Self {
        Self::with_reason(trav, EndReason::Mismatch, parent)
    }
    pub(crate) fn start_len(&self) -> usize {
        self.start_path().map(|p| p.len()).unwrap_or_default()
    }
    pub(crate) fn start_path(&self) -> Option<&'_ StartPath> {
        match &self.path {
            PathEnum::Range(e) => Some(e.path.start_path()),
            PathEnum::Postfix(e) => Some(e.path.start_path()),
            PathEnum::Prefix(_) => None,
            PathEnum::Complete(_) => None,
        }
    }
    pub(crate) fn is_final(&self) -> bool {
        self.reason == EndReason::QueryEnd
            && matches!(self.path, PathEnum::Complete(_))
    }
    pub(crate) fn entry_location(&self) -> Option<ChildLocation> {
        match &self.path {
            PathEnum::Range(state) => Some(
                GraphRootChild::<Start>::graph_root_child_location(&state.path),
            ),
            PathEnum::Postfix(_) => None,
            PathEnum::Prefix(_) => None,
            PathEnum::Complete(_) => None,
        }
    }
    pub(crate) fn state_direction(&self) -> StateDirection {
        match self.path {
            PathEnum::Range(_) => StateDirection::TopDown,
            PathEnum::Postfix(_) => StateDirection::BottomUp,
            PathEnum::Prefix(_) => StateDirection::TopDown,
            PathEnum::Complete(_) => StateDirection::BottomUp,
        }
    }
    pub(crate) fn end_path(&self) -> Option<&'_ EndPath> {
        match &self.path {
            PathEnum::Range(e) => Some(e.path.end_path()),
            PathEnum::Postfix(_) => None,
            PathEnum::Prefix(e) => Some(e.path.end_path()),
            PathEnum::Complete(_) => None,
        }
    }
    pub(crate) fn is_complete(&self) -> bool {
        matches!(self.path, PathEnum::Complete(_))
    }
}

//impl TargetKey for EndState {
//    fn target_key(&self) -> DirectedKey {
//        match &self.path {
//            PathEnum::Range(p) => p.target.into(),
//            PathEnum::Postfix(_) => self.root_key().into(),
//            PathEnum::Prefix(p) => p.target.into(),
//            PathEnum::Complete(c) => c.target_key(),
//        }
//    }
//}

impl RootKey for EndState {
    fn root_key(&self) -> UpKey {
        UpKey::new(
            match &self.path {
                PathEnum::Range(s) => s.path.root_parent(),
                PathEnum::Postfix(p) => p.path.root_parent(),
                PathEnum::Prefix(p) => p.path.root_parent(),
                PathEnum::Complete(c) => c.root_parent(),
            },
            match &self.path {
                PathEnum::Range(s) => s.root_pos.into(),
                PathEnum::Postfix(p) => p.root_pos.into(),
                PathEnum::Prefix(_) => 0.into(),
                PathEnum::Complete(_) => 0.into(),
            },
        )
    }
}
impl_root! { GraphRoot for EndState, self =>
    match &self.path {
        PathEnum::Complete(c) => c.root_parent(),
        PathEnum::Range(p) => p.path.root_parent(),
        PathEnum::Postfix(p) => p.path.root_parent(),
        PathEnum::Prefix(p) => p.path.root_parent(),
    }
}
