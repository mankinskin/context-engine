use std::{
    borrow::{
        Borrow,
        BorrowMut,
    },
    fmt::Debug,
    iter::IntoIterator,
};

use crate::{
    ToToken,
    graph::vertex::{
        pattern::pattern_range::PatternRangeIndex,
        wide::Wide,
    },
};

use crate::graph::vertex::token::Token;

pub(crate) mod id;
pub(crate) mod pattern_range;

pub type Pattern = Vec<Token>;
pub(crate) type PatternView<'a> = &'a [Token];
pub(crate) type Patterns = Vec<Pattern>;

pub trait PatternWidth: IntoPattern {
    fn pattern_width(&self) -> usize;
}
impl PatternWidth for Pattern {
    fn pattern_width(&self) -> usize {
        pattern_width(self)
    }
}
/// trait for types which can be converted to a pattern with a known size
pub trait IntoPattern: Sized
//IntoIterator<Item = Self::Elem, IntoIter = Self::Iter> + Sized + Borrow<[Token]> + Debug
{
    //type Iter: ExactSizeIterator + DoubleEndedIterator<Item = Self::Elem>;
    //type Elem: ToToken;

    //fn into_pattern(self) -> Pattern {
    //    self.into_iter().map(|x| x.to_child()).collect()
    //}
    fn into_pattern(self) -> Pattern;
    fn is_empty(&self) -> bool;
}

impl<const N: usize> IntoPattern for [Token; N] {
    fn into_pattern(self) -> Pattern {
        self.into_iter().collect()
    }
    fn is_empty(&self) -> bool {
        N == 0
    }
}
impl IntoPattern for Token {
    fn into_pattern(self) -> Pattern {
        Some(self).into_iter().collect()
    }
    fn is_empty(&self) -> bool {
        false
    }
}
//impl<It: IntoIterator<Item = Token> + Borrow<[Token]>> IntoPattern for It {
//    fn into_pattern(self) -> Pattern {
//        self.into_iter().collect()
//    }
//    fn is_empty(&self) -> bool {
//        (*self).borrow().is_empty()
//    }
//}
impl IntoPattern for &'_ [Token] {
    fn into_pattern(self) -> Pattern {
        self.iter().map(Clone::clone).collect()
    }
    fn is_empty(&self) -> bool {
        (*self).is_empty()
    }
}
impl IntoPattern for Pattern {
    fn into_pattern(self) -> Pattern {
        self
    }
    fn is_empty(&self) -> bool {
        (*self).is_empty()
    }
}
impl<T: IntoPattern + Clone> IntoPattern for &'_ T {
    fn into_pattern(self) -> Pattern {
        self.clone().into_pattern()
    }
    fn is_empty(&self) -> bool {
        (*self).is_empty()
    }
}

//impl<C, It, T> IntoPattern for T
//where
//    C: ToToken,
//    It: DoubleEndedIterator<Item = C> + ExactSizeIterator,
//    T: IntoIterator<Item = C, IntoIter = It> + Borrow<[Token]> + Debug,
//{
//    type Iter = It;
//    type Elem = C;
//}

/// trait for types which can be converted to a pattern with a known size
pub(crate) trait AsPatternMut: BorrowMut<Vec<Token>> + Debug {}

impl<T> AsPatternMut for T where T: BorrowMut<Vec<Token>> + Debug {}

pub fn pattern_width<T: Borrow<Token>>(
    pat: impl IntoIterator<Item = T>
) -> usize {
    pat.into_iter().map(|c| c.borrow().width()).sum()
}

pub(crate) fn pattern_pre_ctx<T: Borrow<Token>>(
    pat: impl IntoIterator<Item = T>,
    sub_index: usize,
) -> impl IntoIterator<Item = T> {
    pat.into_iter().take(sub_index)
}

pub(crate) fn pattern_post_ctx<T: Borrow<Token>>(
    pat: impl IntoIterator<Item = T>,
    sub_index: usize,
) -> impl IntoIterator<Item = T> {
    pattern_post(pat, sub_index + 1)
}
pub(crate) fn pattern_post<T: Borrow<Token>>(
    pat: impl IntoIterator<Item = T>,
    sub_index: usize,
) -> impl IntoIterator<Item = T> {
    pat.into_iter().skip(sub_index)
}
pub(crate) fn pattern_pre<T: Borrow<Token>>(
    pat: impl IntoIterator<Item = T>,
    sub_index: usize,
) -> impl IntoIterator<Item = T> {
    pattern_pre_ctx(pat, sub_index + 1)
}

pub(crate) fn prefix<T: ToToken + Clone>(
    pattern: &'_ [T],
    index: usize,
) -> Vec<T> {
    pattern.get(0..index).unwrap_or(pattern).to_vec()
}

pub(crate) fn infix<T: ToToken + Clone>(
    pattern: &'_ [T],
    start: usize,
    end: usize,
) -> Vec<T> {
    pattern.get(start..end).unwrap_or(&[]).to_vec()
}

pub(crate) fn postfix<T: ToToken + Clone>(
    pattern: &'_ [T],
    index: usize,
) -> Vec<T> {
    pattern.get(index..).unwrap_or(&[]).to_vec()
}

#[track_caller]
#[tracing::instrument(skip(pattern, range, replace))]
pub(crate) fn replace_in_pattern(
    mut pattern: impl AsPatternMut,
    range: impl PatternRangeIndex,
    replace: impl IntoPattern,
) -> Pattern {
    pattern
        .borrow_mut()
        .splice(range, replace.into_pattern())
        .collect()
}

pub(crate) fn single_child_patterns(
    halves: Vec<Pattern>
) -> Result<Token, Vec<Pattern>> {
    match (halves.len(), halves.first()) {
        (1, Some(first)) =>
            single_child_pattern(first.clone()).map_err(|_| halves),
        _ => Err(halves),
    }
}

pub(crate) fn single_child_pattern(half: Pattern) -> Result<Token, Pattern> {
    match (half.len(), half.first()) {
        (1, Some(first)) => Ok(*first),
        _ => Err(half),
    }
}

/// Split a pattern before the specified index
pub(crate) fn split_pattern_at_index<T: ToToken + Clone>(
    pattern: &'_ [T],
    index: usize,
) -> (Vec<T>, Vec<T>) {
    (prefix(pattern, index), postfix(pattern, index))
}

pub(crate) fn split_context<T: ToToken + Clone>(
    pattern: &'_ [T],
    index: usize,
) -> (Vec<T>, Vec<T>) {
    (prefix(pattern, index), postfix(pattern, index + 1))
}

pub(crate) fn double_split_context(
    pattern: PatternView<'_>,
    left_index: usize,
    right_index: usize,
) -> (Pattern, Pattern, Pattern) {
    let (prefix, rem) = split_context(pattern, left_index);
    if left_index < right_index {
        let (infix, postfix) =
            split_context(&rem, right_index - (left_index + 1));
        (prefix, infix, postfix)
    } else {
        (prefix, vec![], rem)
    }
}
