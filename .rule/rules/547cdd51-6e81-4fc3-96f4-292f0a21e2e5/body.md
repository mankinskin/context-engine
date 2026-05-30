1. resolves the repo root
2. loads `.audit.toml`
3. syncs source files into `.audit/audit.sqlite3`
4. prunes stale index rows not seen in the latest scan
5. collects file length, compiler warning, test success, coverage, and static complexity metrics
6. returns raw metrics plus actionable findings and deduplicated fix instructions