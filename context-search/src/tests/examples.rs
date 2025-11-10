//! Integration tests demonstrating the Search API usage patterns
//! from SEARCH_API_EXAMPLES.md

#[cfg(test)]
use {
    crate::{
        search::Find,
        state::end::PathEnum,
    },
    context_trace::{
        logging::format_utils::pretty,
        path::accessors::root::GraphRootPattern,
        *,
    },
    pretty_assertions::assert_eq,
    tracing::{
        debug,
        info,
        trace,
    },
};

#[test]
fn example_basic_sequence_search() {
    // Initialize tracing for this test - log file will be cleaned up on success
    let _tracing = init_test_tracing!();

    info!("Starting basic sequence search test");

    // Create graph and insert atoms
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    debug!("Created empty hypergraph");

    let a = graph.insert_atom(Atom::Element('a'));
    let b = graph.insert_atom(Atom::Element('b'));
    let c = graph.insert_atom(Atom::Element('c'));
    debug!(
        a = %pretty(&a),
        b = %pretty(&b),
        c = %pretty(&c),
        "Inserted atoms"
    );

    // Create a pattern [a, b, c]
    let abc = graph.insert_pattern([a, b, c]);
    info!(abc = %pretty(&abc), "Created pattern [a, b, c]");

    // Search for [b, c] in the graph
    let graph = HypergraphRef::from(graph);
    let query = [b, c];
    debug!(query = %pretty(&query), "Searching for [b, c]");
    let result = graph.find_ancestor(&query[..]);

    // Verify we found the query pattern
    assert!(result.is_ok(), "Search should succeed");
    let response = result.unwrap();
    trace!(response = %pretty(&response), "Got response");

    // Since [b, c] was never inserted as a complete pattern, we expect a Postfix
    // (it matches indices 1-2 of the parent pattern [a, b, c])
    // But the query should be fully matched (QueryEnd)
    assert_eq!(
        response.end.reason,
        crate::state::end::EndReason::QueryEnd,
        "Query should be fully matched"
    );
    assert!(
        matches!(response.end.path, crate::state::end::PathEnum::Postfix(_)),
        "Path should be Postfix since [b, c] doesn't start at the beginning of [a, b, c]"
    );

    // Verify the path points to abc as the parent
    let parent = response.end.path.root_parent();
    assert_eq!(parent, abc);
    info!("Test completed successfully");
}

#[test]
fn example_single_atom_error() {
    // Create graph and insert single atom
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    let a = graph.insert_atom(Atom::Element('a'));

    // Try to search for just the atom 'a'
    let graph = HypergraphRef::from(graph);
    let query = [a];
    let result = graph.find_ancestor(&query[..]);

    // Should return error - single atom has no parent pattern
    assert!(result.is_err());
}
#[test]
fn example_helper_methods() {
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    let a = graph.insert_atom(Atom::Element('a'));
    let b = graph.insert_atom(Atom::Element('b'));
    let ab = graph.insert_pattern([a, b]);

    let graph = HypergraphRef::from(graph);
    let query = [a, b];
    let response = graph.find_ancestor(&query[..]).unwrap();

    // Check if complete
    assert!(response.is_complete());

    // Get as Option
    let opt_path = response.as_complete();
    assert!(opt_path.is_some());
    assert_eq!(opt_path.unwrap().root_pattern_location().parent, ab);
}

#[test]
fn example_token_pattern_element() {
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    let h = graph.insert_atom(Atom::Element('h'));
    let e = graph.insert_atom(Atom::Element('e'));
    let l = graph.insert_atom(Atom::Element('l'));
    let o = graph.insert_atom(Atom::Element('o'));

    let hel = graph.insert_pattern([h, e, l]);
    let llo = graph.insert_pattern([l, l, o]);
    let _hello = graph.insert_pattern([hel, llo]);

    let graph = HypergraphRef::from(graph);
    let query = [e, l];
    let response = graph.find_ancestor(&query[..]).unwrap();

    assert!(response.is_complete());
    let path = response.unwrap_complete();
    assert_eq!(path.root_pattern_location().parent, hel);
}

#[test]
fn example_hierarchical_parent_search() {
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    let a = graph.insert_atom(Atom::Element('a'));
    let b = graph.insert_atom(Atom::Element('b'));
    let c = graph.insert_atom(Atom::Element('c'));
    let d = graph.insert_atom(Atom::Element('d'));

    let ab = graph.insert_pattern([a, b]);
    let _bc = graph.insert_pattern([b, c]);
    let cd = graph.insert_pattern([c, d]);
    let abcd = graph.insert_pattern([ab, cd]);

    let graph = HypergraphRef::from(graph);

    // Find parent of [a, b]
    let query1 = [a, b];
    let response1 = graph.find_parent(&query1).unwrap();
    assert!(response1.is_complete());
    assert_eq!(
        response1.unwrap_complete().root_pattern_location().parent,
        ab
    );

    // Find parent of [ab, cd]
    let query2 = [ab, cd];
    let response2 = graph.find_parent(&query2).unwrap();
    assert!(response2.is_complete());
    assert_eq!(
        response2.unwrap_complete().root_pattern_location().parent,
        abcd
    );
}

#[test]
fn example_hierarchical_ancestor_search() {
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    let a = graph.insert_atom(Atom::Element('a'));
    let b = graph.insert_atom(Atom::Element('b'));
    let c = graph.insert_atom(Atom::Element('c'));
    let d = graph.insert_atom(Atom::Element('d'));

    let ab = graph.insert_pattern([a, b]);
    let _bc = graph.insert_pattern([b, c]);
    let cd = graph.insert_pattern([c, d]);
    let abcd = graph.insert_pattern([ab, cd]);

    let graph = HypergraphRef::from(graph);

    // Find ancestor with [b, c, d] - should find abcd
    let query = [b, c, d];
    let response = graph.find_ancestor(&query).unwrap();

    // Result is incomplete (Postfix) because we matched starting from 'b'
    assert!(!response.is_complete());

    // The path should point to abcd pattern
    match &response.end.path {
        PathEnum::Postfix(postfix) => {
            assert_eq!(postfix.path.root_pattern_location().parent, abcd);
        },
        _ => panic!("Expected Postfix variant"),
    }
}

#[test]
fn example_incomplete_postfix() {
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    let a = graph.insert_atom(Atom::Element('a'));
    let b = graph.insert_atom(Atom::Element('b'));
    let c = graph.insert_atom(Atom::Element('c'));
    let abc = graph.insert_pattern([a, b, c]);

    let graph = HypergraphRef::from(graph);

    // Search for [b, c] using find_ancestor
    let query = [b, c];
    let response = graph.find_ancestor(&query).unwrap();

    // This is a Postfix match - starts at position 1 in [a,b,c]
    assert!(!response.is_complete());
    match &response.end.path {
        PathEnum::Postfix(postfix) => {
            assert_eq!(postfix.root_pos, 1.into());
            assert_eq!(postfix.path.root_pattern_location().parent, abc);
        },
        _ => panic!("Expected Postfix"),
    }
}

#[test]
fn example_incomplete_prefix() {
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    let a = graph.insert_atom(Atom::Element('a'));
    let b = graph.insert_atom(Atom::Element('b'));
    let c = graph.insert_atom(Atom::Element('c'));
    let abc = graph.insert_pattern([a, b, c]);

    let graph = HypergraphRef::from(graph);

    // Search for [a, b] using find_ancestor
    let query = [a, b];
    let response = graph.find_ancestor(&query).unwrap();

    // This is a Prefix match - matches beginning but not all of [a,b,c]
    assert!(!response.is_complete());
    match &response.end.path {
        PathEnum::Prefix(prefix) => {
            assert_eq!(prefix.path.root_pattern_location().parent, abc);
        },
        _ => panic!("Expected Prefix"),
    }
}

#[test]
fn example_pattern_location_access() {
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    let a = graph.insert_atom(Atom::Element('a'));
    let b = graph.insert_atom(Atom::Element('b'));
    let c = graph.insert_atom(Atom::Element('c'));

    let ab = graph.insert_pattern([a, b]);
    let abc = graph.insert_pattern([ab, c]);

    let graph = HypergraphRef::from(graph);

    // Search for [ab, c]
    let query = [ab, c];
    let response = graph.find_ancestor(&query[..]).unwrap();
    let path = response.unwrap_complete();

    // Access the pattern location
    let location = path.root_pattern_location();
    assert_eq!(location.parent, abc);
}

#[test]
fn example_multiple_alternatives() {
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    let a = graph.insert_atom(Atom::Element('a'));
    let b = graph.insert_atom(Atom::Element('b'));
    let c = graph.insert_atom(Atom::Element('c'));

    // Create two different patterns with same subsequence
    let ab1 = graph.insert_pattern([a, b]);
    let abc1 = graph.insert_pattern([ab1, c]);

    let _ab2 = graph.insert_pattern([a, b]);
    let abc2 = graph.insert_pattern([a, b, c]);

    let graph = HypergraphRef::from(graph);

    // Search finds one of the alternatives
    let query = [a, b, c];
    let response = graph.find_ancestor(&query[..]).unwrap();

    assert!(response.is_complete());
    let path = response.unwrap_complete();

    // Should find one of the two abc patterns
    let found_parent = path.root_pattern_location().parent;
    assert!(found_parent == abc1 || found_parent == abc2);
}

#[test]
fn example_pattern_width() {
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    let a = graph.insert_atom(Atom::Element('a'));
    let b = graph.insert_atom(Atom::Element('b'));

    // Create pattern with multiple ways to compose [a, b]
    let _ab1 = graph.insert_pattern([a, b]);
    let _ab2 = graph.insert_pattern([a, b]);

    let graph = HypergraphRef::from(graph);

    // Search can find any of the alternatives
    let query = [a, b];
    let response = graph.find_ancestor(&query[..]).unwrap();
    assert!(response.is_complete());
}

#[test]
fn example_expect_complete() {
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    let a = graph.insert_atom(Atom::Element('a'));
    let b = graph.insert_atom(Atom::Element('b'));
    let ab = graph.insert_pattern([a, b]);

    let graph = HypergraphRef::from(graph);
    let query = [a, b];
    let response = graph.find_ancestor(&query[..]).unwrap();

    // Using expect_complete with custom message
    let path = response.expect_complete("Expected complete match for [a, b]");
    assert_eq!(path.root_pattern_location().parent, ab);
}
