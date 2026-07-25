#!/usr/bin/env python3
"""Model-aware cost gate for price-awareness orchestration.

Transport-agnostic enforcement core for the "context stack price awareness"
track. Given the calling model's identity and a tool name, it resolves the
model's output price from the shared price table produced by
``sync_model_prices.py`` (``model_prices.json``) and decides whether the call
may proceed directly or must be delegated to a cheaper sub-agent.

Policy (see AGENTS.md "Model cost awareness & routing"):

* The driving field is ``output_mtok`` (USD per 1,000,000 output tokens).
* The threshold is ``X = 15``. The gate fires when ``output_mtok > X``
  (strictly greater), matching the AGENTS.md rule.
* When the running model is above threshold ("orchestrator mode"), calls to
  *token-heavy* tools are refused with delegation guidance; calls to *light*
  tools (planning / delegation, e.g. ``runSubagent``) pass through.
* Models at or below threshold pass through unchanged.

This module is the single decision point shared by both enforcement transports:

* **Client/extension layer (option C):** the client, which knows the active
  model, shells out to this gate (or imports :func:`evaluate`) before a tool
  call reaches any MCP server. This is the real enforcement surface because the
  model cannot spoof its own identity there.
* **Explicit-parameter wrapper (option A, portable fallback):** a wrapper that
  accepts a ``caller_model`` argument for non-VS-Code transports and calls the
  same :func:`evaluate`.

Cost is always resolved from the shared mapping; it is never hardcoded per tool
or per agent.

Stdlib only. No third-party dependencies.
"""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any

# Default threshold on ``output_mtok`` (USD per 1M output tokens). Equivalent to
# 1500 credits/1M at 100 credits = $1. Keep this in sync with the AGENTS.md
# "Orchestrator-mode threshold" rule.
DEFAULT_THRESHOLD_X = 15.0

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
    output_mtok: float | None
    threshold_x: float
    orchestrator: bool
    tool_class: str
    reason: str
    guidance: str | None

    def to_dict(self) -> dict[str, Any]:
        return {
            "decision": self.decision,
            "model": self.model,
            "tool": self.tool,
            "output_mtok": self.output_mtok,
            "threshold_x": self.threshold_x,
            "orchestrator": self.orchestrator,
            "tool_class": self.tool_class,
            "reason": self.reason,
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
    models: list[dict[str, Any]] | None = None,
    x: float = DEFAULT_THRESHOLD_X,
    *,
    price_table: Path = DEFAULT_PRICE_TABLE,
    unknown_model_orchestrates: bool = True,
) -> Decision:
    """Decide whether ``model`` may call ``tool`` directly.

    * Below/at threshold -> ``allow`` (cheap model may execute directly).
    * Above threshold + token-heavy tool -> ``delegate`` (refused with guidance).
    * Above threshold + light/always-allowed tool -> ``allow`` (planning /
      delegation stays on the expensive model).

    When the model is unknown to the price table, the gate is conservative by
    default (``unknown_model_orchestrates=True``): it treats the model as an
    orchestrator so an unlisted expensive model is not silently allowed to run
    token-heavy work.
    """
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
        return Decision(
            "allow", model, tool, output_mtok, x, orchestrator, tool_class, reason, None
        )

    if tool_class in ("always_allowed", "light"):
        return Decision(
            "allow",
            model,
            tool,
            output_mtok,
            x,
            orchestrator,
            tool_class,
            f"orchestrator model, '{tool_class}' tool; planning/delegation allowed",
            None,
        )

    # orchestrator + token_heavy -> refuse with delegation guidance.
    return Decision(
        "delegate",
        model,
        tool,
        output_mtok,
        x,
        orchestrator,
        tool_class,
        "orchestrator model must delegate token-heavy tool to a cheaper sub-agent",
        _delegation_guidance(model, tool),
    )


# Exit codes for shell integration (option C client hook / option A wrapper).
EXIT_ALLOW = 0
EXIT_DELEGATE = 3
EXIT_ERROR = 2


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--model", required=True, help="Calling model id (e.g. claude-opus-4-1).")
    parser.add_argument("--tool", required=True, help="Tool name being invoked.")
    parser.add_argument(
        "--x",
        type=float,
        default=DEFAULT_THRESHOLD_X,
        help=f"Threshold on output_mtok (default: {DEFAULT_THRESHOLD_X:g}).",
    )
    parser.add_argument(
        "--price-table",
        type=Path,
        default=DEFAULT_PRICE_TABLE,
        help="Path to model_prices.json (default: next to this script).",
    )
    parser.add_argument(
        "--format",
        choices=("text", "json"),
        default="text",
        help="Output format (default: text).",
    )
    args = parser.parse_args(argv)

    try:
        decision = evaluate(args.model, args.tool, x=args.x, price_table=args.price_table)
    except CostGateError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return EXIT_ERROR

    if args.format == "json":
        print(json.dumps(decision.to_dict(), indent=2, ensure_ascii=False))
    else:
        price = "unknown" if decision.output_mtok is None else f"{decision.output_mtok:g}"
        print(f"decision: {decision.decision}")
        print(f"model:    {decision.model} (output_mtok={price}, x={decision.threshold_x:g})")
        print(f"tool:     {decision.tool} [{decision.tool_class}]")
        print(f"reason:   {decision.reason}")
        if decision.guidance:
            print(f"guidance: {decision.guidance}")

    return EXIT_DELEGATE if decision.decision == "delegate" else EXIT_ALLOW


if __name__ == "__main__":  # pragma: no cover
    raise SystemExit(main())
