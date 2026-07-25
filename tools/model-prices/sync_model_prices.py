#!/usr/bin/env python3
"""Extract and synchronize an LLM model price table from pydantic/genai-prices.

Source of truth: https://github.com/pydantic/genai-prices (MIT).
The upstream ``prices/data_slim.json`` file is an array of providers; each
provider carries a list of models, and each model carries per-million-token
prices that may be a plain number, a tiered ``{base, tiers}`` object, or a list
of conditional prices.

This script downloads that file, flattens it into a compact price table, and
writes it to a local JSON file. On re-run it only rewrites the output when the
upstream content hash changes, so it can be used as a sync step.

Stdlib only. No third-party dependencies.
"""

from __future__ import annotations

import argparse
import datetime as _dt
import hashlib
import json
import sys
import urllib.error
import urllib.request
from pathlib import Path
from typing import Any

RAW_BASE = "https://raw.githubusercontent.com/pydantic/genai-prices/main/prices"
SLIM_URL = f"{RAW_BASE}/data_slim.json"
FULL_URL = f"{RAW_BASE}/data.json"

# Per-million-token price fields we surface in the flattened table.
PRICE_FIELDS = (
    "input_mtok",
    "output_mtok",
    "cache_read_mtok",
    "cache_write_mtok",
)


def fetch(url: str, timeout: float) -> bytes:
    """Download ``url`` and return the raw bytes."""
    request = urllib.request.Request(
        url, headers={"User-Agent": "context-engine-model-price-sync/1.0"}
    )
    with urllib.request.urlopen(request, timeout=timeout) as response:
        return response.read()


def scalar_price(value: Any) -> float | None:
    """Reduce a price field to a single USD/mtok number.

    Accepts a plain number or a tiered ``{base, tiers}`` object (base rate is
    used). Returns ``None`` when the field is absent or not understood.
    """
    if value is None:
        return None
    if isinstance(value, (int, float)):
        return float(value)
    if isinstance(value, dict) and "base" in value:
        base = value["base"]
        if isinstance(base, (int, float)):
            return float(base)
    return None


def resolve_prices(prices: Any) -> dict[str, Any]:
    """Resolve a model's ``prices`` (ModelPrice or conditional list) to one map.

    For a conditional list, prefer the last entry with no constraint (the
    always-valid price); otherwise fall back to the first entry.
    """
    if isinstance(prices, dict):
        return prices
    if isinstance(prices, list) and prices:
        chosen = prices[0]
        for entry in prices:
            if isinstance(entry, dict) and entry.get("constraint") is None:
                chosen = entry
        if isinstance(chosen, dict):
            return chosen.get("prices", {}) or {}
    return {}


def flatten(providers: list[dict[str, Any]]) -> list[dict[str, Any]]:
    """Flatten providers/models into a list of price rows sorted by id."""
    rows: list[dict[str, Any]] = []
    for provider in providers:
        provider_id = provider.get("id", "")
        provider_name = provider.get("name", "")
        for model in provider.get("models", []):
            price_map = resolve_prices(model.get("prices"))
            row: dict[str, Any] = {
                "provider_id": provider_id,
                "provider_name": provider_name,
                "model_id": model.get("id", ""),
                "context_window": model.get("context_window"),
                "deprecated": bool(model.get("deprecated", False)),
            }
            for field in PRICE_FIELDS:
                row[field] = scalar_price(price_map.get(field))
            rows.append(row)
    rows.sort(key=lambda r: (r["provider_id"], r["model_id"]))
    return rows


def build_document(source_url: str, source_sha256: str, rows: list[dict[str, Any]]) -> dict[str, Any]:
    return {
        "_meta": {
            "source": "pydantic/genai-prices",
            "source_url": source_url,
            "source_sha256": source_sha256,
            "synced_at": _dt.datetime.now(_dt.timezone.utc).isoformat(),
            "model_count": len(rows),
            "price_unit": "USD per 1,000,000 tokens",
            "note": "Indicative estimates only; upstream is best-effort. Not authoritative billing data.",
        },
        "models": rows,
    }


def load_existing_sha(output_path: Path) -> str | None:
    if not output_path.exists():
        return None
    try:
        existing = json.loads(output_path.read_text(encoding="utf-8"))
    except (json.JSONDecodeError, OSError):
        return None
    if isinstance(existing, dict):
        return existing.get("_meta", {}).get("source_sha256")
    return None


def _fmt(value: Any) -> str:
    if value is None:
        return "-"
    if isinstance(value, float):
        return f"{value:g}"
    return str(value)


def query_table(output_path: Path, needle: str | None, fmt: str) -> int:
    """Print matching rows from the local price table without any network I/O.

    ``needle`` is a case-insensitive substring matched against ``provider_id``
    and ``model_id``; ``None`` lists everything.
    """
    if not output_path.exists():
        print(
            f"error: {output_path} not found; run a sync first (see --help).",
            file=sys.stderr,
        )
        return 2
    try:
        document = json.loads(output_path.read_text(encoding="utf-8"))
    except (json.JSONDecodeError, OSError) as exc:
        print(f"error: cannot read {output_path}: {exc}", file=sys.stderr)
        return 2

    rows = document.get("models", []) if isinstance(document, dict) else []
    if needle:
        low = needle.lower()
        rows = [
            r
            for r in rows
            if low in r.get("provider_id", "").lower()
            or low in r.get("model_id", "").lower()
        ]

    if not rows:
        print("no matching models" if needle else "no models in table", file=sys.stderr)
        return 1

    if fmt == "json":
        print(json.dumps(rows, indent=2, ensure_ascii=False))
        return 0

    columns = [
        ("provider_id", "provider"),
        ("model_id", "model"),
        ("input_mtok", "in$/M"),
        ("output_mtok", "out$/M"),
        ("cache_read_mtok", "cread$/M"),
        ("cache_write_mtok", "cwrite$/M"),
        ("context_window", "ctx"),
    ]
    if fmt == "csv":
        print(",".join(header for _, header in columns))
        for row in rows:
            print(",".join(_fmt(row.get(key)) for key, _ in columns))
        return 0

    # Aligned text table (default).
    cells = [[header for _, header in columns]]
    cells += [[_fmt(row.get(key)) for key, _ in columns] for row in rows]
    widths = [max(len(r[i]) for r in cells) for i in range(len(columns))]
    for i, row in enumerate(cells):
        print("  ".join(cell.ljust(widths[j]) for j, cell in enumerate(row)))
        if i == 0:
            print("  ".join("-" * widths[j] for j in range(len(columns))))
    print(f"\n{len(rows)} model(s)")
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--output",
        type=Path,
        default=Path(__file__).with_name("model_prices.json"),
        help="Path to the local price table JSON (default: model_prices.json next to this script).",
    )
    parser.add_argument(
        "--full",
        action="store_true",
        help="Use the full data.json instead of the slimmed data_slim.json.",
    )
    parser.add_argument(
        "--source-url",
        default=None,
        help="Override the remote source URL entirely.",
    )
    parser.add_argument(
        "--force",
        action="store_true",
        help="Rewrite the output even when the upstream content is unchanged.",
    )
    parser.add_argument(
        "--timeout",
        type=float,
        default=30.0,
        help="HTTP timeout in seconds (default: 30).",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Exit 1 if the local table is out of date; do not write anything.",
    )
    parser.add_argument(
        "--query",
        metavar="TEXT",
        default=None,
        help="Offline: print rows whose provider or model id contains TEXT (no sync).",
    )
    parser.add_argument(
        "--list",
        dest="list_all",
        action="store_true",
        help="Offline: print every model in the local table (no sync).",
    )
    parser.add_argument(
        "--format",
        choices=("table", "csv", "json"),
        default="table",
        help="Output format for --query/--list (default: table).",
    )
    args = parser.parse_args(argv)

    if args.query is not None or args.list_all:
        return query_table(args.output, args.query, args.format)

    source_url = args.source_url or (FULL_URL if args.full else SLIM_URL)

    try:
        raw = fetch(source_url, timeout=args.timeout)
    except (urllib.error.URLError, TimeoutError) as exc:
        print(f"error: failed to fetch {source_url}: {exc}", file=sys.stderr)
        return 2

    remote_sha = hashlib.sha256(raw).hexdigest()
    local_sha = load_existing_sha(args.output)
    up_to_date = local_sha == remote_sha

    if args.check:
        if up_to_date:
            print(f"up to date ({args.output.name}, sha256={remote_sha[:12]})")
            return 0
        print(f"out of date: {args.output.name} (local={local_sha}, remote={remote_sha[:12]})")
        return 1

    if up_to_date and not args.force:
        print(f"up to date, no changes written ({args.output.name}, sha256={remote_sha[:12]})")
        return 0

    try:
        providers = json.loads(raw)
    except json.JSONDecodeError as exc:
        print(f"error: upstream is not valid JSON: {exc}", file=sys.stderr)
        return 2
    if not isinstance(providers, list):
        print("error: upstream JSON root is not an array of providers", file=sys.stderr)
        return 2

    rows = flatten(providers)
    document = build_document(source_url, remote_sha, rows)

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(
        json.dumps(document, indent=2, ensure_ascii=False) + "\n", encoding="utf-8"
    )
    action = "updated" if local_sha else "created"
    print(f"{action} {args.output} ({len(rows)} models, sha256={remote_sha[:12]})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
