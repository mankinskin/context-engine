use std::{
    borrow::Borrow,
    num::NonZeroUsize,
};

use derive_more::{
    Deref,
    DerefMut,
};
use linked_hash_map::LinkedHashMap;

use crate::{
    interval::{
        IntervalGraph,
        partition::{
            Infix,
            Postfix,
            Prefix,
            info::{
                InfoPartition,
                borders::PartitionBorders,
                range::role::{
                    In,
                    Post,
                    Pre,
                },
            },
        },
    },
    join::{
        context::{
            frontier::FrontierSplitIterator,
            node::merge::NodeMergeCtx,
            pattern::PatternJoinCtx,
        },
        joined::partition::JoinedPartition,
        partition::{
            Join,
            JoinPartition,
            info::JoinPartitionInfo,
        },
    },
    split::{
        Split,
        SplitMap,
        cache::{
            position::PosKey,
            vertex::SplitVertexCache,
        },
        position_splits,
        vertex::{
            PosSplitCtx,
            TokenTracePositions,
            VertexSplits,
            node::{
                AsNodeTraceCtx,
                NodeTraceCtx,
            },
            output::RootMode,
            pattern::{
                GetPatternCtx,
                GetPatternTraceCtx,
                PatternTraceCtx,
            },
        },
    },
};
use context_trace::*;

#[derive(Debug)]
pub struct LockedFrontierCtx<'a> {
    pub trav: <HypergraphRef as HasGraphMut>::GuardMut<'a>,
    pub interval: &'a IntervalGraph,
    pub splits: &'a SplitMap,
}
impl<'a> LockedFrontierCtx<'a> {
    pub fn new(ctx: &'a mut FrontierSplitIterator) -> Self {
        Self {
            trav: ctx.trav.graph_mut(),
            interval: &ctx.frontier.interval,
            splits: &ctx.splits,
        }
    }
}
#[derive(Debug, Deref, DerefMut)]
pub struct NodeJoinCtx<'a> {
    #[deref]
    #[deref_mut]
    pub ctx: LockedFrontierCtx<'a>,
    pub index: Token,
}

impl<'a> NodeJoinCtx<'a> {
    pub fn new(
        index: Token,
        ctx: &'a mut FrontierSplitIterator,
    ) -> Self {
        NodeJoinCtx {
            index,
            ctx: LockedFrontierCtx::new(ctx),
        }
    }
}
impl<'a> AsNodeTraceCtx for NodeJoinCtx<'a> {
    fn as_trace_context<'t>(&'t self) -> NodeTraceCtx<'t>
    where
        Self: 't,
        'a: 't,
    {
        NodeTraceCtx::new(self.patterns(), self.borrow().index)
    }
}
impl GetPatternTraceCtx for NodeJoinCtx<'_> {
    fn get_pattern_trace_context<'c>(
        &'c self,
        pattern_id: &PatternId,
    ) -> PatternTraceCtx<'c>
    where
        Self: 'c,
    {
        PatternTraceCtx::new(
            self.index.to_pattern_location(*pattern_id),
            self.as_trace_context().patterns.get(pattern_id).unwrap(),
        )
    }
}
impl GetPatternCtx for NodeJoinCtx<'_> {
    type PatternCtx<'c>
        = PatternJoinCtx<'c>
    where
        Self: 'c;

    fn get_pattern_context<'c>(
        &'c self,
        pattern_id: &PatternId,
    ) -> Self::PatternCtx<'c>
    where
        Self: 'c,
    {
        let ctx = self.get_pattern_trace_context(pattern_id);
        //let pos_splits = self.vertex_cache().pos_splits();
        PatternJoinCtx {
            ctx,
            splits: self.splits, //pos_splits
                                 //    .iter()
                                 //    .map(|pos| PosSplitCtx::from(pos).fetch_split(&self.ctx.interval))
                                 //    .collect(),
        }
    }
}
impl NodeJoinCtx<'_> {
    pub fn patterns(&self) -> &ChildPatterns {
        self.ctx.trav.expect_child_patterns(self.index)
    }
}

impl NodeJoinCtx<'_> {
    pub fn vertex_cache(&self) -> &SplitVertexCache {
        self.interval.cache.get(&self.index.vertex_index()).unwrap()
    }
    pub fn join_partitions(&mut self) -> LinkedHashMap<PosKey, Split> {
        let partitions = self.insert_partitions();
        assert_eq!(
            *self.index.width(),
            partitions.iter().map(|t| *t.width()).sum::<usize>()
        );
        let pos_splits = self.vertex_cache();
        assert_eq!(partitions.len(), pos_splits.len() + 1,);
        NodeMergeCtx::new(self).merge_node(&partitions)
    }
    pub fn insert_partitions(&mut self) -> Vec<Token> {
        let pos_splits = self.vertex_cache().clone();
        let len = pos_splits.len();
        assert!(len > 0);
        let mut iter = pos_splits.iter().map(|(&pos, splits)| VertexSplits {
            pos,
            splits: (splits.borrow() as &TokenTracePositions).clone(),
        });

        let mut prev = iter.next().unwrap();
        let mut parts = Vec::with_capacity(1 + len);
        parts.push(Prefix::new(&prev).join_partition(self).into());
        for offset in iter {
            parts.push(Infix::new(&prev, &offset).join_partition(self).into());
            prev = offset;
        }
        parts.push(Postfix::new(prev).join_partition(self).into());
        //println!("{:#?}", parts);
        parts
    }
    pub fn join_root_partitions(&mut self) -> Token {
        let root_mode = self.interval.cache.root_mode;
        let index = self.index;
        let offsets = self.vertex_cache().clone();
        let mut offset_iter = offsets.iter().map(PosSplitCtx::from);
        let offset = offset_iter.next().unwrap();

        tracing::debug!(
            root_token = ?index,
            root_mode = ?root_mode,
            split_offset = ?offset.pos,
            "join_root_partitions - processing root vertex"
        );

        match root_mode {
            RootMode::Prefix => Prefix::new(offset)
                .join_partition(self)
                .inspect(|part| {
                    tracing::debug!(
                        prefix_index = ?part.index,
                        perfect = ?part.perfect,
                        delta = ?part.delta,
                        "join_root_partitions - Prefix mode: partition created"
                    );
                    if part.perfect.is_none() {
                        // Get the pattern entry info from delta
                        if let Some((&pattern_id, &entry_index)) = part.delta.iter().next() {
                            // Get the prefix partition's pattern
                            let prefix_pattern = self.ctx.trav.expect_child_patterns(part.index);
                            
                            // Get the first token from the prefix pattern (e.g., he from [he, l])
                            let first_token = if let Some((_pid, pattern)) = prefix_pattern.iter().next() {
                                pattern[0]
                            } else {
                                part.index
                            };
                            
                            // Only create wrapper if the prefix contains joined elements
                            if first_token != part.index {
                                // Calculate the entry index where prefix ends
                                // This is trickier for prefix - we need to find the last entry covered by the prefix
                                // For now, let's find the entry after the prefix end
                                let patterns = self.patterns();
                                let pattern = patterns.get(&pattern_id).unwrap();
                                
                                // The entry right after where the prefix ends
                                let next_entry_index = entry_index + 1;
                                if next_entry_index < pattern.len() {
                                    let next_entry_token = *self.ctx.trav.expect_child_at(
                                        index.to_child_location(SubLocation::new(pattern_id, next_entry_index))
                                    );
                                    
                                    // Get the last token from the prefix pattern (e.g., l from [he, l])
                                    let last_prefix_token = if let Some((_pid, pattern)) = prefix_pattern.iter().next() {
                                        pattern[pattern.len() - 1]
                                    } else {
                                        part.index
                                    };
                                    
                                    // Get the complement token from the next entry
                                    let next_patterns = self.ctx.trav.expect_child_patterns(next_entry_token);
                                    let complement_token = if let Some((_pid, pattern)) = next_patterns.iter().next() {
                                        pattern[pattern.len() - 1]  // Last child
                                    } else {
                                        next_entry_token  // fallback
                                    };
                                    
                                    tracing::info!(
                                        root = ?index,
                                        entry_index = entry_index,
                                        first_token = ?first_token,
                                        next_entry_token = ?next_entry_token,
                                        complement_token = ?complement_token,
                                        "join_root_partitions - Prefix mode: creating wrapper vertex from entry range"
                                    );
                                    
                                    // Create new wrapper vertex with two patterns:
                                    // 1. [he, ld] - using first token and next entry
                                    // 2. [hel, d] - using prefix and complement
                                    let wrapper = self.ctx.trav.insert_patterns(vec![
                                        Pattern::from(vec![first_token, next_entry_token]),
                                        Pattern::from(vec![part.index, complement_token]),
                                    ]);
                                    
                                    tracing::info!(
                                        wrapper = ?wrapper,
                                        "join_root_partitions - Prefix mode: wrapper vertex created with both patterns"
                                    );
                                    
                                    // Replace the pattern entries with the wrapper
                                    let loc = index.to_pattern_location(pattern_id);
                                    self.ctx.trav.replace_in_pattern(loc, 0..=next_entry_index, wrapper);
                                    
                                    tracing::info!(
                                        "join_root_partitions - Prefix mode: replaced entries in root pattern"
                                    );
                                } else {
                                    tracing::debug!(
                                        "join_root_partitions - Prefix mode: no next entry, skipping wrapper"
                                    );
                                }
                            } else {
                                tracing::debug!(
                                    "join_root_partitions - Prefix mode: prefix has no joined elements, skipping wrapper"
                                );
                            }
                        } else {
                            tracing::warn!(
                                "join_root_partitions - Prefix mode: no delta info, skipping wrapper"
                            );
                        }
                    }
                })
                .map(|part| part.index),
            RootMode::Postfix => Postfix::new(offset)
                .join_partition(self)
                .inspect(|part| {
                    tracing::debug!(
                        postfix_index = ?part.index,
                        perfect = ?part.perfect,
                        delta = ?part.delta,
                        "join_root_partitions - Postfix mode: partition created"
                    );
                    if part.perfect.is_none() {
                        // Get the pattern entry info from delta
                        // The delta tells us which pattern entry the postfix starts from
                        if let Some((&pattern_id, &entry_index)) = part.delta.iter().next() {
                            // Get the postfix partition's pattern
                            let postfix_pattern = self.ctx.trav.expect_child_patterns(part.index);
                            
                            // Get the last token from the postfix pattern (e.g., cd from [b, cd])
                            let last_token = if let Some((_pid, pattern)) = postfix_pattern.iter().next() {
                                pattern[pattern.len() - 1]
                            } else {
                                part.index // fallback to whole partition
                            };
                            
                            // Only create wrapper if the postfix contains joined elements
                            // (i.e., last_token is different from part.index)
                            if last_token != part.index {
                                // Get the original child token at the entry where postfix starts
                                // This gives us the full token (e.g., ab from entry 1)
                                let entry_token = *self.ctx.trav.expect_child_at(
                                    index.to_child_location(SubLocation::new(pattern_id, entry_index))
                                );
                                
                                // Get the first token from the postfix pattern (e.g., b from [b, cd])
                                let _first_postfix_token = if let Some((_pid, pattern)) = postfix_pattern.iter().next() {
                                    pattern[0]
                                } else {
                                    part.index
                                };
                                
                                // Get the complement token (e.g., a from ab, given b)
                                // entry_token should have pattern [a, b], so we get the first child
                                let entry_patterns = self.ctx.trav.expect_child_patterns(entry_token);
                                let complement_token = if let Some((_pid, pattern)) = entry_patterns.iter().next() {
                                    pattern[0]  // First child is 'a'
                                } else {
                                    entry_token  // fallback
                                };
                                
                                tracing::info!(
                                    root = ?index,
                                    entry_index = entry_index,
                                    entry_token = ?entry_token,
                                    last_token = ?last_token,
                                    complement_token = ?complement_token,
                                    "join_root_partitions - Postfix mode: creating wrapper vertex from entry range"
                                );
                                
                                // Create new wrapper vertex with two patterns:
                                // 1. [ab, cd] - using full entry token
                                // 2. [a, bcd] - using complement and postfix
                                let wrapper = self.ctx.trav.insert_patterns(vec![
                                    Pattern::from(vec![entry_token, last_token]),
                                    Pattern::from(vec![complement_token, part.index]),
                                ]);
                                
                                tracing::info!(
                                    wrapper = ?wrapper,
                                    "join_root_partitions - Postfix mode: wrapper vertex created with both patterns"
                                );
                                
                                // Replace the pattern entries with the wrapper
                                // The wrapper should replace entries [entry_index..] in the root pattern
                                let loc = index.to_pattern_location(pattern_id);
                                self.ctx.trav.replace_in_pattern(loc, entry_index.., wrapper);
                                
                                tracing::info!(
                                    "join_root_partitions - Postfix mode: replaced entries in root pattern"
                                );
                            } else {
                                tracing::debug!(
                                    "join_root_partitions - Postfix mode: postfix has no joined elements, skipping wrapper"
                                );
                            }
                        } else {
                            tracing::warn!(
                                "join_root_partitions - Postfix mode: no delta info, skipping wrapper"
                            );
                        }
                    } else {
                        tracing::debug!(
                            "join_root_partitions - Postfix mode: perfect match, no wrapper needed"
                        );
                    }
                })
                .map(|part| part.index),
            RootMode::Infix => {
                let loffset = offset;
                let roffset = offset_iter.next().unwrap();
                Infix::new(loffset, roffset)
                    .join_partition(self)
                    .map(|part| {
                        self.join_incomplete_infix(
                            part, loffset, roffset, index,
                        )
                    })
            },
        }
        .unwrap_or_else(|c| c)
    }

    pub fn join_incomplete_infix<'c>(
        &mut self,
        part: JoinedPartition<In<Join>>,
        loffset: PosSplitCtx<'c>,
        roffset: PosSplitCtx<'c>,
        index: Token,
    ) -> Token {
        let loffset = (*loffset.pos, loffset.split.clone());
        let roffset = (*roffset.pos, roffset.split.clone() - part.delta);

        if (None, None) == part.perfect.into() {
            // no perfect border
            //        [               ]
            // |     |      |      |     |   |
            let (offset, pre) = match Prefix::new(loffset).join_partition(self)
            {
                Ok(part) =>
                    ((roffset.0, (roffset.1.clone() - part.delta)), part.index),
                Err(ch) => (roffset, ch),
            };
            let post: Token = Postfix::new(offset).join_partition(self).into();
            self.trav.add_pattern_with_update(
                index,
                Pattern::from(vec![pre, part.index, post]),
            );
        } else if part.perfect.0 == part.perfect.1 {
            // perfect borders in same pattern
            //       [               ]
            // |     |       |       |      |
            let (ll, rl) = (part.perfect.0.unwrap(), part.perfect.1.unwrap());
            let lpos = loffset.1.pattern_splits[&ll].sub_index();
            let rpos = roffset.1.pattern_splits[&rl].sub_index();
            self.ctx.trav.replace_in_pattern(
                index.to_pattern_location(ll),
                lpos..rpos,
                vec![part.index],
            )
        } else {
            // one or both are perfect in different patterns
            let loffset = (loffset.0, &loffset.1);
            let roffset = (roffset.0, &roffset.1);
            if let Some(rp) = part.perfect.1 {
                //           [              ]
                // |     |       |     |    |     |

                let (wrap_offset, li) = {
                    let pre_brds: PartitionBorders<Pre<Join>> =
                        Prefix::new(loffset).partition_borders(self);
                    let rp_brd = &pre_brds.borders[&rp];
                    let li = rp_brd.sub_index;
                    let lc = self.trav.expect_child_at(
                        self.index.to_child_location(SubLocation::new(rp, li)),
                    );
                    let outer_offset = NonZeroUsize::new(
                        rp_brd.start_offset.unwrap().get() + *lc.width(),
                    )
                    .unwrap();
                    (position_splits(self.patterns(), outer_offset), li)
                };
                let ri = roffset.1.pattern_splits[&rp].sub_index();

                //prev_offset.1 = prev_offset.1 - pre.delta;

                let info = Infix::new(&wrap_offset, roffset)
                    .info_partition(self)
                    .unwrap();
                let wrap_patterns =
                    JoinPartitionInfo::from(info).into_joined_patterns(self);
                let wrap_pre = match Infix::new(wrap_offset, loffset)
                    .join_partition(self)
                {
                    Ok(p) => p.index,
                    Err(c) => c,
                };
                let wrapper = self.trav.insert_patterns(
                    std::iter::once(Pattern::from(vec![wrap_pre, part.index]))
                        .chain(wrap_patterns.patterns),
                );
                let loc = index.to_pattern_location(rp);
                self.trav.replace_in_pattern(loc, li..ri, vec![wrapper]);

                //let patterns = wrap_patterns.patterns.clone();
                //offset.1 = offset.1 - wrap_patterns.delta;
                //let wrapper = ctx.graph.insert_patterns(
                //    std::iter::once(vec![pre.index, part.index])
                //        .chain(patterns),
                //);

                //let ri = offset.1.pattern_splits[&rp].sub_index;
                //let loc = index.to_pattern_location(rp);
                //ctx.graph.replace_in_pattern(
                //    loc,
                //    0..ri,
                //    [wrapper],
                //);
            }
            if let Some(lp) = part.perfect.0 {
                //       [                 ]
                // |     |       |      |      |   |

                // find wrapping offsets
                let (wrap_offset, ri) = {
                    let post_brds: PartitionBorders<Post<Join>> =
                        Postfix::new(roffset).partition_borders(self);
                    let lp_brd = &post_brds.borders[&lp];
                    let ri = lp_brd.sub_index;
                    let rc = self.trav.expect_child_at(
                        self.index.to_child_location(SubLocation::new(lp, ri)),
                    );
                    let outer_offset = NonZeroUsize::new(
                        lp_brd.start_offset.unwrap().get() + *rc.width(),
                    )
                    .unwrap();
                    (position_splits(self.patterns(), outer_offset), ri)
                };

                let li = loffset.1.pattern_splits[&lp].sub_index();

                let info = Infix::new(loffset, &wrap_offset)
                    .info_partition(self)
                    .unwrap();
                let wrap_patterns =
                    JoinPartitionInfo::from(info).into_joined_patterns(self);
                let wrap_post = match Infix::new(roffset, wrap_offset)
                    .join_partition(self)
                {
                    Ok(p) => p.index,
                    Err(c) => c,
                };

                let wrapper = self.trav.insert_patterns(
                    std::iter::once(Pattern::from(vec![part.index, wrap_post]))
                        .chain(wrap_patterns.patterns),
                );
                let loc = index.to_pattern_location(lp);
                self.trav.replace_in_pattern(loc, li..ri + 1, vec![wrapper]);
            }
        }
        part.index
    }
}
