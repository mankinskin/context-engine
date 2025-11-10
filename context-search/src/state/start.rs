use crate::{
    cursor::{
        PathCursor,
        PatternCursor,
        PatternPrefixCursor,
    },
    r#match::{
        iterator::MatchIterator,
        root_cursor::CompareParentBatch,
    },
    search::{
        searchable::ErrorState,
        FoldCtx,
    },
    state::end::EndState,
    traversal::{
        policy::DirectedTraversalPolicy,
        TraversalKind,
    },
    Response,
};
use context_trace::{
    logging::format_utils::pretty,
    path::{
        accessors::child::RootedLeafToken,
        BaseQuery,
    },
    *,
};
use tracing::{
    debug,
    instrument,
    trace,
    warn,
};

pub(crate) trait ToCursor: StartFoldPath {
    fn to_cursor<G: HasGraph>(
        self,
        trav: &G,
    ) -> PathCursor<Self>;
}
impl<P: StartFoldPath> ToCursor for P {
    fn to_cursor<G: HasGraph>(
        self,
        trav: &G,
    ) -> PathCursor<Self> {
        PathCursor {
            atom_position: self.calc_width(trav).into(),
            path: self,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InputLocation {
    Location(PatternLocation),
    PatternChild { sub_index: usize, token: Token },
}
impl GraphRoot for InputLocation {
    fn root_parent(&self) -> Token {
        match self {
            InputLocation::Location(loc) => loc.parent,
            InputLocation::PatternChild { token, .. } => *token,
        }
    }
}

pub trait StartFoldPath:
BaseQuery
//+ LeafTokenPosMut<End>
+ PathAppend
+ PathPop
+ MoveRootIndex<Right, End>
+ RootedLeafToken<End>
+ RootPattern
+ CalcWidth
{
    fn to_range_path(self) -> PatternRangePath;
    //fn complete(pattern: impl IntoPattern) -> Self;
    //fn new_directed<
    //    D: PatternDirection,
    //>(query: Pattern) -> Result<Self, (ErrorReason, Self)>;
    //// returns Location in graphs or sub index in root pattern
    //fn into_start_ctx<G: HasGraph>(
    //    self,
    //    trav: &G,
    //) -> Result<StartCtx, ErrorState> {
    //    let cursor = self.to_cursor(trav);
    //    let location = self.input_location(trav);
    //    Ok(StartCtx {
    //        location,
    //        cursor,
    //    })
    //}
    fn input_location<G: HasGraph>(
        &self,
        trav: &G,
    ) -> InputLocation {
        trace!("Determining input_location for path");
        
        if let Some(loc) = self.role_leaf_token_location::<End>() {
            debug!("Found leaf token location: {}", pretty(&loc));
            let pattern_loc = loc.into_pattern_location();
            debug!("Converted to pattern location: {}", pretty(&pattern_loc));
            InputLocation::Location(pattern_loc)
        } else {
            debug!("No leaf token location, getting pattern child");
            let sub_index = self.role_root_child_index::<End>();
            let token = self.role_rooted_leaf_token::<End, _>(trav);
            debug!("Pattern child: token={}, sub_index={}", pretty(&token), sub_index);
            
            // This is where the panic will happen - when we try to use this token
            // and it doesn't have children
            trace!("Checking token vertex data in graph");
            if let Ok(vertex_data) = trav.graph().get_vertex(token.vertex_index()) {
                trace!("Token vertex data: {}", pretty(vertex_data));
                let child_patterns = vertex_data.child_patterns();
                if child_patterns.is_empty() {
                    warn!("WARNING: Token {} has no child patterns! This will cause a panic.", 
                          pretty(&token));
                    warn!("This typically means you're trying to search for atoms directly without creating a pattern first.");
                    warn!("Consider using find_sequence() instead of find_ancestor() for raw atoms.");
                }
            }
            
            InputLocation::PatternChild {
                sub_index,
                token,
            }
        }
    }
}

//

impl StartFoldPath for PatternRangePath {
    fn to_range_path(self) -> PatternRangePath {
        self
    }
    //fn complete(query: impl IntoPattern) -> Self {
    //    let query = query.into_pattern();
    //    let len = query.len();
    //    Self::new_range(query, 0, len - 1)
    //}
    //fn new_directed<D: PatternDirection>(
    //    query: Pattern
    //) -> Result<Self, (ErrorReason, Self)> {
    //    let entry = D::head_index(&query);
    //    let query = query.into_pattern();
    //    let len = query.len();
    //    let query = Self::new_range(query, entry, entry);
    //    match len {
    //        0 => Err((ErrorReason::EmptyPatterns, query)),
    //        1 => Err((
    //            ErrorReason::SingleIndex(Box::new(IndexWithPath::from(
    //                query.clone(),
    //            ))),
    //            query,
    //        )),
    //        _ => Ok(query),
    //    }
    //}
}
impl StartFoldPath for PatternEndPath {
    fn to_range_path(self) -> PatternRangePath {
        self.into_range(0)
    }
    //fn complete(query: impl IntoPattern) -> Self {
    //    let query = query.into_pattern();
    //    let len = query.len();
    //    Self::new_range(query, 0, len - 1)
    //}
    //fn new_directed<D>(query: Pattern) -> Result<Self, (ErrorReason, Self)> {
    //    let query = query.into_pattern();
    //    let len = query.len();
    //    let p = Self::new_empty(query, 0);
    //    match len {
    //        0 => Err((ErrorReason::EmptyPatterns, p)),
    //        1 => Err((
    //            ErrorReason::SingleIndex(Box::new(
    //                PatternRangePath::from(p.clone()).into(),
    //            )),
    //            p,
    //        )),
    //        _ => Ok(p),
    //    }
    //}
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct StartCtx {
    pub(crate) cursor: PatternCursor,
}

//impl HasVertexIndex for StartCtx {
//    fn vertex_index(&self) -> VertexIndex {
//        self.location.parent.vertex_index()
//    }
//}
//impl Wide for StartCtx {
//    fn width(&self) -> usize {
//        self.location.parent.width()
//    }
//}
impl StartCtx {
    //pub fn start_index(&self) -> Token {
    //    match self.location {
    //        Ok(ref loc) => loc.parent,
    //        Err(ref e) => e.index,
    //    }
    //}
    //
    pub(crate) fn get_parent_batch<K: TraversalKind>(
        &self,
        trav: &K::Trav,
    ) -> Result<CompareParentBatch, ErrorState> {
        let mut cursor = self.cursor.clone();
        let parent = self.cursor.path.role_root_child_token::<End, _>(trav);
        if cursor.advance(trav).is_continue() {
            let batch = K::Policy::gen_parent_batch(trav, parent, |trav, p| {
                parent.into_parent_state(trav, p)
            });

            Ok(CompareParentBatch { batch, cursor })
        } else {
            Err(ErrorState {
                reason: ErrorReason::SingleIndex(Box::new(IndexWithPath {
                    index: parent,
                    path: self.cursor.path.clone().into(),
                })),
                found: None,
            })
        }
    }
}

pub trait Searchable: Sized {
    fn start_search<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState>;

    #[instrument(skip(self, trav))]
    fn search<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<Response, ErrorState> {
        debug!("Searchable::search - starting search");
        match self.start_search::<K>(trav) {
            Ok(ctx) => {
                debug!("Start search successful, beginning fold");
                Ok(ctx.search())
            },
            Err(err) => {
                debug!("Start search failed: {}", pretty(&err));
                Err(err)
            },
        }
    }
}

impl Searchable for PatternCursor {
    #[instrument(skip(self, trav))]
    fn start_search<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        debug!("PatternCursor::start_search");

        // Get the starting token from the query pattern for the MatchIterator
        let start_token = self.path.role_root_child_token::<End, _>(&trav);
        debug!("Starting search from token: {}", pretty(&start_token));

        let start = StartCtx {
            cursor: self.clone(),
        };

        match start.get_parent_batch::<K>(&trav) {
            Ok(p) => {
                debug!(
                    "First ParentBatch obtained with {} items",
                    p.batch.len()
                );
                trace!("ParentBatch details: {}", pretty(&p));

                Ok(FoldCtx {
                    last_match: EndState::init_fold(self),
                    matches: MatchIterator::start_parent(
                        trav,
                        start_token,
                        p,
                    ),
                })
            },
            Err(err) => {
                debug!("Failed to get parent batch: {}", pretty(&err));
                Err(err)
            },
        }
    }
}

impl<T: Searchable + Clone> Searchable for &T {
    fn start_search<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        self.clone().start_search(trav)
    }
}

impl<const N: usize> Searchable for &'_ [Token; N] {
    #[instrument(skip(self, trav), fields(token_count = N))]
    fn start_search<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        debug!("Searchable for [Token; {}] - creating PatternRangePath", N);
        trace!("Tokens: {:?}", self);
        
        // Delegate to slice implementation which handles atom special case
        self.as_slice().start_search::<K>(trav)
    }
}
impl Searchable for &'_ [Token] {
    #[instrument(skip(self, trav), fields(token_count = self.len()))]
    fn start_search<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        debug!("Searchable for &[Token] - creating PatternRangePath from {} tokens", self.len());
        trace!("Tokens: {:?}", self);
        
        // Convert the token slice to a PatternRangePath and start the search
        // This works for both atoms and composite patterns now thanks to MatchState::Query
        PatternRangePath::from(self).start_search::<K>(trav)
    }
}
impl Searchable for Pattern {
    fn start_search<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        PatternRangePath::from(self).start_search::<K>(trav)
    }
}

impl Searchable for PatternEndPath {
    fn start_search<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        self.to_range_path()
            .to_cursor(&trav)
            .start_search::<K>(trav)
    }
}
impl Searchable for PatternRangePath {
    #[instrument(skip(self, trav), fields(path = ?self))]
    fn start_search<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        debug!("PatternRangePath::start_search - converting to cursor");
        trace!("PatternRangePath details: {}", pretty(&self));
        
        let range_path = self.to_range_path();
        debug!("Converted to range_path: {}", pretty(&range_path));
        
        let cursor = range_path.to_cursor(&trav);
        debug!("Created cursor: {}", pretty(&cursor));
        
        cursor.start_search::<K>(trav)
    }
}
impl Searchable for PatternPrefixCursor {
    fn start_search<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        PatternCursor::from(self).start_search(trav)
    }
}
