use crate::{
    cursor::PatternCursor,
    state::result::BaseResponse,
    CompleteState,
    IncompleteState,
};
#[cfg(test)]
use {
    crate::{
        search::Searchable,
        state::end::{
            postfix::PostfixEnd,
            EndKind,
            EndReason,
            EndState,
        },
        state::result::Response,
    },
    context_trace::*,
    itertools::*,
    pretty_assertions::{
        assert_eq,
        assert_matches,
    },
};

#[test]
fn find_ancestor1() {
    let Env1 {
        graph,
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
        i,
        ab,
        bc,
        abc,
        abcd,
        ababababcdefghi,
        ..
    } = &*Env1::get_expected();
    let a_bc_pattern = vec![Token::new(a, 1), Token::new(bc, 2)];
    let ab_c_pattern = vec![Token::new(ab, 2), Token::new(c, 1)];
    let a_bc_d_pattern =
        vec![Token::new(a, 1), Token::new(bc, 2), Token::new(d, 1)];
    let b_c_pattern = vec![Token::new(b, 1), Token::new(c, 1)];
    let bc_pattern = vec![Token::new(bc, 2)];
    let a_b_c_pattern =
        vec![Token::new(a, 1), Token::new(b, 1), Token::new(c, 1)];

    let query = bc_pattern;
    assert_eq!(
        graph.find_ancestor(&query),
        Err(ErrorReason::SingleIndex(Box::new(IndexWithPath {
            index: *bc,
            path: query.into()
        }))),
        "bc"
    );

    let query = b_c_pattern;
    assert_matches!(
        graph.find_ancestor(&query),
        Ok(Response::Complete(CompleteState {
            root: x,
            ..
        })) if x.index == *bc,
        "b_c"
    );

    println!("################## A_BC");
    let query = a_bc_pattern;
    assert_matches!(
        graph.find_ancestor(&query),
        Ok(Response::Complete(CompleteState {
            root: x,
            ..
        })) if x.index == *abc,
        "a_bc"
    );

    let query = ab_c_pattern;
    assert_matches!(
        graph.find_ancestor(&query),
        Ok(Response::Complete(CompleteState {
            root: x,
            ..
        })) if x.index == *abc,
        "ab_c"
    );

    let query = a_bc_d_pattern;
    assert_matches!(
        graph.find_ancestor(&query),
        Ok(Response::Complete(CompleteState {
            root: x,
            ..
        })) if x.index == *abcd,
        "a_bc_d"
    );

    let query = a_b_c_pattern.clone();
    assert_matches!(
        graph.find_ancestor(&query),
        Ok(Response::Complete(CompleteState {
            root: x,
            ..
        })) if x.index == *abc,
        "a_b_c"
    );

    let query: Vec<_> = [a, b, a, b, a, b, a, b, c, d, e, f, g, h, i]
        .into_iter()
        .cloned()
        .collect();
    assert_matches!(
        graph.find_ancestor(&query),
        Ok(Response::Complete(CompleteState {
            root: x,
            ..
        })) if x.index == *ababababcdefghi,
        "a_b_a_b_a_b_a_b_c_d_e_f_g_h_i"
    );

    let query = [&a_b_c_pattern[..], &[Token::new(c, 1)]].concat();
    assert_matches!(
        graph.find_ancestor(&query),
        Ok(Response::Complete(CompleteState {
            root: x,
            ..
        })) if x.index == *abc,
        "a_b_c_c"
    );
}

#[test]
fn find_ancestor2() {
    use context_trace::*;

    let mut graph = Hypergraph::<BaseGraphKind>::default();
    insert_atoms!(graph, {a, b, x, y, z});
    insert_patterns!(graph,
        ab => [a, b],
        by => [b, y],
        yz => [y, z],
        xa => [x, a],
    );
    insert_patterns!(graph,
        xab => [[x, ab],[xa, b]],
    );
    insert_patterns!(graph,
        (xaby, xaby_ids) => [[xa, by],[xab,y]],
        (xabyz, xabyz_ids) => [[xaby, z],[xab,yz]],
    );
    let xa_by_id = xaby_ids[0];
    let xaby_z_id = xabyz_ids[0];
    //assert_eq!(xaby_z_id, 8);
    let graph = HypergraphRef::from(graph);
    let query = vec![by, z];
    let byz_found = graph.find_ancestor(&query).unwrap();

    assert_eq!(
        byz_found,
        Response::Incomplete(IncompleteState {
            end_state: EndState {
                reason: EndReason::QueryEnd,
                kind: EndKind::Postfix(PostfixEnd {
                    root_pos: 2.into(),
                    path: RootedRolePath::new(
                        PatternLocation::new(xabyz, xaby_z_id,),
                        RolePath::new(
                            0,
                            vec![ChildLocation::new(xaby, xa_by_id, 1,)],
                        ),
                    )
                }),
            },
            root: IndexWithPath {
                index: xabyz,
                path: RootedRangePath::new(
                    query.clone(),
                    RolePath::new_empty(1),
                    RolePath::new_empty(1),
                ),
            },
            base: BaseResponse {
                start: by,
                cursor: PatternCursor {
                    path: RootedRolePath::new(
                        query.clone(),
                        RolePath::new_empty(1),
                    ),
                    atom_position: 3.into(),
                },
                cache: TraceCache {
                    entries: HashMap::from_iter([
                        (
                            xabyz.index,
                            VertexCache {
                                index: xabyz,
                                top_down: FromIterator::from_iter([]),
                                bottom_up: FromIterator::from_iter([(
                                    2.into(), // width of by
                                    PositionCache::new(
                                        Default::default(),
                                        HashMap::from_iter([(
                                            DirectedKey::up(xaby, 2), // width of by
                                            SubLocation::new(xaby_z_id, 0),
                                        ),]),
                                    )
                                )]),
                            }
                        ),
                        (
                            xaby.index,
                            VertexCache {
                                index: xaby,
                                top_down: FromIterator::from_iter([]),
                                bottom_up: FromIterator::from_iter([(
                                    2.into(), // width of by
                                    PositionCache::new(
                                        HashSet::from_iter([]),
                                        HashMap::from_iter([(
                                            DirectedKey::up(by, 2), // width of by
                                            SubLocation::new(xa_by_id, 1)
                                        )]),
                                    )
                                )]),
                            }
                        ),
                        (
                            by.index,
                            VertexCache {
                                index: by,
                                top_down: FromIterator::from_iter([]),
                                bottom_up: FromIterator::from_iter([]),
                            }
                        ),
                    ]),
                }
            }
        })
    );
}

#[test]
fn find_ancestor3() {
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    let (a, b, _w, x, y, z) = graph
        .insert_atoms([
            Atom::Element('a'),
            Atom::Element('b'),
            Atom::Element('w'),
            Atom::Element('x'),
            Atom::Element('y'),
            Atom::Element('z'),
        ])
        .into_iter()
        .next_tuple()
        .unwrap();
    // 6
    let ab = graph.insert_pattern(vec![a, b]);
    let by = graph.insert_pattern(vec![b, y]);
    let yz = graph.insert_pattern(vec![y, z]);
    let xa = graph.insert_pattern(vec![x, a]);
    // 10
    let (xab, xab_ids) =
        graph.insert_patterns_with_ids([vec![x, ab], vec![xa, b]]);
    let x_ab_id = xab_ids[0];
    //assert_eq!(x_ab_id, 4);
    // 11
    let (xaby, xaby_ids) =
        graph.insert_patterns_with_ids([vec![xab, y], vec![xa, by]]);
    let xab_y_id = xaby_ids[0];
    //assert_eq!(xab_y_id, 6);
    // 12
    let _xabyz = graph.insert_patterns([vec![xaby, z], vec![xab, yz]]);
    let gr = HypergraphRef::from(graph);

    let query = vec![ab, y];
    let aby_found = gr.find_ancestor(&query).unwrap();
    assert_eq!(
        aby_found.clone(),
        Response::Incomplete(IncompleteState {
            base: BaseResponse {
                start: ab,
                cursor: PatternCursor {
                    path: RootedRolePath::new(
                        query.clone(),
                        RolePath::new_empty(1),
                    ),
                    atom_position: 3.into(),
                },
                cache: TraceCache {
                    entries: HashMap::from_iter([
                        (
                            xaby.index,
                            VertexCache {
                                index: xaby,
                                top_down: FromIterator::from_iter([]),
                                bottom_up: FromIterator::from_iter([(
                                    2.into(),
                                    PositionCache::new(
                                        Default::default(),
                                        HashMap::from_iter([(
                                            DirectedKey::up(xab, 2),
                                            SubLocation::new(xab_y_id, 0),
                                        ),]),
                                    )
                                )]),
                            }
                        ),
                        (
                            xab.index,
                            VertexCache {
                                index: xab,
                                top_down: FromIterator::from_iter([]),
                                bottom_up: FromIterator::from_iter([(
                                    2.into(),
                                    PositionCache::new(
                                        HashSet::from_iter([]),
                                        HashMap::from_iter([(
                                            DirectedKey::up(ab, 2),
                                            SubLocation::new(x_ab_id, 1),
                                        )]),
                                    )
                                )]),
                            }
                        ),
                        (
                            ab.index,
                            VertexCache {
                                index: ab,
                                top_down: FromIterator::from_iter([]),
                                bottom_up: FromIterator::from_iter([]),
                            }
                        ),
                    ]),
                }
            },
            root: IndexWithPath {
                index: xaby,
                path: RootedRangePath::new(
                    query.clone(),
                    RolePath::new_empty(1),
                    RolePath::new_empty(1),
                ),
            },
            end_state: EndState {
                reason: EndReason::QueryEnd,
                kind: EndKind::Postfix(PostfixEnd {
                    root_pos: 2.into(),
                    path: RootedRolePath::new(
                        PatternLocation::new(xaby, xab_y_id),
                        RolePath::new(
                            0,
                            vec![ChildLocation::new(xab, x_ab_id, 1)],
                        ),
                    )
                })
            },
        })
    );
}
