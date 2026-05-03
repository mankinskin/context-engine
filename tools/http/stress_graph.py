#!/usr/bin/env python3
"""
Backend stress test for the ticket-viewer /api/graph/subgraph endpoint.

Usage:
    python tools/http/stress_graph.py [--base-url URL] [--workspace WS]
                                      [--depth N] [--timeout SEC]

Phases:
    1. Baseline  – sequential requests, establishes healthy latency.
    2. Threshold – concurrency sweep (2, 4, 8, 12, 16, 24, 32) to find
                   the point where errors appear.
    3. Sustained – 30s of the failing concurrency level to assess recovery.
"""

import argparse
import concurrent.futures
import statistics
import sys
import time
import urllib.error
import urllib.request
from dataclasses import dataclass, field
from typing import List, Optional


# ---------------------------------------------------------------------------
# Types
# ---------------------------------------------------------------------------

@dataclass
class Result:
    status: int       # HTTP status code, 0 = connection error / timeout
    elapsed: float    # seconds


@dataclass
class Summary:
    label: str
    results: List[Result] = field(default_factory=list)

    def ok(self) -> List[Result]:
        return [r for r in self.results if r.status == 200]

    def errors(self) -> List[Result]:
        return [r for r in self.results if r.status != 200]

    def timeouts(self) -> int:
        return sum(1 for r in self.results if r.status == 0)

    def report(self) -> str:
        ok = self.ok()
        errs = self.errors()
        latencies = [r.elapsed for r in ok]
        parts = [
            f"{self.label:<28}",
            f"total={len(self.results):4d}",
            f"ok={len(ok):4d}",
            f"err={len(errs):4d}",
            f"timeout={self.timeouts():4d}",
        ]
        if latencies:
            latencies.sort()
            n = len(latencies)
            p50 = latencies[n // 2]
            p95 = latencies[int(n * 0.95)]
            p99 = latencies[int(n * 0.99)]
            parts += [
                f"avg={statistics.mean(latencies):.3f}s",
                f"p50={p50:.3f}s",
                f"p95={p95:.3f}s",
                f"p99={p99:.3f}s",
                f"max={max(latencies):.3f}s",
            ]
        return "  ".join(parts)


def log(msg: str) -> None:
    print(msg, flush=True)


# ---------------------------------------------------------------------------
# HTTP helpers
# ---------------------------------------------------------------------------

def fetch_one(url: str, timeout: float) -> Result:
    """Make one GET request; return (status, elapsed).  0 = timeout/error."""
    t0 = time.perf_counter()
    try:
        req = urllib.request.urlopen(url, timeout=timeout)
        req.read()
        return Result(status=req.status, elapsed=time.perf_counter() - t0)
    except urllib.error.HTTPError as e:
        return Result(status=e.code, elapsed=time.perf_counter() - t0)
    except Exception:
        return Result(status=0, elapsed=time.perf_counter() - t0)


# ---------------------------------------------------------------------------
# Phases
# ---------------------------------------------------------------------------

def phase_baseline(ids: List[str], base_url: str, workspace: str,
                   depth: int, timeout: float, n: int = 20) -> Summary:
    s = Summary("baseline (sequential)")
    urls = [
        f"{base_url}/api/graph/subgraph?workspace={workspace}&root={ids[i % len(ids)]}&depth={depth}"
        for i in range(n)
    ]
    for url in urls:
        s.results.append(fetch_one(url, timeout))
    return s


def phase_concurrency(ids: List[str], base_url: str, workspace: str,
                      depth: int, timeout: float,
                      concurrency: int, n: int = 160) -> Summary:
    s = Summary(f"concurrency={concurrency:2d} n={n}")
    urls = [
        f"{base_url}/api/graph/subgraph?workspace={workspace}&root={ids[i % len(ids)]}&depth={depth}"
        for i in range(n)
    ]
    with concurrent.futures.ThreadPoolExecutor(max_workers=concurrency) as ex:
        futures = [ex.submit(fetch_one, url, timeout) for url in urls]
        for f in concurrent.futures.as_completed(futures):
            s.results.append(f.result())
    return s


def phase_sustained(ids: List[str], base_url: str, workspace: str,
                    depth: int, timeout: float,
                    concurrency: int, duration_sec: float = 30.0) -> Summary:
    s = Summary(f"sustained c={concurrency} t={int(duration_sec)}s")
    deadline = time.perf_counter() + duration_sec

    def worker(idx: int) -> Optional[Result]:
        if time.perf_counter() >= deadline:
            return None
        url = (
            f"{base_url}/api/graph/subgraph"
            f"?workspace={workspace}&root={ids[idx % len(ids)]}&depth={depth}"
        )
        return fetch_one(url, timeout)

    with concurrent.futures.ThreadPoolExecutor(max_workers=concurrency) as ex:
        idx = 0
        pending = set()
        while True:
            # Keep the pool full while the deadline hasn't passed
            while len(pending) < concurrency and time.perf_counter() < deadline:
                pending.add(ex.submit(worker, idx))
                idx += 1
            if not pending:
                break
            done, pending = concurrent.futures.wait(
                pending, return_when=concurrent.futures.FIRST_COMPLETED
            )
            for f in done:
                r = f.result()
                if r is not None:
                    s.results.append(r)
            if time.perf_counter() >= deadline and not pending:
                break

    return s


def check_recovery(base_url: str, workspace: str, depth: int,
                   timeout: float, ids: List[str]) -> None:
    url = f"{base_url}/api/graph/subgraph?workspace={workspace}&root={ids[0]}&depth={depth}"
    r = fetch_one(url, timeout)
    tag = "OK  " if r.status == 200 else "FAIL"
    log(f"    recovery: {tag} (HTTP {r.status}, {r.elapsed:.3f}s)")


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------

def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base-url",   default="http://localhost:3002")
    parser.add_argument("--workspace",  default="default")
    parser.add_argument("--depth",      type=int,   default=4)
    parser.add_argument("--timeout",    type=float, default=10.0)
    parser.add_argument("--ticket-limit", type=int, default=40,
                        help="Max ticket IDs to use as rotation pool")
    args = parser.parse_args()

    base = args.base_url.rstrip("/")
    ws   = args.workspace

    # ── Fetch ticket IDs ──────────────────────────────────────────────────
    log(f"Fetching ticket IDs from {base}/api/tickets ...")
    try:
        req = urllib.request.urlopen(
            f"{base}/api/tickets?workspace={ws}&limit={args.ticket_limit}",
            timeout=10,
        )
        import json
        body = json.loads(req.read())
    except Exception as e:
        print(f"ERROR: could not fetch ticket list: {e}", file=sys.stderr)
        sys.exit(1)

    ids = [t["id"] for t in body.get("items", [])]
    if not ids:
        print("ERROR: no tickets found in workspace", file=sys.stderr)
        sys.exit(1)
    log(f"Using {len(ids)} ticket IDs as rotation pool.\n")

    results: List[Summary] = []

    # ── Phase 1: baseline ─────────────────────────────────────────────────
    log("=== Phase 1: Baseline (sequential) ===")
    s = phase_baseline(ids, base, ws, args.depth, args.timeout, n=20)
    results.append(s)
    log(f"  {s.report()}")
    check_recovery(base, ws, args.depth, args.timeout, ids)

    # ── Phase 2: concurrency sweep ────────────────────────────────────────
    log("\n=== Phase 2: Concurrency sweep ===")
    sweep_levels = [2, 4, 6, 8, 12, 16, 24, 32]
    failing_level: Optional[int] = None
    for c in sweep_levels:
        n_req = max(c * 8, 40)
        log(f"  running c={c} n={n_req} ...", )
        s = phase_concurrency(ids, base, ws, args.depth, args.timeout,
                              concurrency=c, n=n_req)
        results.append(s)
        ok_pct = 100.0 * len(s.ok()) / len(s.results) if s.results else 0
        log(f"  {s.report()}  ok%={ok_pct:.0f}%")
        if ok_pct < 80 and failing_level is None:
            failing_level = c
        check_recovery(base, ws, args.depth, args.timeout, ids)

    # ── Phase 3: sustained load at failure point ──────────────────────────
    if failing_level is not None:
        log(f"\n=== Phase 3: Sustained load at c={failing_level} for 30s ===")
        s = phase_sustained(ids, base, ws, args.depth, args.timeout,
                            concurrency=failing_level, duration_sec=30.0)
        results.append(s)
        ok_pct = 100.0 * len(s.ok()) / len(s.results) if s.results else 0
        log(f"  {s.report()}  ok%={ok_pct:.0f}%")
        check_recovery(base, ws, args.depth, args.timeout, ids)
    else:
        log("\n=== Phase 3: skipped (no failure level detected in sweep) ===")

    # ── Final summary ─────────────────────────────────────────────────────
    log("\n=== Summary ===")
    for s in results:
        ok_pct = 100.0 * len(s.ok()) / len(s.results) if s.results else 0
        marker = "OK " if ok_pct >= 95 else ("!  " if ok_pct >= 70 else "ERR")
        log(f"  [{marker}] {s.report()}  ok%={ok_pct:.0f}%")

    if failing_level is not None:
        log(f"\n  Failure threshold: concurrency >= {failing_level}")
    else:
        log("\n  No failures detected across sweep levels.")


if __name__ == "__main__":
    main()
