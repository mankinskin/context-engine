use crate::{
    compare::{
        iterator::CompareIterator,
        parent::ParentCompareState,
        state::{
            CompareState,
            TokenMatchState::*,
        },
    },
    cursor::PatternCursor,
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
    *,
};
pub(crate) type CompareQueue = VecDeque<CompareState>;

use derive_more::{
    Deref,
    DerefMut,
};
use std::{
    collections::VecDeque,
    fmt::Debug,
    ops::ControlFlow::{
        self,
        Break,
        Continue,
    },
};
#[derive(Debug)]
pub(crate) struct RootCursor<G: HasGraph> {
    pub(crate) state: Box<CompareState>,
    pub(crate) trav: G,
}
impl<G: HasGraph> Iterator for RootCursor<G> {
    type Item = ControlFlow<EndReason>;

    fn next(&mut self) -> Option<Self::Item> {
        let prev_state = self.state.clone();
        match self.advanced() {
            Continue(_) => Some(
                // next position
                match CompareIterator::new(&self.trav, *self.state.clone())
                    .compare()
                {
                    Match(c) => {
                        *self.state = c;
                        Continue(())
                    },
                    Mismatch(_) => {
                        self.state = prev_state;
                        Break(EndReason::Mismatch)
                    },
                },
            ),
            // end of this root
            Break(None) => None,
            // end of query
            Break(Some(end)) => Some(Break(end)),
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
    fn advanced(&mut self) -> ControlFlow<Option<EndReason>> {
        let rooted_path = self.state.rooted_path();
        let can_advance = rooted_path.can_advance(&self.trav);

        let cursor = &self.state.cursor;
        tracing::debug!(
            "RootCursor::advanced - child_state can_advance={}, child_state={:?}",
            can_advance,
            rooted_path
        );
        tracing::debug!("RootCursor::advanced - query cursor={:?}", cursor);

        if can_advance {
            // Both query and child_state can advance together
            // Query advance is guaranteed to succeed since can_advance returned true
            let advance_result = self.query_advanced();
            if advance_result.is_break() {
                panic!(
                    "query_advanced returned Break when can_advance was true"
                );
            }

            let cursor_end_index =
                self.state.cursor.role_root_child_index::<End>();
            let cursor_pattern_len = {
                let graph = self.trav.graph();
                self.state.cursor.path.root_pattern::<G>(&graph).len()
            };

            tracing::debug!(
                "RootCursor::advanced - both advanced to index {}, pattern_len={}",
                cursor_end_index,
                cursor_pattern_len
            );

            if cursor_end_index >= cursor_pattern_len {
                tracing::debug!("RootCursor::advanced - query past pattern end, returning QueryEnd");
                Break(Some(EndReason::QueryEnd))
            } else {
                tracing::debug!("RootCursor::advanced - advancing child_state to stay in sync");
                let _ = self.path_advanced();
                Continue(())
            }
        } else {
            // Child state cannot advance further in the graph
            // Try to advance the query cursor to see if it's also complete
            tracing::debug!("RootCursor::advanced - child_state cannot advance, attempting to advance query");

            match self.query_advanced() {
                Continue(_) => {
                    // Query advanced successfully but child_state could not
                    // We need to search in parents for the next query token
                    let cursor_end_index =
                        self.state.cursor.role_root_child_index::<End>();
                    let cursor_pattern_len = {
                        let graph = self.trav.graph();
                        self.state.cursor.path.root_pattern::<G>(&graph).len()
                    };

                    tracing::debug!(
                        "RootCursor::advanced - query advanced to index {}, pattern_len={}, need to search in parents",
                        cursor_end_index,
                        cursor_pattern_len
                    );

                    // The query should not be past the end if advance succeeded
                    debug_assert!(
                        cursor_end_index < cursor_pattern_len,
                        "Query advanced but is past pattern end"
                    );

                    // Signal to search in parents (next_parents will be called)
                    // The cursor is already advanced for the parent search
                    Break(None)
                },
                Break(_) => {
                    // Query cannot advance - both query and child_state are exhausted
                    // This means we've matched the entire query (QueryEnd)
                    tracing::debug!("RootCursor::advanced - query and child_state both exhausted, returning QueryEnd");
                    Break(Some(EndReason::QueryEnd))
                },
            }
        }
    }
    fn query_advanced(&mut self) -> ControlFlow<()> {
        self.state.cursor.advance(&self.trav)
    }
    fn path_advanced(&mut self) -> ControlFlow<()> {
        self.state.rooted_path_mut().advance(&self.trav)
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
                    ..
                } = *self.state;
                let root_pos = *child_state.root_pos();
                let path = child_state.rooted_path().clone();
                let target_index =
                    path.role_rooted_leaf_token::<End, _>(&self.trav);
                let pos = cursor.atom_position;
                let target = DownKey::new(target_index, pos.into());
                Ok(EndState {
                    cursor,
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
