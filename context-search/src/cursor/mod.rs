use context_trace::*;

use crate::fold::StartFoldPath;
pub(crate) mod path;
pub(crate) mod position;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathCursor<P> {
    pub(crate) path: P,
    pub(crate) atom_position: AtomPosition,
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
        }
    }
}
impl From<PatternCursor> for PatternPrefixCursor {
    fn from(value: PatternCursor) -> Self {
        Self {
            path: value.path.into(),
            atom_position: value.atom_position,
        }
    }
}
impl<P> From<P> for PathCursor<P> {
    fn from(value: P) -> Self {
        Self {
            path: value,
            atom_position: 0.into(),
        }
    }
}
