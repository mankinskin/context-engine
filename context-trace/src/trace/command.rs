use tracing::debug;

use crate::{
    AtomPosition,
    CalcOffset,
    graph::vertex::{
        pattern::pattern_width,
        wide::Wide,
    },
    path::{
        RolePathUtils,
        accessors::{
            role::{
                End,
                Start,
            },
            root::RootPattern,
        },
        structs::rooted::{
            index_range::IndexRangePath,
            role_path::{
                IndexEndPath,
                IndexStartPath,
            },
        },
    },
    trace::{
        BottomUp,
        RoleTraceKey,
        TopDown,
        TraceCtx,
        TraceRole,
        cache::{
            key::directed::{
                down::DownKey,
                up::{
                    UpKey,
                    UpPosition,
                },
            },
            new::NewTraceEdge,
        },
        has_graph::HasGraph,
        traceable::Traceable,
    },
};
#[derive(Debug)]
pub enum TraceCommand {
    Postfix(PostfixCommand),
    Prefix(PrefixCommand),
    Range(RangeCommand),
}
impl Traceable for TraceCommand {
    fn trace<G: super::has_graph::HasGraph>(
        self,
        ctx: &mut super::TraceCtx<G>,
    ) {
        match self {
            Self::Postfix(cmd) => cmd.trace(ctx),
            Self::Prefix(cmd) => cmd.trace(ctx),
            Self::Range(cmd) => cmd.trace(ctx),
        }
    }
}

#[derive(Debug)]
pub struct PostfixCommand {
    pub path: IndexStartPath,
    pub add_edges: bool,
    pub root_up_key: RoleTraceKey<Start>,
}
impl Traceable for PostfixCommand {
    fn trace<G: HasGraph>(
        self,
        ctx: &mut TraceCtx<G>,
    ) {
        let first = self.path.role_leaf_token_location::<Start>().unwrap();
        let start_index = *ctx.trav.graph().expect_child_at(first);
        tracing::trace!(
            ?first,
            ?start_index,
            "PostfixCommand: leaf and start_index"
        );
        let prev = TraceRole::<Start>::trace_sub_path(
            ctx,
            &self.path,
            UpKey {
                index: start_index,
                pos: start_index.width().into(),
            },
            self.add_edges,
        );
        tracing::trace!(?prev, "PostfixCommand: trace_sub_path returned");
        let location = self.path.role_root_child_location::<Start>();
        let new = NewTraceEdge::<BottomUp> {
            target: self.root_up_key,
            prev,
            location,
        };
        tracing::trace!(?new, "PostfixCommand: creating NewTraceEdge");
        ctx.cache.add_state(new, self.add_edges);
    }
}
#[derive(Debug)]
pub struct PrefixCommand {
    pub path: IndexEndPath,
    pub add_edges: bool,
    pub end_pos: AtomPosition,
}
impl Traceable for PrefixCommand {
    fn trace<G: HasGraph>(
        self,
        ctx: &mut TraceCtx<G>,
    ) {
        let root_exit = self.path.role_root_child_location::<End>();
        let exit_key = DownKey {
            pos: self.end_pos.into(),
            index: root_exit.parent,
        };
        let target = DownKey {
            index: *ctx.trav.graph().expect_child_at(root_exit),
            pos: exit_key.pos,
        };
        let new = NewTraceEdge::<TopDown> {
            target,
            prev: exit_key,
            location: root_exit,
        };
        ctx.cache.add_state(new, self.add_edges);

        TraceRole::<End>::trace_sub_path(
            ctx,
            &self.path,
            target,
            self.add_edges,
        );
    }
}

#[derive(Debug)]
pub struct RangeCommand {
    pub path: IndexRangePath,
    pub add_edges: bool,
    pub root_pos: UpPosition,
    pub end_pos: AtomPosition,
}
impl Traceable for RangeCommand {
    fn trace<G: HasGraph>(
        self,
        ctx: &mut TraceCtx<G>,
    ) {
        let first = self.path.role_leaf_token_location::<Start>().unwrap();
        let start_index = *ctx.trav.graph().expect_child_at(first);
        let prev = TraceRole::<Start>::trace_sub_path(
            ctx,
            &self.path,
            UpKey {
                index: start_index,
                pos: start_index.width().into(),
            },
            self.add_edges,
        );
        //let location = self.path.role_root_child_location::<Start>();
        //let new = UpEdit {
        //    target: self.root_up_key.clone(),
        //    prev,
        //    location,
        //};
        let root_entry = self.path.role_root_child_location::<Start>();
        //let root_entry_index = *ctx.trav.graph().expect_child_at(&root_entry);
        let root_up_key = UpKey {
            index: root_entry.parent,
            pos: self.root_pos,
        };
        let new = NewTraceEdge::<BottomUp> {
            target: root_up_key,
            prev,
            location: root_entry,
        };
        ctx.cache.add_state(new, self.add_edges);

        let root_exit = self.path.role_root_child_location::<End>();

        let exit_key = DownKey {
            pos: self.end_pos.into(),
            index: root_exit.parent,
        };
        let target = DownKey {
            pos: exit_key.pos,
            index: *ctx.trav.graph().expect_child_at(root_exit),
        };
        let new = NewTraceEdge::<TopDown> {
            target,
            prev: exit_key,
            location: root_exit,
        };
        ctx.cache.add_state(new, self.add_edges);

        TraceRole::<End>::trace_sub_path(
            ctx,
            &self.path,
            target,
            self.add_edges,
        );
    }
}
