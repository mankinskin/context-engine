use context_trace::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct PrefixEnd {
    pub(crate) path: IndexEndPath,
    pub(crate) target: DownKey,
    pub(crate) end_pos: AtomPosition,
}
impl From<&PrefixEnd> for PrefixCommand {
    fn from(value: &PrefixEnd) -> Self {
        PrefixCommand {
            add_edges: true,
            path: value.path.clone(),
            end_pos: value.end_pos,
        }
    }
}
impl Traceable for &PrefixEnd {
    fn trace<G: HasGraph>(
        self,
        ctx: &mut SearchContext<G>,
    ) {
        PrefixCommand::from(self).trace(ctx)
    }
}
