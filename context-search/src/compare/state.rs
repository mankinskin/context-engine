use crate::{
    compare::parent::ParentCompareState,
    cursor::{
        Candidate,
        CursorState,
        MarkMatchState,
        Matched,
        Mismatched,
        PathCursor,
        PatternCursor,
        PatternPrefixCursor,
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
        accessors::{
            child::RootedLeafToken,
            has_path::IntoRootedRolePath,
        },
        RolePathUtils,
    },
    PatternPrefixPath,
    RootChildIndex,
    RootChildIndexMut,
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
    ops::ControlFlow::{
        self,
        Break,
        Continue,
    },
};
use tracing::debug;
use CompareNext::*;
use PathPairMode::*;

pub(crate) type CompareQueue = VecDeque<CompareState<Candidate>>;

// Type aliases for clarity
pub(crate) type CandidateCompareState = CompareState<Candidate>;
pub(crate) type MatchedCompareState = CompareState<Matched>;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub(crate) enum PathPairMode {
    GraphMajor,
    QueryMajor,
}

/// State for comparing a candidate position against the graph.
///
/// Generic over state `S` to track the cursor's processing status:
/// - `CompareState<Candidate>` - Unprocessed, cursor exploring ahead (Candidate state)
/// - `CompareState<Matched>` - Processed and matched successfully (cursor in Matched state)
/// - `CompareState<Mismatched>` - Processed but failed to match (cursor in Mismatched state)
///
/// # Cursor State Semantics
///
/// This struct maintains two cursors that track different states:
///
/// - **`cursor`** (state controlled by generic `S`): The position being evaluated for matching.
///   - In `CompareState<Candidate>`: exploring ahead to test if tokens match
///   - In `CompareState<Matched>`: confirmed as matching
///   - In `CompareState<Mismatched>`: confirmed as mismatched
///   - `atom_position` tracks how many atoms have been scanned (including this candidate)
///   - Uses PatternPrefixPath (only End role) for efficient comparison
///
/// - **`checkpoint`** (always Matched state): The last confirmed matching position.
///   - Marks where we were BEFORE advancing into the current token
///   - Always in Matched state (represents confirmed progress)
///   - `atom_position` reflects confirmed consumed atoms
///   - Updated by RootCursor after confirming a match is part of the largest contiguous sequence
///   - Uses PatternRangePath (Start and End) to track the matched range
///
/// # atom_position Tracking
///
/// The `atom_position` field represents the number of atoms consumed:
/// - Starts at 0 at the beginning of a pattern
/// - Increments by token width when advancing
/// - For prefix decomposition: accumulates widths across sub-tokens
/// - checkpoint.atom_position ≤ cursor.atom_position (cursor explores ahead)
///
/// Example: Matching pattern [a,b,c] where b,c form a composite token "bc":
/// ```text
/// Position:  0    1    2    3
/// Pattern:   a    b    c    
/// Tokens:    a   [bc]
///
/// After matching 'a':
///   checkpoint.atom_position = 1 (consumed 'a')
///   cursor.atom_position = 1 (about to test 'bc')
///
/// While testing 'bc':
///   checkpoint.atom_position = 1 (still at 'a')
///   cursor.atom_position = 3 (would consume through 'c')
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Deref, DerefMut)]
pub(crate) struct CompareState<S: CursorState = Candidate> {
    #[deref]
    #[deref_mut]
    pub(crate) child_state: ChildState,

    /// Cursor: state controlled by generic parameter S
    pub(crate) cursor: PathCursor<PatternPrefixPath, S>,

    /// Checkpoint: last confirmed match (always Matched state)
    pub(crate) checkpoint: PathCursor<PatternRangePath, Matched>,

    pub(crate) target: DownKey,
    pub(crate) mode: PathPairMode,
}

#[derive(Clone, Debug)]
pub(crate) enum CompareNext {
    /// Result of comparing the candidate (matched or mismatched)
    Match(CompareState<Matched>),
    Mismatch(CompareState<Mismatched>),
    /// Candidate needs decomposition into prefixes for comparison
    Prefixes(ChildQueue<CompareState<Candidate>>),
}

impl CompareState<Candidate> {
    pub(crate) fn parent_state(&self) -> ParentCompareState {
        // Convert prefix path cursor back to range path for parent state
        let range_path = self.checkpoint.path.clone();
        let cursor = PathCursor {
            path: range_path,
            atom_position: self.cursor.atom_position,
            _state: PhantomData,
        };

        ParentCompareState {
            parent_state: self.child_state.parent_state(),
            cursor,
        }
    }

    /// Update the checkpoint with the candidate cursor's position
    /// Returns a PatternCursor (range path) that can be marked as mismatched
    fn update_checkpoint_with_candidate(&self) -> PatternCursor {
        PathCursor {
            path: self.checkpoint.path.clone(),
            atom_position: self.cursor.atom_position,
            _state: PhantomData,
        }
    }

    fn mode_prefixes<G: HasGraph>(
        &self,
        trav: &G,
        mode: PathPairMode,
    ) -> ChildQueue<CompareState<Candidate>> {
        tracing::debug!(
            old_mode = ?self.mode,
            new_mode = ?mode,
            "mode_prefixes: creating new state with different mode"
        );
        CompareState {
            mode,
            ..self.clone()
        }
        .prefix_states(trav)
    }

    /// Generate token states for index prefixes.
    ///
    /// Decomposes composite tokens into their constituent sub-tokens for finer-grained comparison.
    /// - GraphMajor mode: Decomposes the graph path token
    /// - QueryMajor mode: Decomposes the query cursor token (with proper atom_position tracking)
    pub(crate) fn prefix_states<G: HasGraph>(
        &self,
        trav: &G,
    ) -> ChildQueue<CompareState<Candidate>> {
        tracing::debug!(
            mode = ?self.mode,
            child_state = ?self.child_state,
            cursor = ?self.cursor,
            "==> ENTERING prefix_states"
        );

        match self.mode {
            GraphMajor => {
                let checkpoint_pos = *self.checkpoint.cursor_pos();
                tracing::debug!(
                    "GraphMajor: calling child_state.prefix_states"
                );
                let prefixes = self.child_state.prefix_states(trav);

                tracing::trace!(
                    mode = "GraphMajor",
                    num_prefixes = prefixes.len(),
                    checkpoint_pos = ?checkpoint_pos,
                    "decomposing graph path token into prefixes"
                );

                let result: ChildQueue<CompareState<Candidate>> = prefixes
                    .into_iter()
                    .enumerate()
                    .map(|(i, (sub, child_state))| {
                        let token = sub.token();
                        let target_pos = checkpoint_pos.into();
                        tracing::debug!(
                            prefix_idx = i,
                            sub_width = token.width(),
                            "GraphMajor: creating prefix state"
                        );
                        CompareState {
                            target: DownKey::new(token, target_pos),
                            child_state,
                            mode: self.mode,
                            cursor: self.cursor.clone(),
                            checkpoint: self.checkpoint.clone(),
                        }
                    })
                    .collect();
                tracing::debug!(
                    num_results = result.len(),
                    "<== EXITING prefix_states (GraphMajor)"
                );
                result
            },
            QueryMajor => {
                // When decomposing the query cursor's token into prefixes, we need to track
                // position relative to the checkpoint, not the advanced cursor position
                let base_position = self.checkpoint.atom_position;
                tracing::debug!(
                    "QueryMajor: calling cursor.prefix_states_from"
                );
                let cursor_prefixes =
                    self.cursor.prefix_states_from(trav, base_position);

                tracing::trace!(
                    mode = "QueryMajor",
                    cursor_pos = ?self.cursor.atom_position,
                    base_pos = ?base_position,
                    num_prefixes = cursor_prefixes.len(),
                    "decomposing query cursor token into prefixes"
                );

                let result: ChildQueue<CompareState<Candidate>> =
                    cursor_prefixes
                        .into_iter()
                        .enumerate()
                        .map(|(i, (sub, cursor))| {
                            tracing::trace!(
                                prefix_idx = i,
                                sub_width = sub.token().width(),
                                cursor_pos = ?cursor.atom_position,
                                "created prefix state"
                            );
                            CompareState {
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
                tracing::debug!(
                    num_results = result.len(),
                    "<== EXITING prefix_states (QueryMajor)"
                );
                result
            },
        }
    }
    /// Compare a candidate against the graph to determine if tokens match.
    ///
    /// Returns:
    /// - `Match(CompareState<Matched>)` if tokens are identical - cursor transitions to Matched state
    /// - `Mismatch(CompareState<Mismatched>)` if both are atoms (width=1) and don't match
    /// - `Prefixes(queue)` if tokens need decomposition into sub-tokens for finer comparison
    ///
    /// The checkpoint (always Matched state) is NOT updated here - that's RootCursor's responsibility
    /// after determining this match is part of the largest contiguous match.
    pub(crate) fn next_match<G: HasGraph>(
        self,
        trav: &G,
    ) -> CompareNext {
        use Ordering::*;
        let path_leaf =
            self.rooted_path().role_rooted_leaf_token::<End, _>(trav);
        let query_leaf = self.cursor.role_rooted_leaf_token::<End, _>(trav);

        tracing::debug!(
            path_leaf = ?path_leaf,
            query_leaf = ?query_leaf,
            path_width = path_leaf.width(),
            query_width = query_leaf.width(),
            cursor_pos = ?self.cursor.atom_position,
            checkpoint_pos = ?self.checkpoint.atom_position,
            mode = ?self.mode,
            "==> next_match: comparing candidate tokens"
        );

        if path_leaf == query_leaf {
            tracing::debug!(
                token = path_leaf.index,
                width = path_leaf.width(),
                "tokens matched"
            );
            // Mark as matched using trait method
            CompareNext::Match(self.mark_match())
        } else if path_leaf.width() == 1 && query_leaf.width() == 1 {
            tracing::debug!(
                path_token = path_leaf.index,
                query_token = query_leaf.index,
                "atom mismatch - both width 1 but different"
            );
            // Mark as mismatched using trait method (checkpoint not updated here)
            CompareNext::Mismatch(self.mark_mismatch())
        } else {
            tracing::debug!(
                path_width = path_leaf.width(),
                query_width = query_leaf.width(),
                mode = ?match path_leaf.width().cmp(&query_leaf.width()) {
                    Equal => "both",
                    Greater => "graph_major",
                    Less => "query_major",
                },
                "tokens need decomposition - calling mode_prefixes"
            );
            let prefixes = match path_leaf.width().cmp(&query_leaf.width()) {
                Equal => {
                    tracing::debug!(
                        "Equal width: calling both GraphMajor and QueryMajor"
                    );
                    self.mode_prefixes(trav, GraphMajor)
                        .into_iter()
                        .chain(self.mode_prefixes(trav, QueryMajor))
                        .collect()
                },
                Greater => {
                    tracing::debug!("GraphMajor: path_width > query_width");
                    self.mode_prefixes(trav, GraphMajor)
                },
                Less => {
                    tracing::debug!("QueryMajor: path_width < query_width");
                    self.mode_prefixes(trav, QueryMajor)
                },
            };
            tracing::debug!(
                num_prefixes = prefixes.len(),
                "<== next_match: returning Prefixes"
            );
            Prefixes(prefixes)
        }
    }
}

impl MarkMatchState for CompareState<Candidate> {
    type Matched = CompareState<Matched>;
    type Mismatched = CompareState<Mismatched>;

    fn mark_match(self) -> Self::Matched {
        CompareState {
            child_state: self.child_state,
            cursor: self.cursor.mark_match(),
            checkpoint: self.checkpoint,
            target: self.target,
            mode: self.mode,
        }
    }

    fn mark_mismatch(self) -> Self::Mismatched {
        CompareState {
            child_state: self.child_state,
            cursor: self.cursor.mark_mismatch(),
            checkpoint: self.checkpoint,
            target: self.target,
            mode: self.mode,
        }
    }
}

impl CompareState<Matched> {
    /// Convert a matched state to a candidate state for the next comparison.
    /// - Old matched cursor becomes the new checkpoint
    /// - Old matched cursor is advanced to get the new candidate cursor
    /// - Old checkpoint is discarded
    /// Uses move semantics to consume the matched state.
    pub(crate) fn into_next_candidate<G: HasGraph>(
        mut self,
        trav: &G,
    ) -> Result<CompareState<Candidate>, CompareState<Matched>> {
        tracing::debug!(
            cursor = ?self.cursor,
            "==> into_next_candidate: converting matched to candidate"
        );
        // Convert the old matched cursor to a checkpoint (PrefixPath -> RangePath)
        tracing::debug!("into_next_candidate: about to clone cursor");
        let cursor_clone = self.cursor.clone();
        tracing::debug!("into_next_candidate: cloned cursor, now converting to PatternCursor");
        let new_checkpoint: PatternCursor = cursor_clone.into();
        tracing::debug!("into_next_candidate: converted to PatternCursor");

        tracing::debug!("into_next_candidate: calling cursor.advance");
        // Advance the cursor to get the new candidate position
        match self.cursor.advance(trav) {
            Continue(_) => {
                tracing::debug!("into_next_candidate: advance succeeded");
                // Successfully advanced - convert to candidate
                let candidate_cursor = self.cursor.as_candidate();

                Ok(CompareState {
                    child_state: self.child_state,
                    cursor: candidate_cursor,
                    checkpoint: new_checkpoint,
                    target: self.target,
                    mode: self.mode,
                })
            },
            Break(_) => {
                tracing::debug!("into_next_candidate: advance failed, returning matched state");
                // Cannot advance - return the matched state back
                Err(self)
            },
        }
    }
}

impl From<CompareState<Candidate>> for ChildQueue<CompareState<Candidate>> {
    fn from(val: CompareState<Candidate>) -> Self {
        ChildQueue::from_iter([val])
    }
}

impl IntoAdvanced for CompareState<Candidate> {
    type Next = Self;
    fn into_advanced<G: HasGraph>(
        self,
        trav: &G,
    ) -> Result<Self, Self> {
        match self.child_state.into_advanced(trav) {
            Ok(child_state) => Ok(CompareState {
                child_state,
                ..self
            }),
            Err(child_state) => Ok(CompareState {
                child_state,
                ..self
            }),
        }
    }
}

impl IntoAdvanced for CompareState<Matched> {
    type Next = Self;
    fn into_advanced<G: HasGraph>(
        self,
        trav: &G,
    ) -> Result<Self, Self> {
        match self.child_state.into_advanced(trav) {
            Ok(child_state) => Ok(CompareState {
                child_state,
                cursor: self.cursor,
                checkpoint: self.checkpoint,
                target: self.target,
                mode: self.mode,
            }),
            Err(child_state) => Ok(CompareState {
                child_state,
                cursor: self.cursor,
                checkpoint: self.checkpoint,
                target: self.target,
                mode: self.mode,
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
        tracing::debug!(
            leaf = ?leaf,
            "==> PrefixStates trait: getting prefix_children"
        );
        let prefix_children =
            trav.graph().expect_vertex(leaf).prefix_children::<G>();
        tracing::debug!(
            num_children = prefix_children.len(),
            "PrefixStates trait: got prefix_children"
        );
        let result = prefix_children
            .iter()
            .sorted_unstable_by(|a, b| {
                b.token().width().cmp(&a.token().width())
            })
            .map(|sub| {
                let mut next = self.clone();
                next.path_append(leaf.to_child_location(*sub.sub_location()));
                (sub.clone(), next)
            })
            .collect();
        tracing::debug!("<== PrefixStates trait: returning prefixes");
        result
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
