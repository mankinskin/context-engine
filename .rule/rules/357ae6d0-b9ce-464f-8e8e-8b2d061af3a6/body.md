# Subgraph → filter new tickets → health check
ticket subgraph abcd1234 --json \
  | jq -r '.payload.nodes[] | select(.state=="new") | .id' \
  | ticket health --stdin --toon