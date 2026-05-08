**Correctness & Reactivity (frontend)**
- [ ] All signal reads that must re-run on change are inside reactive closures,
      not computed once outside the `view!` macro.
- [ ] State updated correctly on all paths (including edge cases like empty data).