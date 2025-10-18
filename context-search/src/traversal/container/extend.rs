use crate::traversal::state::traversal::TraversalState;

pub(crate) trait ExtendStates {
    fn extend<
        It: DoubleEndedIterator + Iterator<Item = (usize, TraversalState)>,
        T: IntoIterator<Item = (usize, TraversalState), IntoIter = It>,
    >(
        &mut self,
        next: T,
    );
}
