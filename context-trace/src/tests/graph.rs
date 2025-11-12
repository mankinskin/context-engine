use crate::{
    graph::{
        Hypergraph,
        kind::BaseGraphKind,
    },
    insert_atoms,
    insert_patterns,
};

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
