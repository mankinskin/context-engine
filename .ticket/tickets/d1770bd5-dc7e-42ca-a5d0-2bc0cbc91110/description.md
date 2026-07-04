# [ticket-store] Exact entity moves from root store to child stores

Generated: 2026-06-28

Selection rule: include only root-store entities with an unambiguous child-owner signal from component and/or title tags.

Post-snapshot placement lock: later observability/logging coordination tickets that span `memory-api`, `log-api`, and `context-stack` stay in the root `context-engine` workspace as the lowest common ancestor. Do not add `73b2cd22`, `84673399`, `bce26d30`, `ff6637f5`, or `1dffcf23` to the child-store move set; instead, keep linking their lower-crate specialized tickets from the parent workspace trackers.

## Move summary
- memory-api: 50 tickets, 15 specs
- memory-viewers: 17 tickets, 2 specs
- viewer-api: 11 tickets, 4 specs
- total: 99 entities

## Move to memory-api

### Tickets
- 1fd0c182-f4b4-486b-b757-fe47e3238e43 | [rule-mcp][rule-http] Workspace-resolution parity — nested-root awareness + pure transport | .ticket/tickets/1fd0c182-f4b4-486b-b757-fe47e3238e43/ticket.toml
- 23f1c81b-3c71-4b4b-9e6f-81ee7c43a30b | [ticket-http] Add no-auto-init E2E for missing .ticket workspace | .ticket/tickets/23f1c81b-3c71-4b4b-9e6f-81ee7c43a30b/ticket.toml
- 07836f41-7fa5-4e41-8411-1c7cf8aeee75 | [ticket-cli] Make get/search/list workspace-aware across nested roots | .ticket/tickets/07836f41-7fa5-4e41-8411-1c7cf8aeee75/ticket.toml
- 0e375356-b74e-48c4-8f1d-77cd28e055bc | [ticket-api][ticket-cli][ticket-mcp][ticket-http] Implement scoped selectors for board and next | .ticket/tickets/0e375356-b74e-48c4-8f1d-77cd28e055bc/ticket.toml
- 10cf2a19-356c-4e69-b0f3-b930d68dc0ce | [ticket-http] Expose workflow trees and actionable ordering metadata | .ticket/tickets/10cf2a19-356c-4e69-b0f3-b930d68dc0ce/ticket.toml
- 15837e16-8755-4eb1-8b36-6c4453899e46 | [ticket-cli][ticket-mcp] Integrate recent-unblock ordering into workflow surfaces | .ticket/tickets/15837e16-8755-4eb1-8b36-6c4453899e46/ticket.toml
- 185419e0-bea4-4c7b-abda-1e92193f32e7 | [ticket-api] Allow bidirectional ticket state transitions by default | memory-api/.ticket/tickets/185419e0-bea4-4c7b-abda-1e92193f32e7/ticket.toml
- 27558fde-37b0-43eb-86c6-cfbe2d99a0b8 | [ticket-mcp][ticket-http] Workspace-resolution parity — nested-root awareness + pure transport (first run) | .ticket/tickets/27558fde-37b0-43eb-86c6-cfbe2d99a0b8/ticket.toml
- 33565741-c3ce-4697-91d3-092a803aaac0 | [ticket-system] Instruction updates: mandatory review gate and diligent state progression | .ticket/tickets/33565741-c3ce-4697-91d3-092a803aaac0/ticket.toml
- 385f2521-b318-403b-a4ea-195a47e5c453 | [ticket-api] Unify multi-step state transitions across update and close flows | .ticket/tickets/385f2521-b318-403b-a4ea-195a47e5c453/ticket.toml
- 39239e48-828a-41d8-a697-9cf02e980da9 | [memory-api] Transport-layer workspace-resolution parity (tracker) | .ticket/tickets/39239e48-828a-41d8-a697-9cf02e980da9/ticket.toml
- 50307cce-5a93-4668-9481-a3af5985ea1b | [ticket-vscode] Cover no-.ticket server launch without implicit init | .ticket/tickets/50307cce-5a93-4668-9481-a3af5985ea1b/ticket.toml
- 51e2210c-829b-4f7f-865e-99d120d8fd7d | [memory-matrix] Add missing-store explicit-init policy coverage | .ticket/tickets/51e2210c-829b-4f7f-865e-99d120d8fd7d/ticket.toml
- 5318aedd-5188-4bfa-ad7d-a6d76e3243f1 | [spec-mcp][spec-http] Workspace-resolution parity — nested-root awareness + pure transport | .ticket/tickets/5318aedd-5188-4bfa-ad7d-a6d76e3243f1/ticket.toml
- 632974d1-ce70-446a-b210-068840041116 | [audit-mcp][audit-http] Workspace-resolution parity — nested-root awareness + pure transport | .ticket/tickets/632974d1-ce70-446a-b210-068840041116/ticket.toml
- 3a5df74c-2192-4187-b048-3f6285f20db4 | [memory-index] Ticket index: one-line entries, state-ordered, collapsible state headers, clickable manifest links | .ticket/tickets/3a5df74c-2192-4187-b048-3f6285f20db4/ticket.toml
- 3afaab37-b228-4051-bc46-618db4e0b82b | [ticket-api][bug] update_ticket resets state to `new` on field/description patch; transition_states silently no-ops | .ticket/tickets/3afaab37-b228-4051-bc46-618db4e0b82b/ticket.toml
- 3b6a2a26-bd4e-44ce-ba15-41594b809b9a | [ticket-api] Derive blocker and unlock trees with frontier leaf metrics | .ticket/tickets/3b6a2a26-bd4e-44ce-ba15-41594b809b9a/ticket.toml
- 3d72029b-cf2d-49bb-9dde-00587304b857 | [ticket-api] Materialize recent-unblock and blocker-progress facts | .ticket/tickets/3d72029b-cf2d-49bb-9dde-00587304b857/ticket.toml
- 416ebd52-447d-44e4-a4ad-23162d37e0b1 | [ticket-http] Return only authoritative resolved hits in workspace-aware search | .ticket/tickets/416ebd52-447d-44e4-a4ad-23162d37e0b1/ticket.toml
- 4a48b371-7dc0-4bf2-badb-747a8f00a0fc | [ticket-api][ticket-cli][ticket-mcp][ticket-http] Unify board-aware next filtering across workflow surfaces | .ticket/tickets/4a48b371-7dc0-4bf2-badb-747a8f00a0fc/ticket.toml
- 5ad5ab28-6c81-4916-9574-d2c470e03a31 | [ticket-api][audit-api] Strengthen canonical ticket health validation | .ticket/tickets/5ad5ab28-6c81-4916-9574-d2c470e03a31/ticket.toml
- 5ad77aba-c7f7-4058-854e-dd0412746c7c | [ticket-mcp][spec-mcp][rule-api] Add self-describing capability catalog and help surfaces | .ticket/tickets/5ad77aba-c7f7-4058-854e-dd0412746c7c/ticket.toml
- 61cb6557-e559-4eae-8e59-ea0d520a3bee | [ticket-cli][ticket-mcp] Add consolidated ticket detail/context read surface | .ticket/tickets/61cb6557-e559-4eae-8e59-ea0d520a3bee/ticket.toml
- 61cbc31f-c66d-46bf-807e-0d4236e04c9e | [ticket-cli] Explain why tickets are absent from next | .ticket/tickets/61cbc31f-c66d-46bf-807e-0d4236e04c9e/ticket.toml
- 6484d4b7-e24b-4c13-999c-d0b00928d97c | [ticket-cli][ticket-http][ticket-mcp] Build larger-integration parity routine for workflow and health surfaces | .ticket/tickets/6484d4b7-e24b-4c13-999c-d0b00928d97c/ticket.toml
- 6848ffa2-4e31-4985-beff-cba01af9b7ca | [ticket-system] Add effort field for token-budget estimates | .ticket/tickets/6848ffa2-4e31-4985-beff-cba01af9b7ca/ticket.toml
- 68a08b34-000b-4585-8354-4b1a26a15f4b | [ticket-cli] Scope-aware board and next for multi-root workspaces | .ticket/tickets/68a08b34-000b-4585-8354-4b1a26a15f4b/ticket.toml
- 68e3c713-3c35-4d7e-af0c-b4a55a3253c0 | [ticket-cli] Fix next --filter matching for prefix and substring queries | .ticket/tickets/68e3c713-3c35-4d7e-af0c-b4a55a3253c0/ticket.toml
- 74fd59ca-8253-4e18-99bd-0b1fa204c6d6 | [ticket-cli] Remove constant blocker-progress field from board show JSON recommendations | .ticket/tickets/74fd59ca-8253-4e18-99bd-0b1fa204c6d6/ticket.toml
- 7f7fe4a8-a1d6-44b4-baf9-9500f6db40a5 | [memory-index] Define domain digest input contract for generated index entries | .ticket/tickets/7f7fe4a8-a1d6-44b4-baf9-9500f6db40a5/ticket.toml
- 86cde60c-49db-4820-a3a9-37c472ca1c2f | [ticket-api] Distinguish deferred and meta work from actionable tickets | .ticket/tickets/86cde60c-49db-4820-a3a9-37c472ca1c2f/ticket.toml
- 8ab31960-f3fa-4a2b-b2ac-f807e1a15fdc | [memory-api][ticket-api][ticket-cli][ticket-mcp][ticket-http] Implement expressive ticket query and ordering | .ticket/tickets/8ab31960-f3fa-4a2b-b2ac-f807e1a15fdc/ticket.toml
- 8bb97b73-9dbc-43ee-9939-46b3ddf2612f | [ticket-cli][ticket-mcp] Explain invalid state transitions with allowed next states | .ticket/tickets/8bb97b73-9dbc-43ee-9939-46b3ddf2612f/ticket.toml
- 8d95b98c-df79-46a7-affa-afa061c0dfff | [ticket-http] Fix child-owned workspace refs for viewer follow-up requests | .ticket/tickets/8d95b98c-df79-46a7-affa-afa061c0dfff/ticket.toml
- 8de93812-3a8c-4937-9f09-05a9a9b86309 | [ticket-cli] Canonicalize board subcommand option naming | .ticket/tickets/8de93812-3a8c-4937-9f09-05a9a9b86309/ticket.toml
- 9491f6b7-c11b-4d94-aed6-f5c6ea004e8a | [session-api] Plan and scaffold Copilot chat-session capture in memory-api | .ticket/tickets/9491f6b7-c11b-4d94-aed6-f5c6ea004e8a/ticket.toml
- 999d9316-fc79-4bb1-b629-7cba52eced31 | [architecture][ticket-api] Adopt neutral shared APIs and alias retirement gate | .ticket/tickets/999d9316-fc79-4bb1-b629-7cba52eced31/ticket.toml
- a9514081-35c2-4162-b62d-3baf4a14ec8b | [spec] Define explicit-init-only memory store contract | .ticket/tickets/a9514081-35c2-4162-b62d-3baf4a14ec8b/ticket.toml
- 9acf1ef1-a7fb-40af-8a7a-4df89ac9a93f | [ticket-api] Allow reverse ticket state transitions through schema | .ticket/tickets/9acf1ef1-a7fb-40af-8a7a-4df89ac9a93f/ticket.toml
- a3cc8e3e-7cb9-413c-a4df-966df77859d5 | [ticket-system] Undo support: revert last update via --undo flag | .ticket/tickets/a3cc8e3e-7cb9-413c-a4df-966df77859d5/ticket.toml
- a4c31280-66d3-44a3-9a5d-13d4fbde1bfe | [ticket-api] Fix health false positives for description and resolved dependencies | .ticket/tickets/a4c31280-66d3-44a3-9a5d-13d4fbde1bfe/ticket.toml
- a98ea0e1-d3e8-47e4-aa28-b6a39296cd45 | [ticket-system] Force sync: reconcile index from disk ticket.toml | .ticket/tickets/a98ea0e1-d3e8-47e4-aa28-b6a39296cd45/ticket.toml
- acefc2ae-e257-4bc8-a4c7-0ec3137e374d | [ticket-api] Validation-aware dependency requirements and health model | .ticket/tickets/acefc2ae-e257-4bc8-a4c7-0ec3137e374d/ticket.toml
- c031aeb0-f374-4d57-9d46-2463dfa8571d | [ticket-api][ticket-cli][ticket-mcp][ticket-http] Define minimal workflow and health core plus adapter responsibilities | .ticket/tickets/c031aeb0-f374-4d57-9d46-2463dfa8571d/ticket.toml
- c5e9bb39-d784-4d0c-8de1-3885013cddce | [memory-index] Ticket store index generator with git hook integration | .ticket/tickets/c5e9bb39-d784-4d0c-8de1-3885013cddce/ticket.toml
- cf4246c3-6539-4f1c-a876-6d34073db7b3 | [ticket-api][ticket-cli][ticket-mcp][ticket-http] Track workflow and health surface convergence | .ticket/tickets/cf4246c3-6539-4f1c-a876-6d34073db7b3/ticket.toml
- d187d817-d3f5-49ca-8925-8d06b5824912 | [ticket-cli][spec-cli][rule-cli][audit-cli] Add TOON input and output support | .ticket/tickets/d187d817-d3f5-49ca-8925-8d06b5824912/ticket.toml
- d241a482-6fc7-468e-b0a3-748cb72d07eb | [ticket-cli][spec-cli] Normalize sibling CLI grammar and JSON envelopes | .ticket/tickets/d241a482-6fc7-468e-b0a3-748cb72d07eb/ticket.toml
- d39e9e08-5104-461b-83ff-bd4361e967d9 | [ticket-cli] Add blockers command and nested tree rendering | .ticket/tickets/d39e9e08-5104-461b-83ff-bd4361e967d9/ticket.toml
- d74412e4-1d0e-4679-8725-e5da6f266fe9 | [ticket-api][ticket-cli] Blueprint blocker trees and recently-unblocked workflow ordering | .ticket/tickets/d74412e4-1d0e-4679-8725-e5da6f266fe9/ticket.toml
- dd2947da-d4d2-4c8a-9a9a-3633060ff4c5 | [ticket-api] Reconcile aggregate scan, prune, and search visibility | .ticket/tickets/dd2947da-d4d2-4c8a-9a9a-3633060ff4c5/ticket.toml
- def7fa82-6f4a-4354-b52d-ae7ea9623648 | [ticket-cli][ticket-mcp] Make stale board entries directly check-outable | .ticket/tickets/def7fa82-6f4a-4354-b52d-ae7ea9623648/ticket.toml
- e3961a54-ea4c-4ce6-aee9-da67a15bf2c7 | [memory-api] Path normalization kernel design + UNC/verbatim regression guard tests | memory-api/.ticket/tickets/e3961a54-ea4c-4ce6-aee9-da67a15bf2c7/ticket.toml
- e6bdafbe-3538-47a3-8837-1f8e74fb13e8 | [memory-api] Track explicit-init-only store creation validation | .ticket/tickets/e6bdafbe-3538-47a3-8837-1f8e74fb13e8/ticket.toml
- e83264db-e634-4c7c-811d-4413a1e3416a | [ticket-vscode] Prevent aborted list tickets request after server start | memory-api/.ticket/tickets/e83264db-e634-4c7c-811d-4413a1e3416a/ticket.toml
- e8e3ef17-313f-4cb7-aa9c-6447a18d36a3 | [memory-api] Implement path normalization kernel and migrate CLI/MCP/HTTP path surfaces | .ticket/tickets/e8e3ef17-313f-4cb7-aa9c-6447a18d36a3/ticket.toml
- ef0ebf38-7f55-4bd7-bf0c-0b416650ee0b | [memory-api][ticket-cli][spec-cli][rule-cli] Unify child-workspace resolution across CLI tools | .ticket/tickets/ef0ebf38-7f55-4bd7-bf0c-0b416650ee0b/ticket.toml
- cc78d33d-1744-4945-bb77-f0fd1142568e | [memory-matrix] Subprocess failure bundle capture for transport cells | .ticket/tickets/cc78d33d-1744-4945-bb77-f0fd1142568e/ticket.toml
- f2285a55-91d1-48ad-9af7-e8c55ce9bd4d | [spec] spec-api <-> ticket-api: link component entities and detect drift | .ticket/tickets/f2285a55-91d1-48ad-9af7-e8c55ce9bd4d/ticket.toml
- f3305925-7217-4ff3-8c4e-820ebc1e6de5 | [ticket-cli] Graph rendering and closure-aware dependency display | .ticket/tickets/f3305925-7217-4ff3-8c4e-820ebc1e6de5/ticket.toml
- fcf9eb04-394e-4b1b-acf2-4da54f3d3f6c | [ticket-http] Remove special default workspace naming and replace opaque server errors | .ticket/tickets/fcf9eb04-394e-4b1b-acf2-4da54f3d3f6c/ticket.toml

### Specs
- 1d62442b-61dc-4eeb-9b7c-e933f84470f2 | ticket-api state transition path unification | .spec/specs/1d62442b-61dc-4eeb-9b7c-e933f84470f2/spec.toml
- 42e8d710-2199-4178-9ab3-dea8d61bfc4a | ticket-cli: graph rendering and closure-aware dependency display | .spec/specs/42e8d710-2199-4178-9ab3-dea8d61bfc4a/spec.toml
- 449fe68a-541c-4804-bbfd-476af783f80c | Domain digest input contract for generated index entries | .spec/specs/449fe68a-541c-4804-bbfd-476af783f80c/spec.toml
- 4f7d84d0-9876-43d5-9dd6-90a7f3ebc56c | memory-api README schema adoption | .spec/specs/4f7d84d0-9876-43d5-9dd6-90a7f3ebc56c/spec.toml
- 53c70cae-731b-41b5-bd1a-1de9a98eb36f | Git hook automation for store-index regeneration | .spec/specs/53c70cae-731b-41b5-bd1a-1de9a98eb36f/spec.toml
- 6571abcf-b1b9-4259-b81c-78783e227467 | Cross-store workspace interaction architecture | .spec/specs/6571abcf-b1b9-4259-b81c-78783e227467/spec.toml
- 6e63979a-f29b-4c6f-a4b7-5264fd9c29d4 | Add TOON format support across the memory-api CLI suite | .spec/specs/6e63979a-f29b-4c6f-a4b7-5264fd9c29d4/spec.toml
- 9074b2ef-c8fe-4bb0-a987-87063078c1ff | Workflow and health parity for ticket interfaces | .spec/specs/9074b2ef-c8fe-4bb0-a987-87063078c1ff/spec.toml
- 9109f12a-cc02-47ae-948f-98008b6c167d | Shared rendering pipeline integration for generated store indexes | .spec/specs/9109f12a-cc02-47ae-948f-98008b6c167d/spec.toml
- a595eb0c-f9f1-4e29-a425-120df5334f7d | Scoped selector contract for board and next workflow discovery | .spec/specs/a595eb0c-f9f1-4e29-a425-120df5334f7d/spec.toml
- b4d038e0-ade9-459b-8ba3-92fd81d80e6a | ticket-api: validation-aware dependency requirements and health model | .spec/specs/b4d038e0-ade9-459b-8ba3-92fd81d80e6a/spec.toml
- bf217ce5-8890-4749-9a2d-deffb6d0f4dd | Domain-owned thin generator architecture for store indexes | .spec/specs/bf217ce5-8890-4749-9a2d-deffb6d0f4dd/spec.toml
- c4f7b0ae-9690-4cc2-b25f-c8ec49a504d0 | peek-cli consumption and level-of-detail validation for generated indexes | .spec/specs/c4f7b0ae-9690-4cc2-b25f-c8ec49a504d0/spec.toml
- c598ddb2-4d3a-4b81-90ea-8b25a54b8469 | Benchmarking and profiling plan for store-index generation | .spec/specs/c598ddb2-4d3a-4b81-90ea-8b25a54b8469/spec.toml
- fa5265cc-e82f-4bbe-b1b5-dad7e6672d06 | Tracker-improvement effort field | .spec/specs/fa5265cc-e82f-4bbe-b1b5-dad7e6672d06/spec.toml

## Move to memory-viewers

### Tickets
- 0515479f-a5c2-47c6-b8c2-3961dfa6dcf7 | Plan: MCP crate docs — extend MCP server for crate API documentation | .ticket/tickets/0515479f-a5c2-47c6-b8c2-3961dfa6dcf7/ticket.toml
- 06a194e8-d883-45a4-9693-6a4b9123ec5a | Port: doc-viewer Leptos frontend | .ticket/tickets/06a194e8-d883-45a4-9693-6a4b9123ec5a/ticket.toml
- 06e00e0b-42ce-4a74-aea2-392302dd68f7 | [log-viewer] Integrate GraphOpEvent replay with 3D graph visualization | .ticket/tickets/06e00e0b-42ce-4a74-aea2-392302dd68f7/ticket.toml
- 0866e27f-ae67-4eb0-9199-00650317e7c3 | [ticket-viewer] Fix asset follow-up file selection and owning-workspace fetch | .ticket/tickets/0866e27f-ae67-4eb0-9199-00650317e7c3/ticket.toml
- 14df656e-cef2-470e-9530-ef760b6c462c | [ticket-viewer][ticket-vscode] Surface the next-work workflow in frontends | .ticket/tickets/14df656e-cef2-470e-9530-ef760b6c462c/ticket.toml
- 178b4091-53c9-45ae-b975-890a23b5f25d | [ticket-viewer] Normalize release E2E suite to workspace-aware assumptions | .ticket/tickets/178b4091-53c9-45ae-b975-890a23b5f25d/ticket.toml
- 1e119a99-375a-479b-80ce-98cb63d92436 | [ticket-viewer] Update graph SSE subscription to listen for ticket.upsert events | .ticket/tickets/1e119a99-375a-479b-80ce-98cb63d92436/ticket.toml
- 4a9b49fd-58e0-404c-a120-47ef277dcf9f | [ticket-viewer] Keep filtered explorer state authoritative under live refresh | .ticket/tickets/4a9b49fd-58e0-404c-a120-47ef277dcf9f/ticket.toml
- 53a6d689-7d31-40ce-b807-4314285b4bfd | [ticket-viewer] Add mixed-workspace endpoint ownership matrix regression tests | .ticket/tickets/53a6d689-7d31-40ce-b807-4314285b4bfd/ticket.toml
- 5d5c7bbb-fac2-49ba-aa19-37bf6e2aac34 | [ticket-viewer] Add cache invalidation for graph layout on ticket updates | .ticket/tickets/5d5c7bbb-fac2-49ba-aa19-37bf6e2aac34/ticket.toml
- 75fde4f5-ca1c-4bcf-9530-36a3da59a8f1 | [ticket-viewer] Targeted node update on ticket.upsert + fix invalidate_workspace version no-op | .ticket/tickets/75fde4f5-ca1c-4bcf-9530-36a3da59a8f1/ticket.toml
- 884ad295-9b75-4ad6-938d-6ab73c8efa6b | [ticket-viewer] Avoid click panic when backend is offline | .ticket/tickets/884ad295-9b75-4ad6-938d-6ab73c8efa6b/ticket.toml
- 88f87410-e0fa-4196-a461-805050670d08 | [spec-viewer] Integrate graph improvements (selection, rendering tiers, panel framing, 2D mode) | .ticket/tickets/88f87410-e0fa-4196-a461-805050670d08/ticket.toml
- a08a6153-126e-4e4a-8333-0e651817d8ea | [ticket-viewer] Add workflow ordering and blocker-tree surfaces | .ticket/tickets/a08a6153-126e-4e4a-8333-0e651817d8ea/ticket.toml
- bf295665-a075-4cfb-9a86-f54e96918695 | [log-viewer] Integrate graph improvements (selection, rendering tiers, panel framing, 2D mode) | .ticket/tickets/bf295665-a075-4cfb-9a86-f54e96918695/ticket.toml
- c33419c2-3fff-4ce2-9b53-8882d6918e53 | [ticket-viewer] Tracker: complete mixed-workspace regression coverage | .ticket/tickets/c33419c2-3fff-4ce2-9b53-8882d6918e53/ticket.toml
- fe7effea-6b70-4b16-8c00-bc7e910a0fde | [ticket-viewer] Test graph reactivity with ticket state updates | .ticket/tickets/fe7effea-6b70-4b16-8c00-bc7e910a0fde/ticket.toml

### Specs
- 98b4f75d-3628-470d-a5cc-c91b6cc9811a | ticket-viewer: graph focus, property-based rendering, and 2D presentation mode | .spec/specs/98b4f75d-3628-470d-a5cc-c91b6cc9811a/spec.toml
- cfbb4500-4632-4a95-96a1-838dc4dccfd5 | memory-viewers aggregate README schema adoption | .spec/specs/cfbb4500-4632-4a95-96a1-838dc4dccfd5/spec.toml

## Move to viewer-api

### Tickets
- 08c86dbd-72b8-446b-a930-30ef3352d604 | [viewer-api] Create comprehensive E2E test suite for graph improvements | .ticket/tickets/08c86dbd-72b8-446b-a930-30ef3352d604/ticket.toml
- 254ac30d-26c0-4bfe-8a66-b10ab9e4843a | [viewer-api] Generalize graph improvements to spec-viewer and log-viewer | .ticket/tickets/254ac30d-26c0-4bfe-8a66-b10ab9e4843a/ticket.toml
- 6dc44fbb-4480-4bad-853c-79b8171dd73b | [viewer-api] Anchor SVG edge endpoints to world_to_screen instead of getBoundingClientRect | .ticket/tickets/6dc44fbb-4480-4bad-853c-79b8171dd73b/ticket.toml
- 7db89f25-9395-45b3-a35d-8c5c219067f8 | [viewer-api] Eliminate per-frame DOM reflow: analytic node rects + skip unchanged LOD writes | .ticket/tickets/7db89f25-9395-45b3-a35d-8c5c219067f8/ticket.toml
- 8d6895a5-dce8-47c1-98ce-212fd0ae2e08 | [viewer-api][audit] Fix viewer-api-dioxus compile failure and restore llvm-cov coverage collection | .ticket/tickets/8d6895a5-dce8-47c1-98ce-212fd0ae2e08/ticket.toml
- 97a9ed0b-4442-4514-8c67-09e3393f79a7 | [viewer-api] render_frame: compute VP once and collect node rects once per frame | .ticket/tickets/97a9ed0b-4442-4514-8c67-09e3393f79a7/ticket.toml
- c6bf5b7a-f822-44bb-8d2b-86c966031ca6 | [viewer-api] Enlarge Graph3D directed edge arrow tips | .ticket/tickets/c6bf5b7a-f822-44bb-8d2b-86c966031ca6/ticket.toml
- c79e2630-3d49-454b-998f-fb52c24303f4 | [viewer-api] Default visual validation to external fullscreen Chromium | .ticket/tickets/c79e2630-3d49-454b-998f-fb52c24303f4/ticket.toml
- d7d582c2-5734-4818-acf1-382f67bfdb89 | [viewer-api] Adopt shared README schema and parent-linked child READMEs | .ticket/tickets/d7d582c2-5734-4818-acf1-382f67bfdb89/ticket.toml
- e8d9bfcd-d729-43a6-8efa-4554af609d0c | [viewer-api] Update Graph3D component documentation and examples | .ticket/tickets/e8d9bfcd-d729-43a6-8efa-4554af609d0c/ticket.toml
- f685dca9-1a67-4b0b-bc14-a88d6ef1226d | [viewer-api] Guard sync_render_state against resetting layout during active interaction | .ticket/tickets/f685dca9-1a67-4b0b-bc14-a88d6ef1226d/ticket.toml

### Specs
- 5f9a1652-943f-4d98-8812-a4f7ca1d5e61 | viewer-api README schema adoption | .spec/specs/5f9a1652-943f-4d98-8812-a4f7ca1d5e61/spec.toml
- 88c88341-5f9c-4e59-87c7-9176e4afc26a | temp | .spec/specs/88c88341-5f9c-4e59-87c7-9176e4afc26a/spec.toml
- bca2c4a5-b39e-4896-91f2-8453a1f4ff60 | Generalize graph improvements across all memory-viewers | .spec/specs/bca2c4a5-b39e-4896-91f2-8453a1f4ff60/spec.toml
- d8c6114b-1188-4bc4-a8fb-dbfd3b1816ee | probe | .spec/specs/d8c6114b-1188-4bc4-a8fb-dbfd3b1816ee/spec.toml
