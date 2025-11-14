use crate::{
    cursor::PatternCursor,
    r#match::iterator::SearchIterator,
    state::{
        end::{
            EndReason,
            EndState,
            MatchState,
            PathEnum,
        },
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
use context_trace::{
    logging::format_utils::pretty,
    *,
};
use tracing::{
    debug,
    instrument,
    trace,
};
pub(crate) mod context;
pub(crate) mod ext;
pub(crate) mod final_state;
pub(crate) mod searchable;

pub(crate) type SearchResult = Result<Response, ErrorReason>;
#[allow(dead_code)]
pub trait Find: HasGraph {
    fn ctx(&self) -> SearchCtx<Self>;

    #[instrument(skip(self, pattern), fields(pattern_len))]
    fn find_sequence(
        &self,
        pattern: impl IntoIterator<Item = impl AsAtom<AtomOf<TravKind<Self>>>>,
    ) -> SearchResult {
        let iter = atomizing_iter(pattern.into_iter());
        let atoms: Vec<_> = iter.collect();
        tracing::Span::current().record("pattern_len", atoms.len());
        debug!(pattern_len = atoms.len(), "finding sequence pattern");
        trace!(atoms = %pretty(&atoms), "pattern atoms");

        let pattern = self.graph().get_atom_children(atoms.into_iter())?;
        debug!(pattern = %pretty(&pattern), "created pattern token");

        self.find_ancestor(pattern)
    }

    // find largest matching direct parent
    #[instrument(skip(self, searchable))]
    fn find_parent(
        &self,
        searchable: impl Searchable,
    ) -> SearchResult {
        debug!("starting parent search");
        let result = searchable
            .search::<ParentSearchTraversal<Self>>(self.ctx())
            .map_err(|err| err.reason);

        match &result {
            Ok(_response) => debug!("parent search succeeded"),
            Err(reason) =>
                debug!(reason = %pretty(reason), "parent search failed"),
        }
        result
    }

    /// find largest matching ancestor for pattern
    #[instrument(skip(self, searchable))]
    fn find_ancestor(
        &self,
        searchable: impl Searchable,
    ) -> SearchResult {
        debug!("starting ancestor search");
        let result = searchable
            .search::<AncestorSearchTraversal<Self>>(self.ctx())
            .map_err(|err| err.reason);

        match &result {
            Ok(_response) => debug!("ancestor search succeeded"),
            Err(reason) =>
                debug!(reason = %pretty(reason), "ancestor search failed"),
        }
        result
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
pub struct SearchState<K: TraversalKind> {
    pub(crate) matches: SearchIterator<K>,
    //pub(crate) start_index: Token,
    pub(crate) last_match: MatchState,
}

impl<K: TraversalKind> Iterator for SearchState<K> {
    type Item = EndState;
    fn next(&mut self) -> Option<Self::Item> {
        trace!("searching for next match");
        match self.matches.find_next() {
            Some(end) => {
                debug!("found end state");

                // Get the start length from the previous match or query
                let start_len = match &self.last_match {
                    MatchState::Located(prev_end) => prev_end.start_len(),
                    MatchState::Query(_) => {
                        // First match: start from beginning of query
                        debug!("first match found, transitioning from Query to Located state");
                        0
                    },
                };
                debug!(start_pos = start_len, "tracing from position");

                TraceStart {
                    end: &end,
                    pos: start_len,
                }
                .trace(&mut self.matches.trace_ctx);

                // Update last_match to the located state
                self.last_match = MatchState::Located(end.clone());
                Some(end.clone())
            },
            None => {
                trace!("no more matches found");
                None
            },
        }
    }
}

impl<K: TraversalKind> SearchState<K> {
    #[instrument(skip(self))]
    pub(crate) fn search(mut self) -> Response {
        debug!("starting fold search");
        debug!(matches = ?&self.matches, "initial state");

        let mut iteration = 0;
        while let Some(end) = &mut self.next() {
            iteration += 1;
            debug!(iteration, "tracing end state");
            end.trace(&mut self.matches.trace_ctx);
        }

        debug!(iterations = iteration, "fold completed");

        // Get the final end state
        let end = match self.last_match {
            MatchState::Located(end_state) => {
                debug!("final state is located");
                end_state
            },
            MatchState::Query(query_path) => {
                // No matches were found - need to create an appropriate error/incomplete state
                debug!("no matches found, still in query state");
                // TODO: Create proper EndState for "no match" case
                // For now, create a minimal EndState
                // The query_path has a Pattern root, get the first token
                let start_token = query_path.path_root()[0];
                let cursor = PatternCursor {
                    atom_position: AtomPosition::default(),
                    path: query_path.clone(),
                    _state: std::marker::PhantomData,
                };
                EndState {
                    reason: EndReason::Mismatch,
                    path: PathEnum::Complete(IndexRangePath::new_empty(
                        IndexRoot::from(PatternLocation::new(
                            start_token,
                            PatternId::default(),
                        )),
                    )),
                    cursor,
                }
            },
        };

        trace!(end = %pretty(&end), "final end state");

        let trace_ctx = &mut self.matches.trace_ctx;
        end.trace(trace_ctx);

        let response = Response {
            cache: self.matches.trace_ctx.cache,
            end,
        };

        debug!("search complete");
        response
    }
}
