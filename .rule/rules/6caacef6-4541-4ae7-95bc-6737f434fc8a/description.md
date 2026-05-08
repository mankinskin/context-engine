# HTTP-level stress test (requires running ticket-viewer server)
python tools/http/stress_graph.py          # concurrency sweep 2–32
python tools/http/bench2.py               # verbose per-request phase timing