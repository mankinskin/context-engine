//! Compact formatting implementations for context-search types

mod cursor_format;

use crate::{
    compare::state::{
        CompareResult,
        CompareState,
        PathPairMode,
    },
    cursor::{
        CursorState,
        PathCursor,
    },
    r#match::{
        iterator::SearchIterator,
        SearchNode,
    },
    state::end::PathEnum,
    traversal::TraversalKind,
};
use context_trace::{
    impl_display_via_compact,
    logging::compact_format::{
        write_indent,
        Compact,
        CompactFormat,
    },
    AtomPosition,
    HasTargetPos,
};
use std::fmt;

impl<Q, I> CompactFormat for CompareState<Q, I>
where
    Q: CursorState,
    I: CursorState,
{
    fn fmt_compact(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        let mode_str = match self.mode {
            PathPairMode::GraphMajor => "G",
            PathPairMode::QueryMajor => "Q",
        };

        let query_pos: usize = self.cursor.atom_position.into();
        let index_pos: usize =
            (*self.child_cursor.child_state.target_pos()).into();
        let checkpoint_pos: usize = self.checkpoint.atom_position.into();

        write!(
            f,
            "Compare(mode:{}, query@{}, index@{}, checkpoint@{})",
            mode_str, query_pos, index_pos, checkpoint_pos
        )
    }

    fn fmt_indented(
        &self,
        f: &mut fmt::Formatter,
        indent: usize,
    ) -> fmt::Result {
        let query_state =
            std::any::type_name::<Q>().split("::").last().unwrap_or("?");
        let index_state =
            std::any::type_name::<I>().split("::").last().unwrap_or("?");

        write_indent(f, indent)?;
        writeln!(f, "CompareState<{}, {}> {{", query_state, index_state)?;

        write_indent(f, indent + 1)?;
        writeln!(f, "mode: {:?},", self.mode)?;

        write_indent(f, indent + 1)?;
        write!(f, "child_cursor: ")?;
        self.child_cursor.child_state.fmt_indented(f, indent + 1)?;
        writeln!(f, ",")?;

        write_indent(f, indent + 1)?;
        write!(f, "query: ")?;
        self.cursor.fmt_indented(f, indent + 1)?;
        writeln!(f, ",")?;

        write_indent(f, indent + 1)?;
        write!(f, "checkpoint: ")?;
        self.checkpoint.fmt_indented(f, indent + 1)?;
        writeln!(f, ",")?;

        write_indent(f, indent)?;
        write!(f, "}}")
    }
}

impl CompactFormat for CompareResult {
    fn fmt_compact(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            CompareResult::FoundMatch(state) => {
                write!(f, "Match(")?;
                state.fmt_compact(f)?;
                write!(f, ")")
            },
            CompareResult::Mismatch(state) => {
                write!(f, "Mismatch(")?;
                state.fmt_compact(f)?;
                write!(f, ")")
            },
            CompareResult::Prefixes(queue) => {
                write!(f, "Prefixes(count:{})", queue.len())
            },
        }
    }

    fn fmt_indented(
        &self,
        f: &mut fmt::Formatter,
        indent: usize,
    ) -> fmt::Result {
        match self {
            CompareResult::FoundMatch(state) => {
                write_indent(f, indent)?;
                writeln!(f, "FoundMatch(")?;
                state.fmt_indented(f, indent + 1)?;
                writeln!(f)?;
                write_indent(f, indent)?;
                write!(f, ")")
            },
            CompareResult::Mismatch(state) => {
                write_indent(f, indent)?;
                writeln!(f, "Mismatch(")?;
                state.fmt_indented(f, indent + 1)?;
                writeln!(f)?;
                write_indent(f, indent)?;
                write!(f, ")")
            },
            CompareResult::Prefixes(queue) => {
                write_indent(f, indent)?;
                write!(f, "Prefixes(count:{})", queue.len())
            },
        }
    }
}

impl<K: TraversalKind> CompactFormat for SearchIterator<K> {
    fn fmt_compact(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        let queue_size = self.queue.nodes.len();
        let cache_size = self.trace_ctx.cache.entries.len();
        let has_complete = self.last_complete_match.is_some();

        write!(
            f,
            "SearchIterator(cache:{}, queue:{}, complete:{})",
            cache_size, queue_size, has_complete
        )
    }

    fn fmt_indented(
        &self,
        f: &mut fmt::Formatter,
        indent: usize,
    ) -> fmt::Result {
        write_indent(f, indent)?;
        writeln!(f, "SearchIterator {{")?;

        // TraceCtx - show cache info
        write_indent(f, indent + 1)?;
        writeln!(f, "trace_ctx: TraceCtx {{")?;
        write_indent(f, indent + 2)?;
        writeln!(f, "cache_entries: {},", self.trace_ctx.cache.entries.len())?;
        write_indent(f, indent + 1)?;
        writeln!(f, "}},")?;

        // SearchQueue - show nodes
        write_indent(f, indent + 1)?;
        writeln!(f, "queue: SearchQueue {{")?;
        write_indent(f, indent + 2)?;
        writeln!(f, "queue_size: {},", self.queue.nodes.len())?;

        if !self.queue.nodes.is_empty() {
            write_indent(f, indent + 2)?;
            writeln!(f, "nodes: [")?;

            for (i, node) in self.queue.nodes.iter().enumerate() {
                write_indent(f, indent + 3)?;
                write!(f, "[{}] ", i)?;

                match node {
                    SearchNode::ParentCandidate(state) => {
                        writeln!(f, "ParentCandidate {{")?;
                        write_indent(f, indent + 4)?;
                        let cursor_pos: usize =
                            state.cursor.atom_position.into();
                        writeln!(f, "cursor_pos: {},", cursor_pos)?;
                        write_indent(f, indent + 4)?;
                        writeln!(f, "cursor_path:")?;
                        state.cursor.path.fmt_indented(f, indent + 5)?;
                        writeln!(f, ",")?;
                        write_indent(f, indent + 4)?;
                        writeln!(f, "parent_state:")?;
                        state.parent_state.path.fmt_indented(f, indent + 5)?;
                        writeln!(f)?;
                        write_indent(f, indent + 3)?;
                        writeln!(f, "}},")?;
                    },
                    SearchNode::PrefixQueue(queue) => {
                        writeln!(f, "PrefixQueue(size:{}),", queue.len())?;
                    },
                }
            }

            write_indent(f, indent + 2)?;
            writeln!(f, "],")?;
        }

        write_indent(f, indent + 1)?;
        writeln!(f, "}},")?;

        // last_complete_match
        write_indent(f, indent + 1)?;
        if let Some(ref end_state) = self.last_complete_match {
            writeln!(f, "last_complete_match: Some(")?;
            write_indent(f, indent + 2)?;
            writeln!(f, "EndState {{")?;
            write_indent(f, indent + 3)?;
            writeln!(f, "reason: {:?},", end_state.reason)?;
            write_indent(f, indent + 3)?;
            let cursor_pos: usize = end_state.cursor.atom_position.into();
            writeln!(f, "cursor_pos: {},", cursor_pos)?;
            write_indent(f, indent + 3)?;
            write!(f, "path: ")?;
            match &end_state.path {
                PathEnum::Range(p) => {
                    p.fmt_indented(f, indent + 3)?;
                },
                PathEnum::Prefix(p) => {
                    p.fmt_indented(f, indent + 3)?;
                },
                PathEnum::Postfix(p) => {
                    p.fmt_indented(f, indent + 3)?;
                },
                PathEnum::Complete(p) => {
                    p.fmt_indented(f, indent + 3)?;
                },
            }
            writeln!(f)?;
            write_indent(f, indent + 2)?;
            writeln!(f, "}}")?;
            write_indent(f, indent + 1)?;
            writeln!(f, "),")?;
        } else {
            writeln!(f, "last_complete_match: None,")?;
        }

        write_indent(f, indent)?;
        write!(f, "}}")
    }
}

// Implement Display for types to enable % formatting in tracing without Compact wrapper
impl_display_via_compact!(CompareResult);
impl_display_via_compact!(CompareState<Q, I> where Q: CursorState, I: CursorState);
impl_display_via_compact!(SearchIterator<K> where K: TraversalKind);
