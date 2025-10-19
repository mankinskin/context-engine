use std::collections::VecDeque;

use crate::{
    compare::{
        iterator::CompareIterator,
        parent::ParentCompareState,
        state::{
            CompareState,
            TokenMatchState,
        },
    },
    r#match::root_cursor::RootCursor,
    traversal::{
        policy::DirectedTraversalPolicy,
        TraversalKind,
    },
};
use context_trace::*;

use derive_new::new;
pub(crate) mod iterator;
pub(crate) mod root_cursor;

#[derive(Debug, new)]
pub(crate) struct MatchCtx {
    #[new(default)]
    pub(crate) nodes: VecDeque<TraceNode>,
}

#[derive(Debug)]
pub(crate) struct RootSearchIterator<'a, K: TraversalKind> {
    pub(crate) ctx: &'a mut MatchCtx,
    pub(crate) trav: &'a K::Trav,
}
impl<'a, K: TraversalKind> RootSearchIterator<'a, K> {
    pub(crate) fn new(
        trav: &'a K::Trav,
        ctx: &'a mut MatchCtx,
    ) -> Self {
        Self { ctx, trav }
    }

    pub(crate) fn find_root_cursor(
        mut self
    ) -> Option<RootCursor<&'a K::Trav>> {
        self.find_map(|root| root).map(|state| RootCursor {
            trav: self.trav,
            state: Box::new(state),
        })
    }
}

impl<K: TraversalKind> Iterator for RootSearchIterator<'_, K> {
    type Item = Option<CompareState>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ctx.nodes.pop_front().and_then(|node| {
            PolicyNode::<'_, K>::new(node, self.trav).consume()
        }) {
            Some(Append(next)) => {
                self.ctx.nodes.extend(next);
                Some(None)
            },
            Some(Match(cs)) => {
                self.ctx.nodes.clear();
                Some(Some(cs))
            },
            Some(Pass) => Some(None),
            None => None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum TraceStep {
    Append(Vec<TraceNode>),
    Match(CompareState),
    Pass,
}
use TraceStep::*;

#[derive(Debug)]
pub(crate) enum TraceNode {
    Parent(ParentCompareState),
    Child(ChildQueue<CompareState>),
}
use TraceNode::*;

#[derive(Debug, new)]
struct PolicyNode<'a, K: TraversalKind>(TraceNode, &'a K::Trav);

impl<K: TraversalKind> PolicyNode<'_, K> {
    fn consume(self) -> Option<TraceStep> {
        match self.0 {
            Parent(parent) => match parent.into_advanced(&self.1) {
                Ok(state) => PolicyNode::<K>::new(
                    Child(ChildQueue::from_iter([state.token])),
                    self.1,
                )
                .consume(),
                Err(parent) => Some(Append(
                    K::Policy::next_batch(self.1, &parent)
                        .into_iter()
                        .flat_map(|batch| batch.parents)
                        .map(|parent_state| ParentCompareState {
                            parent_state,
                            cursor: parent.cursor.clone(),
                        })
                        .map(Parent)
                        .collect(),
                )),
            },
            Child(queue) => {
                let mut compare_iter =
                    CompareIterator::<&K::Trav>::new(self.1, queue);
                match compare_iter.next() {
                    Some(Some(TokenMatchState::Match(cs))) => Some(Match(cs)),
                    Some(Some(TokenMatchState::Mismatch(_))) => Some(Pass),
                    Some(None) =>
                        Some(Append(vec![Child(compare_iter.children.queue)])),
                    None => None,
                }
            },
        }
    }
}
