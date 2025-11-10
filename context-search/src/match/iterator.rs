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
    state::end::EndState,
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
pub(crate) struct MatchIterator<K: TraversalKind>(
    pub(crate) TraceCtx<K::Trav>,
    pub(crate) MatchCtx,
);
impl<K: TraversalKind> MatchIterator<K> {
    #[instrument(skip(trav), fields(start_index = ?start_index))]
    pub(crate) fn start_index(
        trav: K::Trav,
        start_index: Token,
    ) -> Self {
        debug!("Creating MatchIterator from start index");
        MatchIterator::new(
            TraceCtx {
                trav,
                cache: TraceCache::new(start_index),
            },
            MatchCtx::new(),
        )
    }

    #[instrument(skip(trav, p), fields(start_index = ?start_index, parent_count = p.len()))]
    pub(crate) fn start_parent(
        trav: K::Trav,
        start_index: Token,
        p: CompareParentBatch,
    ) -> Self {
        debug!("Creating MatchIterator from parent batch");
        trace!("Parent batch: {}", pretty(&p));

        MatchIterator::new(
            TraceCtx {
                trav,
                cache: TraceCache::new(start_index),
            },
            MatchCtx {
                nodes: FromIterator::from_iter(
                    p.into_compare_batch().into_iter().map(TraceNode::Parent),
                ),
            },
        )
    }
}

impl<K: TraversalKind> MatchIterator<K> {
    pub(crate) fn find_next(&mut self) -> Option<EndState> {
        trace!("MatchIterator::find_next");
        self.find_map(Some)
    }
}

impl<K: TraversalKind> Iterator for MatchIterator<K> {
    type Item = EndState;

    fn next(&mut self) -> Option<Self::Item> {
        trace!("MatchIterator::next - searching for root cursor");

        match RootSearchIterator::<K>::new(&self.0.trav, &mut self.1)
            .find_root_cursor()
        {
            Some(root_cursor) => {
                debug!("Found root cursor: {}", pretty(&root_cursor));

                Some(match root_cursor.find_end() {
                    Ok(end) => {
                        debug!("Successfully found EndState: {}", pretty(&end));
                        end
                    },
                    Err(root_cursor) => {
                        trace!(
                            "Could not find end, searching for next parents"
                        );

                        match root_cursor.next_parents::<K>(&self.0.trav) {
                            Err(end) => {
                                debug!(
                                    "No more parents, returning end state: {}",
                                    pretty(&end)
                                );
                                *end
                            },
                            Ok((parent, batch)) => {
                                let batch_size = batch.len();
                                debug!("Found next parent batch: parent={}, batch_size={}", 
                                       pretty(&parent), batch_size);
                                trace!("Batch details: {}", pretty(&batch));

                                assert!(!batch.is_empty());

                                // next batch
                                self.1.nodes.extend(
                                    batch
                                        .into_compare_batch()
                                        .into_iter()
                                        .map(Parent),
                                );

                                EndState::mismatch(&self.0.trav, parent)
                            },
                        }
                    },
                })
            },
            None => {
                trace!("No root cursor found, iteration complete");
                None
            },
        }
    }
}
