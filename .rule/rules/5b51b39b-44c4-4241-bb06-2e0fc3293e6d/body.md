### Command Chaining (pipe via --stdin)

```bash
# List tickets → pipe IDs → health check
ticket list --where priority=high --json \
  | jq -r '.payload.items[].id' \
  | ticket health --stdin --toon