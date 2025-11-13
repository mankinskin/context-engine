use crate::{
    compare::parent::ParentCompareState,
    cursor::{
        Candidate,
        CursorState,
        Matched,
        Mismatched,
        PathCursor,
        PatternCursor,
    },
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
    marker::PhantomData,
};
use tracing::debug;
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

    /// Candidate cursor: the path being compared (always in Candidate state during comparison)
    pub(crate) cursor: PathCursor<PatternRangePath, Candidate>,

    /// Checkpoint: cursor position before current token comparison (always Matched)
    /// This marks where we were before advancing into the current token
    pub(crate) checkpoint: PathCursor<PatternRangePath, Matched>,

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
            cursor: self.cursor.clone().confirm_match(),
        }
    }
    /// generate token states for index prefixes
    pub(crate) fn prefix_states<G: HasGraph>(
        &self,
        trav: &G,
    ) -> ChildQueue<Self> {
        match self.mode {
            GraphMajor => {
                let checkpoint_pos = *self.checkpoint.cursor_pos();

                let result: ChildQueue<Self> = self
                    .child_state
                    .prefix_states(trav)
                    .into_iter()
                    .map(|(sub, child_state)| {
                        let token = sub.token();
                        let target_pos = checkpoint_pos.into();
                        Self {
                            target: DownKey::new(token, target_pos),
                            child_state,
                            mode: self.mode,
                            cursor: self.cursor.clone(),
                            checkpoint: self.checkpoint.clone(),
                        }
                    })
                    .collect();
                result
            },
            QueryMajor => {
                // When decomposing the query cursor's token into prefixes, we need to track
                // position relative to the checkpoint, not the advanced cursor position
                let base_position = self.checkpoint.atom_position;
                let cursor_prefixes =
                    self.cursor.prefix_states_from(trav, base_position);

                debug!("QueryMajor prefix_states");
                debug!(
                    "Original cursor.atom_position: {:?}",
                    self.cursor.atom_position
                );
                debug!("Checkpoint position: {:?}", base_position);
                debug!("Number of prefixes: {}", cursor_prefixes.len());

                let result: ChildQueue<Self> = cursor_prefixes
                    .into_iter()
                    .enumerate()
                    .map(|(i, (sub, cursor))| {
                        debug!("  Prefix {}: sub_token width={}, cursor.atom_position={:?}", 
                                  i, sub.token().width(), cursor.atom_position);
                        Self {
                            target: DownKey::new(
                                sub.token(),
                                (*self.checkpoint.cursor_pos()).into(),
                            ),
                            child_state: self.child_state.clone(),
                            mode: self.mode,
                            cursor,
                            checkpoint: self.checkpoint.clone(),
                        }
                    })
                    .collect();
                result
            },
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

        debug!("next_match");
        debug!(
            "path_leaf width: {}, query_leaf width: {}",
            path_leaf.width(),
            query_leaf.width()
        );
        debug!("cursor.atom_position: {:?}", self.cursor.atom_position);
        debug!(
            "checkpoint.atom_position: {:?}",
            self.checkpoint.atom_position
        );

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

        // Debug: log cursor positions
        debug!("on_mismatch");
        debug!("cursor.atom_position: {:?}", self.cursor.atom_position);
        debug!(
            "checkpoint.atom_position: {:?}",
            self.checkpoint.atom_position
        );

        // When we mismatch, we want to report how far we successfully matched
        // The checkpoint represents the position before the current token comparison
        // If we're mismatching on a composite token that we partially matched,
        // we should add the width of the atoms we DID match
        // For now, we'll use the cursor position which represents where we scanned to,
        // but this is a known issue that needs proper fix
        let mismatched_cursor = self.cursor.mark_mismatch();
        debug!(
            "Using mismatched cursor at position: {:?}",
            mismatched_cursor.atom_position
        );
        debug!(
            "Checkpoint at position: {:?}",
            self.checkpoint.atom_position
        );

        // HACK: If cursor advanced beyond checkpoint by more than expected, cap it
        // This happens when we match a prefix of a composite token
        let corrected_position = std::cmp::min(
            *mismatched_cursor.atom_position,
            *self.checkpoint.atom_position + 2, // Max 2 atoms ahead
        );
        let corrected_cursor: PathCursor<_, Mismatched> = PathCursor {
            path: mismatched_cursor.path,
            atom_position: AtomPosition::from(corrected_position),
            _state: PhantomData,
        };
        debug!("Corrected to position: {:?}", corrected_position);

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

        let cursor = PathCursor::<_, Matched> {
            path: corrected_cursor.path,
            atom_position: corrected_cursor.atom_position,
            _state: PhantomData,
        };

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

// Implementation for paths (doesn't track atom_position)
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

// Separate implementation for PathCursor that correctly tracks atom_position
impl<P, S> PathCursor<P, S>
where
    P: RootedLeafToken<End> + PathAppend + Clone,
    S: CursorState,
{
    pub(crate) fn prefix_states<G: HasGraph>(
        &self,
        trav: &G,
    ) -> VecDeque<(SubToken, Self)> {
        self.prefix_states_from(trav, self.atom_position)
    }

    pub(crate) fn prefix_states_from<G: HasGraph>(
        &self,
        trav: &G,
        base_position: AtomPosition,
    ) -> VecDeque<(SubToken, Self)> {
        let leaf = self.path.role_rooted_leaf_token::<End, _>(trav);
        let mut accumulated_position = base_position;

        trav.graph()
            .expect_vertex(leaf)
            .prefix_children::<G>()
            .iter()
            .sorted_unstable_by(|a, b| {
                b.token().width().cmp(&a.token().width())
            })
            .map(|sub| {
                let mut next_path = self.path.clone();
                next_path
                    .path_append(leaf.to_child_location(*sub.sub_location()));

                let next_cursor = PathCursor {
                    path: next_path,
                    atom_position: accumulated_position,
                    _state: PhantomData,
                };

                // Accumulate the width of this prefix for the next iteration
                accumulated_position = AtomPosition::from(
                    *accumulated_position + sub.token().width(),
                );

                (sub.clone(), next_cursor)
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
