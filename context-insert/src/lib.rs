pub mod insert;
pub mod interval;
pub mod join;
pub mod split;

#[cfg(test)]
pub mod tests;

// Auto-generated pub use statements
pub use crate::{
    insert::{
        ToInsertCtx,
        context::InsertCtx,
        result::InsertResult,
    },
    interval::{
        IntervalGraph,
        init::InitInterval,
    },
    split::{
        TraceBack,
        TraceFront,
        TraceSide,
        cache::{
            SplitCache,
            position::{
                PosKey,
                SplitPositionCache,
            },
            vertex::SplitVertexCache,
        },
        trace::states::SplitStates,
        vertex::output::RootMode,
    },
};

pub(crate) use crate::{
    interval::partition::{
        delta::PatternSubDeltas,
        info::range::{
            mode::InVisitMode,
            role::{
                BooleanPerfectOf,
                In,
                OffsetsOf,
                Post,
                Pre,
                RangeRole,
            },
        },
    },
    split::{
        ChildTracePos,
        cleaned_position_splits,
        trace::SplitTraceCtx,
        vertex::{
            ChildTracePositions,
            ToVertexSplitPos,
            VertexSplits,
            output::{
                CompleteLocations,
                InnerNode,
            },
            position::{
                HasInnerOffset,
                Offset,
                SubSplitLocation,
            },
        },
    },
};
