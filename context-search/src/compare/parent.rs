use crate::{
    compare::state::CompareState,
    cursor::{
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

use crate::compare::state::PathPairMode::GraphMajor;
use context_trace::{
    graph::vertex::token::Token,
    path::accessors::has_path::IntoRootedRolePath,
    trace::cache::key::directed::down::DownKey,
};
#[derive(Clone, Debug, PartialEq, Eq, Deref, DerefMut)]
pub(crate) struct CompareRootState {
    #[deref]
    #[deref_mut]
    pub(crate) token: CompareState,
    pub(crate) root_parent: ParentState,
}

#[derive(Clone, Debug, Deref, DerefMut)]
pub(crate) struct ParentCompareState {
    #[deref]
    #[deref_mut]
    pub(crate) parent_state: ParentState,
    pub(crate) cursor: PatternCursor,
}
impl IntoAdvanced for ParentCompareState {
    type Next = CompareRootState;
    fn into_advanced<G: HasGraph>(
        self,
        trav: &G,
    ) -> Result<Self::Next, Self> {
        match self.parent_state.into_advanced(trav) {
            Ok(next) => {
                // Convert checkpoint (PatternRangePath) to prefix path (PatternPrefixPath) for cursor
                let prefix_path =
                    self.cursor.path.clone().into_rooted_role_path();
                let cursor = PathCursor {
                    path: prefix_path,
                    atom_position: self.cursor.atom_position,
                    _state: PhantomData,
                };

                Ok(CompareRootState {
                    token: CompareState {
                        child_state: next.child_state,
                        cursor,
                        checkpoint: self.cursor,
                        mode: GraphMajor,
                        target: DownKey::new(Token::new(0, 0), 0.into()),
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
