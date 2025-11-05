use crate::{
    cursor::PatternCursor,
    fold::{
        foldable::ErrorState,
        FoldCtx,
        StartLocationResult,
    },
    r#match::root_cursor::CompareParentBatch,
    traversal::{
        policy::DirectedTraversalPolicy,
        TraversalKind,
    },
    Response,
};
use context_trace::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct StartCtx {
    pub(crate) location: StartLocationResult,
    pub(crate) cursor: PatternCursor,
}

impl HasVertexIndex for StartCtx {
    fn vertex_index(&self) -> VertexIndex {
        self.location.parent.vertex_index()
    }
}
impl Wide for StartCtx {
    fn width(&self) -> usize {
        self.location.parent.width()
    }
}
impl StartCtx {
    pub(crate) fn get_parent_batch<K: TraversalKind>(
        &self,
        trav: &K::Trav,
    ) -> Result<CompareParentBatch, ErrorState> {
        let mut cursor = self.cursor.clone();
        if cursor.advance(trav).is_continue() {
            let batch = K::Policy::gen_parent_batch(
                trav,
                self.location.parent,
                |trav, p| self.location.parent.into_parent_state(trav, p),
            );

            Ok(CompareParentBatch { batch, cursor })
        } else {
            Err(ErrorState {
                reason: ErrorReason::SingleIndex(Box::new(IndexWithPath {
                    index: self.location.parent,
                    path: self.cursor.path.clone().into(),
                })),
                found: None,
            })
        }
    }
}
