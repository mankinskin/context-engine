---
description: "Use when performing the repository commit workflow. Covers checking status, regenerating generated outputs, staging logical batches, committing, and verifying clean state."
---

1. Check status before staging:

git status --short
git submodule foreach --recursive 'git status --short && echo "=== $name ==="'

2. Regenerate generated outputs when applicable:

cargo run -p rule-cli --bin rule -- sync-targets --config rule-targets.yaml

3. Stage in logical batches and commit each batch with an appropriate conventional commit message.

4. Update submodule pointers last (deepest-first cadence).

5. Verify clean state:

git status --short
