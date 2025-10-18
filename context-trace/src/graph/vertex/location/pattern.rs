use std::{
    cmp::PartialEq,
    fmt::Debug,
    ops::Range,
};

use crate::{
    HasPatternId,
    graph::vertex::{
        PatternId,
        location::{
            ChildLocation,
            HasParent,
        },
        pattern::Pattern,
    },
};

use crate::graph::vertex::token::Token;

pub(crate) struct PatternRangeLocation {
    pub parent: Token,
    pub id: PatternId,
    pub range: Range<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub struct PatternLocation {
    pub parent: Token,
    pub pattern_id: PatternId,
}

impl HasPatternId for PatternLocation {
    fn pattern_id(&self) -> PatternId {
        self.pattern_id
    }
}
impl HasParent for PatternLocation {
    fn parent(&self) -> &Token {
        &self.parent
    }
}
impl PatternLocation {
    pub fn new(
        parent: Token,
        pattern_id: PatternId,
    ) -> Self {
        Self { parent, pattern_id }
    }
    pub(crate) fn to_child_location(
        &self,
        sub_index: usize,
    ) -> ChildLocation {
        ChildLocation {
            parent: self.parent,
            pattern_id: self.pattern_id,
            sub_index,
        }
    }
    pub(crate) fn with_range(
        self,
        range: Range<usize>,
    ) -> PatternRangeLocation {
        PatternRangeLocation {
            parent: self.parent,
            id: self.pattern_id,
            range,
        }
    }
    //pub(crate) fn get_pattern<
    //    'a: 'g,
    //    'g,
    //    T: Atomize,
    //    Trav: HasGraph<T> + 'a,
    //>(&'a self, trav: &'a Trav) -> Option<&Pattern> {
    //    trav.graph().get_pattern_at(self).ok()
    //}
    //pub(crate) fn expect_pattern<
    //    'a: 'g,
    //    'g,
    //    T: Atomize,
    //    Trav: HasGraph<T> + 'a,
    //>(&'a self, trav: &'a Trav) -> &Pattern {
    //    trav.graph().expect_pattern_at(self)
    //}
    pub(crate) fn get_pattern_in<'a>(
        &self,
        patterns: &'a crate::graph::vertex::TokenPatterns,
    ) -> Option<&'a Pattern> {
        patterns.get(&self.pattern_id)
    }
    pub(crate) fn expect_pattern_in<'a>(
        &self,
        patterns: &'a crate::graph::vertex::TokenPatterns,
    ) -> &'a Pattern {
        self.get_pattern_in(patterns)
            .expect("Expected Pattern not present in TokenPatterns!")
    }
}

pub trait IntoPatternLocation {
    fn into_pattern_location(self) -> PatternLocation;
}

impl<P: IntoPatternLocation + Copy> IntoPatternLocation for &'_ P {
    fn into_pattern_location(self) -> PatternLocation {
        (*self).into_pattern_location()
    }
}
impl IntoPatternLocation for PatternLocation {
    fn into_pattern_location(self) -> PatternLocation {
        self
    }
}

pub trait HasPatternLocation {
    fn pattern_location(&self) -> &PatternLocation;
}
