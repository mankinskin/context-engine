use crate::{
    r#match::iterator::MatchIterator,
    state::{
        end::EndState,
        start::Searchable,
    },
    traversal::TraceStart,
    Response,
    TraversalKind,
};
use context::{
    AncestorSearchTraversal,
    ParentSearchTraversal,
    SearchCtx,
};
use context_trace::*;
use tracing::debug;
pub(crate) mod context;
pub(crate) mod ext;
pub(crate) mod final_state;
pub(crate) mod searchable;

pub(crate) type SearchResult = Result<Response, ErrorReason>;
#[allow(dead_code)]
pub trait Find: HasGraph {
    fn ctx(&self) -> SearchCtx<Self>;
    fn find_sequence(
        &self,
        pattern: impl IntoIterator<Item = impl AsAtom<AtomOf<TravKind<Self>>>>,
    ) -> SearchResult {
        let iter = atomizing_iter(pattern.into_iter());
        let pattern = self.graph().get_atom_children(iter)?;
        self.find_ancestor(pattern)
    }
    // find largest matching direct parent
    fn find_parent(
        &self,
        searchable: impl Searchable,
    ) -> SearchResult {
        searchable
            .search::<ParentSearchTraversal<Self>>(self.ctx())
            .map_err(|err| err.reason)
    }
    /// find largest matching ancestor for pattern
    fn find_ancestor(
        &self,
        searchable: impl Searchable,
    ) -> SearchResult {
        searchable
            .search::<AncestorSearchTraversal<Self>>(self.ctx())
            .map_err(|err| err.reason)
    }
}

impl Find for &Hypergraph {
    fn ctx(&self) -> SearchCtx<Self> {
        SearchCtx::new(self)
    }
}

impl Find for HypergraphRef {
    fn ctx(&self) -> SearchCtx<Self> {
        SearchCtx::new(self.clone())
    }
}

/// context for running fold traversal
#[derive(Debug)]
pub struct FoldCtx<K: TraversalKind> {
    pub(crate) matches: MatchIterator<K>,
    //pub(crate) start_index: Token,
    pub(crate) last_match: EndState,
}

impl<K: TraversalKind> Iterator for FoldCtx<K> {
    type Item = EndState;
    fn next(&mut self) -> Option<Self::Item> {
        match self.matches.find_next() {
            Some(end) => {
                debug!("Found end {:#?}", end);
                TraceStart {
                    end: &end,
                    pos: self.last_match.start_len(),
                }
                .trace(&mut self.matches.0);

                self.last_match = end.clone();
                Some(end.clone())
            },
            None => None,
        }
    }
}

impl<K: TraversalKind> FoldCtx<K> {
    pub(crate) fn search(mut self) -> Response {
        debug!("Starting fold {:#?}", self);

        while let Some(end) = &mut self.next() {
            end.trace(&mut self.matches.0);
        }
        // last end
        let end = self.last_match;
        let trace_ctx = &mut self.matches.0;
        end.trace(trace_ctx);
        Response {
            cache: self.matches.0.cache,
            //start: self.start_index,
            end,
        }
    }
}
