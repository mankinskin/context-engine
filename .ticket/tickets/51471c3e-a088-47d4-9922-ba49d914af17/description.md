# Impl: Sandbox Manager for Per-Assignment Worktree + Branch Isolation

## Purpose

Provision and manage isolated execution environments for agent sessions. Per ADR-1, v1 uses **Docker containers** (primary) with **Podman** as a Linux CI alternative, orchestrated through the `bollard` crate. Each assignment gets its own git worktree, branch, container, and network namespace so that parallel agent sessions cannot interfere with each other.

The sandbox manager is the bridge between the orchestrator's session scheduler and the container runtime. It owns the full lifecycle: create worktree → start container → bind-mount worktree → enforce network policy → cleanup on completion or failure.

## Component Boundaries

### In scope
- `SandboxManager` service with `ContainerRuntime` trait for Docker/Podman portability
- `bollard`-based Docker implementation of `ContainerRuntime`
- Per-session git worktree creation via `git worktree add` (using `git2` or `gix` crate)
- Branch naming: `aoh/{agent-id}/{ticket-slug}` (ADR-11, canonical convention per `02412b9a`)
- Container provisioning with pre-built agent image
- Bind-mount of git worktree into container
- Per-session Docker network with allow-list proxy for outbound traffic control
- GPU flags for browser workloads: `--use-gl=angle` (Windows/WSL2), `--use-gl=egl` (Linux)
- Secret injection into container via one-time HTTP/Unix-socket fetch with nonce + TTL (ADR-13; full trust model in `db784443`)
- Idempotent cleanup: container stop/remove, worktree remove, network remove
- Reconciliation: detect and clean up orphaned containers/worktrees on orchestrator restart
- Health check: verify container is running and worktree is intact

### Out of scope
- Firecracker/microVM isolation (deferred per ADR-1 — background research only)
- Cloud sandbox providers (E2B, Daytona)
- OS-level namespace isolation without Docker
- Container image building (assumes pre-built image)
- MCP server socket setup inside container (owned by session/MCP layer, though sandbox exposes the socket path)

## Key Data Types

```rust
/// Trait abstracting Docker vs Podman runtime.
#[async_trait]
trait ContainerRuntime {
    async fn create_container(&self, spec: ContainerSpec) -> Result<ContainerId>;
    async fn start_container(&self, id: &ContainerId) -> Result<()>;
    async fn stop_container(&self, id: &ContainerId, timeout: Duration) -> Result<()>;
    async fn remove_container(&self, id: &ContainerId) -> Result<()>;
    async fn inspect_container(&self, id: &ContainerId) -> Result<ContainerInfo>;
    async fn create_network(&self, spec: NetworkSpec) -> Result<NetworkId>;
    async fn remove_network(&self, id: &NetworkId) -> Result<()>;
}

/// Full specification for an agent session sandbox.
struct SandboxSpec {
    session_id: SessionId,
    ticket_id: TicketId,
    agent_id: AgentId,
    branch_slug: String,
    image: String,
    gpu_flags: GpuFlags,
    network_allow_list: Vec<String>,  // allowed outbound hosts
    secret_delivery: SecretDeliveryConfig,
}

/// Handle to a running sandbox with accessor methods.
struct Sandbox {
    session_id: SessionId,
    container_id: ContainerId,
    network_id: NetworkId,
    worktree_path: PathBuf,
    branch_name: String,
    mcp_socket_path: PathBuf,
}

enum GpuFlags {
    Angle,  // Windows/WSL2
    Egl,    // Linux
    None,   // No GPU needed
}
```

## Design Decisions Mapped from ADRs

| ADR | Implication |
|---|---|
| ADR-1 (Container BaaS) | Docker primary, Podman alternative; `bollard` for orchestration; GPU flags for browser workloads |
| ADR-3 (GitHub remote) | Worktree branches use canonical `aoh/{agent-id}/{ticket-slug}` naming (ADR-11) |
| ADR-7 (Per-session MCP) | Sandbox exposes `mcp_socket_path` for the session's MCP server |
| ADR-8 (Persona identity) | Agent ID from persona system used in branch name and container labels |
| ADR-9 (Session revival) | `reconcile()` detects existing worktrees/containers for revival; same worktree reused where possible |
| `db784443` (Trust boundaries) | Secret injection follows the approved delivery path; no secrets in env vars in CI/prod |

## Acceptance Criteria

- [ ] `ContainerRuntime` trait implemented for Docker via `bollard`
- [ ] Each assignment provisions an isolated git worktree with canonical branch naming
- [ ] Container starts with bind-mounted worktree and per-session network
- [ ] Network allow-list proxy restricts outbound traffic to configured hosts only
- [ ] GPU flags are applied correctly per platform (Windows/WSL2 vs Linux)
- [ ] CWD and branch invariants are validated after provisioning
- [ ] Idempotent cleanup removes container, network, and worktree without error on repeated calls
- [ ] Reconciliation on startup detects and cleans orphaned containers/worktrees
- [ ] Secret injection path is integrated (or stubbed pending `db784443` resolution)
- [ ] Unit tests cover: provision → verify → cleanup, orphan reconciliation, and double-cleanup idempotency
