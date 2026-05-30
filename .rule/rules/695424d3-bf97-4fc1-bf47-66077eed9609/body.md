# Topgraph → health check all reverse dependencies
ticket topgraph abcd1234 --json \
  | jq -r '.payload.nodes[].id' \
  | ticket health --stdin --json
```