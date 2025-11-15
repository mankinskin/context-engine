use crate::{
    r#match::{
        root_cursor::CompareParentBatch,
        RootFinder,
        SearchNode::{
            self,
            ParentCandidate,
        },
        SearchQueue,
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
pub(crate) struct SearchIterator<K: TraversalKind> {
    pub(crate) trace_ctx: TraceCtx<K::Trav>,
    pub(crate) queue: SearchQueue,
    /// Tracks the largest complete match found so far
    /// (matches that reached end of root and continued to parents)
    pub(crate) last_complete_match: Option<EndState>,
}
impl<K: TraversalKind> SearchIterator<K> {
    #[context_trace::instrument_sig(skip(trav), fields(start_index = %start_index))]
    pub(crate) fn start_index(
        trav: K::Trav,
        start_index: Token,
    ) -> Self {
        debug!("creating match iterator from start index");
        SearchIterator {
            trace_ctx: TraceCtx {
                trav,
                cache: TraceCache::new(start_index),
            },
            queue: SearchQueue::new(),
            last_complete_match: None,
        }
    }

    #[context_trace::instrument_sig(skip(trav, p), fields(start_index = %start_index, parent_count = p.len()))]
    pub(crate) fn start_parent(
        trav: K::Trav,
        start_index: Token,
        p: CompareParentBatch,
    ) -> Self {
        debug!("creating match iterator from parent batch");
        trace!(
            batch_details = %pretty(&p),
            "parent batch composition"
        );

        SearchIterator {
            trace_ctx: TraceCtx {
                trav,
                cache: TraceCache::new(start_index),
            },
            queue: SearchQueue {
                nodes: FromIterator::from_iter(
                    p.into_compare_batch()
                        .into_iter()
                        .map(SearchNode::ParentCandidate),
                ),
            },
            last_complete_match: None,
        }
    }
}

impl<K: TraversalKind> SearchIterator<K> {
    pub(crate) fn find_next(&mut self) -> Option<EndState> {
        trace!("finding next match");
        self.find_map(Some)
    }
}

impl<K: TraversalKind> Iterator for SearchIterator<K> {
    type Item = EndState;

    fn next(&mut self) -> Option<Self::Item> {
        trace!("searching for root cursor");

        match RootFinder::<K>::new(&self.trace_ctx.trav, &mut self.queue)
            .find_root_cursor()
        {
            Some(root_cursor) => {
                debug!("found root cursor");

                Some(match root_cursor.find_end() {
                    Ok(end) => {
                        // RootCursor found an end (partial match or immediate mismatch/query end)
                        debug!(reason = %end.reason, "found end state");
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
                                // Return last_complete_match if we have one, otherwise this end state
                                if let Some(complete) =
                                    self.last_complete_match.take()
                                {
                                    debug!("returning last complete match");
                                    complete
                                } else {
                                    *end
                                }
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

                                // Save this complete match before exploring parents
                                let complete_match = EndState::query_end(
                                    &self.trace_ctx.trav,
                                    parent.clone(),
                                );
                                debug!("saving complete match before parent exploration");
                                self.last_complete_match = Some(complete_match);

                                // Add parent batch to queue for next iteration
                                self.queue.nodes.extend(
                                    batch
                                        .into_compare_batch()
                                        .into_iter()
                                        .map(ParentCandidate),
                                );

                                // Continue exploring - call next() recursively
                                match self.next() {
                                    Some(end_state) => end_state,
                                    None => {
                                        // Queue exhausted - return the saved complete match
                                        self.last_complete_match.take().expect(
                                            "last_complete_match should be set",
                                        )
                                    },
                                }
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
