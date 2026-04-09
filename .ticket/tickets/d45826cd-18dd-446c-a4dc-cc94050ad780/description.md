# [AOH][Design] Reusable Agent Persona Store — Identity Assignment and Lifecycle

## Context

**User decision (Q8):** Unique generated personas per session, **reusable** — the same persona can be revived across multiple sessions. A persona is a persistent identity with a name, email, and a characterization that appears in git commits, ticket records, and agent prompts.

**User decision (Q9, linked):** Session revival reuses archived session context + summary injection. The persona assigned to a ticket remains consistent across the initial session and any revival.

## Design Goals

1. Persona pool: a configured set of named agent identities (Rust TOML config file)
2. Assignment: orchestrator assigns the **least recently used** persona that is not currently active
3. Reuse: when a ticket is revived, the **same persona** that originally worked on it is reassigned
4. Persistence: persona → ticket assignments survive orchestrator restarts (stored in ticket fields)
5. Git identity: each persona has a stable `name + email` used for `git config user.*` in every worktree it works in
6. Character sketches: optional brief personality trait set fed into the agent kickoff prompt (e.g., "methodical, tests-first, verbose commit messages")

## Persona Record Format

Stored in `config/personas.toml` in the repository (or orchestrator config directory):

```toml
[[personas]]
id = "agent-petal"
display_name = "Petal"
git_name = "Agent Petal"
git_email = "petal@aoh.local"
traits = ["methodical", "tests-first", "verbose commit messages"]
created_at = "2026-04-09"

[[personas]]
id = "agent-cedar"
display_name = "Cedar"
git_name = "Agent Cedar"
git_email = "cedar@aoh.local"
traits = ["fast iteration", "minimal commits", "prefers refactoring"]
created_at = "2026-04-09"

[[personas]]
id = "agent-fern"
display_name = "Fern"
git_name = "Agent Fern"
git_email = "fern@aoh.local"
traits = ["thorough documentation", "safety-focused", "asks questions before acting"]
created_at = "2026-04-09"

# ... up to N personas where N ≥ max-concurrent-sessions (20)
```

## Persona Lifecycle States

```
Available ──assign──▶ Active ──complete──▶ Idle (cooldown)
                         │                     │
                         │ revival             │ expire cooldown
                         ▼                     ▼
                    Active (same          Available
                    ticket, same
                    persona)
```

- **Available**: can be assigned to a new ticket
- **Active**: currently running a session
- **Idle** (optional cooldown period): recently completed; LRU ranking places these last
- Revival: ticket lookup → find persona by `ticket.fields.assigned_agent_id` → re-assign same persona

## Persona Assignment Algorithm

```
function assign_persona(ticket_id):
    1. Look up ticket.fields["assigned_agent_id"]
    2. If set → return that persona (same persona as before, for revival)
    3. If not set:
       a. Filter personas where state == Available
       b. Sort by last_used_at ascending (least recently used first)
       c. Assign first; set ticket.fields["assigned_agent_id"] = persona.id
       d. Record assignment in persona state: last_used_at = now, current_ticket = ticket_id
    4. Configure git worktree: user.name = persona.git_name, user.email = persona.git_email
```

## Persona State Persistence

State stored in the ticket system (not in persona config):

- `ticket.fields["assigned_agent_id"]` — written at assignment, read on revival
- Orchestrator in-memory state: `HashMap<PersonaId, PersonaState>` rebuilt on restart from ticket scan

Persona config (`personas.toml`) is static; the ticket store is the source of truth for runtime state.

## Git Configuration per Worktree

```bash
git -C .aoh/worktrees/{session-id} config user.name  "{persona.git_name}"
git -C .aoh/worktrees/{session-id} config user.email "{persona.git_email}"
git -C .aoh/worktrees/{session-id} config core.sshCommand "ssh -i {session-key-path}"
```

Git user config is **worktree-local** (not global) — ensures no cross-contamination between concurrent agents.

### SSH Key per Persona (Optional)
- Each persona gets a dedicated SSH key pair for remote operations
- Keys stored in `config/persona-keys/{persona-id}/id_ed25519`
- Key registered as a Deploy Key on GitHub for the repository
- Advantage: per-agent audit trail on remote; revocable independently

## Persona Generation

For a pool of 20+ personas (supporting 5-20 concurrent sessions + buffer):

Names drawn from nature/plant/element vocabulary (consistent with existing "Petal", "Cedar", "Fern"):
```
Alder, Ash, Bay, Birch, Briar, Brook, Cedar, Clover, Dew, Elder,
Elm, Fern, Flint, Gale, Glen, Heath, Iris, Ivy, Juniper, Laurel,
Linden, Maple, Meadow, Moss, Oak, Opal, Pebble, Petal, Pine, Reed,
River, Rowan, Sage, Slate, Stone, Thorn, Vale, Willow, Wren
```

Traits generated from a matrix of dimensions:
- Pace: `fast` | `methodical` | `balanced`
- Commit style: `atomic` | `squash-prefer` | `verbose-messages`
- Testing posture: `tests-first` | `tests-last` | `tests-concurrent`
- Communication: `terse-output` | `explanatory-output` | `asks-before-acting`

Each persona gets 2-3 orthogonal traits.

## Integration with Kickoff Prompt

Persona traits are injected into the agent session kickoff prompt:

```markdown
## Your Identity
You are **{persona.display_name}** (git: `{persona.git_name} <{persona.git_email}>`).
All commits must use this exact identity.

Your working style: {persona.traits joined as natural language}.
For example, "{trait example behavior}".
```

## Rust Interface Design

```rust
pub struct Persona {
    pub id: PersonaId,
    pub display_name: String,
    pub git_name: String,
    pub git_email: String,
    pub traits: Vec<String>,
}

pub struct PersonaStore {
    personas: Vec<Persona>,
}

impl PersonaStore {
    pub fn load(config: &Path) -> Result<Self>;
    
    /// Assign a persona for a ticket. Returns same persona on revival.
    pub fn assign(&self, ticket_id: &TicketId, 
                  active: &ActivePersonaMap) -> Result<&Persona>;
    
    /// Release a persona back to the available pool.
    pub fn release(&self, persona_id: &PersonaId, 
                   active: &mut ActivePersonaMap);
    
    /// Generate a pool of N personas with diverse trait combinations.
    pub fn generate_pool(n: usize) -> Vec<Persona>;
}
```

## Acceptance Criteria

- [ ] `personas.toml` schema defined and validated at startup
- [ ] Assignment algorithm implemented with LRU ordering
- [ ] Revival path validated: same persona returned for same ticket ID on second call
- [ ] Git worktree user config applied per persona; verified in git log `--format="%ae"`
- [ ] Persona pool of 25 personas generated with diverse trait combinations
- [ ] `PersonaStore` compiles and passes unit tests for assign/release/revival
- [ ] Kickoff prompt template includes persona identity and traits
- [ ] SSH key per persona option documented (with generation and GitHub registration steps)