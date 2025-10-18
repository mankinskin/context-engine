#[cfg(test)]
use {
    crate::graph::{
        Hypergraph,
        kind::BaseGraphKind,
        vertex::atom::Atom,
    },
    itertools::Itertools,
};

#[macro_export]
macro_rules! insert_patterns {
    ($graph:ident,
        $(
            $name:ident => [
                $([$($pat:expr),*]),*$(,)?
            ]
        ),*$(,)?
    ) => {

        $(
            let $name = $graph.insert_patterns([$(vec![$($pat),*]),*]);
        )*
    };
    ($graph:ident,
        $(
            $name:ident =>
                [$($pat:expr),*]
        ),*$(,)?
    ) => {

        $(
            let $name = $graph.insert_pattern([$($pat),*]);
        )*
    };
    ($graph:ident,
        $(
            ($name:ident, $idname:ident) => [
                $([$($pat:expr),*]),*$(,)?
            ]
        ),*$(,)?
    ) => {

        $(
            let ($name, $idname) = $crate::HasGraphMut::graph_mut(&mut $graph).insert_patterns_with_ids([$(vec![$($pat),*]),*]);
        )*
    };
    ($graph:ident,
        $(
            ($name:ident, $idname:ident) =>
                [$($pat:expr),*]
        ),*$(,)?
    ) => {

        $(
            let ($name, $idname) = $crate::HasGraphMut::graph_mut(&mut $graph).insert_pattern_with_id([$($pat),*]);
            let $idname = $idname.unwrap();
        )*
    };
}
#[test]
fn test_to_petgraph() {
    let mut graph = Hypergraph::<BaseGraphKind>::default();
    let (a, b, c, d) = graph
        .insert_atoms([
            Atom::Element('a'),
            Atom::Element('b'),
            Atom::Element('c'),
            Atom::Element('d'),
        ])
        .into_iter()
        .next_tuple()
        .unwrap();
    // ab cd
    // abc d
    // a bcd

    let ab = graph.insert_pattern(vec![a, b]);
    let bc = graph.insert_pattern(vec![b, c]);
    let abc = graph.insert_patterns([vec![ab, c], vec![a, bc]]);
    let cd = graph.insert_pattern(vec![c, d]);
    let bcd = graph.insert_patterns([vec![bc, d], vec![b, cd]]);
    let _abcd = graph.insert_patterns([vec![abc, d], vec![a, bcd]]);
    let pg = graph.to_petgraph();
    pg.write_to_file("assets/test_graph1.dot")
        .expect("Failed to write assets/test_graph1.dot file!");
}
