use crate::{
    compare::state::CompareState,
    cursor::{
        Candidate,
        ChildCursor,
        PathCursor,
        PatternCursor,
    },
};
use context_trace::*;
use derive_more::{
    Deref,
    DerefMut,
};
use std::{
    fmt::Debug,
    marker::PhantomData,
};
use tracing::debug;

use crate::compare::state::PathPairMode::GraphMajor;
use context_trace::{
    graph::vertex::token::Token,
    path::accessors::has_path::IntoRootedRolePath,
    trace::cache::key::directed::down::{
        DownKey,
        DownPosition,
    },
};
#[derive(Clone, Debug, PartialEq, Eq, Deref, DerefMut)]
pub(crate) struct CompareRootState {
    #[deref]
    #[deref_mut]
    pub(crate) token: CompareState<Candidate, Candidate>,
    pub(crate) root_parent: ParentState,
}

#[derive(Clone, Debug, Deref, DerefMut)]
pub(crate) struct ParentCompareState {
    #[deref]
    #[deref_mut]
    pub(crate) parent_state: ParentState,
    pub(crate) cursor: PatternCursor,
}
#[context_trace::instrument_trait_impl]
impl StateAdvance for ParentCompareState {
    type Next = CompareRootState;
    #[context_trace::instrument_sig(skip(self, trav))]
    fn advance_state<G: HasGraph>(
        self,
        trav: &G,
    ) -> Result<Self::Next, Self> {
        match self.parent_state.advance_state(trav) {
            Ok(next) => {
                // Convert checkpoint (PatternRangePath) to prefix path (PatternPrefixPath) for cursor
                let prefix_path =
                    self.cursor.path.clone().into_rooted_role_path();
                let cursor = PathCursor {
                    path: prefix_path.clone(),
                    atom_position: self.cursor.atom_position,
                    _state: PhantomData,
                };
                debug!(
                    child_cursor=%next.child_state,
                    "Created child_cursor from parent_state"
                );
                // Get the token that index_cursor points to
                let index_token = next
                    .child_state
                    .leaf_token(trav)
                    .expect("child_state should point to a valid token");
                let cursor_position = self.cursor.atom_position;

                Ok(CompareRootState {
                    token: CompareState {
                        child_cursor: ChildCursor {
                            child_state: next.child_state,
                            _state: PhantomData,
                        },
                        cursor,
                        checkpoint: self.cursor,
                        mode: GraphMajor,
                        target: DownKey::new(
                            index_token,
                            DownPosition(cursor_position),
                        ),
                    },
                    root_parent: next.root_parent,
                })
            },
            Err(parent_state) => Err(Self {
                parent_state,
                cursor: self.cursor,
            }),
        }
    }
}
