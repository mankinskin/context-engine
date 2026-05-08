Required responses:
1. Agent still active: run `board heartbeat <entry-id>` to renew.
2. Work abandoned: run `board check-out <ticket-id>` then clean.
3. Remove stale entries: `board clean preview --include-stale`, then
   `board clean apply --token <token> --include-stale`.