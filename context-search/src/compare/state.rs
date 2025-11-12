use crate::{
    compare::parent::ParentCompareState,
    cursor::PatternCursor,
    state::end::{
        EndReason,
        EndState,
        PathEnum,
    },
};
use context_trace::{
    graph::vertex::token::{
        HasSubLocation,
        SubToken,
    },
    path::{
        accessors::child::RootedLeafToken,
        RolePathUtils,
    },
    *,
};
use derive_more::{
    Deref,
    DerefMut,
};
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::VecDeque,
    fmt::Debug,
};
use CompareNext::*;
use PathPairMode::*;

pub(crate) type CompareQueue = VecDeque<CompareState>;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub(crate) enum PathPairMode {
    GraphMajor,
    QueryMajor,
}
#[derive(Clone, Debug)]
pub(crate) enum TokenMatchState {
    Mismatch(EndState),
    Match(CompareState),
}
use TokenMatchState::*;

#[derive(Clone, Debug, PartialEq, Eq, Deref, DerefMut)]
pub(crate) struct CompareState {
    #[deref]
    #[deref_mut]
    pub(crate) child_state: ChildState,
    /// Current cursor position (may be candidate or matched)
    pub(crate) cursor: PatternCursor,
    /// Last confirmed matched cursor position
    pub(crate) matched_cursor: PatternCursor,
    pub(crate) target: DownKey,
    pub(crate) mode: PathPairMode,
}
#[derive(Clone, Debug)]
pub(crate) enum CompareNext {
    MatchState(TokenMatchState),
    Prefixes(ChildQueue<CompareState>),
}
impl CompareState {
    fn mode_prefixes<G: HasGraph>(
        &self,
        trav: &G,
        mode: PathPairMode,
    ) -> ChildQueue<Self> {
        Self {
            mode,
            ..self.clone()
        }
        .prefix_states(trav)
    }
    pub(crate) fn parent_state(&self) -> ParentCompareState {
        ParentCompareState {
            parent_state: self.child_state.parent_state(),
            cursor: self.cursor.clone(),
        }
    }
    /// generate token states for index prefixes
    pub(crate) fn prefix_states<G: HasGraph>(
        &self,
        trav: &G,
    ) -> ChildQueue<Self> {
        match self.mode {
            GraphMajor => self
                .child_state
                .prefix_states(trav)
                .into_iter()
                .map(|(sub, child_state)| Self {
                    target: DownKey::new(
                        sub.token(),
                        (*self.matched_cursor.cursor_pos()).into(),
                    ),
                    child_state,
                    mode: self.mode,
                    cursor: self.cursor.clone(),
                    matched_cursor: self.matched_cursor.clone(),
                })
                .collect(),
            QueryMajor => self
                .cursor
                .prefix_states(trav)
                .into_iter()
                .map(|(sub, cursor)| Self {
                    target: DownKey::new(
                        sub.token(),
                        (*self.matched_cursor.cursor_pos()).into(),
                    ),
                    child_state: self.child_state.clone(),
                    mode: self.mode,
                    cursor,
                    matched_cursor: self.matched_cursor.clone(),
                })
                .collect(),
        }
    }
    pub(crate) fn next_match<G: HasGraph>(
        self,
        trav: &G,
    ) -> CompareNext {
        use Ordering::*;
        let path_leaf =
            self.rooted_path().role_rooted_leaf_token::<End, _>(trav);
        let query_leaf = self.cursor.role_rooted_leaf_token::<End, _>(trav);

        if path_leaf == query_leaf {
            //debug!(
            //    "Matched\n\tlabel: {}\n\troot: {}\n\tpos: {}",
            //    trav.graph().index_string(path_leaf),
            //    trav.graph().index_string(self.path.root.location.parent),
            //    self.cursor.width()
            //);
            MatchState(Match(self))
        } else if path_leaf.width() == 1 && query_leaf.width() == 1 {
            MatchState(Mismatch(self.on_mismatch(trav)))
        } else {
            Prefixes(match path_leaf.width().cmp(&query_leaf.width()) {
                Equal => self
                    .mode_prefixes(trav, GraphMajor)
                    .into_iter()
                    .chain(self.mode_prefixes(trav, QueryMajor))
                    .collect(),
                Greater => self.mode_prefixes(trav, GraphMajor),
                Less => self.mode_prefixes(trav, QueryMajor),
            })
        }
    }

    fn on_mismatch<G: HasGraph>(
        self,
        trav: &G,
    ) -> EndState {
        use EndReason::*;
        use PathEnum::*;
        let BaseState {
            prev_pos,
            mut path,
            mut root_pos,
        } = self.child_state.base;

        // TODO: Fix this
        let index = loop {
            if path.role_root_child_index::<Start>()
                == path.role_root_child_index::<End>()
            {
                if (&mut root_pos, &mut path).path_lower(trav).is_break() {
                    let graph = trav.graph();
                    let pattern = graph.expect_pattern_at(
                        path.clone().path_root().pattern_location(),
                    );
                    let entry = path.start_path().root_child_index();
                    *root_pos = *prev_pos;
                    break Some(pattern[entry]);
                }
            } else {
                break None;
            }
        };
        let cursor = self.cursor;
        let kind = if let Some(_) = index {
            Complete(path)
        } else {
            let target = DownKey::new(
                path.role_rooted_leaf_token::<End, _>(trav),
                cursor.atom_position.into(),
            );
            PathEnum::from_range_path(path, root_pos, target, trav)
        };
        EndState {
            reason: Mismatch,
            cursor,
            path: kind,
        }
    }
}

impl From<CompareState> for ChildQueue<CompareState> {
    fn from(val: CompareState) -> Self {
        ChildQueue::from_iter([val])
    }
}

impl IntoAdvanced for CompareState {
    type Next = Self;
    fn into_advanced<G: HasGraph>(
        self,
        trav: &G,
    ) -> Result<Self, Self> {
        match self.child_state.into_advanced(trav) {
            Ok(child_state) => Ok(Self {
                child_state,
                ..self
            }),
            Err(child_state) => Ok(Self {
                child_state,
                ..self
            }),
        }
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
        let leaf = self.role_rooted_leaf_token::<End, _>(trav);
        trav.graph()
            .expect_vertex(leaf)
            .prefix_children::<G>()
            .iter()
            .sorted_unstable_by(|a, b| {
                b.token().width().cmp(&a.token().width())
            })
            .map(|sub| {
                let mut next = self.clone();
                next.path_append(leaf.to_child_location(*sub.sub_location()));
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
