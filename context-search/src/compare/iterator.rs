use crate::{
    compare::state::{
        CompareNext,
        CompareState,
    },
    cursor::{
        Candidate,
        Matched,
        Mismatched,
    },
};
use context_trace::*;

use std::fmt::Debug;

use crate::compare::state::CompareNext::*;

#[derive(Debug)]
pub(crate) struct CompareIterator<G: HasGraph> {
    pub(crate) children: ChildIterator<G, CompareState<Candidate>>,
}

impl<G: HasGraph> CompareIterator<G> {
    pub(crate) fn new(
        trav: G,
        queue: impl Into<ChildQueue<CompareState<Candidate>>>,
    ) -> Self {
        Self {
            children: ChildIterator::new(trav, queue),
        }
    }
    pub(crate) fn find_match(self) -> Option<CompareState<Matched>> {
        match self.compare() {
            Mismatch(_) => None,
            Match(state) => Some(state),
            Prefixes(_) =>
                unreachable!("compare() always returns Match or Mismatch"),
        }
    }
    pub(crate) fn compare(mut self) -> CompareNext {
        self.find_map(|flow| flow).unwrap()
    }
}
impl<G: HasGraph> Iterator for CompareIterator<G> {
    type Item = Option<CompareNext>;
    fn next(&mut self) -> Option<Self::Item> {
        tracing::debug!(
            queue_len = self.children.queue.len(),
            "CompareIterator::next called"
        );
        self.children.next().map(|cs| {
            tracing::debug!(
                state = ?cs,
                "CompareIterator: processing state, calling next_match"
            );
            match cs.next_match(&self.children.trav) {
                Prefixes(next) => {
                    tracing::debug!(
                        num_prefixes = next.len(),
                        "CompareIterator: got Prefixes, extending queue"
                    );
                    self.children.queue.extend(next);
                    None
                },
                result => {
                    tracing::debug!(
                        result = ?result,
                        "CompareIterator: got result (Match/Mismatch)"
                    );
                    Some(result)
                },
            }
        })
    }
}
