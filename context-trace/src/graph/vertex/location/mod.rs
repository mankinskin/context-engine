use std::{
    cmp::PartialEq,
    fmt::Debug,
};

use child::*;
use pattern::*;

use crate::{PatternId};

pub(crate) mod child;
pub(crate) mod pattern;

#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub(crate) struct SubLocation {
    pub(crate) pattern_id: PatternId,
    pub(crate) sub_index: usize,
}

impl SubLocation {
    pub(crate) fn new(
        pattern_id: PatternId,
        sub_index: usize,
    ) -> Self {
        Self {
            pattern_id,
            sub_index,
        }
    }
}

impl From<ChildLocation> for SubLocation {
    fn from(value: ChildLocation) -> Self {
        value.to_sub_location()
    }
}
