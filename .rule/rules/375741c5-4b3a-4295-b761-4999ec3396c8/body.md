### Chaining subgraph → health_check in MCP

1. Call `subgraph` with `{"workspace": "default", "root": "<id>", "depth": 3}`
2. Extract node IDs from `response.nodes[].id`
3. Call `health_check` with `{"workspace": "default", "ids": ["<id1>", "<id2>", ...]}`