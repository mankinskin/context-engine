#[cfg(test)]
use crate::graph::{
    Hypergraph,
    kind::BaseGraphKind,
};
use crate::insert_atoms;

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
            let $name = $crate::HasGraphMut::graph_mut(&mut $graph).insert_patterns([$(vec![$($pat),*]),*]);
        )*
    };
    ($graph:ident,
        $(
            $name:ident =>
                [$($pat:expr),*]
        ),*$(,)?
    ) => {

        $(
            let $name = $crate::HasGraphMut::graph_mut(&mut $graph).insert_pattern([$($pat),*]);
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
    insert_atoms!(graph, {a, b, c, d});
    // ab cd
    // abc d
    // a bcd

    insert_patterns!(graph,
        ab => [a, b],
        bc => [b, c],
        cd => [c, d],
    );
    insert_patterns!(graph,
        abc => [[ab, c], [a, bc]],
        bcd => [[bc, d], [b, cd]],
        _abcd => [[abc, d], [a, bcd]]
    );
    let pg = graph.to_petgraph();
    pg.write_to_file("assets/test_graph1.dot")
        .expect("Failed to write assets/test_graph1.dot file!");
}
