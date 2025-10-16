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

pub mod direction;
pub mod path;

pub mod graph;
pub mod trace;

#[cfg(any(test, feature = "test-api"))]
pub mod tests;

#[cfg(not(any(test, feature = "test-api")))]
pub use std::collections::{
    HashMap,
    HashSet,
};
#[cfg(any(test, feature = "test-api"))]
pub use {
    ::charify,
    std::hash::{
        BuildHasherDefault,
        DefaultHasher,
    },
};
#[cfg(any(test, feature = "test-api"))]
pub type HashSet<T> =
    std::collections::HashSet<T, BuildHasherDefault<DefaultHasher>>;
#[cfg(any(test, feature = "test-api"))]
pub type HashMap<K, V> =
    std::collections::HashMap<K, V, BuildHasherDefault<DefaultHasher>>;

#[cfg(any(test, feature = "test-api"))]
pub use tests::{
    assert_parents,
    env::{
        Env1,
        TestEnv,
    },
    init_tracing,
};

// Essential public re-exports for context-search
pub use crate::{
    direction::{
        Left,
        Right,
    },
    graph::getters::vertex::VertexSet,
    // Only the items that are already public or made public
    graph::vertex::{
        VertexIndex,
        child::Child,
        has_vertex_index::{
            HasVertexIndex,
            ToChild,
        },
        location::child::ChildLocation,
        pattern::{
            Pattern,
            id::PatternId,
        },
        token::tokenizing_iter,
        wide::Wide,
    },
    // ===== Missing imports from context-search =====
    graph::{
        Hypergraph,
        HypergraphRef,
        getters::{
            ErrorReason,
            IndexWithPath,
        },
        kind::TokenOf,
        vertex::token::AsToken,
    },
    path::accessors::has_path::HasRootedRolePath,
    path::accessors::role::{
        End,
        PathRole,
        Start,
    },
    path::accessors::root::RootPattern,
    path::accessors::root::{
        GraphRoot,
        PatternRoot,
    },
    path::mutators::lower::PathLower,
    path::mutators::move_path::advance::Advance,
    path::mutators::move_path::key::TokenPosition,
    path::mutators::simplify::PathSimplify,
    path::structs::role_path::CalcOffset,
    path::{
        accessors::{
            child::{
                PathChild,
                RootChildIndex,
                root::{
                    GraphRootChild,
                    RootChild,
                },
            },
            complete::PathComplete,
            has_path::HasPath,
        },
        mutators::{
            adapters::IntoAdvanced,
            append::PathAppend,
            move_path::{
                key::MoveKey,
                path::MovePath,
                root::MoveRootIndex,
            },
            pop::PathPop,
        },
        structs::{
            query_range_path::FoldablePath,
            role_path::RolePath,
            rooted::{
                index_range::IndexRangePath,
                pattern_range::{
                    PatternPostfixPath,
                    PatternRangePath,
                },
                role_path::{
                    IndexEndPath,
                    IndexStartPath,
                    PatternEndPath,
                    RootedRolePath,
                },
                root::IndexRoot,
                split_path::RootedSplitPathRef,
            },
            sub_path::SubPath,
        },
    },
    trace::child::state::PrefixStates,
    // Core types that are already properly exposed
    trace::child::state::RootChildState,
    trace::{
        StateDirection,
        TraceCtx,
        cache::{
            TraceCache,
            key::{
                directed::{
                    DirectedKey,
                    HasTokenPosition,
                    down::DownKey,
                    up::UpKey,
                },
                props::{
                    CursorPosition,
                    RootKey,
                    TargetKey,
                },
            },
        },
        child::{
            iterator::{
                ChildIterator,
                ChildQueue,
            },
            state::ChildState,
        },
        command::{
            PostfixCommand,
            PrefixCommand,
            RangeCommand,
        },
        has_graph::{
            HasGraph,
            TravKind,
        },
        state::{
            InnerKind,
            parent::ParentState,
        },
        traceable::Traceable,
    },
    trace::{
        cache::key::props::LeafKey,
        state::{
            BaseState,
            parent::ParentBatch,
        },
    },
};
// Auto-generated pub(crate) use statements for internal use only
pub(crate) use crate::{
    direction::Direction,
    graph::vertex::location::pattern::PatternLocation,
    trace::cache::{
        new::EditKind,
        position::PositionCache,
        vertex::positions::DirectedPositions,
    },
};
