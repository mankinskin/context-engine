use crate::{
    cursor::PatternCursor,
    r#match::iterator::SearchIterator,
    state::{
        end::{
            range::RangeEnd,
            EndReason,
            EndState,
            MatchState,
            PathCoverage,
        },
        matched::MatchedEndState,
        start::Searchable,
    },
    traversal::{
        policy::DirectedTraversalPolicy,
        TraceStart,
    },
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

impl<K: TraversalKind> SearchState<K> {
    /// Extract parent batch from a matched state for queue repopulation
    /// Converts matched root to parent nodes for continued exploration
    fn extract_parent_batch(
        &self,
        matched_state: &MatchedEndState,
    ) -> Option<Vec<crate::r#match::SearchNode>> {
        use crate::{
            compare::parent::ParentCompareState,
            r#match::SearchNode,
        };

        // Extract IndexRangePath and cursor from matched state
        let (index_path, cursor) = matched_state.to_parent_state()?;

        debug!(
            "Extracting parents for matched root: {}",
            index_path.root_parent()
        );

        // Create ChildState from IndexRangePath
        // ChildState wraps IndexRangePath with a current_pos for traversal
        let child_state = ChildState {
            current_pos: cursor.atom_position,
            path: index_path,
        };

        // Get ParentState from ChildState
        let parent_state = child_state.parent_state();

        // Use traversal policy to get parent batch
        let batch =
            K::Policy::next_batch(&self.matches.trace_ctx.trav, &parent_state)?;

        // Convert to SearchNode::ParentCandidate
        let parent_nodes: Vec<SearchNode> = batch
            .parents
            .into_iter()
            .map(|ps| {
                SearchNode::ParentCandidate(ParentCompareState {
                    parent_state: ps,
                    cursor: cursor.clone(),
                })
            })
            .collect();

        debug!(
            "Found {} parents for continued exploration",
            parent_nodes.len()
        );
        Some(parent_nodes)
    }
}

impl<K: TraversalKind> Iterator for SearchState<K> {
    type Item = MatchedEndState;
    fn next(&mut self) -> Option<Self::Item> {
        trace!("searching for next match");
        match self.matches.find_next() {
            Some(matched_state) => {
                debug!(
                    is_complete = matched_state.is_complete(),
                    "found matched state"
                );

                // Compare with previous best match (if any)
                let should_update = match &self.last_match {
                    MatchState::Located(prev_match) => {
                        // Already have a matched root - compare widths
                        // Smaller root token is better (more specific match)
                        let current_width = matched_state.root_parent().width.0;
                        let prev_width = prev_match.root_parent().width.0;
                        current_width < prev_width
                    },
                    MatchState::Query(_) => {
                        // First match - always update
                        true
                    },
                };

                if should_update {
                    debug!(
                        width = matched_state.root_parent().width.0,
                        is_first_match =
                            matches!(&self.last_match, MatchState::Query(_)),
                        is_complete = matched_state.is_complete(),
                        "updating last_match to better match"
                    );

                    // Queue clearing: TEMPORARILY DISABLED FOR DEBUGGING
                    // When first COMPLETE match found, clear candidate parents
                    // and add only parents of matched root for continued exploration
                    //
                    // NOTE: Only clear for Complete matches, not Partial matches
                    // Partial matches mean query continues, so we might find better matches elsewhere
                    let _is_first_match =
                        matches!(&self.last_match, MatchState::Query(_));
                    let _is_complete = matched_state.is_complete();

                    if false {
                        // DISABLED: is_first_match && is_complete
                        debug!(
                            "First COMPLETE match found - clearing queue of {} unmatched candidate parents",
                            self.matches.queue.nodes.len()
                        );

                        // Clear queue: remove all unmatched candidate parents
                        // Substring-graph invariant: all future matches reachable from this root's ancestors
                        self.matches.queue.nodes.clear();

                        // Add parents of matched root for continued exploration
                        if let Some(parent_nodes) =
                            self.extract_parent_batch(&matched_state)
                        {
                            debug!(
                                "Adding {} parent nodes of matched root to queue",
                                parent_nodes.len()
                            );
                            self.matches.queue.nodes.extend(parent_nodes);
                        } else {
                            debug!(
                                "No parents to add (Prefix path or root has no parents)"
                            );
                        }
                    }

                    // Incremental start path tracing
                    // Trace only the NEW portion of the start path to avoid duplicate cache entries
                    if let Some(start_path) =
                        matched_state.path().try_start_path()
                    {
                        // Calculate how much of start path was already traced
                        let prev_start_len = match &self.last_match {
                            MatchState::Located(prev) => prev.start_len(),
                            MatchState::Query(_) => 0,
                        };
                        let current_start_len = start_path.len();

                        // Only trace the NEW segment (avoid duplicate cache entries)
                        if current_start_len > prev_start_len {
                            debug!(
                                "Tracing incremental start path: prev_len={}, current_len={}, new_segment_start={}",
                                prev_start_len, current_start_len, prev_start_len
                            );
                            // Trace from prev_start_len onwards (skips already-traced prefix)
                            TraceStart {
                                end: &matched_state,
                                pos: prev_start_len,
                            }
                            .trace(&mut self.matches.trace_ctx);
                        } else {
                            debug!("Start path not longer than previous - no new segment to trace");
                        }
                    }

                    self.last_match =
                        MatchState::Located(matched_state.clone());
                } else {
                    debug!("not updating last_match - current match not better than previous");
                }

                Some(matched_state)
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
        while let Some(matched_state) = &mut self.next() {
            iteration += 1;
            debug!(iteration, "tracing matched state");
            debug!(
                "About to trace MatchedEndState: is_complete={}, path_variant={}",
                matched_state.is_complete(),
                match matched_state.path() {
                    PathCoverage::Range(_) => "Range",
                    PathCoverage::Postfix(_) => "Postfix",
                    PathCoverage::Prefix(_) => "Prefix",
                    PathCoverage::EntireRoot(_) => "EntireRoot",
                }
            );
            matched_state.trace(&mut self.matches.trace_ctx);
            debug!("Finished tracing MatchedEndState");
        }

        debug!(iterations = iteration, "fold completed");

        // Get the final matched state
        let end = match self.last_match {
            MatchState::Located(matched_state) => {
                debug!("final state is located");
                matched_state
            },
            MatchState::Query(query_path) => {
                // No matches were found - need to create a partial match at position 0
                debug!("no matches found, still in query state");
                // Create a PartialMatchState with checkpoint 0 (no progress)
                let start_token = query_path.path_root()[0];
                let cursor = PatternCursor {
                    atom_position: AtomPosition::default(),
                    path: query_path.clone(),
                    _state: std::marker::PhantomData,
                };
                // Use an EntireRoot path with empty range to represent "no match"
                let path = PathCoverage::EntireRoot(IndexRangePath::new_empty(
                    IndexRoot::from(PatternLocation::new(
                        start_token,
                        PatternId::default(),
                    )),
                ));
                MatchedEndState::Partial(
                    crate::state::matched::PartialMatchState { path, cursor },
                )
            },
        };

        trace!(end = %pretty(&end), "final matched state");

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
