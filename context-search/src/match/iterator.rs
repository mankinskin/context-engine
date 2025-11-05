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
use context_trace::*;
use derive_new::new;
use tracing::debug;

#[derive(Debug, new)]
pub(crate) struct MatchIterator<K: TraversalKind>(
    pub(crate) TraceCtx<K::Trav>,
    pub(crate) MatchCtx,
);
impl<K: TraversalKind> MatchIterator<K> {
    pub(crate) fn start_index(
        trav: K::Trav,
        start_index: Token,
    ) -> Self {
        MatchIterator::new(
            TraceCtx {
                trav,
                cache: TraceCache::new(start_index),
            },
            MatchCtx::new(),
        )
    }
    pub(crate) fn start_parent(
        trav: K::Trav,
        start_index: Token,
        p: CompareParentBatch,
    ) -> Self {
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
        self.find_map(Some)
    }
}

impl<K: TraversalKind> Iterator for MatchIterator<K> {
    type Item = EndState;

    fn next(&mut self) -> Option<Self::Item> {
        match RootSearchIterator::<K>::new(&self.0.trav, &mut self.1)
            .find_root_cursor()
        {
            Some(root_cursor) => Some({
                debug!("Found root cursor {:#?}", root_cursor);
                match root_cursor.find_end() {
                    Ok(end) => {
                        debug!("Found EndState {:#?}", end);
                        end
                    },
                    Err(root_cursor) =>
                        match root_cursor.next_parents::<K>(&self.0.trav) {
                            Err(end) => {
                                debug!("No more parents {:?}", end);
                                *end
                            },
                            Ok((parent, batch)) => {
                                debug!("Next Batch {:?}", (&parent, &batch));
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
                        },
                }
            }),
            None => None,
        }
    }
}
