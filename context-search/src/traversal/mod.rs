pub(crate) mod policy;

use crate::{
    container::StateContainer,
    cursor::PatternCursor,
    fold::foldable::ErrorState,
    r#match::{
        iterator::MatchIterator,
        MatchCtx,
        TraceNode::Parent,
    },
    state::{
        end::{
            postfix::PostfixEnd,
            EndKind,
            EndReason,
            EndState,
        },
        start::StartCtx,
    },
    CompleteState,
};
use context_trace::{
    path::accessors::has_path::HasRootedRolePath,
    *,
};
use derive_new::new;
use policy::DirectedTraversalPolicy;
use std::fmt::Debug;
use tracing::debug;

pub trait TraversalKind: Debug + Default {
    type Trav: HasGraph;
    type Container: StateContainer;
    type Policy: DirectedTraversalPolicy<Trav = Self::Trav>;
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum OptGen<Y> {
    Yield(Y),
    Pass,
}

pub(crate) trait HasTraversalCtx<K: TraversalKind> {
    fn traversal_context(&self) -> &TraversalCtx<K>;
}
pub(crate) trait TryIntoTraversalCtx<K: TraversalKind> {
    fn try_into_traversal_context(self) -> Result<TraversalCtx<K>, ErrorState>;
}
pub(crate) trait IntoTraversalCtx<K: TraversalKind> {
    fn into_traversal_context(self) -> TraversalCtx<K>;
}

/// context for generating next states
#[derive(Debug, new)]
pub(crate) struct TraversalCtx<K: TraversalKind> {
    pub(crate) match_iter: MatchIterator<K>,
    pub(crate) last_match: EndState,
}
impl<K: TraversalKind> Unpin for TraversalCtx<K> {}

impl<K: TraversalKind> IntoTraversalCtx<K> for TraversalCtx<K> {
    fn into_traversal_context(self) -> TraversalCtx<K> {
        self
    }
}
impl<K: TraversalKind> TryIntoTraversalCtx<K> for StartCtx<K> {
    fn try_into_traversal_context(self) -> Result<TraversalCtx<K>, ErrorState> {
        match self.get_parent_batch() {
            Ok(p) => {
                debug!("First ParentBatch {:?}", p);
                Ok(TraversalCtx {
                    last_match: EndState {
                        reason: EndReason::QueryEnd,
                        kind: EndKind::Complete(CompleteState::new_token(
                            self.index, &self.trav,
                        )),
                        cursor: self.cursor,
                    },
                    match_iter: MatchIterator::new(
                        TraceCtx {
                            trav: self.trav,
                            cache: TraceCache::new(self.index),
                        },
                        MatchCtx {
                            nodes: FromIterator::from_iter(
                                p.into_compare_batch().into_iter().map(Parent),
                            ),
                        },
                    ),
                })
            },
            Err(err) => Err(err),
        }
    }
}
impl<K: TraversalKind> Iterator for TraversalCtx<K> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        match self.match_iter.find_next() {
            Some(end) => {
                debug!("Found end {:#?}", end);
                TraceStart {
                    end: &end,
                    pos: self.last_match.start_len(),
                }
                .trace(&mut self.match_iter.0);
                self.last_match = end;
                Some(())
            },
            None => None,
        }
    }
}
#[derive(Clone, Debug)]
pub(crate) struct TraceStart<'a> {
    pub(crate) end: &'a EndState,
    pub(crate) pos: usize,
}

impl Traceable for TraceStart<'_> {
    fn trace<G: HasGraph>(
        self,
        ctx: &mut TraceCtx<G>,
    ) {
        if let Some(mut p) = match self.end.kind.clone() {
            EndKind::Postfix(p) => Some(p),
            EndKind::Range(p) => Some(PostfixEnd {
                path: p.path.into_rooted_role_path(),
                root_pos: p.root_pos,
            }),
            _ => None,
        } {
            p.rooted_role_path_mut().drain(0..self.pos);
            p.trace(ctx);
        }
    }
}

impl<K: TraversalKind> HasGraph for &'_ TraversalCtx<K> {
    type Kind = TravKind<K::Trav>;
    type Guard<'g>
        = <K::Trav as HasGraph>::Guard<'g>
    where
        Self: 'g;
    fn graph(&self) -> Self::Guard<'_> {
        self.match_iter.0.trav.graph()
    }
}

impl<K: TraversalKind> HasGraph for &mut TraversalCtx<K> {
    type Kind = TravKind<K::Trav>;
    type Guard<'g>
        = <K::Trav as HasGraph>::Guard<'g>
    where
        Self: 'g;
    fn graph(&self) -> Self::Guard<'_> {
        self.match_iter.0.trav.graph()
    }
}
