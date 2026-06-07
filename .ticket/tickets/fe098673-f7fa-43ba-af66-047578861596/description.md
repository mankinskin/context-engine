Define the generated context index hierarchy and semantic reference format for repository-wide context artifacts. This work covers markdown index trees, machine-readable sidecars, workspace DAG indexing, and the generation rules for tickets, specs, rules, audit status, tests, and memory workspaces.

Resolved design decisions:
- D1: Across the entire file tree for local index nodes. Use README files for folder-level index files, store workspace folders for workspace-level index files, and the .agents folder for agent-client consumable instruction hooks.
- D2: Use git hooks extensively and profile commands for low commit latency.
- D3: Full depth, with one file per node and a folder for child nodes with the canonical name.
- D4: Categorize based on slug for now.
- D5: Commit generated outputs to git.
- D6: Wait for dependent bootstrap work to be implemented before tying in required new features for those crates.
- D7: Add test catalog entries as not-run; the catalog is a complete registry.
- D8: Use TOON for minimal token cost risk; references should remain slim but dense.
- D9: All workspaces are DAG nodes with multiple parents and children and should contain a configuration folder for the tool; each workspace indexes parent and child workspace names and locations.

Open follow-up questions:
- exact root layout for the generated markdown folder hierarchy
- exact TOON sidecar record shape and validation contract
- workspace config folder naming convention