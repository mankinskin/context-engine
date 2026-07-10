Here is the comprehensive architectural blueprint and handoff checklist for your all-Rust autonomous agent harness workspace.

## System architecture schematic

```
                               +-------------------------------------------------+

                               |               Cargo Workspace Root              |
                               +-------------------------------------------------+
                                                        |
       +------------------------+-----------------------+------------------------+

       |                        |                       |                        |
       v                        v                       v                        v
+--------------+        +---------------+       +---------------+        +--------------+

| agent-shared |        |  agent-core   |       | agent-server  |        |  agent-uapi  |
| (Rust Types) |        | (Agent Logic) |       | (Axum Engine) |        | (WASM & TUI) |
+--------------+        +---------------+       +---------------+        +--------------+
       ^                        |                       |                        |

       |                        |                       | (WebSockets /          | (Compiled via
       | (Shared Dependency)    | (Spawns & Tracks)     |  JSON-RPC Stream)      |  target-native
       |                        v                       v                        |  & wasm32)
       |                +---------------+       +---------------+                |
       +----------------|  MCP Client   |       | Client UIs    |<---------------+

                        |  (rmcp Spec)  |       | - Ratatui TUI |
                        +---------------+       | - Dioxus WASM |
                                                +---------------+
```

## Component interaction matrix
Component CratePrimary Role & Ecosystem TechUpstream / Downstream Boundaryagent-sharedSingle source of truth for message schemas, system states, and token events via `serde`.Independent / Imported by Core, Server, and Frontends.agent-coreManages the ReAct loop, handles dynamic project `.agentguidance` injection, executes plugins, and drives LLM provider queries via `genai`.Upstream: `agent-shared` / Downstream: `agent-server` loop orchestrator.agent-serverHigh-performance, async `axum` routing hub. Manages `tokio::sync::broadcast` channels to distribute engine events.Upstream: `agent-core` / Downstream: WebSocket consumers (TUI & GUI).agent-uapiContains dual compilation paths for interfaces: `ratatui` (Native terminal backend) and `dioxus` (WASM browser build running via `wasm-bindgen`).Upstream: WebSockets from `agent-server` / Downstream: Render loop presentation layer.
---

## Production implementation checklist

## 1. Workspace foundational architecture

- Cargo Configuration: Setup workspace root `Cargo.toml` with explicit member paths for `agent-shared`, `agent-core`, `agent-server`, and frontend directories.
- Type Uniformity: Enforce matching `serde(tag = "type", content = "payload")` enums on all message events inside `agent-shared` to prevent compilation if API payloads drift.
- Async Runtime Isolation: Dedicate a distinct, supervised `tokio::runtime` for the agent loop inside `agent-core` so UI event drops cannot lock up core file-writing logic.

## 2. Core execution engine (ReAct loop)

- LLM Protocol Abstraction: Configure the `genai` layer to transparently forward requests to OpenRouter, Anthropic, or OpenAI based on environmental variables.
- Safety Isolation: Wrap all fallback execution commands (`tokio::process::Command`) in an explicit Docker sandbox wrapper (using the `bollard` crate) to isolate running agents from the host workspace root.
- MCP Compliance: Implement standard JSON-RPC Stdio handshakes utilizing the official `rmcp` client to register external custom tools at startup.
- Interceptors & Hooks: Establish an async execution hook trait array inside `agent-core` to check for token limits and budget limits before committing tokens to expensive frontier endpoints.
- Guidance Manifest: Inject parsing logic that enforces a strict local `.agentguidance.json` configuration array into the early system prompt generator stage.

## 3. Streaming server infrastructure

- State Machine Integrity: Implement a clean atomic lifecycle machine (`axum::extract::State`) that avoids multi-thread write deadlocks over asynchronous task transitions.
- WebSocket Broadcast Channels: Initialize standard `tokio::sync::broadcast::Sender<LifecycleEvent>` channels to safely clone real-time stream frames for multiple frontends simultaneously.
- Cross-Origin Configuration: Configure global CORS rules inside `axum` to allow local cross-origin connections between the native dev server and the browser-targeted WebAssembly client.

## 4. User interface presentation tier

- TUI Refresh Synchronizer: Configure a dedicated input thread event handler in `ratatui` to read background WebSocket updates asynchronously, preserving stable 60 FPS refresh rates.
- WASM Framework Pipeline: Stand up a `dioxus` front-end setup compiling natively into a optimized `wasm32-unknown-unknown` target.
- Diff Viewer Integration: Integrate the `similar` diff rendering engine inside both frontend pipelines to preview visual code modifications before hitting execution confirmation windows.
- Terminal Canvas Render: Link a working JavaScript-backed `xterm.js` binding layer into the WASM client using explicit `web-sys` hooks for real-time terminal stdout mirrors.

---
To help complete the implementation phase, would you like to:

- Generate a complete, ready-to-use root workspace `Cargo.toml` file with all feature flags set?
- Build the exact Rust structure for the unified JSON-RPC WebSocket parsing loop used by the frontends?
- Detail the Docker sandboxing code block via `bollard` for secure local terminal executions?