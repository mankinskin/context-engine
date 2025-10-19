pub(crate) mod ancestor;
pub(crate) mod consecutive;
pub(crate) mod parent;

#[cfg(test)]
use {
    crate::search::Searchable,
    crate::{
        cursor::PatternCursor,
        fold::result::FinishedKind,
        state::end::{
            range::RangeEnd,
            EndKind,
            EndReason,
            EndState,
        },
    },
    context_trace::tests::env::Env1,

    context_trace::*,
    itertools::*,
    pretty_assertions::assert_eq,

    std::iter::FromIterator,
};

#[test]
fn find_sequence() {
    let Env1 {
        graph,
        abc,
        ababababcdefghi,
        a,
        ..
    } = &*Env1::get_expected();
    assert_eq!(
        graph.find_sequence("a".chars()),
        Err(ErrorReason::SingleIndex(Box::new(IndexWithPath {
            index: *a,
            path: vec![*a].into(),
        }))),
    );
    let query = graph.graph().expect_atom_children("abc".chars());
    let abc_found = graph.find_ancestor(&query);
    assert_eq!(
        abc_found.map(|r| r.kind),
        Ok(FinishedKind::Complete(*abc)),
        "abc"
    );
    let query = graph
        .graph()
        .expect_atom_children("ababababcdefghi".chars());
    let ababababcdefghi_found = graph.find_ancestor(&query);
    assert_eq!(
        ababababcdefghi_found.map(|r| r.kind),
        Ok(FinishedKind::Complete(*ababababcdefghi)),
        "ababababcdefghi"
    );
}
#[test]
fn find_pattern1() {
    let mut base_graph =
        context_trace::graph::Hypergraph::<BaseGraphKind>::default();
    insert_atoms!(base_graph, {a, b, x, y, z});
    // index 6
    insert_patterns!(base_graph,
        (yz, y_z_id) => [y, z],
        (xab, x_a_b_id) => [x, a, b],
    );
    insert_patterns!(base_graph,
        _xyz => [x, yz],
        _xabz => [xab, z],
    );
    insert_patterns!(base_graph,
        (xabyz, xab_yz_id) => [xab, yz]
    );

    let graph_ref = HypergraphRef::from(base_graph);

    let query = vec![a, b, y, x];
    let aby_found = graph_ref
        .find_ancestor(query.clone())
        .expect("Search failed");

    let expected_cache = build_trace_cache!(
        xab => (
            BU { 1 => a -> (x_a_b_id, 1) },
            TD {},
        ),
        xabyz => (
            BU { 2 => xab -> (xab_yz_id, 0) },
            TD { 2 => yz -> (xab_yz_id, 1) },
        ),
        yz => (
            BU {},
            TD { 2 => y -> (y_z_id, 0) },
        ),
    );

    assert_eq!(
        aby_found.cache.entries[&xab.index], expected_cache.entries[&xab.index],
        "xab"
    );
    assert_eq!(
        aby_found.cache.entries[&xabyz.index],
        expected_cache.entries[&xabyz.index],
        "xabyz"
    );
    assert_eq!(
        aby_found.cache.entries[&yz.index], expected_cache.entries[&yz.index],
        "yz"
    );
    assert_eq!(aby_found.cache.entries.len(), 5);
    assert_eq!(
        aby_found.kind,
        FinishedKind::Incomplete(Box::new(EndState {
            reason: EndReason::Mismatch,
            kind: EndKind::Range(RangeEnd {
                root_pos: 2.into(),
                target: DownKey::new(y, 3.into()),
                path: RootedRangePath::new(
                    PatternLocation::new(xabyz, xab_yz_id),
                    RolePath::new(
                        0,
                        vec![ChildLocation::new(xab, x_a_b_id, 1)],
                    ),
                    RolePath::new(1, vec![ChildLocation::new(yz, y_z_id, 0)],),
                ),
            }),
            cursor: PatternCursor {
                path: RootedRolePath::new(
                    query.clone(),
                    RolePath::new(2, vec![]),
                ),
                atom_position: 3.into(),
            },
        }))
    );
}
