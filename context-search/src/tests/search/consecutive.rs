use crate::{
    state::result::HasBaseResponse,
    UnwrapComplete,
};
use context_trace::{
    GraphRoot,
    RootChild,
};

#[cfg(test)]
use {
    crate::cursor::PatternCursor,
    crate::search::Searchable,
    crate::state::result::Response,
    context_trace::tests::env::Env1,
    context_trace::PatternPrefixPath,
    context_trace::{
        graph::vertex::token::Token,

        tests::env::TestEnv,
    },
    pretty_assertions::assert_matches,
};

#[test]
fn find_consecutive1() {
    let Env1 {
        graph,
        a,
        b,
        c,
        g,
        h,
        i,
        abc,
        ghi,
        ..
    } = &*Env1::get_expected();
    //let a_bc_pattern = [Token::new(a, 1), Token::new(bc, 2)];
    //let ab_c_pattern = [Token::new(ab, 2), Token::new(c, 1)];
    let g_h_i_a_b_c_pattern = vec![
        Token::new(g, 1),
        Token::new(h, 1),
        Token::new(i, 1),
        Token::new(a, 1),
        Token::new(b, 1),
        Token::new(c, 1),
    ];

    let query =
        PatternCursor::from(PatternPrefixPath::from(g_h_i_a_b_c_pattern));
    let fin = graph.find_ancestor(&query);
    assert_matches!(
        fin,
        Ok(Response::Complete(x)
        ) if x.path.root_parent() == *ghi,
        "+g_h_i_a_b_c"
    );
    let query = fin.unwrap().unwrap_complete().make_cursor();
    assert_matches!(
        graph.find_ancestor(query),
        Ok(Response::Complete(x)
        ) if x.path.root_parent() == *abc,
        "g_h_i_+a_b_c"
    );
}
