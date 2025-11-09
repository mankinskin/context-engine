use tracing::Level;

use crate::{
    graph::vertex::parent::PatternIndex,
    *,
};

pub mod grammar;
#[macro_use]
pub mod graph;

pub mod env;

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_target(false)
        .init();
}

#[macro_export]
macro_rules! assert_patterns {
    ($graph:ident,
        $(
            $name:ident => [
                $([$($pat:expr),*]),*$(,)?
            ]
        ),*$(,)?
    ) => {

        let g = $graph.graph();
        $(
            let pats: HashSet<_> =
                $crate::HasVertexData::vertex(&$name, &g).child_pattern_set().into_iter().collect();
            assert_eq!(pats, hashset![$(vec![$($pat),*]),*]);
        )*
        #[allow(dropping_references)]
        drop(g);
    };
}
#[macro_export]
macro_rules! assert_not_indices {
    ($graph:ident, $($name:ident),*) => {
        $(
        assert_matches!(
            $graph
            .find_sequence(stringify!($name).chars()),
            Err(_) | Ok(Response { kind: ResponseKind::Incomplete(_), .. })
        );
        )*
    };
}

#[macro_export]
macro_rules! expect_atoms {
    ($graph:ident, {$($name:ident),*}) => {

        let g = $graph.graph();
        $(let $name = g.expect_atom_child($crate::charify::charify!($name));)*
        #[allow(dropping_references)]
        drop(g);
    };
}
#[macro_export]
macro_rules! insert_atoms {
    ($graph:ident, {$($name:ident),*}) => {
        use itertools::Itertools;
        let ($($name),*) = $crate::trace::has_graph::HasGraphMut::graph_mut(&mut $graph)
            .insert_atoms([
                $(
                    $crate::graph::vertex::atom::Atom::Element($crate::charify::charify!($name))
                ),*
            ])
            .into_iter()
            .next_tuple()
            .unwrap();
    };
}
pub fn assert_parents(
    graph: &Hypergraph,
    token: impl ToToken,
    parent: impl ToToken,
    pattern_indices: impl IntoIterator<Item = PatternIndex>,
) {
    assert_eq!(
        graph
            .expect_parents(token)
            .clone()
            .into_iter()
            .collect::<HashMap<_, _>>(),
        HashMap::from_iter([(
            parent.vertex_index(),
            Parent {
                pattern_indices: pattern_indices.into_iter().collect(),
                width: parent.width(),
            }
        )])
    );
}

#[macro_export]
macro_rules! build_trace_cache {
    (
        $(
            $entry_root:ident => (BU {
                $(
                    $bu_pos:expr $(=> $($bu_child:ident -> ($bu_pid:expr, $bu_sub:expr)),*)?
                ),* $(,)?
            },
            TD {
                $(
                    $td_pos:expr $(=> $($td_child:ident -> ($td_pid:expr, $td_sub:expr)),*)?
                ),* $(,)?
            }
            $(,)?
        )
        ),*
            $(,)?
    ) => {
        $crate::TraceCache {
            entries: HashMap::from_iter([
                $(
                    ($entry_root.vertex_index(), VertexCache {
                        index: $entry_root,
                        bottom_up: $crate::DirectedPositions::from_iter([
                            $(
                                (
                                    $bu_pos.into(),
                                    PositionCache::new(
                                        Default::default(),
                                        HashMap::from_iter([
                                            $($(
                                                (
                                                    DirectedKey {
                                                        index: $bu_child,
                                                        pos: $crate::DirectedPosition::BottomUp($bu_pos.into()),
                                                    },
                                                    SubLocation::new($bu_pid, $bu_sub),
                                                )
                                            ),*)?
                                        ]),
                                    ),
                                ),
                            )*
                        ]),
                        top_down: $crate::DirectedPositions::from_iter([
                            $(
                                (
                                    $td_pos.into(),
                                    PositionCache::new(
                                        Default::default(),
                                        HashMap::from_iter([
                                            $($(
                                                (
                                                    DirectedKey {
                                                        index: $td_child,
                                                        pos: DirectedPosition::TopDown($td_pos.into()),
                                                    },
                                                    SubLocation::new($td_pid, $td_sub),
                                                ),
                                            ),*)?
                                        ]),
                                    ),
                                ),
                            )*
                        ]),
                    }),
                )*
            ]),
        }
    };
}

#[test]
fn test_build_trace_cache1() {
    let mut graph = HypergraphRef::default();
    insert_atoms!(graph, {h, e, l, d});
    insert_patterns!(graph,
        (ld, ld_id) => [l, d],
        (heldld, heldld_id) => [h, e, ld, ld]
    );
    let cache = build_trace_cache!(
        heldld => (
            BU {},
            TD {2 => ld -> (heldld_id, 2) },
        ),
        ld => (
            BU {},
            TD { 2 => l -> (ld_id, 0) },
        ),
        h => (
            BU {},
            TD {},
        ),
        l => (
            BU {},
            TD { 2 },
        ),
    );
    assert_eq!(
        cache,
        TraceCache {
            entries: HashMap::from_iter([
                (
                    heldld.vertex_index(),
                    VertexCache {
                        index: heldld,
                        bottom_up: DirectedPositions::from_iter([]),
                        top_down: DirectedPositions::from_iter([(
                            2.into(),
                            PositionCache::new(
                                Default::default(),
                                HashMap::from_iter([(
                                    DirectedKey {
                                        index: ld,
                                        pos: DirectedPosition::TopDown(
                                            2.into(),
                                        ),
                                    },
                                    SubLocation::new(heldld_id, 2),
                                )]),
                            ),
                        )]),
                    }
                ),
                (
                    ld.vertex_index(),
                    VertexCache {
                        index: ld,
                        bottom_up: DirectedPositions::from_iter([]),
                        top_down: DirectedPositions::from_iter([(
                            2.into(),
                            PositionCache::new(
                                Default::default(),
                                HashMap::from_iter([(
                                    DirectedKey {
                                        index: l,
                                        pos: DirectedPosition::TopDown(
                                            2.into(),
                                        ),
                                    },
                                    SubLocation::new(ld_id, 0),
                                )]),
                            ),
                        )]),
                    }
                ),
                (
                    h.vertex_index(),
                    VertexCache {
                        index: h,
                        bottom_up: DirectedPositions::from_iter([]),
                        top_down: DirectedPositions::from_iter([]),
                    }
                ),
                (
                    l.vertex_index(),
                    VertexCache {
                        index: l,
                        bottom_up: DirectedPositions::from_iter([]),
                        top_down: DirectedPositions::from_iter([(
                            2.into(),
                            PositionCache::default()
                        )]),
                    }
                ),
            ]),
        }
    );
}

#[test]
fn test_build_trace_cache2() {
    let mut graph = HypergraphRef::default();
    insert_atoms!(graph, {a, b, c, d});

    insert_patterns!(graph,
        (ab, ab_id) => [a, b],
        (ababcd, ababcd_id) => [ab, ab, c, d]
    );
    let cache = build_trace_cache!(
        ababcd => (
            BU { 1 => ab -> (ababcd_id, 1) },
            TD {},
        ),
        ab => (
            BU { 1 => b -> (ab_id, 1) },
            TD {},
        ),
        b => (
            BU {},
            TD {},
        ),
    );
    assert_eq!(
        cache,
        TraceCache {
            entries: HashMap::from_iter([
                (
                    ababcd.vertex_index(),
                    VertexCache {
                        index: ababcd,
                        bottom_up: DirectedPositions::from_iter([(
                            1.into(),
                            PositionCache::new(
                                Default::default(),
                                HashMap::from_iter([(
                                    DirectedKey {
                                        index: ab,
                                        pos: DirectedPosition::BottomUp(
                                            1.into(),
                                        ),
                                    },
                                    SubLocation::new(ababcd_id, 1),
                                )]),
                            ),
                        )]),
                        top_down: DirectedPositions::from_iter([]),
                    }
                ),
                (
                    ab.vertex_index(),
                    VertexCache {
                        index: ab,
                        bottom_up: DirectedPositions::from_iter([(
                            1.into(),
                            PositionCache::new(
                                Default::default(),
                                HashMap::from_iter([(
                                    DirectedKey {
                                        index: b,
                                        pos: DirectedPosition::BottomUp(
                                            1.into(),
                                        ),
                                    },
                                    SubLocation::new(ab_id, 1),
                                )]),
                            ),
                        )]),
                        top_down: DirectedPositions::from_iter([]),
                    }
                ),
                (
                    b.vertex_index(),
                    VertexCache {
                        index: b,
                        bottom_up: DirectedPositions::from_iter([]),
                        top_down: DirectedPositions::from_iter([]),
                    }
                ),
            ]),
        }
    );
}
