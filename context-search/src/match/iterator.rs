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
    state::{
        end::{
            EndReason,
            EndState,
        },
        matched::MatchedEndState,
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
        }
    }
}

impl<K: TraversalKind> SearchIterator<K> {
    pub(crate) fn find_next(&mut self) -> Option<MatchedEndState> {
        trace!("finding next match");
        self.find_map(Some)
    }
}

impl<K: TraversalKind> Iterator for SearchIterator<K> {
    type Item = MatchedEndState;

    fn next(&mut self) -> Option<Self::Item> {
        trace!("searching for root cursor");

        match RootFinder::<K>::new(&self.trace_ctx.trav, &mut self.queue)
            .find_root_cursor()
        {
            Some(root_cursor) => {
                debug!("found root cursor");

                Some(match root_cursor.find_end() {
                    Ok(matched_state) => {
                        // RootCursor found a match - query matched at least partially
                        debug!(
                            is_complete = matched_state.is_complete(),
                            "found matched state"
                        );

                        matched_state
                    },
                    Err(root_cursor) => {
                        // RootCursor reached end of root without conclusion
                        // Need to explore parent tokens to continue comparison
                        trace!("root cursor completed - no conclusion yet, need parents");

                        match root_cursor
                            .next_parents::<K>(&self.trace_ctx.trav)
                        {
                            Err(_end_state) => {
                                // No more parents available - exhausted search without match
                                // Don't return anything, continue to next candidate
                                debug!("no more parents available - continuing search");
                                return self.next();
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
                                // Nodes are prioritized by token width (smaller first)
                                self.queue.nodes.extend(
                                    batch
                                        .into_compare_batch()
                                        .into_iter()
                                        .map(ParentCandidate),
                                );

                                // No match to return yet - continue searching
                                // Recursively call next() to process the parent batch
                                return self.next();
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
