# Configuration Directory

This directory contains configuration files for the context-engine workspace.

## Tracing Configuration

The tracing system supports TOML-based configuration for controlling log output formatting.

### Quick Start

1. Copy the example file:
   ```bash
   cp tracing.toml.example tracing.toml
   ```

2. Edit `tracing.toml` to customize your logging preferences

3. Run tests - the config will be automatically loaded:
   ```bash
   cargo test
   ```

### Configuration Files

- **`tracing.toml.example`** - Template with all available options and documentation
- **`tracing.toml`** - Your local config (gitignored, create from example)
- **`tracing.minimal.toml`** - Example minimal config (events only)
- **`test-config.toml`** - Example test config

### Configuration Options

#### Span Enter Events
- `span_enter.show` - Show/hide span enter messages
- `span_enter.show_fn_signature` - Show function signatures
- `span_enter.show_fields` - Show span fields (parameters)

#### Span Close Events
- `span_close.show` - Show/hide span close messages
- `span_close.show_timing` - Show timing information (time.busy, time.idle)

#### Panic Logging
- `panic.show` - Show/hide panic log events
- `panic.show_message` - Include panic message details in logs
- `panic.show_stderr` - Print 🔥 PANIC: message to stderr

#### General Formatting
- `enable_indentation` - Visual indentation with box-drawing characters (┬─, │, └─)
- `show_file_location` - Show file:line for each event
- `enable_ansi` - Enable ANSI colors in terminal

### Using Custom Configs

Override the default config location with the `TRACING_CONFIG` environment variable:

```bash
# Use a different config file
TRACING_CONFIG=config/tracing.minimal.toml cargo test

# Use absolute path
TRACING_CONFIG=/path/to/my-config.toml cargo test
```

### Search Order

The tracing system searches for config files in this order:

1. `TRACING_CONFIG` environment variable
2. `{workspace_root}/config/tracing.toml` ← **Primary location**
3. `{workspace_root}/.tracing.toml` (legacy)
4. `{workspace_root}/tracing.toml` (legacy)
5. `./config/tracing.toml` (current directory)
6. `./.tracing.toml` (current directory, legacy)
7. `./tracing.toml` (current directory, legacy)
8. `~/.config/tracing.toml` (user home)
9. Environment variables (if no file found)
10. Default values (all enabled)

### Environment Variables

You can still use environment variables instead of config files. **Note:** Environment variables only apply when no config file is found. If a config file exists, use `TRACING_CONFIG` to specify a different file.

```bash
# Nested configuration (recommended)
TRACING_SPAN_ENTER_SHOW=0              # Hide span enter messages
TRACING_SPAN_CLOSE_SHOW=0              # Hide span close messages
TRACING_SPAN_ENTER_SHOW_FN_SIGNATURE=0 # Hide function signatures
TRACING_PANIC_SHOW=0                   # Disable panic logging
TRACING_PANIC_SHOW_MESSAGE=0           # Hide panic message details
TRACING_PANIC_SHOW_STDERR=0            # Don't print to stderr

# Legacy flat names (still supported)
TRACING_SHOW_FN_SIGNATURE=0
TRACING_SHOW_SPAN_FIELDS=0
TRACING_SHOW_SPAN_TIMING=0
```

### Examples

**Minimal output (events only):**
```toml
[span_enter]
show = false

[span_close]
show = false

enable_indentation = false
show_file_location = false
```

**Disable panic logging:**
```toml
[panic]
show = false
show_stderr = false
```

**Quiet spans (no close messages):**
```toml
[span_close]
show = false
```

**Clean output (no signatures, no fields):**
```toml
[span_enter]
show_fn_signature = false
show_fields = false
```

## Cargo Configuration

Cargo-specific configuration is in `.cargo/config.toml` (workspace-level) and `.cargo/config.local.toml` (user-level, gitignored).

**Note:** Cargo requires its config files to be in the `.cargo/` directory. This is a Cargo convention and cannot be changed.
