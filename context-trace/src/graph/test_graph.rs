//! Test-only graph registry for token string representations
//!
//! This module provides a way to register a hypergraph for use in tests,
//! allowing tokens to dynamically compute their string representation from
//! the graph when formatting for logs.
//!
//! # Usage
//!
//! ```rust
//! use context_trace::{Hypergraph, graph::test_graph};
//! # use context_trace::graph::vertex::atom::Atom;
//!
//! # fn main() {
//! let mut graph = Hypergraph::default();
//! let a = graph.insert_atom(Atom::Element('a'));
//! let b = graph.insert_atom(Atom::Element('b'));
//! let c = graph.insert_atom(Atom::Element('c'));
//! let abc = graph.insert_pattern(vec![a, b, c]);
//!
//! // Register the graph for string representations
//! test_graph::register_test_graph(&graph);
//!
//! // Now tokens will show their string representation
//! println!("{}", a);    // Prints: T0w1("a")
//! println!("{}", b);    // Prints: T1w1("b")
//! println!("{}", abc);  // Prints: T3w3("abc")
//!
//! // Clean up when done
//! test_graph::clear_test_graph();
//! # }
//! ```

use crate::{
    Hypergraph,
    graph::{
        getters::vertex::VertexSet,
        kind::GraphKind,
        vertex::VertexIndex,
    },
};
use once_cell::sync::Lazy;
use std::sync::RwLock;

/// Type-erased graph accessor for getting string representations
trait GraphStringGetter: Send + Sync {
    fn get_token_string(
        &self,
        index: usize,
    ) -> Option<String>;
}

impl<G: GraphKind + Send + Sync> GraphStringGetter for Hypergraph<G>
where
    G::Atom: std::fmt::Display,
{
    fn get_token_string(
        &self,
        index: usize,
    ) -> Option<String> {
        self.get_vertex(VertexIndex::from(index)).ok().map(|_| {
            <Hypergraph<G>>::index_string(self, VertexIndex::from(index))
        })
    }
}

/// Global test graph registry
static TEST_GRAPH: Lazy<RwLock<Option<Box<dyn GraphStringGetter>>>> =
    Lazy::new(|| RwLock::new(None));

/// Register a graph for use in tests
///
/// This allows tokens to look up their string representation when formatting.
/// The graph is stored as a type-erased reference, so it works with any GraphKind.
///
/// # Example
/// ```ignore
/// let mut graph = Hypergraph::default();
/// insert_atoms!(graph, {a, b, c});
///
/// register_test_graph(&graph);
///
/// // Now tokens will show their string representation in logs
/// println!("{}", a); // T1w1("a")
///
/// clear_test_graph();
/// ```
pub fn register_test_graph<G: GraphKind + Send + Sync + 'static>(
    graph: &Hypergraph<G>
) where
    G::Atom: std::fmt::Display,
{
    // We need to clone the graph to store it safely
    // In tests this is acceptable overhead
    let graph_clone = graph.clone();
    *TEST_GRAPH.write().unwrap() = Some(Box::new(graph_clone));
}

/// Access the registered test graph to get string representation
pub fn get_token_string_from_test_graph(index: usize) -> Option<String> {
    TEST_GRAPH
        .read()
        .unwrap()
        .as_ref()
        .and_then(|graph| graph.get_token_string(index))
}

/// Clear the registered test graph
pub fn clear_test_graph() {
    *TEST_GRAPH.write().unwrap() = None;
}
