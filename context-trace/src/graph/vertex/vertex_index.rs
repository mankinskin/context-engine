use std::fmt;
use crate::graph::vertex::{has_vertex_index::HasVertexIndex, VertexIndex};

/// Newtype wrapper for VertexIndex that provides Display with string representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DisplayableVertexIndex(pub VertexIndex);

impl From<VertexIndex> for DisplayableVertexIndex {
    fn from(index: VertexIndex) -> Self {
        Self(index)
    }
}

impl From<DisplayableVertexIndex> for VertexIndex {
    fn from(wrapper: DisplayableVertexIndex) -> Self {
        wrapper.0
    }
}

impl HasVertexIndex for DisplayableVertexIndex {
    fn vertex_index(&self) -> VertexIndex {
        self.0
    }
}

impl fmt::Display for DisplayableVertexIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(any(test, feature = "test-api"))]
        {
            use crate::graph::test_graph;
            if let Some(s) = test_graph::get_token_string_from_test_graph(self.0) {
                return write!(f, "V{}:\"{}\"", self.0, s);
            }
        }
        write!(f, "V{}", self.0)
    }
}
