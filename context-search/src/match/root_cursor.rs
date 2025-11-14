use crate::{
    compare::{
        iterator::CompareIterator,
        parent::ParentCompareState,
        state::{
            CompareNext,
            CompareNext::*,
            CompareState,
        },
    },
    cursor::{
        Candidate,
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
pub(crate) type CompareQueue = VecDeque<CompareState<Candidate>>;

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
pub(crate) struct RootCursor<G: HasGraph> {
    pub(crate) state: Box<CompareState<Candidate>>,
    pub(crate) trav: G,
}
impl<G: HasGraph> Iterator for RootCursor<G> {
    type Item = ControlFlow<EndReason>;

    fn next(&mut self) -> Option<Self::Item> {
        let prev_state = self.state.clone();

        tracing::debug!("RootCursor::next - comparing current candidate");
        // Compare the current candidate state
        match CompareIterator::new(&self.trav, *self.state.clone()).compare() {
            Match(matched_state) => {
                tracing::debug!(
                    "RootCursor::next - got Match, calling into_next_candidate"
                );
                // Convert matched state to candidate state for next iteration
                // This updates the checkpoint to the matched cursor position
                // and advances the cursor to the next position
                match matched_state.into_next_candidate(&self.trav) {
                    Ok(candidate_state) => {
                        *self.state = candidate_state;
                        Some(Continue(()))
                    },
                    Err(matched_state) => {
                        // Cannot advance cursor further after a successful match
                        // Check if query pattern is complete
                        let cursor_index =
                            matched_state.cursor.path.root_child_index();
                        let cursor_pattern_len = {
                            matched_state
                                .cursor
                                .path
                                .root_pattern::<G>(&self.trav.graph())
                                .len()
                        };

                        // Create candidate state from matched (without advancing cursor)
                        // The checkpoint should be the matched position
                        let checkpoint: PatternCursor =
                            matched_state.cursor.clone().into();
                        let candidate_cursor =
                            matched_state.cursor.as_candidate();

                        *self.state = CompareState {
                            child_state: matched_state.child_state,
                            cursor: candidate_cursor,
                            checkpoint,
                            target: matched_state.target,
                            mode: matched_state.mode,
                        };

                        if cursor_index >= cursor_pattern_len - 1 {
                            // Query pattern is complete
                            Some(Break(EndReason::QueryEnd))
                        } else {
                            // Query incomplete but cannot advance in this root
                            // End iteration to signal parent search
                            None
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
impl<G: HasGraph> RootCursor<G> {
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
                    child_state,
                    cursor,
                    checkpoint,
                    ..
                } = *self.state;
                let root_pos = *child_state.root_pos();
                let path = child_state.rooted_path().clone();
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
