use context_trace::*;

use pretty_assertions::assert_eq;

pub mod insert;
pub mod interval;

//#[macro_export]
//macro_rules! insert_patterns2 {
//    ($graph:ident,
//        $(
//            $name1:ident => [
//                $($pat1:ident),*
//                $([$($pat2:expr),*]),*
//                $(,)?
//            ]
//            $(
//                ($name2:ident, $idname:ident) => [
//                    $($pat3:ident),*
//                    $([$($pat4:ident),*]),*
//                    $(,)?
//                ]
//            )?
//        ),*
//        $(,)?
//    ) => {
//
//        $(
//            let $name1: Child = $graph.insert_pattern([$($pat1),*]);
//            let $name1: Child = $graph.insert_patterns(vec![$(vec![$($pat2),*]),*] as Vec<context_trace::graph::vertex::pattern::Pattern>);
//            $(
//                let ($name2, $idname): (Child, _) = $graph.graph_mut().insert_pattern_with_id([$($pat3),*]);
//                let $idname = $idname.unwrap();
//            )?
//            $(let ($name2, $idname): (Child, _) = $graph.graph_mut().insert_patterns_with_ids([$(vec![$($pat4),*]),*]))?
//        )*
//    };
//}
//

#[macro_export]
macro_rules! nz {
    ($x:expr) => {
        std::num::NonZeroUsize::new($x).unwrap()
    };
}
#[macro_export]
macro_rules! build_split_cache {
    (
        $root_mode:expr,
        $(
            $entry_root:ident => {
                $(
                    {
                        $($top:ident: $top_pos:expr),*$(,)?
                    } -> $pos:expr => {
                        $($pid:expr => ($sub:expr, $inner:expr)),*$(,)?
                    }
                ),*$(,)?
            }
        ),*
        $(,)?
    ) => {
        SplitCache {
            root_mode: $root_mode,
            entries: HashMap::from_iter([
                $(
                    (
                        $entry_root.index,
                        SplitVertexCache {
                            positions: BTreeMap::from_iter([
                                $(
                                    (
                                        nz!($pos),
                                        SplitPositionCache {
                                            top: HashSet::from_iter([
                                                $(
                                                    PosKey {
                                                        index: $top.to_owned(),
                                                        pos: nz!($top_pos),
                                                    }
                                                ),*
                                            ]),
                                            pattern_splits: HashMap::from_iter([
                                                $(
                                                    (
                                                        $pid.to_owned(),
                                                        ChildTracePos {
                                                            inner_offset: $inner,
                                                            sub_index: $sub,
                                                        }
                                                    )
                                                ),*
                                            ])
                                        }
                                    )
                                ),*
                            ])
                        }
                    )
                ),*
            ])
        }
    };
}
