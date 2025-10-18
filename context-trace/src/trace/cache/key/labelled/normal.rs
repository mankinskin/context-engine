use crate::{
    graph::vertex::{
        token::Token,
        has_vertex_index::HasVertexIndex,
    },
    trace::has_graph::{
        HasGraph,
        TravAtom,
    },
};
use std::fmt::Display;

pub(crate) type Labelled<T> = T;

pub(crate) fn labelled_key<G: HasGraph, T>(
    _trav: &G,
    index: T,
) -> Labelled<T>
where
    TravAtom<G>: Display,
{
    index
}

#[macro_export]
macro_rules! lab {
    ($x:ident) => {
        $x.vertex_index()
    };
}
