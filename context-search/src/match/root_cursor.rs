use crate::{
    compare::{
        iterator::CompareIterator,
        parent::ParentCompareState,
        state::{
            CompareResult,
            CompareResult::*,
            CompareState,
        },
    },
    cursor::{
        Candidate,
        ChildCursor,
        CursorState,
        Matched,
        PathCursor,
        PatternCursor,
    },
    state::{
        end::{
            EndReason,
            EndState,
            PathCoverage,
        },
        matched::{
            MatchedEndState,
            MismatchState,
            QueryExhaustedState,
        },
    },
    traversal::{
        policy::DirectedTraversalPolicy,
        TraversalKind,
    },
};
use context_trace::{
    path::RolePathUtils,
    End,
    *,
};

use derive_more::{
    Deref,
    DerefMut,
};
use std::{
    collections::VecDeque,
    fmt::Debug,
    marker::PhantomData,
    ops::ControlFlow::{
        self,
        Break,
        Continue,
    },
};

// Type alias for advance_cursors return type
/// Result of advancing both cursors:
/// - Ok: Both cursors advanced successfully
/// - Err: Contains EndReason and optionally a cursor needing parent exploration
pub(crate) type AdvanceCursorsResult<G> = Result<
    RootCursor<G, Candidate, Candidate>,
    (EndReason, Option<RootCursor<G, Candidate, Matched>>),
>;

#[derive(Debug)]
pub(crate) struct RootCursor<
    G: HasGraph + Clone,
    Q: CursorState = Matched,
    I: CursorState = Matched,
> {
    pub(crate) state: Box<CompareState<Q, I>>,
    pub(crate) trav: G,
}

impl<G: HasGraph + Clone> RootCursor<G, Matched, Matched> {
    /// Advance a Matched cursor to a Candidate cursor by advancing both query and index
    /// Returns Ok(Candidate cursor) if both advanced successfully
    /// Returns Err(EndState) if query ended (QueryExhausted) or if need parent exploration
    pub(crate) fn advance_to_candidate(
        self
    ) -> Result<
        RootCursor<G, Candidate, Candidate>,
        Result<MatchedEndState, RootCursor<G, Candidate, Matched>>,
    > {
        let matched_state = *self.state;
        let trav = self.trav;

        // Try to advance query cursor
        match matched_state.advance_query_cursor(&trav) {
            Ok(query_advanced) => {
                // Query advanced, now try index
                match query_advanced.advance_index_cursor(&trav) {
                    Ok(both_advanced) => {
                        // Both cursors advanced - return Candidate cursor
                        Ok(RootCursor {
                            state: Box::new(both_advanced),
                            trav,
                        })
                    },
                    Err(query_only_advanced) => {
                        // Index ended but query continues - need parent exploration
                        Err(Err(RootCursor {
                            state: Box::new(CompareState {
                                child_cursor: query_only_advanced.child_cursor,
                                cursor: query_only_advanced.cursor,
                                checkpoint: query_only_advanced.checkpoint,
                                checkpoint_child: query_only_advanced
                                    .checkpoint_child,
                                target: query_only_advanced.target,
                                mode: query_only_advanced.mode,
                            }),
                            trav,
                        }))
                    },
                }
            },
            Err(matched_state) => {
                // Query ended - complete match
                let root_pos =
                    *matched_state.child_cursor.child_state.target_pos();
                let path = matched_state.child_cursor.child_state.path.clone();
                let root_parent = path.root_parent();
                let target_index = path.role_rooted_leaf_token::<End, _>(&trav);
                let last_token_width_value = target_index.width();
                let end_pos = AtomPosition::from(
                    *matched_state.cursor.atom_position
                        - last_token_width_value,
                );
                tracing::debug!("root_cursor process_candidate_match: root_parent={}, root_pos={}, cursor.atom_position={}, last_token_width={}, end_pos={}",
                    root_parent, usize::from(root_pos), *matched_state.cursor.atom_position,
                    last_token_width_value, usize::from(end_pos));

                let target = DownKey::new(target_index, end_pos.into());
                Err(Ok(MatchedEndState::QueryExhausted(QueryExhaustedState {
                    cursor: matched_state.checkpoint,
                    path: PathCoverage::from_range_path(
                        path, root_pos, target, &trav,
                    ),
                })))
            },
        }
    }

    /// Process a matched cursor and iterate to find end state
    /// Uses iterator to advance through matches until QueryExhausted or need parent exploration
    pub(crate) fn find_end(
        self
    ) -> Result<MatchedEndState, RootCursor<G, Candidate, Matched>> {
        // Try to advance to candidate
        match self.advance_to_candidate() {
            Ok(candidate_cursor) => {
                // We have a candidate cursor - iterate it to find the end
                candidate_cursor.find_end()
            },
            Err(Ok(matched_state)) => {
                // Query ended immediately - return the matched state
                Ok(matched_state)
            },
            Err(Err(need_parent)) => {
                // Need parent exploration immediately
                Err(need_parent)
            },
        }
    }
}

impl<G: HasGraph + Clone> RootCursor<G, Candidate, Candidate> {
    /// Advance a Candidate cursor to a Matched cursor by comparing and matching
    /// Returns Ok(Matched cursor) if comparison resulted in a match
    /// Returns Err(Some(MatchedEndState)) if hit QueryExhausted or Mismatch during iteration
    /// Returns Err(None) if iterator completed without conclusion - need parent exploration (returns self)
    pub(crate) fn advance_to_matched(
        mut self
    ) -> Result<RootCursor<G, Matched, Matched>, Result<MatchedEndState, Self>>
    {
        // Iterate until we get a match or need to stop
        loop {
            match self.next() {
                Some(Continue(())) => {
                    // Comparison resulted in match and both cursors advanced
                    // self has been updated to new candidate state, continue
                    continue;
                },
                Some(Break(reason)) => {
                    // Hit an end condition (QueryExhausted or Mismatch)
                    // Check if this is a valid match before destructuring
                    if reason == EndReason::Mismatch
                        && *self.state.checkpoint.atom_position.as_ref() == 0
                    {
                        // No progress - not a valid match, continue iteration
                        continue;
                    }

                    // Valid match - destructure and create matched state
                    let CompareState {
                        child_cursor,
                        cursor,
                        checkpoint,
                        checkpoint_child,
                        ..
                    } = *self.state;

                    // For Mismatch, use checkpoint_child path (state at last match)
                    // For QueryExhausted, use current child_cursor path
                    let (path, root_pos) = match reason {
                        EndReason::QueryExhausted => {
                            let root_pos =
                                *child_cursor.child_state.target_pos();
                            (child_cursor.child_state.path.clone(), root_pos)
                        },
                        EndReason::Mismatch => {
                            let root_pos =
                                *checkpoint_child.child_state.target_pos();
                            (
                                checkpoint_child.child_state.path.clone(),
                                root_pos,
                            )
                        },
                    };

                    let target_index =
                        path.role_rooted_leaf_token::<End, _>(&self.trav);
                    let last_token_width_value = target_index.width();

                    let (end_cursor, end_pos) = match reason {
                        EndReason::QueryExhausted => (
                            checkpoint.clone(),
                            AtomPosition::from(
                                *cursor.atom_position - last_token_width_value,
                            ),
                        ),
                        EndReason::Mismatch => {
                            // For Mismatch, checkpoint already points to position AFTER last match
                            // We need end_pos to be position of last matched token's END
                            // which is checkpoint.atom_position (already accounts for last match width)
                            (checkpoint.clone(), checkpoint.atom_position)
                        },
                    };

                    let target = DownKey::new(target_index, end_pos.into());
                    let path_enum = PathCoverage::from_range_path(
                        path, root_pos, target, &self.trav,
                    );

                    // Create appropriate matched state based on reason
                    let matched_state = match reason {
                        EndReason::QueryExhausted =>
                            MatchedEndState::QueryExhausted(
                                QueryExhaustedState {
                                    cursor: end_cursor,
                                    path: path_enum,
                                },
                            ),
                        EndReason::Mismatch => {
                            // We already filtered checkpoint == 0 above
                            MatchedEndState::Mismatch(MismatchState {
                                cursor: end_cursor,
                                path: path_enum,
                            })
                        },
                    };

                    return Err(Ok(matched_state));
                },
                None => {
                    // Iterator completed without Break - need parent exploration
                    return Err(Err(self));
                },
            }
        }
    }

    /// Find the end state by iterating through candidate comparisons
    /// Returns Ok(MatchedEndState) if we reach QueryExhausted or Mismatch with progress
    /// Returns Err if we need parent exploration
    pub(crate) fn find_end(
        self
    ) -> Result<MatchedEndState, RootCursor<G, Candidate, Matched>> {
        match self.advance_to_matched() {
            Ok(_matched_cursor) => {
                // Got a matched cursor - this shouldn't happen in find_end
                // because advance_to_matched loops until it gets an EndState or needs parents
                unreachable!("advance_to_matched returned Ok(Matched) - should return Err with EndState")
            },
            Err(Ok(end_state)) => {
                // Found an end state (QueryExhausted or Mismatch)
                Ok(end_state)
            },
            Err(Err(candidate_cursor)) => {
                // Need parent exploration - convert <Candidate, Candidate> to <Candidate, Matched>
                Err(RootCursor {
                    state: Box::new(CompareState {
                        child_cursor: ChildCursor {
                            child_state: candidate_cursor
                                .state
                                .child_cursor
                                .child_state
                                .clone(),
                            _state: PhantomData,
                        },
                        cursor: candidate_cursor.state.cursor.clone(),
                        checkpoint: candidate_cursor.state.checkpoint.clone(),
                        checkpoint_child: candidate_cursor
                            .state
                            .checkpoint_child
                            .clone(),
                        target: candidate_cursor.state.target,
                        mode: candidate_cursor.state.mode,
                    }),
                    trav: candidate_cursor.trav,
                })
            },
        }
    }
}

// Keep the old find_end implementation but remove the confusing loop
impl<G: HasGraph + Clone> RootCursor<G, Matched, Matched> {
    /// Process a matched cursor: advance and convert to either iterable candidate cursor or immediate end
    pub(crate) fn process_match(
        self
    ) -> Result<RootCursor<G, Candidate, Candidate>, EndReason> {
        match self.advance_cursors() {
            Ok(candidate_cursor) => {
                // Both cursors advanced - can continue iterating
                Ok(candidate_cursor)
            },
            Err((reason, _)) => {
                // Could not advance
                Err(reason)
            },
        }
    }

    /// Advance cursors after a match and transition to Candidate state
    /// Returns Ok with both-advanced state, or Err with reason for failure
    pub(crate) fn advance_cursors(self) -> AdvanceCursorsResult<G> {
        let matched_state = *self.state;

        // Step 1: Try to advance QUERY cursor
        match matched_state.advance_query_cursor(&self.trav) {
            Ok(query_advanced) => {
                // Step 2: Try to advance INDEX cursor
                match query_advanced.advance_index_cursor(&self.trav) {
                    Ok(both_advanced) => {
                        tracing::debug!("both cursors advanced successfully");
                        // Both cursors advanced - return as Candidate state
                        Ok(RootCursor {
                            state: Box::new(both_advanced),
                            trav: self.trav,
                        })
                    },
                    Err(_query_only_advanced) => {
                        tracing::debug!(
                            "index cursor cannot advance - graph path ended"
                        );
                        // INDEX ENDED, QUERY CONTINUES
                        // Return cursor in <Candidate, Matched> state for parent exploration
                        Err((
                            EndReason::Mismatch,
                            Some(RootCursor {
                                state: Box::new(CompareState {
                                    child_cursor: _query_only_advanced
                                        .child_cursor,
                                    cursor: _query_only_advanced.cursor,
                                    checkpoint: _query_only_advanced.checkpoint,
                                    checkpoint_child: _query_only_advanced
                                        .checkpoint_child,
                                    target: _query_only_advanced.target,
                                    mode: _query_only_advanced.mode,
                                }),
                                trav: self.trav,
                            }),
                        ))
                    },
                }
            },
            Err(_matched_state) => {
                tracing::debug!(
                    "query cursor cannot advance - query pattern ended"
                );
                // QUERY ENDED - no cursor to return
                Err((EndReason::QueryExhausted, None))
            },
        }
    }
}

impl<G: HasGraph + Clone> Iterator for RootCursor<G, Candidate, Candidate> {
    type Item = ControlFlow<EndReason>;

    fn next(&mut self) -> Option<Self::Item> {
        let prev_state = self.state.clone();

        tracing::debug!("comparing current candidate");
        // Compare the current candidate state
        match CompareIterator::new(&self.trav, *self.state.clone()).compare() {
            FoundMatch(matched_state) => {
                tracing::debug!(
                    "got Match, creating Matched RootCursor to advance"
                );

                // Create a Matched RootCursor and try to advance
                let matched_cursor = RootCursor {
                    state: Box::new(matched_state),
                    trav: self.trav.clone(),
                };

                match matched_cursor.advance_cursors() {
                    Ok(candidate_cursor) => {
                        // Both cursors advanced - update to candidate state and continue
                        *self = candidate_cursor;
                        Some(Continue(()))
                    },
                    Err((reason, cursor_opt)) => {
                        // Could not advance one or both cursors
                        match (reason, cursor_opt) {
                            (EndReason::QueryExhausted, None) => {
                                // Query cursor ended - QueryExhausted match
                                tracing::debug!("query pattern ended - QueryExhausted match found");
                                Some(Break(EndReason::QueryExhausted))
                            },
                            (EndReason::Mismatch, Some(_)) => {
                                // Index ended but query continues - need parent exploration
                                tracing::debug!("index ended, query continues - returning None for parent exploration");
                                None
                            },
                            _ => unreachable!("invalid advance state"),
                        }
                    },
                }
            },
            Mismatch(_) => {
                // Mismatch found - check if this is after some matches (partial match)
                // or immediate mismatch (no match)
                // The checkpoint atom_position tells us if we've made progress
                if self.state.checkpoint.atom_position != AtomPosition::from(0)
                {
                    // We had matches before this mismatch
                    // This is a PARTIAL MATCH - the success case!
                    // This root contains the largest contiguous match
                    Some(Break(EndReason::Mismatch))
                } else {
                    // Immediate mismatch, no matches yet
                    // Revert and break to try other paths
                    self.state = prev_state;
                    Some(Break(EndReason::Mismatch))
                }
            },
            Prefixes(_) => unreachable!("compare() never returns Prefixes"),
        }
    }
}

impl<G: HasGraph + Clone> RootCursor<G, Candidate, Matched> {
    /// Convert to Candidate state for both cursors to enable parent exploration
    pub(crate) fn into_candidate(self) -> RootCursor<G, Candidate, Candidate> {
        let state = *self.state;
        RootCursor {
            state: Box::new(CompareState {
                child_cursor: state.child_cursor.as_candidate(),
                cursor: state.cursor,
                checkpoint: state.checkpoint,
                checkpoint_child: state.checkpoint_child,
                target: state.target,
                mode: state.mode,
            }),
            trav: self.trav,
        }
    }

    pub(crate) fn next_parents<K: TraversalKind>(
        self,
        trav: &K::Trav,
    ) -> Result<(ParentCompareState, CompareParentBatch), Box<EndState>> {
        // Convert to Candidate first, then call next_parents
        self.into_candidate().next_parents::<K>(trav)
    }
}

impl<G: HasGraph + Clone> RootCursor<G, Candidate, Candidate> {
    pub(crate) fn next_parents<K: TraversalKind>(
        self,
        trav: &K::Trav,
    ) -> Result<(ParentCompareState, CompareParentBatch), Box<EndState>> {
        let parent = self.state.parent_state();
        // Note: The cursor should already be advanced by the `advanced()` method
        // before this function is called. We don't advance it again here.
        if let Some(batch) = K::Policy::next_batch(trav, &parent.parent_state) {
            let batch = CompareParentBatch {
                batch,
                cursor: parent.cursor.clone(),
            };
            Ok((parent, batch))
        } else {
            Err(Box::new(EndState::mismatch(trav, parent)))
        }
    }
}

#[derive(Debug, Clone, Deref, DerefMut)]
pub(crate) struct CompareParentBatch {
    #[deref]
    #[deref_mut]
    pub(crate) batch: ParentBatch,
    pub(crate) cursor: PatternCursor,
}
impl CompareParentBatch {
    pub(crate) fn into_compare_batch(self) -> VecDeque<ParentCompareState> {
        self.batch
            .parents
            .into_iter()
            .map(|parent_state| ParentCompareState {
                parent_state,
                cursor: self.cursor.clone(),
            })
            .collect()
    }
}
