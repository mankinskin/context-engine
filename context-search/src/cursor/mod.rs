use context_trace::*;
use std::marker::PhantomData;

pub(crate) mod path;
pub(crate) mod position;
pub trait CursorPath: GraphRoot {}
impl<T: GraphRoot> CursorPath for T {}

// State marker types for PathCursor
mod sealed {
    pub trait Sealed {}
}

/// Trait for cursor state markers
pub trait CursorState: sealed::Sealed {}

/// Matched state: cursor is at a confirmed matching position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Matched;
impl sealed::Sealed for Matched {}
impl CursorState for Matched {}

/// Candidate state: cursor has advanced to a position that needs comparison
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Candidate;
impl sealed::Sealed for Candidate {}
impl CursorState for Candidate {}

/// Exhausted state: cursor has reached the end of the pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Exhausted;
impl sealed::Sealed for Exhausted {}
impl CursorState for Exhausted {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathCursor<P, State = Matched> {
    pub(crate) path: P,
    pub(crate) atom_position: AtomPosition,
    pub(crate) _state: PhantomData<State>,
}

pub(crate) type PatternCursor = PathCursor<PatternRangePath>;
pub(crate) type IndexCursor = PathCursor<IndexRangePath>;

pub(crate) type PatternPrefixCursor = PathCursor<PatternPrefixPath>;

impl From<PatternPrefixCursor> for PatternCursor {
    fn from(value: PatternPrefixCursor) -> Self {
        let value: PatternCursor = value.into();
        Self {
            path: value.path.into(),
            atom_position: value.atom_position,
            _state: PhantomData,
        }
    }
}
impl From<PatternCursor> for PatternPrefixCursor {
    fn from(value: PatternCursor) -> Self {
        Self {
            path: value.path.into(),
            atom_position: value.atom_position,
            _state: PhantomData,
        }
    }
}
impl<P> From<P> for PathCursor<P> {
    fn from(value: P) -> Self {
        Self {
            path: value,
            atom_position: 0.into(),
            _state: PhantomData,
        }
    }
}

// State transition methods
impl<P> PathCursor<P, Matched> {
    /// Convert a Matched cursor to a Candidate by creating a copy
    /// This preserves the matched position for potential revert
    pub(crate) fn as_candidate(&self) -> PathCursor<P, Candidate>
    where
        P: Clone,
    {
        PathCursor {
            path: self.path.clone(),
            atom_position: self.atom_position,
            _state: PhantomData,
        }
    }
}

impl<P> PathCursor<P, Candidate> {
    /// Confirm a candidate cursor as matched
    pub(crate) fn confirm_match(self) -> PathCursor<P, Matched> {
        PathCursor {
            path: self.path,
            atom_position: self.atom_position,
            _state: PhantomData,
        }
    }

    /// Revert a candidate cursor back to the matched state
    /// by replacing it with the provided matched cursor
    pub(crate) fn revert(
        self,
        matched: PathCursor<P, Matched>,
    ) -> PathCursor<P, Matched> {
        matched
    }
}

impl<P> PathCursor<P, Exhausted> {
    /// Convert an exhausted cursor to matched (for end states)
    pub(crate) fn as_matched(self) -> PathCursor<P, Matched> {
        PathCursor {
            path: self.path,
            atom_position: self.atom_position,
            _state: PhantomData,
        }
    }
}
