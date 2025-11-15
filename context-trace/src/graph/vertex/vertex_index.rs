use crate::graph::vertex::has_vertex_index::HasVertexIndex;
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    fmt,
    ops::Deref,
};

/// Newtype for vertex indices that provides Display with string representation
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct VertexIndex(pub usize);

impl From<usize> for VertexIndex {
    fn from(index: usize) -> Self {
        Self(index)
    }
}

impl From<VertexIndex> for usize {
    fn from(index: VertexIndex) -> Self {
        index.0
    }
}

impl Deref for VertexIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for VertexIndex {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        #[cfg(any(test, feature = "test-api"))]
        {
            use crate::graph::test_graph;
            if let Some(s) =
                test_graph::get_token_string_from_test_graph(self.0)
            {
                return write!(f, "V{}:\"{}\"", self.0, s);
            }
        }
        write!(f, "V{}", self.0)
    }
}
