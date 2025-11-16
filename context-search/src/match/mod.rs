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
            CompareResult,
            CompareResult::*,
            CompareState,
            MatchedCompareState,
        },
    },
    cursor::{
        Candidate,
        Matched,
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
pub(crate) struct SearchQueue {
    #[new(default)]
    pub(crate) nodes: VecDeque<SearchNode>,
}

impl std::fmt::Display for SearchQueue {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        write!(f, "SearchQueue(len={})", self.nodes.len())
    }
}

#[derive(Debug)]
pub(crate) struct RootFinder<'a, K: TraversalKind> {
    pub(crate) ctx: &'a mut SearchQueue,
    pub(crate) trav: &'a K::Trav,
}
impl<'a, K: TraversalKind> RootFinder<'a, K> {
    pub(crate) fn new(
        trav: &'a K::Trav,
        ctx: &'a mut SearchQueue,
    ) -> Self {
        Self { ctx, trav }
    }

    pub(crate) fn find_root_cursor(
        mut self
    ) -> Option<RootCursor<&'a K::Trav, Matched, Matched>> {
        self.find_map(|root| root).map(|matched_state| {
            // Return a Matched RootCursor - caller will advance cursors
            RootCursor {
                trav: self.trav,
                state: Box::new(matched_state),
            }
        })
    }
}

impl<K: TraversalKind> Iterator for RootFinder<'_, K> {
    type Item = Option<MatchedCompareState>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ctx.nodes.pop_front().and_then(|node| {
            NodeConsumer::<'_, K>::new(node, self.trav).consume()
        }) {
            Some(QueueMore(next)) => {
                self.ctx.nodes.extend(next);
                Some(None)
            },
            Some(NodeResult::FoundMatch(matched_state)) => {
                // Found a root match - return it for RootCursor creation
                // RootCursor will handle cursor advancement and determine when to add cache entries
                Some(Some(matched_state))
            },
            Some(Skip) => Some(None),
            None => None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum NodeResult {
    QueueMore(Vec<SearchNode>),
    FoundMatch(MatchedCompareState),
    Skip,
}
use NodeResult::*;

#[derive(Debug)]
pub(crate) enum SearchNode {
    ParentCandidate(ParentCompareState),
    PrefixQueue(ChildQueue<CompareState<Candidate, Candidate>>),
}
use SearchNode::*;
#[derive(Debug, new)]
struct NodeConsumer<'a, K: TraversalKind>(SearchNode, &'a K::Trav);

impl<K: TraversalKind> NodeConsumer<'_, K> {
    fn compare_next(
        trav: &K::Trav,
        queue: ChildQueue<CompareState<Candidate, Candidate>>,
    ) -> Option<NodeResult> {
        let mut compare_iter = CompareIterator::<&K::Trav>::new(trav, queue);
        match compare_iter.next() {
            Some(Some(CompareResult::FoundMatch(matched_state))) => {
                // Return the matched state directly without conversion
                // RootCursor will handle the conversion to Candidate with checkpoint update
                Some(NodeResult::FoundMatch(matched_state))
            },
            Some(Some(CompareResult::Mismatch(_))) => Some(Skip),
            Some(Some(CompareResult::Prefixes(_))) => {
                unreachable!("compare_iter.next() should never return Prefixes - they are consumed by the iterator")
            },
            Some(None) =>
                Some(QueueMore(vec![PrefixQueue(compare_iter.children.queue)])),
            None => None,
        }
    }
    fn consume(self) -> Option<NodeResult> {
        match self.0 {
            ParentCandidate(parent) => match parent.advance_state(&self.1) {
                Ok(state) => Self::compare_next(
                    self.1,
                    ChildQueue::from_iter([state.token]),
                ),
                Err(parent) => Some(QueueMore(
                    K::Policy::next_batch(self.1, &parent)
                        .into_iter()
                        .flat_map(|batch| batch.parents)
                        .map(|parent_state| ParentCompareState {
                            parent_state,
                            cursor: parent.cursor.clone(),
                        })
                        .map(ParentCandidate)
                        .collect(),
                )),
            },
            PrefixQueue(queue) => Self::compare_next(self.1, queue),
        }
    }
}
