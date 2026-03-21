# Design: ticket HTTP API + SSE event contract + auth model

## Objective
Define the contract for ticket server APIs and live graph updates with required auth.

## Decisions already fixed
- Auth required in v1.
- SSE as transport.
- No endpoint versioning in v1.
- Workspace switch support is required.

## Endpoint proposal
- `GET /api/workspaces`
- `GET /api/tickets?workspace=<name>&state=<optional>&query=<optional>&limit=<n>&cursor=<token>`
- `GET /api/tickets/{id}?workspace=<name>`
- `GET /api/edges?workspace=<name>&kind=depends_on|blocks`
- `GET /api/graph/subgraph?workspace=<name>&root=<id>&depth=<n>&direction=in|out|both&limit=<n>&cursor=<token>`
- `GET /api/stream?workspace=<name>` (SSE)
- `GET /healthz`

## Auth contract
- Header: `Authorization: Bearer <token>`
- Token source: env/config (see token lifecycle ticket).
- Failure response: 401/403 with structured error payload.

## Error envelope
- JSON shape:
  - `code` string
  - `message` string
  - `request_id` string
  - `details` object (optional)

## SSE event proposal
- `ticket.upsert`
- `ticket.delete`
- `edge.upsert`
- `edge.delete`
- `ticket.conflict`
- `snapshot.ready`

## Open points handed to child design tickets
- Final event schema freeze is covered by ticket `09a32876-665c-476c-9587-8dcb3acd6e6a`.
- Hook/fallback semantics are covered by ticket `24aa7e5e-1d62-4f35-a4f7-b056a0b8abce`.
- Subgraph pagination semantics are covered by ticket `e79fdc1f-2bfb-410f-931c-dbb744cd209e`.

## Checklist
- [ ] Endpoint list approved
- [ ] Auth envelope approved
- [ ] SSE event names approved
- [ ] Error/status code mapping approved
- [ ] Contract handoff note posted to impl tickets
