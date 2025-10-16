use crate::{
    fold::{
        foldable::ErrorState,
        result::FinishedKind,
    },
    r#match::iterator::CompareParentBatch,
    traversal::{
        policy::DirectedTraversalPolicy,
        state::cursor::PatternCursor,
        TraversalKind,
    },
};
use context_trace::{
    trace::state::IntoParentState,
    *,
};

#[derive(Debug, PartialEq, Eq)]
pub struct StartCtx<K: TraversalKind> {
    pub index: Child,
    pub cursor: PatternCursor,
    pub trav: K::Trav,
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
    pub fn get_parent_batch(&self) -> Result<CompareParentBatch, ErrorState> {
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
                found: Some(FinishedKind::Complete(self.index)),
            })
        }
    }
}

//impl RootKey for StartState {
//    fn root_key(&self) -> UpKey {
//        UpKey::new(self.index, TokenPosition(self.index.width()).into())
//    }
//}
