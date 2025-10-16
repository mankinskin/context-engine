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
    graph::{
        Hypergraph,
        HypergraphRef,
        getters::{
            ErrorReason,
            IndexWithPath,
            vertex::VertexSet,
        },
        kind::{
            BaseGraphKind,
            TokenOf,
        },
        vertex::{
            VertexIndex,
            child::{
                Child,
                HasChild,
            },
            has_vertex_data::HasVertexData,
            has_vertex_index::{
                HasVertexIndex,
                ToChild,
            },
            location::{
                SubLocation,
                child::{
                    ChildLocation,
                    HasSubIndex,
                },
                pattern::{
                    HasPatternLocation,
                    PatternLocation,
                },
            },
            parent::Parent,
            pattern::{
                Pattern,
                id::PatternId,
            },
            token::{
                AsToken,
                Token,
                tokenizing_iter,
            },
            wide::Wide,
        },
    },
    path::{
        RolePathUtils,
        accessors::{
            child::{
                PathChild,
                root::{
                    GraphRootChild,
                    RootChild,
                },
            },
            complete::PathComplete,
            has_path::{
                HasPath,
                HasRootedPath,
                IntoRootedPath,
                IntoRootedRolePath,
            },
            role::{
                End,
                PathRole,
                Start,
            },
            root::{
                GraphRoot,
                PatternRoot,
                RootPattern,
            },
        },
        mutators::{
            adapters::IntoAdvanced,
            append::PathAppend,
            lower::PathLower,
            move_path::{
                advance::{
                    Advance,
                    CanAdvance,
                },
                key::{
                    MoveKey,
                    TokenPosition,
                },
                path::MovePath,
                root::MoveRootIndex,
            },
            pop::PathPop,
            simplify::PathSimplify,
        },
        structs::{
            query_range_path::FoldablePath,
            role_path::{
                CalcOffset,
                RolePath,
            },
            rooted::{
                RootedRangePath,
                index_range::IndexRangePath,
                pattern_range::{
                    PatternPostfixPath,
                    PatternRangePath,
                },
                role_path::{
                    IndexEndPath,
                    IndexStartPath,
                    PatternEndPath,
                    RootChildIndex,
                    RootedRolePath,
                    range::{
                        EndPath,
                        HasEndPath,
                        HasStartPath,
                        StartPath,
                    },
                },
                root::{
                    IndexRoot,
                    RootedPath,
                },
            },
            sub_path::SubPath,
        },
    },
    trace::{
        StateDirection,
        TraceCtx,
        cache::{
            TraceCache,
            key::{
                directed::{
                    DirectedKey,
                    DirectedPosition,
                    HasTokenPosition,
                    down::DownKey,
                    up::UpKey,
                },
                props::{
                    CursorPosition,
                    LeafKey,
                    RootKey,
                    TargetKey,
                },
            },
            position::PositionCache,
            vertex::{
                VertexCache,
                positions::DirectedPositions,
            },
        },
        child::{
            ChildTracePos,
            iterator::{
                ChildIterator,
                ChildQueue,
            },
            state::{
                ChildState,
                PrefixStates,
                RootChildState,
            },
        },
        command::{
            PostfixCommand,
            PrefixCommand,
            RangeCommand,
        },
        has_graph::{
            HasGraph,
            HasGraphMut,
            TravKind,
        },
        state::{
            BaseState,
            HasPrevPos,
            HasRootPos,
            InnerKind,
            parent::{
                ParentBatch,
                ParentState,
            },
        },
        traceable::Traceable,
    },
};
