use context_trace::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RangeEnd {
    pub(crate) path: IndexRangePath,
    pub(crate) target: DownKey,
    pub(crate) root_pos: AtomPosition,
    pub(crate) end_pos: AtomPosition,
}
impl LeafKey for RangeEnd {
    fn leaf_location(&self) -> ChildLocation {
        self.path.leaf_location()
    }
}

impl Traceable for &RangeEnd {
    fn trace<G: HasGraph>(
        self,
        ctx: &mut SearchContext<G>,
    ) {
        RangeCommand::from(self).trace(ctx)
    }
}

impl From<&RangeEnd> for RangeCommand {
    fn from(value: &RangeEnd) -> Self {
        RangeCommand {
            add_edges: true,
            path: value.path.clone(),
            root_pos: value.root_pos.into(),
            end_pos: value.end_pos,
        }
    }
}
