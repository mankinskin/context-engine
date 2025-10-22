use crate::{
    cursor::PatternCursor,
    fold::{
        foldable::ErrorState,
        FoldCtx,
    },
    r#match::root_cursor::CompareParentBatch,
    traversal::{
        policy::DirectedTraversalPolicy,
        TraversalKind,
        TryIntoTraversalCtx,
    },
    CompleteState,
    Response,
};
use context_trace::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct StartCtx<K: TraversalKind> {
    pub(crate) index: Token,
    pub(crate) cursor: PatternCursor,
    pub(crate) trav: K::Trav,
}

impl<K: TraversalKind> HasVertexIndex for StartCtx<K> {
    fn vertex_index(&self) -> VertexIndex {
        self.index.vertex_index()
    }
}
impl<K: TraversalKind> Wide for StartCtx<K> {
    fn width(&self) -> usize {
        self.index.width()
    }
}
impl<K: TraversalKind> StartCtx<K> {
    pub(crate) fn get_parent_batch(
        &self
    ) -> Result<CompareParentBatch, ErrorState> {
        let mut cursor = self.cursor.clone();
        if cursor.advance(&self.trav).is_continue() {
            let batch = K::Policy::gen_parent_batch(
                &self.trav,
                self.index,
                |trav, p| self.index.into_parent_state(trav, p),
            );

            Ok(CompareParentBatch { batch, cursor })
        } else {
            Err(ErrorState {
                reason: ErrorReason::SingleIndex(Box::new(IndexWithPath {
                    index: self.index,
                    path: self.cursor.path.clone().into(),
                })),
                found: None,
            })
        }
    }
}
