use context_trace::{
    path::{
        accessors::has_path::{
            HasRolePath,
            HasRootedRolePath,
        },
        RolePathUtils,
    },
    *,
};
use postfix::PostfixEnd;
use prefix::PrefixEnd;
use range::RangeEnd;

use crate::{
    compare::parent::ParentCompareState,
    traversal::state::cursor::PatternCursor,
};

pub(crate) mod postfix;
pub(crate) mod prefix;
pub(crate) mod range;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum EndKind {
    Range(RangeEnd),
    Postfix(PostfixEnd),
    Prefix(PrefixEnd),
    Complete(Child),
}
impl EndKind {
    pub(crate) fn from_range_path<G: HasGraph>(
        mut path: IndexRangePath,
        root_pos: TokenPosition,
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
            (true, true, true, true) => EndKind::Complete(path.root_parent()),
            (true, true, false, _) | (true, true, true, false) =>
                EndKind::Prefix(PrefixEnd {
                    path: path.into(),
                    target,
                }),
            (false, _, true, true) | (true, false, true, true) => {
                let path: IndexStartPath = path.into();
                EndKind::Postfix(PostfixEnd { path, root_pos })
            },
            _ => EndKind::Range(RangeEnd {
                path,
                root_pos,
                target,
            }),
        }
    }
    pub(crate) fn from_start_path<G: HasGraph>(
        mut path: IndexStartPath,
        root_pos: TokenPosition,
        trav: &G,
    ) -> Self {
        path.role_path_mut().simplify(trav);
        match (
            path.is_at_border::<_, Start>(trav.graph()),
            path.raw_child_path().is_empty(),
        ) {
            (true, true) => EndKind::Complete(path.root_parent()),
            _ => EndKind::Postfix(PostfixEnd { path, root_pos }),
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

#[derive(Clone, Debug)]
pub(crate) struct TraceStart<'a>(pub(crate) &'a EndState, pub(crate) usize);

impl Traceable for TraceStart<'_> {
    fn trace<G: HasGraph>(
        self,
        ctx: &mut TraceCtx<G>,
    ) {
        if let Some(mut p) = match self.0.kind.clone() {
            EndKind::Postfix(p) => Some(p),
            EndKind::Range(p) => Some(PostfixEnd {
                path: p.path.into_rooted_role_path(),
                root_pos: p.root_pos,
            }),
            _ => None,
        } {
            p.rooted_role_path_mut().drain(0..self.1);
            p.trace(ctx);
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EndState {
    pub(crate) reason: EndReason,
    pub(crate) kind: EndKind,
    pub cursor: PatternCursor,
}
impl_cursor_pos! {
    CursorPosition for EndState, self => self.cursor.relative_pos
}

impl Traceable for &EndState {
    fn trace<G: HasGraph>(
        self,
        ctx: &mut TraceCtx<G>,
    ) {
        match &self.kind {
            EndKind::Range(p) => p.trace(ctx),
            EndKind::Prefix(p) => p.trace(ctx),
            EndKind::Postfix(p) => p.trace(ctx),
            _ => {},
        }
    }
}
impl EndState {
    pub(crate) fn with_reason<G: HasGraph>(
        trav: G,
        reason: EndReason,
        parent: ParentCompareState,
    ) -> Self {
        let root_pos = *parent.parent_state.root_pos();
        Self {
            reason,
            kind: EndKind::from_start_path(
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
    pub(crate) fn mismatch<G: HasGraph>(
        trav: G,
        parent: ParentCompareState,
    ) -> Self {
        Self::with_reason(trav, EndReason::Mismatch, parent)
    }
    pub(crate) fn is_final(&self) -> bool {
        self.reason == EndReason::QueryEnd
            && matches!(self.kind, EndKind::Complete(_))
    }
    pub(crate) fn entry_location(&self) -> Option<ChildLocation> {
        match &self.kind {
            EndKind::Range(state) =>
                Some(GraphRootChild::<Start>::root_child_location(&state.path)),
            EndKind::Postfix(_) => None,
            EndKind::Prefix(_) => None,
            EndKind::Complete(_) => None,
        }
    }
    pub(crate) fn state_direction(&self) -> StateDirection {
        match self.kind {
            EndKind::Range(_) => StateDirection::TopDown,
            EndKind::Postfix(_) => StateDirection::BottomUp,
            EndKind::Prefix(_) => StateDirection::TopDown,
            EndKind::Complete(_) => StateDirection::BottomUp,
        }
    }
    pub(crate) fn start_len(&self) -> usize {
        self.start_path().map(|p| p.len()).unwrap_or_default()
    }
    pub(crate) fn start_path(&self) -> Option<&'_ StartPath> {
        match &self.kind {
            EndKind::Range(e) => Some(e.path.start_path()),
            EndKind::Postfix(e) => Some(e.path.start_path()),
            EndKind::Prefix(_) => None,
            EndKind::Complete(_) => None,
        }
    }
    pub(crate) fn end_path(&self) -> Option<&'_ EndPath> {
        match &self.kind {
            EndKind::Range(e) => Some(e.path.end_path()),
            EndKind::Postfix(_) => None,
            EndKind::Prefix(e) => Some(e.path.end_path()),
            EndKind::Complete(_) => None,
        }
    }
    pub(crate) fn is_complete(&self) -> bool {
        matches!(self.kind, EndKind::Complete(_))
    }
}

impl TargetKey for EndState {
    fn target_key(&self) -> DirectedKey {
        match &self.kind {
            EndKind::Range(p) => p.target.into(),
            EndKind::Postfix(_) => self.root_key().into(),
            EndKind::Prefix(p) => p.target.into(),
            EndKind::Complete(c) => DirectedKey::up(*c, *self.cursor_pos()),
        }
    }
}

impl RootKey for EndState {
    fn root_key(&self) -> UpKey {
        UpKey::new(
            match &self.kind {
                EndKind::Range(s) => s.path.root_parent(),
                EndKind::Postfix(p) => p.path.root_parent(),
                EndKind::Prefix(p) => p.path.root_parent(),
                EndKind::Complete(c) => *c,
            },
            match &self.kind {
                EndKind::Range(s) => s.root_pos.into(),
                EndKind::Postfix(p) => p.root_pos.into(),
                EndKind::Prefix(_) => 0.into(),
                EndKind::Complete(_) => 0.into(),
            },
        )
    }
}
impl_root! { GraphRoot for EndState, self =>
    match &self.kind {
        EndKind::Complete(c) => *c,
        EndKind::Range(p) => p.path.root_parent(),
        EndKind::Postfix(p) => p.path.root_parent(),
        EndKind::Prefix(p) => p.path.root_parent(),
    }
}
