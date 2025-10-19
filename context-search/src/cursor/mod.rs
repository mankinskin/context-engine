use context_trace::*;
pub(crate) mod path;
pub(crate) mod position;

pub(crate) trait ToCursor: FoldablePath {
    fn to_cursor<G: HasGraph>(
        self,
        trav: &G,
    ) -> PathCursor<Self>;
}
impl<P: FoldablePath> ToCursor for P {
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

pub(crate) type PatternRangeCursor = PathCursor<PatternRangePath>;

pub(crate) type PatternCursor = PathCursor<PatternPostfixPath>;

impl From<PatternRangeCursor> for PatternCursor {
    fn from(value: PathCursor<PatternRangePath>) -> Self {
        Self {
            path: value.path.into(),
            atom_position: value.atom_position,
        }
    }
}
