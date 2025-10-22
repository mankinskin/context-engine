#![deny(clippy::disallowed_methods)]
#![feature(test)]
#![feature(assert_matches)]
#![feature(try_blocks)]
//#![feature(hash_drain_filter)]
#![feature(slice_pattern)]
//#![feature(pin_macro)]
#![feature(exact_size_is_empty)]
#![feature(associated_type_defaults)]
//#![feature(return_position_impl_trait_in_trait)]
#![feature(type_changing_struct_update)]

extern crate test;

pub(crate) mod compare;
pub(crate) mod container;
pub(crate) mod cursor;
pub(crate) mod fold;
pub(crate) mod r#match;
pub(crate) mod search;
pub(crate) mod state;
pub(crate) mod traversal;

#[cfg(any(test, feature = "test-api"))]
pub(crate) mod tests;

pub use crate::{
    container::bft::BftQueue,
    fold::foldable::{
        ErrorState,
        StartFold,
    },
    search::{
        context::AncestorPolicy,
        Searchable,
    },
    state::{
        complete::UnwrapComplete,
        result::{
            CompleteState,
            IncompleteState,
            Response,
        },
    },
    traversal::TraversalKind,
};
