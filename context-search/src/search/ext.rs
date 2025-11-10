//pub(crate) trait IntoFoldCtx<K: TraversalKind> {
//    fn start_search(
//        self,
//        trav: K::Trav,
//    ) -> FoldCtx<K>;
//}

//impl<K: TraversalKind, S: ToToken> IntoFoldCtx<K> for S {
//    fn start_search(
//        self,
//        trav: K::Trav,
//    ) -> FoldCtx<K> {
//        let start_index = self.to_child();
//        FoldCtx {
//            matches: MatchIterator::start_index(trav, start_index),
//            //max_width: start_index.width(),
//            start_index,
//            last_match: None,
//        }
//    }
//}
