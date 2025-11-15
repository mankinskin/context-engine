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
    logging::compact_format::Compact,
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
use tracing::{
    debug,
    trace,
};
use CompareResult::*;
use PathPairMode::*;

pub(crate) type CompareQueue = VecDeque<CompareState<Candidate, Candidate>>;

// Type aliases for clarity
pub(crate) type CandidateCompareState = CompareState<Candidate, Candidate>;
pub(crate) type MatchedCompareState = CompareState<Matched, Matched>;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub(crate) enum PathPairMode {
    GraphMajor,
    QueryMajor,
}

impl std::fmt::Display for PathPairMode {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        match self {
            PathPairMode::GraphMajor => write!(f, "GraphMajor"),
            PathPairMode::QueryMajor => write!(f, "QueryMajor"),
        }
    }
}

/// State for comparing a candidate position against the graph.
///
/// Generic over TWO states `Q` (query) and `I` (index) to track both cursors' processing status:
/// - `CompareState<Candidate, Candidate>` - Both cursors unprocessed, exploring ahead
/// - `CompareState<Matched, Matched>` - Both cursors matched successfully
/// - `CompareState<Mismatched, Mismatched>` - Both cursors failed to match
///
/// # Cursor State Semantics
///
/// This struct maintains THREE cursors that track different states:
///
/// - **`cursor`** (state controlled by generic `Q`): The query position being evaluated.
///   - In `CompareState<Candidate, _>`: exploring ahead to test if tokens match
///   - In `CompareState<Matched, _>`: confirmed as matching
///   - In `CompareState<Mismatched, _>`: confirmed as mismatched
///   - `atom_position` tracks how many atoms have been scanned (including this candidate)
///   - Uses PatternPrefixPath (only End role) for efficient comparison
///
/// - **`index_cursor`** (state controlled by generic `I`): The index/graph position being evaluated.
///   - Tracks position in the graph path independently from query cursor
///   - State transitions separately, enabling independent query and index advancement
///   - `atom_position` tracks atoms consumed in the index path
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CompareState<
    Q: CursorState = Candidate,
    I: CursorState = Candidate,
> {
    pub(crate) child_state: ChildState,

    /// Query cursor: state controlled by generic parameter Q
    pub(crate) cursor: PathCursor<PatternPrefixPath, Q>,

    /// Index cursor: state controlled by generic parameter I
    /// Tracks the position and state in the graph/index path
    pub(crate) index_cursor: PathCursor<PatternPrefixPath, I>,

    /// Checkpoint: last confirmed match (always Matched state)
    pub(crate) checkpoint: PathCursor<PatternRangePath, Matched>,

    pub(crate) target: DownKey,
    pub(crate) mode: PathPairMode,
}

impl<Q: CursorState, I: CursorState> CompareState<Q, I> {
    /// Access the rooted path from the child state
    pub(crate) fn rooted_path(&self) -> &IndexRangePath {
        self.child_state.rooted_path()
    }
}

#[derive(Clone, Debug)]
pub(crate) enum CompareResult {
    /// Result of comparing the candidate (matched or mismatched)
    /// Both query and index cursors remain in their respective states
    FoundMatch(CompareState<Matched, Matched>),
    Mismatch(CompareState<Mismatched, Mismatched>),
    /// Candidate needs decomposition into prefixes for comparison
    Prefixes(ChildQueue<CompareState<Candidate, Candidate>>),
}

impl CompareState<Candidate, Candidate> {
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
    ) -> ChildQueue<CompareState<Candidate, Candidate>> {
        debug!(
            old_mode = %self.mode,
            new_mode = %mode,
            "creating new state with different mode"
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
    ) -> ChildQueue<CompareState<Candidate, Candidate>> {
        debug!(
            mode = %self.mode,
            child_state = %self.child_state,
            cursor = %self.cursor,
            "entering prefix_states"
        );

        match self.mode {
            GraphMajor => {
                let checkpoint_pos = *self.checkpoint.cursor_pos();
                debug!("calling child_state.prefix_states");
                let prefixes = self.child_state.prefix_states(trav);

                trace!(
                    mode = "GraphMajor",
                    num_prefixes = prefixes.len(),
                    checkpoint_pos = %checkpoint_pos,
                    "decomposing graph path token into prefixes"
                );

                let result: ChildQueue<CompareState<Candidate, Candidate>> =
                    prefixes
                        .into_iter()
                        .enumerate()
                        .map(|(i, (sub, child_state))| {
                            let token = sub.token();
                            let target_pos = checkpoint_pos.into();
                            debug!(
                                prefix_idx = i,
                                sub_width = token.width(),
                                "creating prefix state"
                            );
                            CompareState {
                                target: DownKey::new(token, target_pos),
                                child_state,
                                mode: self.mode,
                                cursor: self.cursor.clone(),
                                index_cursor: self.index_cursor.clone(),
                                checkpoint: self.checkpoint.clone(),
                            }
                        })
                        .collect();
                debug!(
                    num_results = result.len(),
                    "exiting prefix_states (GraphMajor)"
                );
                result
            },
            QueryMajor => {
                // When decomposing the query cursor's token into prefixes, we need to track
                // position relative to the checkpoint, not the advanced cursor position
                let base_position = self.checkpoint.atom_position;
                debug!("calling cursor.prefix_states_from");
                let cursor_prefixes =
                    self.cursor.prefix_states_from(trav, base_position);

                trace!(
                    mode = "QueryMajor",
                    cursor_pos = %self.cursor.atom_position,
                    base_pos = %base_position,
                    num_prefixes = cursor_prefixes.len(),
                    "decomposing query cursor token into prefixes"
                );

                let result: ChildQueue<CompareState<Candidate, Candidate>> =
                    cursor_prefixes
                        .into_iter()
                        .enumerate()
                        .map(|(i, (sub, cursor))| {
                            trace!(
                                prefix_idx = i,
                                sub_width = sub.token().width(),
                                cursor_pos = %cursor.atom_position,
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
                                index_cursor: self.index_cursor.clone(),
                                checkpoint: self.checkpoint.clone(),
                            }
                        })
                        .collect();
                debug!(
                    num_results = result.len(),
                    "exiting prefix_states (QueryMajor)"
                );
                result
            },
        }
    }
    /// Compare a candidate against the graph to determine if tokens match.
    ///
    /// Returns:
    /// - `FoundMatch(CompareState<Matched, Matched>)` if tokens are identical - both cursors transition to Matched
    /// - `Mismatch(CompareState<Mismatched, Mismatched>)` if both are atoms (width=1) and don't match
    /// - `Prefixes(queue)` if tokens need decomposition into sub-tokens for finer comparison
    ///
    /// The checkpoint (always Matched state) is NOT updated here - that's RootCursor's responsibility
    /// after determining this match is part of the largest contiguous match.
    pub(crate) fn next_match<G: HasGraph>(
        self,
        trav: &G,
    ) -> CompareResult {
        use Ordering::*;
        let path_leaf =
            self.rooted_path().role_rooted_leaf_token::<End, _>(trav);
        let query_leaf = self.cursor.role_rooted_leaf_token::<End, _>(trav);

        debug!(
            path_leaf = %path_leaf,
            query_leaf = %query_leaf,
            path_width = path_leaf.width(),
            query_width = query_leaf.width(),
            cursor_pos = %self.cursor.atom_position,
            checkpoint_pos = %self.checkpoint.atom_position,
            mode = %self.mode,
            "comparing candidate tokens"
        );

        if path_leaf == query_leaf {
            debug!(
                token = *path_leaf.index,
                width = path_leaf.width(),
                "tokens matched"
            );
            // Mark as matched using trait method
            CompareResult::FoundMatch(self.mark_match())
        } else if path_leaf.width() == 1 && query_leaf.width() == 1 {
            debug!(
                path_token = *path_leaf.index,
                query_token = *query_leaf.index,
                "atom mismatch - both width 1 but different"
            );
            // Mark as mismatched using trait method (checkpoint not updated here)
            CompareResult::Mismatch(self.mark_mismatch())
        } else {
            debug!(
                path_width = path_leaf.width(),
                query_width = query_leaf.width(),
                mode = %self.mode,
                "tokens need decomposition - calling mode_prefixes"
            );
            let prefixes = match path_leaf.width().cmp(&query_leaf.width()) {
                Equal => {
                    debug!(
                        "equal width: calling both GraphMajor and QueryMajor"
                    );
                    self.mode_prefixes(trav, GraphMajor)
                        .into_iter()
                        .chain(self.mode_prefixes(trav, QueryMajor))
                        .collect()
                },
                Greater => {
                    debug!("GraphMajor: path_width > query_width");
                    self.mode_prefixes(trav, GraphMajor)
                },
                Less => {
                    debug!("QueryMajor: path_width < query_width");
                    self.mode_prefixes(trav, QueryMajor)
                },
            };
            debug!(num_prefixes = prefixes.len(), "returning Prefixes result");
            Prefixes(prefixes)
        }
    }
}

impl MarkMatchState for CompareState<Candidate, Candidate> {
    type Matched = CompareState<Matched, Matched>;
    type Mismatched = CompareState<Mismatched, Mismatched>;

    fn mark_match(self) -> Self::Matched {
        CompareState {
            child_state: self.child_state,
            cursor: self.cursor.mark_match(),
            index_cursor: self.index_cursor.mark_match(),
            checkpoint: self.checkpoint,
            target: self.target,
            mode: self.mode,
        }
    }

    fn mark_mismatch(self) -> Self::Mismatched {
        CompareState {
            child_state: self.child_state,
            cursor: self.cursor.mark_mismatch(),
            index_cursor: self.index_cursor.mark_mismatch(),
            checkpoint: self.checkpoint,
            target: self.target,
            mode: self.mode,
        }
    }
}

impl CompareState<Matched, Matched> {
    /// Convert a matched state to a candidate state for the next comparison.
    /// - Old matched cursor becomes the new checkpoint
    /// - Old matched cursor is advanced to get the new candidate cursor
    /// - Old checkpoint is discarded
    /// - Child state (graph path) is also advanced
    /// Uses move semantics to consume the matched state.
    pub(crate) fn into_next_candidate<G: HasGraph>(
        mut self,
        trav: &G,
    ) -> Result<
        CompareState<Candidate, Candidate>,
        CompareState<Matched, Matched>,
    > {
        debug!(
            cursor = %self.cursor,
            "converting matched to candidate"
        );
        // Convert the old matched cursor to a checkpoint (PrefixPath -> RangePath)
        debug!("about to clone cursor");
        let cursor_clone = self.cursor.clone();
        debug!("cloned cursor, now converting to PatternCursor");
        let new_checkpoint: PatternCursor = cursor_clone.into();
        debug!("converted to PatternCursor");

        debug!("calling cursor.advance");
        // Advance the cursor to get the new candidate position
        match self.cursor.advance(trav) {
            Continue(_) => {
                debug!("advance succeeded");
                // Convert the old matched cursor to a checkpoint (PrefixPath -> RangePath)
                let candidate_cursor = self.cursor.as_candidate();
                let candidate_index_cursor = self.index_cursor.as_candidate();

                // Also try to advance the child_state (graph path position)
                // If child_state cannot advance, it means we've reached the end of this pattern
                // and should signal completion to trigger parent exploration
                match self.child_state.into_advanced(trav) {
                    Ok(advanced_child_state) => {
                        debug!("child_state advanced successfully");
                        Ok(CompareState {
                            child_state: advanced_child_state,
                            cursor: candidate_cursor,
                            index_cursor: candidate_index_cursor,
                            checkpoint: new_checkpoint,
                            target: self.target,
                            mode: self.mode,
                        })
                    },
                    Err(child_state) => {
                        // child_state cannot advance - we've matched to the end of this pattern
                        // Return Err to signal we need parent exploration
                        // Use the NON-advanced cursor since we couldn't advance the graph path
                        debug!("child_state cannot advance - matched to end of pattern");
                        Err(CompareState {
                            child_state,
                            cursor: self.cursor, // Use original cursor (not advanced)
                            index_cursor: self.index_cursor, // Keep original index cursor
                            checkpoint: new_checkpoint, // But update checkpoint to mark progress
                            target: self.target,
                            mode: self.mode,
                        })
                    },
                }
            },
            Break(_) => {
                debug!("advance failed, returning matched state");
                // Cannot advance - return the matched state back
                Err(self)
            },
        }
    }

    /// Advance only the query cursor to the next token.
    /// Returns CompareState with query in Candidate state, index still in Matched state.
    ///
    /// Returns Err if query cursor cannot advance (query pattern ended).
    pub(crate) fn advance_query_cursor<G: HasGraph>(
        mut self,
        trav: &G,
    ) -> Result<CompareState<Candidate, Matched>, CompareState<Matched, Matched>>
    {
        debug!(
            cursor = %self.cursor,
            "advancing query cursor only"
        );

        // Try to advance the query cursor
        match self.cursor.advance(trav) {
            Continue(_) => {
                debug!("query cursor advance succeeded");
                // Convert to candidate state
                let candidate_cursor = self.cursor.as_candidate();

                Ok(CompareState {
                    child_state: self.child_state,
                    cursor: candidate_cursor,
                    index_cursor: self.index_cursor, // Keep matched state
                    checkpoint: self.checkpoint,
                    target: self.target,
                    mode: self.mode,
                })
            },
            Break(_) => {
                debug!("query cursor cannot advance - query pattern ended");
                Err(self)
            },
        }
    }
}

// Need impl for CompareState<Candidate, Matched> to support advance_index_cursor result
impl CompareState<Candidate, Matched> {
    /// Advance only the index cursor (via child_state) to the next token.
    /// This is used after query cursor has already been advanced.
    /// Returns CompareState with both cursors in Candidate state.
    ///
    /// Returns Err if index cursor cannot advance (graph path ended).
    pub(crate) fn advance_index_cursor<G: HasGraph>(
        self,
        trav: &G,
    ) -> Result<
        CompareState<Candidate, Candidate>,
        CompareState<Candidate, Matched>,
    > {
        debug!(
            index_cursor = %self.index_cursor,
            "advancing index cursor only (query already advanced)"
        );

        // Try to advance the child_state (which represents the index path position)
        match self.child_state.into_advanced(trav) {
            Ok(advanced_child_state) => {
                debug!("index cursor advance succeeded");
                // Convert index cursor to candidate state
                let candidate_index_cursor = self.index_cursor.as_candidate();

                Ok(CompareState {
                    child_state: advanced_child_state,
                    cursor: self.cursor, // Already in Candidate state
                    index_cursor: candidate_index_cursor,
                    checkpoint: self.checkpoint,
                    target: self.target,
                    mode: self.mode,
                })
            },
            Err(child_state) => {
                debug!("index cursor cannot advance - graph path ended");
                Err(CompareState {
                    child_state,
                    cursor: self.cursor,
                    index_cursor: self.index_cursor, // Keep matched state
                    checkpoint: self.checkpoint,
                    target: self.target,
                    mode: self.mode,
                })
            },
        }
    }
}

impl From<CompareState<Candidate, Candidate>>
    for ChildQueue<CompareState<Candidate, Candidate>>
{
    fn from(val: CompareState<Candidate, Candidate>) -> Self {
        ChildQueue::from_iter([val])
    }
}

impl IntoAdvanced for CompareState<Candidate, Candidate> {
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

impl IntoAdvanced for CompareState<Matched, Matched> {
    type Next = Self;
    fn into_advanced<G: HasGraph>(
        self,
        trav: &G,
    ) -> Result<Self, Self> {
        match self.child_state.into_advanced(trav) {
            Ok(child_state) => Ok(CompareState {
                child_state,
                cursor: self.cursor,
                index_cursor: self.index_cursor,
                checkpoint: self.checkpoint,
                target: self.target,
                mode: self.mode,
            }),
            Err(child_state) => Ok(CompareState {
                child_state,
                cursor: self.cursor,
                index_cursor: self.index_cursor,
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
        debug!(
            leaf = %leaf,
            "getting prefix_children"
        );
        let prefix_children =
            trav.graph().expect_vertex(leaf).prefix_children::<G>();
        debug!(num_children = prefix_children.len(), "got prefix_children");
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
        debug!("returning prefixes");
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
