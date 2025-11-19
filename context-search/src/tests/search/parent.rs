#[cfg(test)]
use {
    crate::search::Find,
    crate::state::end::PathCoverage,
    crate::state::matched::{
        QueryExhaustedState,
        MatchedEndState,
    },
    crate::state::result::Response,
    context_trace::tests::env::Env1,
    context_trace::GraphRoot,

    context_trace::{
        graph::{
            getters::{
                ErrorReason,
                IndexWithPath,
            },
            vertex::token::Token,
        },

        tests::env::TestEnv,
    },
    pretty_assertions::{
        assert_eq,
        assert_matches,
    },
};

#[test]
fn find_parent1() {
    let Env1 {
        graph,
        b,
        c,
        ab,
        bc,
        abc,
        ..
    } = &*Env1::get_expected();
    //let a_bc_pattern = [Token::new(a, 1), Token::new(bc, 2)];
    let ab_c_pattern = [Token::new(ab, 2), Token::new(c, 1)];
    //let a_bc_d_pattern =
    //    [Token::new(a, 1), Token::new(bc, 2), Token::new(d, 1)];
    let b_c_pattern = vec![Token::new(b, 1), Token::new(c, 1)];
    let bc_pattern = [Token::new(bc, 2)];
    //let a_b_c_pattern = [Token::new(a, 1), Token::new(b, 1), Token::new(c, 1)];

    let query = bc_pattern;
    assert_eq!(
        graph.find_parent(&query[..]),
        Err(ErrorReason::SingleIndex(Box::new(IndexWithPath {
            index: *bc,
            path: query.into()
        }))),
        "bc"
    );
    let query = b_c_pattern;
    assert_matches!(
        graph.find_parent(&query),
        Ok(Response {
            end: MatchedEndState::QueryExhausted(QueryExhaustedState {
                path: PathCoverage::EntireRoot(ref path),
                ..
            }),
            ..
        }) if path.root_parent() == *bc,
        "b_c"
    );
    let query = ab_c_pattern;
    assert_matches!(
        graph.find_parent(&query),
        Ok(Response {
            end: MatchedEndState::QueryExhausted(QueryExhaustedState {
                path: PathCoverage::EntireRoot(ref path),
                ..
            }),
            ..
        }) if path.root_parent() == *abc,
        "ab_c"
    );
    // enable when bfs for parent-token batches is implemented
    //let query = a_bc_pattern;
    //assert_matches!(
    //    graph.find_parent(&query),
    //    Ok(Response {
    //        kind: ResponseKind::Complete(x),
    //        ..
    //    }) if x == *abc,
    //    "a_bc"
    //);
    //let query = a_bc_d_pattern;
    //assert_matches!(
    //    graph.find_parent(&query),
    //    Ok(Response {
    //        kind: ResponseKind::Complete(x),
    //        ..
    //    }) if x == *abc,
    //    "a_bc_d"
    //);
    //let query = a_b_c_pattern.clone();
    //assert_matches!(
    //    graph.find_parent(&query),
    //    Ok(Response {
    //        kind: ResponseKind::Complete(x),
    //        ..
    //    }) if x == *abc,
    //    "a_b_c"
    //);
    //let query = [&a_b_c_pattern[..], &[Token::new(c, 1)]].concat();
    //assert_matches!(
    //    graph.find_parent(&query),
    //    Ok(Response {
    //        kind: ResponseKind::Complete(x),
    //        ..
    //    }) if x == *abc,
    //    "a_b_c_c"
    //);
}
