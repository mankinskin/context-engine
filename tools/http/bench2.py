import urllib.request, time, json, sys
import http.client

BASE = "http://127.0.0.1:3002"
WS = "default"

def req(conn, path):
    """GET via keepalive conn; return (elapsed_ms, body_dict)."""
    t0 = time.perf_counter()
    conn.request("GET", path)
    resp = conn.getresponse()
    body = json.loads(resp.read())
    return (time.perf_counter() - t0) * 1000, body

conn = http.client.HTTPConnection("127.0.0.1", 3002, timeout=10)

# 1. list_all_edges
print("[1/4] GET /api/edges ...", flush=True)
ms, edges = req(conn, f"/api/edges?workspace={WS}")
print(f"      {ms:.1f}ms  count={len(edges['items'])}")

# 2. list 1 ticket
print("[2/4] GET /api/tickets?limit=1 ...", flush=True)
ms, tkt = req(conn, f"/api/tickets?workspace={WS}&limit=1")
print(f"      {ms:.1f}ms")
root = tkt['items'][0]['id']

# 3. single subgraph depth=4
print(f"[3/4] GET /api/graph/subgraph depth=4 root={root[:8]}... ...", flush=True)
ms, sg = req(conn, f"/api/graph/subgraph?workspace={WS}&root={root}&depth=4")
stats = sg.get('stats', {})
print(f"      {ms:.1f}ms  nodes={stats.get('nodes_returned')}  "
      f"p1={stats.get('phase1_edges_ms')}ms  "
      f"p2={stats.get('phase2_bfs_ms')}ms  "
      f"p3={stats.get('phase3_meta_ms')}ms  "
      f"total_server={stats.get('total_ms')}ms")

# 4. 8 warm sequential subgraph calls
print(f"[4/4] 8x sequential subgraph depth=4 ...", flush=True)
for i in range(8):
    print(f"      [{i+1}/8] ...", end=" ", flush=True)
    ms, sg = req(conn, f"/api/graph/subgraph?workspace={WS}&root={root}&depth=4")
    stats = sg.get('stats', {})
    print(f"{ms:.1f}ms  (server={stats.get('total_ms')}ms)")

conn.close()
