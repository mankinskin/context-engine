## Problem

`.github/copilot-instructions.md` lines 33-34 describe `memory-kernel/` as a development-only submodule for `transport-harness` that is not needed for ordinary work. The statement appears in the `Repository Shape` section (lines 13-39).

The description is wrong: `memory-kernel` is now the neutral shared base layer beneath all domain repositories. Its root library crate is `memory_kernel`; it also hosts `transport-harness`, has its own `github.com/mankinskin/memory-kernel` remote, and has its own push cycle. Agents following the current guidance may incorrectly skip a required dependency while building or testing.

## Required State

Update the `Repository Shape` entry so `memory-kernel` is described as the required neutral shared base layer beneath all domain repositories, names the root `memory_kernel` crate and `transport-harness`, and explains the independent remote and push cycle.

Related restructuring context: ticket `69eb4118` and ticket `1b7e0c3d`.
