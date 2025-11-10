use crate::{
    cursor::PatternCursor,
    r#match::iterator::MatchIterator,
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
    info,
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
        info!("Finding sequence pattern with {} atoms", atoms.len());
        trace!("Pattern atoms: {}", pretty(&atoms));

        let pattern = self.graph().get_atom_children(atoms.into_iter())?;
        debug!("Created pattern token: {}", pretty(&pattern));

        self.find_ancestor(pattern)
    }

    // find largest matching direct parent
    #[instrument(skip(self, searchable))]
    fn find_parent(
        &self,
        searchable: impl Searchable,
    ) -> SearchResult {
        info!("Starting parent search");
        let result = searchable
            .search::<ParentSearchTraversal<Self>>(self.ctx())
            .map_err(|err| err.reason);

        match &result {
            Ok(response) =>
                info!("Parent search succeeded: end={}", pretty(&response.end)),
            Err(reason) => info!("Parent search failed: {}", pretty(reason)),
        }
        result
    }

    /// find largest matching ancestor for pattern
    #[instrument(skip(self, searchable))]
    fn find_ancestor(
        &self,
        searchable: impl Searchable,
    ) -> SearchResult {
        info!("Starting ancestor search");
        let result = searchable
            .search::<AncestorSearchTraversal<Self>>(self.ctx())
            .map_err(|err| err.reason);

        match &result {
            Ok(response) => info!(
                "Ancestor search succeeded: end={}",
                pretty(&response.end)
            ),
            Err(reason) => info!("Ancestor search failed: {}", pretty(reason)),
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
pub struct FoldCtx<K: TraversalKind> {
    pub(crate) matches: MatchIterator<K>,
    //pub(crate) start_index: Token,
    pub(crate) last_match: MatchState,
}

impl<K: TraversalKind> Iterator for FoldCtx<K> {
    type Item = EndState;
    fn next(&mut self) -> Option<Self::Item> {
        trace!("FoldCtx::next - searching for next match");
        match self.matches.find_next() {
            Some(end) => {
                debug!("Found end state: {}", pretty(&end));

                // Get the start length from the previous match or query
                let start_len = match &self.last_match {
                    MatchState::Located(prev_end) => prev_end.start_len(),
                    MatchState::Query(_) => {
                        // First match: start from beginning of query
                        debug!("First match found, transitioning from Query to Located state");
                        0
                    },
                };
                debug!("Tracing from position: {}", start_len);

                TraceStart {
                    end: &end,
                    pos: start_len,
                }
                .trace(&mut self.matches.0);

                // Update last_match to the located state
                self.last_match = MatchState::Located(end.clone());
                Some(end.clone())
            },
            None => {
                trace!("No more matches found");
                None
            },
        }
    }
}

impl<K: TraversalKind> FoldCtx<K> {
    #[instrument(skip(self))]
    pub(crate) fn search(mut self) -> Response {
        info!("Starting fold search");
        debug!(
            "Initial state: matches={}, last_match={}",
            pretty(&self.matches),
            pretty(&self.last_match)
        );

        let mut iteration = 0;
        while let Some(end) = &mut self.next() {
            iteration += 1;
            debug!("Fold iteration {}: tracing end state", iteration);
            trace!("End state details: {}", pretty(end));
            end.trace(&mut self.matches.0);
        }

        info!("Fold completed after {} iterations", iteration);

        // Get the final end state
        let end = match self.last_match {
            MatchState::Located(end_state) => {
                debug!("Final state is located: {}", pretty(&end_state));
                end_state
            },
            MatchState::Query(query_path) => {
                // No matches were found - need to create an appropriate error/incomplete state
                debug!("No matches found, still in query state");
                // TODO: Create proper EndState for "no match" case
                // For now, create a minimal EndState
                // The query_path has a Pattern root, get the first token
                let start_token = query_path.path_root()[0];
                let cursor = PatternCursor {
                    atom_position: AtomPosition::default(),
                    path: query_path.clone(),
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

        debug!("Final end state: {}", pretty(&end));

        let trace_ctx = &mut self.matches.0;
        end.trace(trace_ctx);

        let response = Response {
            cache: self.matches.0.cache,
            end,
        };

        info!("Search complete: {}", pretty(&response));
        response
    }
}
