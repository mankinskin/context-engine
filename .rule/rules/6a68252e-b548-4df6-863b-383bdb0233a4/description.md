```bash
cargo run -p audit-cli --bin audit -- . \
  --max-file-lines 300 \
  --max-cyclomatic-complexity 10 \
  --coverage-warn-below 85
```