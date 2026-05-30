1. **State progress** — tickets closest to `done` appear first (e.g.
   `in-review` > `in-implementation` > `ready` > `new`).
   Progress is determined by the state's index in the schema's `states` list.
2. **Priority** — `critical > high > medium > low > none`.
3. **Creation date** — oldest first (FIFO tiebreaker).