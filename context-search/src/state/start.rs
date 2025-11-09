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
    path::{
        accessors::child::RootedLeafToken,
        BaseQuery,
    },
    *,
};
use tracing::debug;

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
        if let Some(loc) = self.role_leaf_token_location::<End>() {
            InputLocation::Location(loc.into_pattern_location())
        } else {
            InputLocation::PatternChild {
                sub_index: self.role_root_child_index::<End>(),
                token: self.role_rooted_leaf_token::<End, _>(trav),
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
    pub(crate) location: PatternLocation,
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
    fn search<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<Response, ErrorState> {
        match self.start_search::<K>(trav) {
            Ok(ctx) => Ok(ctx.search()),
            Err(err) => Err(err),
        }
    }
}

impl Searchable for PatternCursor {
    fn start_search<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        let in_loc = self.path.input_location(&trav);
        let location = match in_loc.clone() {
            InputLocation::Location(loc) => loc,
            InputLocation::PatternChild { token, .. } => PatternLocation::new(
                token,
                *trav
                    .graph()
                    .expect_vertex(token)
                    .expect_any_child_pattern()
                    .0,
            ),
        };
        let start = StartCtx {
            location,
            cursor: self,
        };
        //let start = self.into_start_ctx(start, &trav);

        match start.get_parent_batch::<K>(&trav) {
            Ok(p) => {
                debug!("First ParentBatch {:?}", p);
                Ok(FoldCtx {
                    //start_index: start.location.parent,
                    last_match: EndState::init_fold(start),
                    matches: MatchIterator::start_parent(
                        trav,
                        location.parent,
                        p,
                    ),
                })
            },
            Err(err) => Err(err),
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
    fn start_search<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        PatternRangePath::from(self).start_search::<K>(trav)
    }
}
impl Searchable for &'_ [Token] {
    fn start_search<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
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
    fn start_search<K: TraversalKind>(
        self,
        trav: K::Trav,
    ) -> Result<FoldCtx<K>, ErrorState> {
        self.to_range_path()
            .to_cursor(&trav)
            .start_search::<K>(trav)
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
