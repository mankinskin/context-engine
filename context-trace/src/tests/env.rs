use crate::graph::{
    Hypergraph,
    HypergraphRef,
    vertex::{
        child::Child,
        pattern::id::PatternId,
        token::Token,
    },
};
use std::sync::{
    Arc,
    RwLock,
    RwLockReadGuard,
    RwLockWriteGuard,
};
pub trait TestEnv {
    fn initialize_expected() -> Self;
    fn get_expected<'a>() -> RwLockReadGuard<'a, Self>;
    fn get_expected_mut<'a>() -> RwLockWriteGuard<'a, Self>;
}
pub struct Env1 {
    pub(crate) graph: HypergraphRef,
    pub(crate) a: Child,
    pub(crate) b: Child,
    pub(crate) c: Child,
    pub(crate) d: Child,
    pub(crate) e: Child,
    pub(crate) f: Child,
    pub(crate) g: Child,
    pub(crate) h: Child,
    pub(crate) i: Child,
    pub(crate) ab: Child,
    pub(crate) bc: Child,
    pub(crate) bc_id: PatternId,
    pub(crate) cd: Child,
    pub(crate) cd_id: PatternId,
    pub(crate) bcd: Child,
    pub(crate) b_cd_id: PatternId,
    pub(crate) bc_d_id: PatternId,
    pub(crate) def: Child,
    pub(crate) d_ef_id: PatternId,
    pub(crate) abc: Child,
    pub(crate) a_bc_id: PatternId,
    pub(crate) abcd: Child,
    pub(crate) a_bcd_id: PatternId,
    pub(crate) abc_d_id: PatternId,
    pub(crate) ef: Child,
    pub(crate) e_f_id: PatternId,
    pub(crate) ghi: Child,
    pub(crate) cdef: Child,
    pub(crate) c_def_id: PatternId,
    pub(crate) cd_ef_id: PatternId,
    pub(crate) efghi: Child,
    pub(crate) abab: Child,
    pub(crate) ababab: Child,
    pub(crate) abcdef: Child,
    pub(crate) abcd_ef_id: PatternId,
    pub(crate) abc_def_id: PatternId,
    pub(crate) ab_cdef_id: PatternId,
    pub(crate) abcdefghi: Child,
    pub(crate) abcd_efghi_id: PatternId,
    pub(crate) abcdef_ghi_id: PatternId,
    pub(crate) ababababcdefghi: Child,
}
pub(crate) fn tokens1(graph: &mut Hypergraph) -> Vec<Child> {
    graph.insert_tokens([
        Token::Element('a'),
        Token::Element('b'),
        Token::Element('c'),
        Token::Element('d'),
        Token::Element('e'),
        Token::Element('f'),
        Token::Element('g'),
        Token::Element('h'),
        Token::Element('i'),
    ])
}
impl TestEnv for Env1 {
    fn initialize_expected() -> Self {
        let mut graph = Hypergraph::default();
        let [a, b, c, d, e, f, g, h, i] = tokens1(&mut graph)[..] else {
            panic!()
        };
        // abcdefghi
        // ababababcdbcdefdefcdefefghefghghi
        // ->
        // abab ab abcdbcdefdefcdefefghefghghi
        // ab abab abcdbcdefdefcdefefghefghghi

        // abcdbcdef def cdef efgh efgh ghi

        // abcd b cdef
        // abcd bcd ef

        // ab cd
        // abc d
        // a bcd
        // index: 9
        let ab = graph.insert_pattern(vec![a, b]);
        let (bc, bc_id) = graph.insert_pattern_with_id(vec![b, c]);
        let (abc, abc_ids) =
            graph.insert_patterns_with_ids([vec![ab, c], vec![a, bc]]);

        let (cd, cd_id) = graph.insert_pattern_with_id(vec![c, d]);
        // 13
        let (bcd, bcd_ids) =
            graph.insert_patterns_with_ids([vec![bc, d], vec![b, cd]]);
        //let abcd = graph.insert_pattern(&[abc, d]);
        //graph.insert_to_pattern(abcd, &[a, bcd]);
        let (abcd, abcd_ids) =
            graph.insert_patterns_with_ids([vec![abc, d], vec![a, bcd]]);
        // index 15
        let (ef, e_f_id) = graph.insert_pattern_with_id(vec![e, f]);
        let gh = graph.insert_pattern(vec![g, h]);
        let ghi = graph.insert_pattern(vec![gh, i]);
        let efgh = graph.insert_pattern(vec![ef, gh]);
        let efghi = graph.insert_patterns([vec![efgh, i], vec![ef, ghi]]);
        let (def, d_ef_id) = graph.insert_pattern_with_id(vec![d, ef]);
        let (cdef, cdef_ids) =
            graph.insert_patterns_with_ids([vec![c, def], vec![cd, ef]]);
        // index 22
        let (abcdef, abcdef_ids) = graph.insert_patterns_with_ids([
            vec![abcd, ef],
            vec![abc, def],
            vec![ab, cdef],
        ]);
        let (abcdefghi, abcdefghi_ids) = graph
            .insert_patterns_with_ids([vec![abcd, efghi], vec![abcdef, ghi]]);
        let aba = graph.insert_pattern(vec![ab, a]);
        // 25
        let abab = graph.insert_patterns([vec![aba, b], vec![ab, ab]]);
        let ababab = graph.insert_patterns([vec![abab, ab], vec![ab, abab]]);
        let ababcd = graph.insert_patterns([
            vec![ab, abcd],
            vec![aba, bcd],
            vec![abab, cd],
        ]);
        // 28
        let ababababcd =
            graph.insert_patterns([vec![ababab, abcd], vec![abab, ababcd]]);
        let ababcdefghi =
            graph.insert_patterns([vec![ab, abcdefghi], vec![ababcd, efghi]]);
        // 30
        let ababababcdefghi = graph.insert_patterns([
            vec![ababababcd, efghi],
            vec![abab, ababcdefghi],
            vec![ababab, abcdefghi],
        ]);
        Env1 {
            graph: HypergraphRef::from(graph),
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
            bc_id: bc_id.unwrap(),
            cd,
            cd_id: cd_id.unwrap(),
            bcd,
            bc_d_id: bcd_ids[0],
            b_cd_id: bcd_ids[1],
            abc,
            a_bc_id: abc_ids[1],
            abcd,
            abc_d_id: abcd_ids[0],
            a_bcd_id: abcd_ids[1],
            ef,
            e_f_id: e_f_id.unwrap(),
            def,
            d_ef_id: d_ef_id.unwrap(),
            ghi,
            abcdef,
            abcd_ef_id: abcdef_ids[0],
            abc_def_id: abcdef_ids[1],
            ab_cdef_id: abcdef_ids[2],
            abcdefghi,
            abcd_efghi_id: abcdefghi_ids[0],
            abcdef_ghi_id: abcdefghi_ids[1],
            cdef,
            c_def_id: cdef_ids[0],
            cd_ef_id: cdef_ids[1],
            efghi,
            abab,
            ababab,
            ababababcdefghi,
        }
    }
    fn get_expected<'a>() -> RwLockReadGuard<'a, Self> {
        CONTEXT.read().unwrap()
    }
    fn get_expected_mut<'a>() -> RwLockWriteGuard<'a, Self> {
        CONTEXT.write().unwrap()
    }
}
lazy_static::lazy_static! {
    pub(crate) static ref
        CONTEXT: Arc<RwLock<Env1>> = Arc::new(RwLock::new(Env1::initialize_expected()));
}
//pub(crate) fn context() -> RwLockReadGuard<'static, Ctx> {
//    CONTEXT.read().unwrap()
//}
//
//pub(crate) fn context_mut() -> RwLockWriteGuard<'static, Ctx> {
//    CONTEXT.write().unwrap()
//}
