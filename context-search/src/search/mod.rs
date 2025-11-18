use crate::{
    cursor::PatternCursor,
    r#match::iterator::SearchIterator,
    state::{
        end::{
            range::RangeEnd,
            EndReason,
            EndState,
            MatchState,
            PathEnum,
        },
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

impl<K: TraversalKind> Iterator for SearchState<K> {
    type Item = EndState;
    fn next(&mut self) -> Option<Self::Item> {
        trace!("searching for next match");
        match self.matches.find_next() {
            Some(end) => {
                debug!("found end state with reason={:?}", end.reason);

                // QueryEnd means query pattern exhausted within this root (match found)
                // Mismatch means comparison failed within this root
                // Both are valid match results - we want the best (smallest root) match

                // Check if this is first match in any root (candidate parent -> matched root cursor)
                let is_first_match =
                    matches!(&self.last_match, MatchState::Query(_));

                let should_update = match &self.last_match {
                    MatchState::Located(prev_end) => {
                        // Already have a matched root - compare widths
                        // Smaller root token is better (more specific match)
                        let current_width = end.path.root_parent().width.0;
                        let prev_width = prev_end.path.root_parent().width.0;
                        current_width < prev_width
                    },
                    MatchState::Query(_) => {
                        // First match in any root - transition from candidate parent to matched root cursor
                        true
                    },
                };

                if should_update {
                    debug!(
                        width = end.path.root_parent().width.0,
                        is_first = is_first_match,
                        "updating last_match to better match"
                    );

                    if is_first_match {
                        debug!("First match in root - clearing candidate parents and adding matched root's parents");

                        // Clear queue: remove unmatched candidate parents
                        // Substring invariant: all future matches reachable from matched root's ancestors
                        let old_queue_size = self.matches.queue.nodes.len();
                        self.matches.queue.nodes.clear();
                        debug!(
                            "Cleared {} candidate parent nodes from queue",
                            old_queue_size
                        );

                        // Add parents of matched root for continued exploration
                        if let Some(parent_batch) =
                            Self::extract_parent_batch(&end, &self.matches)
                        {
                            let parent_count = parent_batch.len();
                            self.matches.queue.nodes.extend(parent_batch);
                            debug!("Added {} parent nodes of matched root to queue", parent_count);
                        } else {
                            debug!("No parents available for matched root");
                        }

                        // Incremental start path tracing
                        // Trace the start path segment from query start (pos 0) to this first match
                        // This traces the path leading TO the matched root
                        if let Some(start_path) = end.start_path() {
                            let start_len = start_path.len();
                            debug!(
                                "Tracing initial start path segment (len={})",
                                start_len
                            );
                            TraceStart { end: &end, pos: 0 }
                                .trace(&mut self.matches.trace_ctx);
                        } else {
                            debug!("No start path to trace (Complete or Prefix path)");
                        }
                    } else {
                        // Not first match - we already have a matched root, comparing widths
                        // Trace start path segments incrementally from previous match
                        if let Some(start_path) = end.start_path() {
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
                                    end: &end,
                                    pos: prev_start_len,
                                }
                                .trace(&mut self.matches.trace_ctx);
                            } else {
                                debug!("Start path not longer than previous - no new segment to trace");
                            }
                        }
                    }

                    self.last_match = MatchState::Located(end.clone());
                } else {
                    debug!("not updating last_match - current match not better than previous");
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
    /// Extract parents of a matched root for continued exploration
    /// Called when first match found in root (candidate parent -> matched root transition)
    /// Returns parent nodes that should be added to queue for ancestor exploration
    fn extract_parent_batch(
        end: &EndState,
        matches: &SearchIterator<K>,
    ) -> Option<Vec<crate::r#match::SearchNode>> {
        use crate::{
            compare::parent::ParentCompareState,
            r#match::SearchNode,
        };

        // Get the root parent from the matched path
        let root_parent = end.path.root_parent();
        debug!("Extracting parents for matched root: {}", root_parent);

        // Get the cursor from the end state
        let cursor = end.cursor.clone();

        // Extract IndexRangePath and root_pos from the matched path
        // Different PathEnum variants need different handling
        let (child_state_path, root_pos) = match &end.path {
            PathEnum::Complete(p) => (p.clone(), cursor.atom_position),
            PathEnum::Range(p) => (p.path.clone(), p.root_pos),
            PathEnum::Postfix(p) => {
                // Postfix uses RootedRolePath, convert to IndexRangePath
                (p.path.clone().into(), p.root_pos)
            },
            PathEnum::Prefix(_) => {
                // Prefix paths don't have a clear parent structure
                debug!("Prefix path - no parent extraction");
                return None;
            },
        };

        // Create ChildState from the path (ChildState is re-exported publicly from context_trace)
        let child_state = ChildState {
            current_pos: root_pos,
            path: child_state_path,
        };

        // Extract parent state using the public parent_state() method
        let parent_state = child_state.parent_state();

        // Get next batch of parents using the traversal policy
        if let Some(batch) =
            K::Policy::next_batch(&matches.trace_ctx.trav, &parent_state)
        {
            let parent_nodes: Vec<SearchNode> = batch
                .parents
                .into_iter()
                .map(|parent_state| {
                    SearchNode::ParentCandidate(ParentCompareState {
                        parent_state,
                        cursor: cursor.clone(),
                    })
                })
                .collect();

            debug!("Found {} parents for matched root", parent_nodes.len());
            Some(parent_nodes)
        } else {
            debug!("No parents available for matched root");
            None
        }
    }

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
