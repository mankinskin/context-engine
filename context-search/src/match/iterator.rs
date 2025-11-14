use crate::{
    r#match::{
        root_cursor::CompareParentBatch,
        MatchCtx,
        RootSearchIterator,
        TraceNode::{
            self,
            Parent,
        },
    },
    state::end::{
        EndReason,
        EndState,
    },
    traversal::TraversalKind,
};
use context_trace::{
    logging::format_utils::pretty,
    *,
};
use derive_new::new;
use tracing::{
    debug,
    instrument,
    trace,
    warn,
};

#[derive(Debug, new)]
pub(crate) struct MatchIterator<K: TraversalKind> {
    pub(crate) trace_ctx: TraceCtx<K::Trav>,
    pub(crate) match_ctx: MatchCtx,
    /// Tracks the largest complete match found so far
    /// (matches that reached end of root and continued to parents)
    pub(crate) last_complete_match: Option<EndState>,
}
impl<K: TraversalKind> MatchIterator<K> {
    #[instrument(skip(trav), fields(start_index = ?start_index))]
    pub(crate) fn start_index(
        trav: K::Trav,
        start_index: Token,
    ) -> Self {
        debug!("creating MatchIterator from start index");
        MatchIterator {
            trace_ctx: TraceCtx {
                trav,
                cache: TraceCache::new(start_index),
            },
            match_ctx: MatchCtx::new(),
            last_complete_match: None,
        }
    }

    #[instrument(skip(trav, p), fields(start_index = ?start_index, parent_count = p.len()))]
    pub(crate) fn start_parent(
        trav: K::Trav,
        start_index: Token,
        p: CompareParentBatch,
    ) -> Self {
        debug!("creating MatchIterator from parent batch");
        trace!(
            batch_details = %pretty(&p),
            "parent batch composition"
        );

        MatchIterator {
            trace_ctx: TraceCtx {
                trav,
                cache: TraceCache::new(start_index),
            },
            match_ctx: MatchCtx {
                nodes: FromIterator::from_iter(
                    p.into_compare_batch().into_iter().map(TraceNode::Parent),
                ),
            },
            last_complete_match: None,
        }
    }
}

impl<K: TraversalKind> MatchIterator<K> {
    pub(crate) fn find_next(&mut self) -> Option<EndState> {
        trace!("finding next match");
        self.find_map(Some)
    }
}

impl<K: TraversalKind> Iterator for MatchIterator<K> {
    type Item = EndState;

    fn next(&mut self) -> Option<Self::Item> {
        trace!("searching for root cursor");

        match RootSearchIterator::<K>::new(
            &self.trace_ctx.trav,
            &mut self.match_ctx,
        )
        .find_root_cursor()
        {
            Some(root_cursor) => {
                debug!("found root cursor");

                Some(match root_cursor.find_end() {
                    Ok(end) => {
                        // RootCursor found an end (partial match or immediate mismatch/query end)
                        debug!("found end state: {:?}", end.reason);
                        end
                    },
                    Err(root_cursor) => {
                        // RootCursor iteration completed without breaking
                        // This means we matched to the end of the root - complete match
                        trace!("root cursor completed - matched to end, exploring parents");

                        match root_cursor
                            .next_parents::<K>(&self.trace_ctx.trav)
                        {
                            Err(end) => {
                                // No more parents available
                                debug!("no more parents available");
                                *end
                            },
                            Ok((parent, batch)) => {
                                debug!(
                                    batch_size = batch.len(),
                                    "found next parent batch"
                                );
                                trace!(
                                    batch_details = %pretty(&batch),
                                    "parent batch composition"
                                );

                                assert!(!batch.is_empty());

                                // Add parent batch to queue for next iteration
                                self.match_ctx.nodes.extend(
                                    batch
                                        .into_compare_batch()
                                        .into_iter()
                                        .map(Parent),
                                );

                                // Return mismatch to signal "continue searching"
                                // The queue now has parents to explore in next iteration
                                EndState::mismatch(&self.trace_ctx.trav, parent)
                            },
                        }
                    },
                })
            },
            None => {
                trace!("no root cursor found, iteration complete");
                None
            },
        }
    }
}
