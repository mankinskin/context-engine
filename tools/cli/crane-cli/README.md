# crane-cli

CLI for transplanting filtered git history from selected source trees into another repository.

## Interface

Use `crane` when you need to preserve commit history for a narrow set of paths, remap those paths, and import the result into a target repository branch.

- `transplant`: resolve the first relevant commit for the selected path set, compute the narrowed history range, stream `git fast-export` through the Rust path transformer, and import the result with `git fast-import`.

## Usage

Build or run from the root workspace:

```bash
cargo build -p crane-cli --bin crane
cargo run -p crane-cli --bin crane -- transplant --help
```

## Verification Flow

Before using `crane` for a production migration:

1. Run the focused crate tests.

```bash
cargo test -p crane-cli
```

2. Run a real dry run against the intended source and target repositories.

```bash
crane transplant \
  --source-repo . \
  --target-repo ../context-stack \
  --target-branch main \
  --import-branch crane/tools-review \
  --mapping tools/cli/context-cli=tools/cli/context-cli \
  --mapping tools/mcp/context-mcp=tools/mcp/context-mcp \
  --mapping tools/http/context-http=tools/http/context-http \
  --mapping tools/context-editor=tools/context-editor \
  --dry-run
```

Review the dry-run output before any live import. The required review metadata is:

- `source_ref`
- `source_commit`
- `anchor_commit`
- `range_spec`
- `target_branch`
- `import_branch`
- `import_ref`
- each emitted `mapping=<source>=<destination>` line

3. Only after the dry-run metadata looks correct should you run the live transplant and inspect the target repository history and cleanliness.