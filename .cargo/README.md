# Cargo Configuration

This directory contains Cargo configuration files for the workspace.

## Files

- **`config.toml`** - Committed project-wide defaults
  - Sets `RUST_BACKTRACE=1` for better debugging
  - Contains commented examples for test logging configuration

- **`config.local.toml`** - Gitignored personal overrides (optional)
  - Copy from `config.local.toml.example` to get started
  - Use this for your personal development preferences
  - Overrides settings from `config.toml`

## Usage

### Using the defaults

Just run cargo commands normally:
```bash
cargo test
cargo build
```

### Personal configuration

1. Copy the example file:
   ```bash
   cp .cargo/config.local.toml.example .cargo/config.local.toml
   ```

2. Edit `.cargo/config.local.toml` with your preferences:
   ```toml
   [env]
   LOG_STDOUT = "1"          # Enable test output
   LOG_FILTER = "trace"      # Set log level
   ```

### One-time overrides

Override environment variables for a single command:
```bash
LOG_STDOUT=1 cargo test
LOG_FILTER=debug cargo test -- --nocapture
```

## Environment Variables

- **`RUST_BACKTRACE`** - Enable backtraces (set to `1` by default)
- **`LOG_STDOUT`** - Enable stdout logging in tests (`1` = enabled, `0` = disabled)
- **`LOG_FILTER`** - Set log level (`error`, `warn`, `info`, `debug`, `trace`)
  - Can be module-specific: `context_search=trace,context_trace=debug`

For more details on tracing configuration, see:
- `context-trace/src/logging/tracing_utils/mod.rs`
- `CHEAT_SHEET.md` debugging section
