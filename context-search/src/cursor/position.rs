use crate::{
    cursor::PathCursor,
    state::start::StartFoldPath,
};
use context_trace::*;

impl_cursor_pos! {
    <R: StartFoldPath> CursorPosition for PathCursor<R>, self => self.atom_position
}
impl<D: Direction, P> MoveKey<D> for PathCursor<P>
where
    AtomPosition: MoveKey<D>,
{
    fn move_key(
        &mut self,
        delta: usize,
    ) {
        self.atom_position.move_key(delta)
    }
}

impl<R: PathRole, P: RootChildIndex<R>> RootChildIndex<R> for PathCursor<P> {
    fn root_child_index(&self) -> usize {
        RootChildIndex::<R>::root_child_index(&self.path)
    }
}
