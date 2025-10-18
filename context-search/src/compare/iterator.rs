use crate::compare::state::{
    CompareState,
    TokenMatchState::{
        self,
        Match,
        Mismatch,
    },
};
use context_trace::*;

use std::fmt::Debug;

use crate::compare::state::CompareNext::*;

#[derive(Debug)]
pub(crate) struct CompareIterator<G: HasGraph> {
    pub(crate) children: ChildIterator<G, CompareState>,
}

impl<G: HasGraph> CompareIterator<G> {
    pub(crate) fn new(
        trav: G,
        queue: impl Into<ChildQueue<CompareState>>,
    ) -> Self {
        Self {
            children: ChildIterator::new(trav, queue),
        }
    }
    pub(crate) fn find_match(self) -> Option<CompareState> {
        match self.compare() {
            Mismatch(_) => None,
            Match(state) => Some(state),
        }
    }
    pub(crate) fn compare(mut self) -> TokenMatchState {
        self.find_map(|flow| flow).unwrap()
    }
}
impl<G: HasGraph> Iterator for CompareIterator<G> {
    type Item = Option<TokenMatchState>;
    fn next(&mut self) -> Option<Self::Item> {
        self.children.next().map(|cs| {
            match cs.next_match(&self.children.trav) {
                Prefixes(next) => {
                    self.children.queue.extend(next);
                    None
                },
                MatchState(state) => Some(state),
            }
        })
    }
}
