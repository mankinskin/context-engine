use std::{
    collections::VecDeque,
    marker::PhantomData,
};

use crate::{
    compare::{
        iterator::CompareIterator,
        parent::ParentCompareState,
        state::{
            CandidateCompareState,
            CompareNext,
            CompareNext::*,
            CompareState,
            MatchedCompareState,
        },
    },
    cursor::{
        Candidate,
        PathCursor,
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

#[derive(Debug, new, Default)]
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
    type Item = Option<CandidateCompareState>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ctx.nodes.pop_front().and_then(|node| {
            PolicyNode::<'_, K>::new(node, self.trav).consume()
        }) {
            Some(Append(next)) => {
                self.ctx.nodes.extend(next);
                Some(None)
            },
            Some(TraceStep::Match(matched_state)) => {
                // Don't clear queue - keep exploring other paths
                // Queue will be sorted by root width to explore smallest parents first
                
                // Convert matched state to candidate with checkpoint update
                // If cursor cannot advance, skip this match
                match matched_state.into_next_candidate(self.trav) {
                    Ok(candidate_state) => Some(Some(candidate_state)),
                    Err(_) => Some(None), // Cannot advance, skip
                }
            },
            Some(Pass) => Some(None),
            None => None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum TraceStep {
    Append(Vec<TraceNode>),
    Match(MatchedCompareState),
    Pass,
}
use TraceStep::*;

#[derive(Debug)]
pub(crate) enum TraceNode {
    Parent(ParentCompareState),
    Child(ChildQueue<CompareState<Candidate>>),
}
use TraceNode::*;
#[derive(Debug, new)]
struct PolicyNode<'a, K: TraversalKind>(TraceNode, &'a K::Trav);

impl<K: TraversalKind> PolicyNode<'_, K> {
    fn compare_next(
        trav: &K::Trav,
        queue: ChildQueue<CompareState<Candidate>>,
    ) -> Option<TraceStep> {
        let mut compare_iter = CompareIterator::<&K::Trav>::new(trav, queue);
        match compare_iter.next() {
            Some(Some(CompareNext::Match(matched_state))) => {
                // Return the matched state directly without conversion
                // RootCursor will handle the conversion to Candidate with checkpoint update
                Some(TraceStep::Match(matched_state))
            },
            Some(Some(CompareNext::Mismatch(_))) => Some(Pass),
            Some(Some(CompareNext::Prefixes(_))) => {
                unreachable!("compare_iter.next() should never return Prefixes - they are consumed by the iterator")
            },
            Some(None) =>
                Some(Append(vec![Child(compare_iter.children.queue)])),
            None => None,
        }
    }
    fn consume(self) -> Option<TraceStep> {
        match self.0 {
            Parent(parent) => match parent.into_advanced(&self.1) {
                Ok(state) => Self::compare_next(
                    self.1,
                    ChildQueue::from_iter([state.token]),
                ),
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
            Child(queue) => Self::compare_next(self.1, queue),
        }
    }
}
