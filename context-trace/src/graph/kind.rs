use crate::{
    direction::{
        Right,
        pattern::PatternDirection,
    },
    graph::vertex::token::Tokenize,
};
use serde::{
    Deserialize,
    Serialize,
    de::DeserializeOwned,
};
use std::fmt::{
    Debug,
    Display,
};

pub trait GraphKind: Debug + Clone + Default + PartialEq + Eq {
    type Token: Tokenize + Display + DeserializeOwned;
    type Direction: PatternDirection;
}

pub type TokenOf<K> = <K as GraphKind>::Token;
pub(crate) type DefaultToken = TokenOf<BaseGraphKind>;
pub(crate) type DirectionOf<K> = <K as GraphKind>::Direction;
pub(crate) type DefaultDirection = DirectionOf<BaseGraphKind>;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct BaseGraphKind;

impl GraphKind for BaseGraphKind {
    type Token = char;
    type Direction = Right;
}
