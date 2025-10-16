pub mod ancestor;
pub mod consecutive;
pub mod parent;

#[cfg(test)]
use {
    crate::search::Searchable,
    crate::{
        fold::result::FinishedKind,
        traversal::state::{
            cursor::PatternCursor,
            end::{
                range::RangeEnd,
                EndKind,
                EndReason,
                EndState,
            },
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
    let query = graph.graph().expect_token_children("abc".chars());
    let abc_found = graph.find_ancestor(&query);
    assert_eq!(
        abc_found.map(|r| r.kind),
        Ok(FinishedKind::Complete(*abc)),
        "abc"
    );
    let query = graph
        .graph()
        .expect_token_children("ababababcdefghi".chars());
    let ababababcdefghi_found = graph.find_ancestor(&query);
    assert_eq!(
        ababababcdefghi_found.map(|r| r.kind),
        Ok(FinishedKind::Complete(*ababababcdefghi)),
        "ababababcdefghi"
    );
}
#[test]
fn find_pattern1() {
    let mut graph =
        context_trace::graph::Hypergraph::<BaseGraphKind>::default();
    let (a, b, _w, x, y, z) = graph
        .insert_tokens([
            Token::Element('a'),
            Token::Element('b'),
            Token::Element('w'),
            Token::Element('x'),
            Token::Element('y'),
            Token::Element('z'),
        ])
        .into_iter()
        .next_tuple()
        .unwrap();
    // index 6
    let (yz, y_z_id) = graph.insert_pattern_with_id(vec![y, z]);
    let (xab, x_a_b_id) = graph.insert_pattern_with_id(vec![x, a, b]);
    let _xyz = graph.insert_pattern(vec![x, yz]);
    let _xabz = graph.insert_pattern(vec![xab, z]);
    let (xabyz, xab_yz_id) = graph.insert_pattern_with_id(vec![xab, yz]);

    let graph_ref = HypergraphRef::from(graph);

    let query = vec![a, b, y, x];
    let aby_found = graph_ref
        .find_ancestor(query.clone())
        .expect("Search failed");
    //info!("{:#?}", aby);

    assert_eq!(
        aby_found.cache.entries[&xab.index],
        VertexCache {
            index: xab,
            bottom_up: FromIterator::from_iter([(
                1.into(),
                PositionCache::with_bottom(HashMap::from_iter([(
                    DirectedKey::up(a, 1),
                    SubLocation::new(x_a_b_id.unwrap(), 1)
                )]))
            )]),
            top_down: FromIterator::from_iter([]),
        }
    );
    assert_eq!(
        aby_found.cache.entries[&xabyz.index],
        VertexCache {
            index: xabyz,
            bottom_up: FromIterator::from_iter([(
                2.into(),
                PositionCache::with_bottom(HashMap::from_iter([(
                    DirectedKey::up(xab, 1),
                    SubLocation::new(xab_yz_id.unwrap(), 0)
                )]))
            )]),
            top_down: FromIterator::from_iter([(
                2.into(),
                PositionCache::with_bottom(HashMap::from_iter([(
                    DirectedKey::down(yz, 2),
                    SubLocation::new(xab_yz_id.unwrap(), 1)
                )]))
            )]),
        }
    );
    assert_eq!(
        aby_found.cache.entries[&yz.index],
        VertexCache {
            index: yz,
            bottom_up: FromIterator::from_iter([]),
            top_down: FromIterator::from_iter([(
                2.into(),
                PositionCache::with_bottom(HashMap::from_iter([(
                    DirectedKey::down(y, 2),
                    SubLocation::new(y_z_id.unwrap(), 0)
                )]))
            )]),
        }
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
                    PatternLocation::new(xabyz, xab_yz_id.unwrap()),
                    RolePath::new(
                        0,
                        vec![ChildLocation::new(xab, x_a_b_id.unwrap(), 1)],
                    ),
                    RolePath::new(
                        1,
                        vec![ChildLocation::new(yz, y_z_id.unwrap(), 0)],
                    ),
                ),
            }),
            cursor: PatternCursor {
                path: RootedRolePath::new(
                    query.clone(),
                    RolePath::new(2, vec![]),
                ),
                relative_pos: 3.into(),
            },
        }))
    );
}
