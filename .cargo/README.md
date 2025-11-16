# Cargo Configuration

This directory contains Cargo configuration files for the workspace.

## Files

- **`config.toml`** - Project-wide Cargo configuration
  - Sets default environment variables for tests
  - Committed to git (modify directly or use overrides below)

## Important: config.local.toml is NOT supported by Cargo

**Cargo does not recognize `config.local.toml`** - only files named exactly `config` or `config.toml` are read.

If you need personal overrides, use one of these methods instead:

## Usage

### Using the defaults

Just run cargo commands normally (uses settings from `config.toml`):
```bash
cargo test
cargo build
```

### Override Methods

Since Cargo doesn't support `config.local.toml`, use one of these approaches:

#### Option 1: Modify config.toml directly (Recommended for project settings)
Edit `.cargo/config.toml` and uncomment/modify the settings you want:
```toml
[env]
LOG_STDOUT = "1"
LOG_FILTER = "debug"
```

#### Option 2: Use direnv (Recommended for personal overrides)
Create a `.envrc` file in the workspace root (gitignored):
```bash
export LOG_STDOUT=1
export LOG_FILTER=debug
```
Then run `direnv allow` to enable it.

#### Option 3: User-level config
Edit `~/.cargo/config.toml` to set your global Cargo preferences:
```toml
[env]
LOG_STDOUT = "1"
LOG_FILTER = "debug"
```

#### Option 4: One-time overrides

Override environment variables for a single command:
```bash
LOG_STDOUT=1 cargo test
LOG_FILTER=debug cargo test -- --nocapture
```

## Environment Variables

- **`RUST_BACKTRACE`** - Enable backtraces (set to `1` by default)
- **`LOG_STDOUT`** - Enable stdout logging in tests (`1` = enabled, `0` = disabled)
  - Can also be set in `config/tracing.toml` with `log_to_stdout = true`
- **`LOG_FILTER`** - Set log level (`error`, `warn`, `info`, `debug`, `trace`)
  - Can be module-specific: `context_search=trace,context_trace=debug`
  - Can also be set in `config/tracing.toml` with `log_filter = "debug"`

**Note:** Settings in `config/tracing.toml` take precedence over environment variables.

For more details on tracing configuration, see:
- `config/tracing.toml.example` - All available options
- `context-trace/src/logging/tracing_utils/mod.rs`
- `CHEAT_SHEET.md` debugging section
