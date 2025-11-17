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
        CursorState,
        Matched,
        PathCursor,
        PatternCursor,
    },
    state::end::{
        EndReason,
        EndState,
        PathEnum,
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
pub(crate) type CompareQueue = VecDeque<CompareState<Candidate, Candidate>>;

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
    /// Process a matched cursor and iterate to find end state
    /// This advances the cursors first, then delegates to the Candidate impl or returns appropriate end
    pub(crate) fn find_end(
        self
    ) -> Result<EndState, RootCursor<G, Candidate, Matched>> {
        let matched_state = *self.state;
        let trav = self.trav;

        // Try to advance query cursor
        match matched_state.advance_query_cursor(&trav) {
            Ok(query_advanced) => {
                // Query advanced, now try index
                match query_advanced.advance_index_cursor(&trav) {
                    Ok(both_advanced) => {
                        // Both cursors advanced - create candidate cursor and iterate
                        let candidate_cursor = RootCursor {
                            state: Box::new(both_advanced),
                            trav,
                        };
                        candidate_cursor.find_end().map_err(|_| {
                            panic!(
                                "Candidate RootCursor completed without Break"
                            )
                        })
                    },
                    Err(query_only_advanced) => {
                        // Index ended but query continues - return for parent exploration
                        Err(RootCursor {
                            state: Box::new(CompareState {
                                child_cursor: query_only_advanced.child_cursor,
                                cursor: query_only_advanced.cursor,
                                checkpoint: query_only_advanced.checkpoint,
                                target: query_only_advanced.target,
                                mode: query_only_advanced.mode,
                            }),
                            trav,
                        })
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
                Ok(EndState {
                    cursor: matched_state.checkpoint,
                    reason: EndReason::QueryEnd,
                    path: PathEnum::from_range_path(
                        path, root_pos, target, &trav,
                    ),
                })
            },
        }
    }

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
    pub(crate) fn advance_cursors(
        self
    ) -> Result<
        RootCursor<G, Candidate, Candidate>,
        (EndReason, Option<RootCursor<G, Candidate, Matched>>),
    > {
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
                Err((EndReason::QueryEnd, None))
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
                            (EndReason::QueryEnd, None) => {
                                // Query cursor ended - complete match
                                tracing::debug!("query pattern ended - complete match found");
                                Some(Break(EndReason::QueryEnd))
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

    pub(crate) fn find_end(mut self) -> Result<EndState, Self> {
        match self.find_map(|flow| match flow {
            Continue(()) => None,
            Break(reason) => Some(reason),
        }) {
            Some(reason) => {
                let CompareState {
                    child_cursor,
                    cursor,
                    checkpoint,
                    ..
                } = *self.state;
                let root_pos = *child_cursor.child_state.target_pos();
                let path = child_cursor.child_state.path.clone();
                let target_index =
                    path.role_rooted_leaf_token::<End, _>(&self.trav);

                // Calculate the END position based on cursor position BEFORE the last advance
                // The cursor has advanced past the last token, so we need to go back by the width of the last token
                let last_token_width_value = target_index.width();
                let end_pos = AtomPosition::from(
                    *cursor.atom_position - last_token_width_value,
                );

                let target = DownKey::new(target_index, end_pos.into());
                Ok(EndState {
                    cursor: checkpoint,
                    reason,
                    path: PathEnum::from_range_path(
                        path, root_pos, target, &self.trav,
                    ),
                })
            },
            None => Err(self),
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
