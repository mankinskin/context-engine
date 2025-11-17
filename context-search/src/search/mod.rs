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

    #[context_trace::instrument_sig(skip(self, pattern), fields(pattern_len))]
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

    /// find largest matching parent for pattern
    #[context_trace::instrument_sig(skip(self, searchable))]
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
    #[context_trace::instrument_sig(skip(self, searchable))]
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
                debug!("found end state with reason={:?}", end.reason);

                // Only update last_match if this is a QueryEnd (successful complete match)
                // Mismatches are exploration attempts, not better matches
                if end.reason == EndReason::QueryEnd {
                    debug!("QueryEnd - updating last_match checkpoint");

                    // Check if we already have a Complete match
                    let current_is_complete =
                        matches!(end.path, PathEnum::Complete(_));
                    let should_update = match &self.last_match {
                        MatchState::Located(prev_end) => {
                            let prev_is_complete = prev_end.is_complete();
                            // Update if: new match is Complete, or previous wasn't Complete
                            // This ensures Complete matches are preferred over non-Complete
                            current_is_complete || !prev_is_complete
                        },
                        MatchState::Query(_) => {
                            // First match: always update
                            true
                        },
                    };

                    if should_update {
                        debug!(
                            is_complete = current_is_complete,
                            "updating last_match to new QueryEnd"
                        );

                        // For ancestor search, each QueryEnd is a match in a different root token.
                        // We always trace from position 0 since each root is independent.
                        debug!(
                            "tracing complete QueryEnd match from position 0"
                        );

                        TraceStart { end: &end, pos: 0 }
                            .trace(&mut self.matches.trace_ctx);

                        // Update last_match to this new complete match
                        self.last_match = MatchState::Located(end.clone());
                    } else {
                        debug!(
                            "not updating last_match - already have Complete match"
                        );
                    }
                } else {
                    debug!("Mismatch - not updating last_match, this is just exploration");
                }

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
    #[context_trace::instrument_sig(skip(self))]
    pub(crate) fn search(mut self) -> Response {
        debug!("starting fold search");
        debug!(queue = %&self.matches.queue, "initial state");

        let mut iteration = 0;
        while let Some(end) = &mut self.next() {
            iteration += 1;
            debug!(iteration, "tracing end state");
            debug!(
                "About to trace EndState: reason={:?}, path_variant={}",
                end.reason,
                match &end.path {
                    PathEnum::Range(_) => "Range",
                    PathEnum::Postfix(_) => "Postfix",
                    PathEnum::Prefix(_) => "Prefix",
                    PathEnum::Complete(_) => "Complete",
                }
            );
            end.trace(&mut self.matches.trace_ctx);
            debug!("Finished tracing EndState");
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
