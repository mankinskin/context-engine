```bash
cargo run -p audit-cli --bin audit -- run <target-context> \
  --max-file-lines 300 \
  --max-cyclomatic-complexity 10 \
  --coverage-warn-below 85
```