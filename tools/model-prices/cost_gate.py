#!/usr/bin/env python3
"""Model-aware cost gate for price-awareness orchestration.

Transport-agnostic enforcement core using a graded budget model with empirical
tool costs. Given the calling model's identity and a tool name, it resolves the
model's base budget from output price, applies optional grant offsets, and
decides whether the call may proceed or must be delegated. Policy (see AGENTS.md):

* base_budget(model): LINEAR inverse of output_mtok on a 1..100 scale
* tool cost: empirical rollup (if sufficient data) else static fallback
* offset grants: optional per-session/subagent budget boosts
* Decision: Allow if cost <= (base_budget + offset); otherwise Delegate

Stdlib only. No third-party dependencies.
"""

from __future__ import annotations

import argparse
import json
import os
import sys
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

# Default threshold (kept for backward compatibility and as budget_zero_price default).
DEFAULT_THRESHOLD_X = 15.0

# Minimum call count for using empirical tool cost from rollup.
MIN_CALLS = 5

DEFAULT_PRICE_TABLE = Path(__file__).with_name("model_prices.json")

# Tools that consume large amounts of context/output tokens when driven directly
# by an expensive model. Matching is a case-insensitive substring test so that
# provider-prefixed tool names (e.g. ``mcp_ticket-mcp_get_ticket``) are covered.
# This is the classification an expensive model must delegate rather than run.
TOKEN_HEAVY_TOOL_SUBSTRINGS: tuple[str, ...] = (
    "read_file",
    "read_notebook_cell_output",
    "semantic_search",
    "grep_search",
    "file_search",
    "list_dir",
    "fetch_webpage",
    "get_log",
    "query_logs",
    "search_all_logs",
    "get_source",
    "peek_read",
    "peek_grep",
    "peek_skeleton",
    "get_ticket_description",
    "spec_get",
    "spec_section_get",
    "session_peek_range",
    "session_peek_skeleton",
    "subgraph",
    "topgraph",
)

# Tools that are always allowed even in orchestrator mode: planning, delegation,
# and lightweight status/mutation calls whose token footprint is small. The
# sub-agent spawn primitive must always be allowed so the orchestrator can
# actually delegate.
ALWAYS_ALLOWED_TOOL_SUBSTRINGS: tuple[str, ...] = (
    "runsubagent",
    "run_subagent",
    "board_check_in",
    "board_check_out",
    "board_heartbeat",
    "update_ticket",
    "workflow_set_status",
)


class CostGateError(Exception):
    """Raised when the price table cannot be loaded or the model is unknown."""


@dataclass(frozen=True)
class ModelBudgetCalibration:
    """Model budget calibration for the graded cost scale."""

    scale_max: int = 100
    budget_zero_price: float = 60.0  # TODO: provisional anchor; re-tune from empirical data


@dataclass(frozen=True)
class Grant:
    """Grant record for budget offsets."""

    offset: int
    model: str | None = None
    expires_at: str | None = None


def load_models(path: Path = DEFAULT_PRICE_TABLE) -> list[dict[str, Any]]:
    """Load the model rows from the shared price table.

    Raises :class:`CostGateError` when the table is missing or malformed.
    """
    if not path.exists():
        raise CostGateError(
            f"{path} not found; run sync_model_prices.py first (see --help)."
        )
    try:
        document = json.loads(path.read_text(encoding="utf-8"))
    except (json.JSONDecodeError, OSError) as exc:  # pragma: no cover - IO edge
        raise CostGateError(f"cannot read {path}: {exc}") from exc
    models = document.get("models", []) if isinstance(document, dict) else []
    if not isinstance(models, list):
        raise CostGateError(f"{path} has no usable 'models' array")
    return models


def resolve_output_mtok(models: list[dict[str, Any]], model: str) -> float | None:
    """Resolve the ``output_mtok`` for ``model`` from the price rows.

    Matching mirrors ``sync_model_prices.py --query``: a case-insensitive
    substring on ``provider_id`` and ``model_id``. An exact (case-insensitive)
    ``model_id`` match is preferred. When several rows match and none is exact,
    the gate is conservative and returns the **maximum** ``output_mtok`` among
    matches, so an ambiguous identifier is never cheaper than its most expensive
    variant. Returns ``None`` when no row matches or matches lack a price.
    """
    low = model.lower()

    exact = [
        r
        for r in models
        if str(r.get("model_id", "")).lower() == low
        and isinstance(r.get("output_mtok"), (int, float))
    ]
    if exact:
        return max(float(r["output_mtok"]) for r in exact)

    matches = [
        r
        for r in models
        if (low in str(r.get("provider_id", "")).lower()
            or low in str(r.get("model_id", "")).lower())
        and isinstance(r.get("output_mtok"), (int, float))
    ]
    if not matches:
        return None
    return max(float(r["output_mtok"]) for r in matches)


def base_budget(
    output_mtok: float | None, cal: ModelBudgetCalibration = ModelBudgetCalibration()
) -> int:
    """Compute base_budget from model's output_mtok using linear inverse mapping.
    Returns a value in [0, scale_max]. Unknown model (None) → 0 (conservative).
    """
    if output_mtok is None:
        return 0
    ratio = 1.0 - (output_mtok / cal.budget_zero_price)
    scaled = round(ratio * cal.scale_max)
    return max(0, min(scaled, cal.scale_max))


def heavy_fallback_cost(
    cal: ModelBudgetCalibration = ModelBudgetCalibration(),
) -> int:
    """Compute the static fallback cost for TokenHeavy tools as the base budget
    at the legacy threshold X. This ensures the fallback reproduces the binary
    boundary and auto-tracks calibration changes.
    """
    ratio = 1.0 - (DEFAULT_THRESHOLD_X / cal.budget_zero_price)
    scaled = round(ratio * cal.scale_max)
    return max(0, min(scaled, cal.scale_max))


def tool_cost_from_rollup(
    tool: str, rollup: dict[str, Any] | None, cal: ModelBudgetCalibration
) -> int | None:
    """Resolve tool cost from rollup if sufficient data exists. Returns None for fallback."""
    if not rollup:
        return None
    tools = rollup.get("report", {}).get("tools", [])
    tool_low = tool.lower()
    matches = [
        t
        for t in tools
        if (tool_low in str(t.get("tool_name", "")).lower()
            or str(t.get("tool_name", "")).lower() in tool_low)
        and t.get("call_count", 0) >= MIN_CALLS
        and isinstance(t.get("cost"), int)
    ]
    if matches:
        return max(t["cost"] for t in matches)
    return None


def tool_cost(
    tool: str,
    rollup: dict[str, Any] | None,
    cal: ModelBudgetCalibration = ModelBudgetCalibration(),
) -> int:
    """Resolve tool cost: empirical rollup (if sufficient data) else static fallback.
    AlwaysAllowed tools always return 0 (bypass budget check).
    """
    tool_class = classify_tool(tool)
    if tool_class == "always_allowed":
        return 0
    # Try rollup
    cost = tool_cost_from_rollup(tool, rollup, cal)
    if cost is not None:
        return cost
    # Static fallback
    if tool_class == "token_heavy":
        return heavy_fallback_cost(cal)
    return 1  # light


def load_grant(
    grant_id: str, model: str, grants_dir: Path | None
) -> int:
    """Load grant offset from grants_dir/<grant_id>.json. Returns 0 on any error."""
    if not grants_dir:
        return 0
    grant_path = grants_dir / f"{grant_id}.json"
    try:
        grant_data = json.loads(grant_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return 0
    
    offset = grant_data.get("offset", 0)
    if not isinstance(offset, int):
        return 0
    
    # Check expiry
    if expires := grant_data.get("expires_at"):
        try:
            exp_time = datetime.fromisoformat(expires.replace("Z", "+00:00"))
            if exp_time < datetime.now(timezone.utc):
                return 0
        except (ValueError, AttributeError):
            return 0
    
    # Check model match
    if grant_model := grant_data.get("model"):
        if str(grant_model).lower() != model.lower():
            return 0
    
    return offset


def is_orchestrator(output_mtok: float, x: float = DEFAULT_THRESHOLD_X) -> bool:
    """Return True when ``output_mtok`` is strictly greater than threshold ``x``."""
    return output_mtok > x


def classify_tool(
    tool: str,
    token_heavy: tuple[str, ...] = TOKEN_HEAVY_TOOL_SUBSTRINGS,
    always_allowed: tuple[str, ...] = ALWAYS_ALLOWED_TOOL_SUBSTRINGS,
) -> str:
    """Classify ``tool`` as ``"always_allowed"``, ``"token_heavy"``, or ``"light"``.

    ``always_allowed`` wins over ``token_heavy`` so the delegation primitive and
    lightweight status calls are never blocked.
    """
    low = tool.lower()
    if any(sub in low for sub in always_allowed):
        return "always_allowed"
    if any(sub in low for sub in token_heavy):
        return "token_heavy"
    return "light"


@dataclass(frozen=True)
class Decision:
    """Outcome of a cost-gate evaluation."""

    decision: str  # "allow" | "delegate"
    model: str
    tool: str
    base_budget: int
    tool_cost: int
    offset: int
    effective_budget: int
    guidance: str | None

    def to_dict(self) -> dict[str, Any]:
        return {
            "decision": self.decision,
            "model": self.model,
            "tool": self.tool,
            "base_budget": self.base_budget,
            "tool_cost": self.tool_cost,
            "offset": self.offset,
            "effective_budget": self.effective_budget,
            "guidance": self.guidance,
        }


def _delegation_guidance(model: str, tool: str) -> str:
    return (
        f"Model '{model}' exceeds the orchestrator threshold "
        f"(output_mtok > {DEFAULT_THRESHOLD_X:g} USD/1M). Do not call the "
        f"token-heavy tool '{tool}' directly. Delegate it to a cheaper "
        f"sub-agent via runSubagent(model=<cheaper>, ...) and aggregate the "
        f"result. Reserve this model for strategic decisions, code/change "
        f"planning, and tool-call planning."
    )


def evaluate(
    model: str,
    tool: str,
    grant_id: str | None = None,
    models: list[dict[str, Any]] | None = None,
    rollup: dict[str, Any] | None = None,
    *,
    price_table: Path = DEFAULT_PRICE_TABLE,
    calibration: ModelBudgetCalibration = ModelBudgetCalibration(),
    grants_dir: Path | None = None,
) -> Decision:
    """Decide whether ``model`` may call ``tool`` directly using graded budget model.

    * AlwaysAllowed tool → Allow (bypass budget).
    * Compute: base_budget, tool_cost, offset.
    * effective = base_budget + offset (capped at 2*scale_max).
    * Allow if cost <= effective; else Delegate with guidance.
    """
    rows = models if models is not None else load_models(price_table)
    output_mtok = resolve_output_mtok(rows, model)
    
    cost = tool_cost(tool, rollup, calibration)
    if cost == 0:
        # Always allowed tool
        return Decision(
            "allow", model, tool, 0, 0, 0, 0, None
        )
    
    base = base_budget(output_mtok, calibration)
    offset = load_grant(grant_id, model, grants_dir) if grant_id else 0
    effective = min(base + offset, 2 * calibration.scale_max)
    
    if cost <= effective:
        return Decision(
            "allow", model, tool, base, cost, offset, effective, None
        )
    
    guidance = (
        f"Tool '{tool}' requires cost {cost} but model '{model}' has effective "
        f"budget {effective} (base {base} + offset {offset}). An offset grant or "
        f"delegation to a cheaper model is required. Delegate via "
        f"runSubagent(model=<cheaper>, ...)."
    )
    return Decision(
        "delegate", model, tool, base, cost, offset, effective, guidance
    )


def evaluate_legacy(
    model: str,
    tool: str,
    models: list[dict[str, Any]] | None = None,
    x: float = DEFAULT_THRESHOLD_X,
    *,
    price_table: Path = DEFAULT_PRICE_TABLE,
    unknown_model_orchestrates: bool = True,
) -> dict[str, Any]:
    """Legacy evaluate for backward compatibility. Returns old Decision dict format."""
    rows = models if models is not None else load_models(price_table)
    output_mtok = resolve_output_mtok(rows, model)
    tool_class = classify_tool(tool)

    if output_mtok is None:
        orchestrator = unknown_model_orchestrates
    else:
        orchestrator = is_orchestrator(output_mtok, x)

    if not orchestrator:
        reason = (
            "model at or below threshold; direct execution allowed"
            if output_mtok is not None
            else "unknown model treated as below threshold; direct execution allowed"
        )
        return {
            "decision": "allow",
            "model": model,
            "tool": tool,
            "output_mtok": output_mtok,
            "threshold_x": x,
            "orchestrator": orchestrator,
            "tool_class": tool_class,
            "reason": reason,
            "guidance": None,
        }

    if tool_class in ("always_allowed", "light"):
        return {
            "decision": "allow",
            "model": model,
            "tool": tool,
            "output_mtok": output_mtok,
            "threshold_x": x,
            "orchestrator": orchestrator,
            "tool_class": tool_class,
            "reason": f"orchestrator model, '{tool_class}' tool; planning/delegation allowed",
            "guidance": None,
        }

    # orchestrator + token_heavy -> refuse with delegation guidance.
    guidance = _delegation_guidance(model, tool)
    return {
        "decision": "delegate",
        "model": model,
        "tool": tool,
        "output_mtok": output_mtok,
        "threshold_x": x,
        "orchestrator": orchestrator,
        "tool_class": tool_class,
        "reason": "orchestrator model must delegate token-heavy tool to a cheaper sub-agent",
        "guidance": guidance,
    }


# Exit codes for shell integration (option C client hook / option A wrapper).
EXIT_ALLOW = 0
EXIT_DELEGATE = 3
EXIT_ERROR = 2


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--model", required=True, help="Calling model id (e.g. claude-opus-4-1).")
    parser.add_argument("--tool", required=True, help="Tool name being invoked.")
    parser.add_argument("--grant-id", help="Optional grant id for budget offset.")
    parser.add_argument(
        "--price-table",
        type=Path,
        default=DEFAULT_PRICE_TABLE,
        help="Path to model_prices.json (default: next to this script).",
    )
    parser.add_argument(
        "--rollup",
        type=Path,
        help="Path to tool metrics rollup JSON (optional).",
    )
    parser.add_argument(
        "--grants-dir",
        type=Path,
        help="Directory with grant JSON files (optional).",
    )
    parser.add_argument(
        "--scale-max",
        type=int,
        default=100,
        help="Budget scale max (default: 100).",
    )
    parser.add_argument(
        "--budget-zero-price",
        type=float,
        default=60.0,
        help="Price at which budget=0 (default: 60.0).",
    )
    parser.add_argument(
        "--format",
        choices=("text", "json"),
        default="text",
        help="Output format (default: text).",
    )
    parser.add_argument(
        "--legacy",
        action="store_true",
        help="Use legacy evaluation (backward compatibility).",
    )
    args = parser.parse_args(argv)

    try:
        if args.legacy:
            result = evaluate_legacy(args.model, args.tool, price_table=args.price_table)
            decision_str = result["decision"]
            guidance = result.get("guidance")
        else:
            rollup_data = None
            if args.rollup and args.rollup.exists():
                rollup_data = json.loads(args.rollup.read_text(encoding="utf-8"))
            
            calibration = ModelBudgetCalibration(
                scale_max=args.scale_max,
                budget_zero_price=args.budget_zero_price,
            )
            
            decision = evaluate(
                args.model,
                args.tool,
                grant_id=args.grant_id,
                price_table=args.price_table,
                rollup=rollup_data,
                calibration=calibration,
                grants_dir=args.grants_dir,
            )
            result = decision.to_dict()
            decision_str = decision.decision
            guidance = decision.guidance
    except CostGateError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return EXIT_ERROR

    if args.format == "json":
        print(json.dumps(result, indent=2, ensure_ascii=False))
    else:
        print(f"decision: {decision_str}")
        if args.legacy:
            price = "unknown" if result.get("output_mtok") is None else f"{result['output_mtok']:g}"
            print(f"model:    {result['model']} (output_mtok={price})")
            print(f"tool:     {result['tool']} [{result['tool_class']}]")
            print(f"reason:   {result['reason']}")
        else:
            print(f"model:    {result['model']}")
            print(f"tool:     {result['tool']}")
            print(f"base:     {result['base_budget']}")
            print(f"cost:     {result['tool_cost']}")
            print(f"offset:   {result['offset']}")
            print(f"effective: {result['effective_budget']}")
        if guidance:
            print(f"guidance: {guidance}")

    return EXIT_DELEGATE if decision_str == "delegate" else EXIT_ALLOW


if __name__ == "__main__":  # pragma: no cover
    raise SystemExit(main())
