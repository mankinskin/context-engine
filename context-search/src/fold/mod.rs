use crate::{
    r#match::iterator::MatchIterator,
    state::{
        end::EndState,
        result::Response,
    },
    traversal::{
        TraceStart,
        TraversalKind,
    },
};
use context_trace::{
    direction::pattern::PatternDirection,
    path::{
        accessors::has_path::HasRolePath,
        BaseQuery,
    },
    *,
};
use std::fmt::Debug;
use tracing::debug;

pub(crate) mod ext;
pub(crate) mod final_state;
pub(crate) mod foldable;

/// context for running fold traversal
#[derive(Debug)]
pub struct FoldCtx<K: TraversalKind> {
    pub(crate) matches: MatchIterator<K>,
    pub(crate) start_index: Token,
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
    fn fold(mut self) -> Response {
        debug!("Starting fold {:#?}", self);

        for end in &mut self.next() {
            //end.trace(&mut self.matches.0);
        }
        // last end
        let end = self.last_match;
        let trace_ctx = &mut self.matches.0;
        end.trace(trace_ctx);
        Response {
            cache: self.matches.0.cache,
            start: self.start_index,
            end,
        }
    }
}

pub trait StartFoldPath:
BaseQuery
//+ LeafTokenPosMut<End>
+ PathAppend
+ PathPop
+ MoveRootIndex<Right, End>
+ LeafToken<End>
+ RootPattern
+ CalcWidth
{
    fn to_range_path(self) -> PatternRangePath;
    fn complete(pattern: impl IntoPattern) -> Self;
    fn new_directed<
        D: PatternDirection,
    >(query: Pattern) -> Result<Self, (ErrorReason, Self)>;
    // returns Location in graphs or sub index in root pattern
    fn start_location<G: HasGraph>(
        &self,
        trav: G,
    ) -> StartLocationResult;
}
pub(crate) type StartLocationResult = Result<PatternLocation, usize>;

//

impl StartFoldPath for PatternRangePath {
    fn start_location<G: HasGraph>(
        &self,
        _trav: G,
    ) -> StartLocationResult {
        self.role_leaf_token_location::<End>()
            .map(context_trace::IntoPatternLocation::into_pattern_location)
            .ok_or(self.end_path().root_child_index())
    }
    fn to_range_path(self) -> PatternRangePath {
        self
    }
    fn complete(query: impl IntoPattern) -> Self {
        let query = query.into_pattern();
        let len = query.len();
        Self::new_range(query, 0, len - 1)
    }
    fn new_directed<D: PatternDirection>(
        query: Pattern
    ) -> Result<Self, (ErrorReason, Self)> {
        let entry = D::head_index(&query);
        let query = query.into_pattern();
        let len = query.len();
        let query = Self::new_range(query, entry, entry);
        match len {
            0 => Err((ErrorReason::EmptyPatterns, query)),
            1 => Err((
                ErrorReason::SingleIndex(Box::new(IndexWithPath::from(
                    query.clone(),
                ))),
                query,
            )),
            _ => Ok(query),
        }
    }
}
impl StartFoldPath for PatternEndPath {
    fn start_location<G: HasGraph>(
        &self,
        trav: G,
    ) -> StartLocationResult {
        self.role_leaf_token_location::<End>()
            .map(context_trace::IntoPatternLocation::into_pattern_location)
            .ok_or(self.end_path().root_child_index())
    }
    fn to_range_path(self) -> PatternRangePath {
        self.into_range(0)
    }
    fn complete(query: impl IntoPattern) -> Self {
        let query = query.into_pattern();
        let len = query.len();
        Self::new_range(query, 0, len - 1)
    }
    fn new_directed<D>(query: Pattern) -> Result<Self, (ErrorReason, Self)> {
        let query = query.into_pattern();
        let len = query.len();
        let p = Self::new_empty(query, 0);
        match len {
            0 => Err((ErrorReason::EmptyPatterns, p)),
            1 => Err((
                ErrorReason::SingleIndex(Box::new(
                    PatternRangePath::from(p.clone()).into(),
                )),
                p,
            )),
            _ => Ok(p),
        }
    }
}
