# [AOH][Research] GitHub API — PR Lifecycle, Branch Management, Code Review

## Objective

Map the GitHub API surface needed to automate the full PR lifecycle from an agent branch through review, user approval/rejection, merge, and branch cleanup. Identify Rust crates and evaluate API call sequences.

## Research Questions

1. What is the minimal API call sequence to: create branch → create PR → poll review state → merge → delete branch?
2. How do we attach structured metadata (agent ID, ticket ID, test results) to a PR in a way the reviewer can read easily?
3. How do we receive change-request comments from the reviewer and route them back to the agent session?
4. Which Rust crates provide typed GitHub API clients?
5. How do branch protection rules interact with bot-created PRs?
6. What webhook events do we need to subscribe to for async review notifications?

## GitHub API Surface

### Authentication
- Personal Access Token (PAT) — simplest
- GitHub App (recommended for production) — per-installation token, scoped permissions
- Fine-grained PAT — scoped to specific repos
- **Research**: which auth model works best for per-agent sessions?

### Branch Operations
- `POST /repos/{owner}/{repo}/git/refs` — create branch
- `DELETE /repos/{owner}/{repo}/git/refs/{ref}` — delete branch after merge
- Branch protection: `GET /repos/{owner}/{repo}/branches/{branch}/protection`
- Worktree branch naming convention: `aoh/{agent-id}/{ticket-slug}` (ADR-11)

### Pull Request Lifecycle
- `POST /repos/{owner}/{repo}/pulls` — create PR with title, body, head, base
- `GET /repos/{owner}/{repo}/pulls/{pull_number}` — poll state
- `PUT /repos/{owner}/{repo}/pulls/{pull_number}/merge` — merge (squash/merge/rebase)
- PR body template: ticket link, agent ID, test results summary, evidence refs
- Draft PR: `draft: true` — create before agent is done to show progress

### Review and Comments
- `GET /repos/{owner}/{repo}/pulls/{pull_number}/reviews` — list reviews
- `POST /repos/{owner}/{repo}/pulls/{pull_number}/comments` — agent posts self-review
- Review states: `APPROVED`, `CHANGES_REQUESTED`, `COMMENTED`
- Change-request comment extraction: diff comments → structured change request

### Webhooks
- `pull_request`: opened, ready_for_review, review_requested, closed, merged
- `pull_request_review`: submitted (approved/changes_requested)
- `pull_request_review_comment`: individual comment events
- Webhook payload validation: HMAC-SHA256 signature check

### GitHub Apps vs PAT
| Factor | PAT | GitHub App |
|---|---|---|
| Scoping | Broad | Per-repo, per-installation |
| Rate limit | 5000/hr | 15000/hr |
| Bot identity | User account | App bot account |
| Expiry | Never (classic) / 90 days (fine-grained) | 1 hour (auto-renewed) |
| Webhook | No (use polling) | Yes (native) |

## Rust Crates

| Crate | Provider | Status | Notes |
|---|---|---|---|
| `octocrab` | GitHub | Active | Async GitHub API client; PAT + App auth |
| `github-webhook` | GitHub | Check status | Webhook payload types |
| `reqwest` + hand-rolled | Any | Always viable | Full control; serialization via serde |

## PR Metadata Template

```markdown
## Agent Implementation Report

**Ticket**: [{ticket-title}](https://...) (`{ticket-id}`)
**Agent ID**: `{agent-id}` | **Session**: `{session-id}`
**Branch**: `aoh/{agent-id}/{ticket-slug}`

### Validation Results
- Tests: {pass}/{total} passing
- Cargo check: {ok/fail}
- Evidence: {evidence-ref-list}

### Implementation Notes
{agent-generated summary}

### Acceptance Criteria Status
{checklist from ticket}
```

## Acceptance Criteria

- [ ] Full PR lifecycle API call sequence documented for GitHub
- [ ] Webhook events identified for async review notifications
- [ ] Rust crate candidates evaluated (octocrab + alternatives)
- [ ] PAT vs GitHub App recommendation with rationale
- [ ] PR metadata template finalized
- [ ] Branch naming convention defined