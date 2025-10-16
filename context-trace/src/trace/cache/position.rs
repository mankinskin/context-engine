use std::num::NonZeroUsize;

use crate::{
    graph::vertex::location::SubLocation,
    *,
};

pub(crate) type Offset = NonZeroUsize;

/// optional offset inside of pattern sub location
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SubSplitLocation {
    pub(crate) location: SubLocation,
    pub(crate) inner_offset: Option<Offset>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PositionCache {
    pub(crate) top: HashSet<DirectedKey>,
    pub(crate) bottom: HashMap<DirectedKey, SubLocation>,
}
pub(crate) enum AddChildLocation {
    Target(ChildLocation),
    Prev(ChildLocation),
}
impl PositionCache {
    pub(crate) fn new(
        cache: &mut TraceCache,
        state: EditKind,
        add_edges: bool,
    ) -> Self {
        // create all bottom edges (created upwards or downwards)
        let mut bottom = HashMap::default();
        match (add_edges, state) {
            (false, _) => {},
            (_, EditKind::Parent(edit)) => {
                // created by upwards traversal
                bottom
                    .insert(edit.prev.into(), edit.location.to_sub_location());
            },
            (_, EditKind::Child(edit)) => {
                // created by downwards traversal
                let prev = cache.force_mut(&(edit.prev.into()));
                prev.bottom.insert(
                    edit.target.into(),
                    edit.location.to_sub_location(),
                );
            },
        }
        Self {
            bottom,
            top: HashSet::default(),
        }
    }
    pub(crate) fn num_parents(&self) -> usize {
        self.top.len()
    }
    pub(crate) fn num_bu_edges(&self) -> usize {
        self.bottom.len()
    }
}
