use derivative::Derivative;

use crate::graph::vertex::{
    location::pattern::PatternLocation,
    pattern::{
        Pattern,
        id::PatternId,
    },
};

#[derive(Debug, Clone, Derivative)]
#[derivative(Hash, PartialEq, Eq)]
pub(crate) struct PatternTraceCtx<'a> {
    pub(crate) loc: PatternLocation,
    #[derivative(Hash = "ignore", PartialEq = "ignore")]
    pub(crate) pattern: &'a Pattern,
}

impl<'p> From<PatternTraceCtx<'p>> for PatternId {
    fn from(value: PatternTraceCtx<'p>) -> Self {
        value.loc.id
    }
}

pub(crate) trait HasPatternTraceCtx {
    fn pattern_trace_context<'a>(&'a self) -> PatternTraceCtx<'a>
    where
        Self: 'a;
}
impl HasPatternTraceCtx for PatternTraceCtx<'_> {
    fn pattern_trace_context<'a>(&'a self) -> PatternTraceCtx<'a>
    where
        Self: 'a,
    {
        self.clone()
    }
}
pub(crate) trait GetPatternTraceCtx {
    fn get_pattern_trace_context<'b>(
        &'b self,
        pattern_id: &PatternId,
    ) -> PatternTraceCtx<'b>
    where
        Self: 'b;
}
pub(crate) trait GetPatternCtx {
    type PatternCtx<'b>: HasPatternTraceCtx
    where
        Self: 'b;
    fn get_pattern_context<'b>(
        &'b self,
        pattern_id: &PatternId,
    ) -> Self::PatternCtx<'b>
    where
        Self: 'b;
}
