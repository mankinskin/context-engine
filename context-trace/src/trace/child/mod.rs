pub(crate) mod bands;
pub(crate) mod iterator;
pub(crate) mod state;

use std::{
    cmp::Ordering,
    num::NonZeroUsize,
};

use crate::{
    HasSubIndex,
    graph::vertex::{
        pattern::{
            IntoPattern,
            id::PatternId,
        },
        wide::Wide,
    },
};

use std::fmt::Debug;
