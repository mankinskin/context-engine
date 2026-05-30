### Batch (CLI-syntax, transactional)

`ticket batch` reads one CLI command per line from stdin (or `--file`). All
commands execute against the same store in order. If any command fails, all
prior writes are rolled back automatically. Blank lines and `#` comments are
ignored.