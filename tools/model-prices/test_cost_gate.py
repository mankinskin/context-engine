#!/usr/bin/env python3
"""Tests for the model-aware cost gate (cost_gate.py).

Uses an inline price fixture so the assertions are deterministic and do not
depend on the current upstream sync. Run with:

    python tools/model-prices/test_cost_gate.py
    python -m pytest tools/model-prices/test_cost_gate.py
"""

from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

import cost_gate as cg

# Inline fixture mirroring model_prices.json shape. Output prices chosen to
# straddle the X = 15 threshold.
FIXTURE_MODELS = [
    {"provider_id": "anthropic", "model_id": "claude-opus-4-1", "output_mtok": 75.0},
    {"provider_id": "anthropic", "model_id": "claude-opus-4-5", "output_mtok": 25.0},
    {"provider_id": "openai", "model_id": "o3", "output_mtok": 40.0},
    {"provider_id": "anthropic", "model_id": "claude-sonnet-4-5", "output_mtok": 15.0},
    {"provider_id": "openai", "model_id": "gpt-5", "output_mtok": 10.0},
    {"provider_id": "openai", "model_id": "gpt-5-mini", "output_mtok": 2.0},
]


def _fixture_file() -> Path:
    tmp = tempfile.NamedTemporaryFile(
        mode="w", suffix=".json", delete=False, encoding="utf-8"
    )
    json.dump({"_meta": {}, "models": FIXTURE_MODELS}, tmp)
    tmp.close()
    return Path(tmp.name)


class ResolveOutputMtokTests(unittest.TestCase):
    def test_exact_model_id_match(self):
        self.assertEqual(cg.resolve_output_mtok(FIXTURE_MODELS, "claude-opus-4-1"), 75.0)

    def test_case_insensitive(self):
        self.assertEqual(cg.resolve_output_mtok(FIXTURE_MODELS, "GPT-5-MINI"), 2.0)

    def test_substring_prefers_max_when_ambiguous(self):
        # "claude-opus" matches both 75 and 25; conservative gate returns max.
        self.assertEqual(cg.resolve_output_mtok(FIXTURE_MODELS, "claude-opus"), 75.0)

    def test_exact_wins_over_substring(self):
        # Exact "gpt-5" must not be pulled up to gpt-5-mini's value or vice versa.
        self.assertEqual(cg.resolve_output_mtok(FIXTURE_MODELS, "gpt-5"), 10.0)

    def test_unknown_model_returns_none(self):
        self.assertIsNone(cg.resolve_output_mtok(FIXTURE_MODELS, "no-such-model"))


class ThresholdTests(unittest.TestCase):
    def test_strictly_greater(self):
        self.assertTrue(cg.is_orchestrator(15.0001))
        self.assertFalse(cg.is_orchestrator(15.0))  # boundary: not strictly greater
        self.assertFalse(cg.is_orchestrator(14.9999))


class ClassifyToolTests(unittest.TestCase):
    def test_token_heavy(self):
        self.assertEqual(cg.classify_tool("read_file"), "token_heavy")
        self.assertEqual(cg.classify_tool("mcp_ticket-mcp_get_ticket_description"), "token_heavy")

    def test_always_allowed_wins(self):
        self.assertEqual(cg.classify_tool("runSubagent"), "always_allowed")
        self.assertEqual(cg.classify_tool("board_check_in"), "always_allowed")

    def test_light_default(self):
        self.assertEqual(cg.classify_tool("some_unknown_tool"), "light")


class EvaluateTests(unittest.TestCase):
    def test_expensive_model_token_heavy_delegates(self):
        d = cg.evaluate("claude-opus-4-1", "read_file", models=FIXTURE_MODELS)
        self.assertEqual(d.decision, "delegate")
        self.assertTrue(d.orchestrator)
        self.assertIn("delegate", (d.guidance or "").lower())
        self.assertIn("runSubagent", d.guidance or "")

    def test_expensive_model_light_tool_allows(self):
        d = cg.evaluate("o3", "runSubagent", models=FIXTURE_MODELS)
        self.assertEqual(d.decision, "allow")
        self.assertTrue(d.orchestrator)

    def test_boundary_sonnet_allows_token_heavy(self):
        # out=15 is NOT strictly greater than X=15 -> not an orchestrator.
        d = cg.evaluate("claude-sonnet-4-5", "read_file", models=FIXTURE_MODELS)
        self.assertEqual(d.decision, "allow")
        self.assertFalse(d.orchestrator)

    def test_cheap_model_token_heavy_allows(self):
        d = cg.evaluate("gpt-5-mini", "grep_search", models=FIXTURE_MODELS)
        self.assertEqual(d.decision, "allow")
        self.assertFalse(d.orchestrator)

    def test_unknown_model_conservative_delegates_token_heavy(self):
        d = cg.evaluate("mystery-model", "semantic_search", models=FIXTURE_MODELS)
        self.assertEqual(d.decision, "delegate")
        self.assertTrue(d.orchestrator)

    def test_unknown_model_opt_out_allows(self):
        d = cg.evaluate(
            "mystery-model",
            "semantic_search",
            models=FIXTURE_MODELS,
            unknown_model_orchestrates=False,
        )
        self.assertEqual(d.decision, "allow")


class CliTests(unittest.TestCase):
    def setUp(self):
        self.table = _fixture_file()

    def tearDown(self):
        self.table.unlink(missing_ok=True)

    def test_cli_delegate_exit_code(self):
        code = cg.main(
            ["--model", "claude-opus-4-1", "--tool", "read_file",
             "--price-table", str(self.table), "--format", "json"]
        )
        self.assertEqual(code, cg.EXIT_DELEGATE)

    def test_cli_allow_exit_code(self):
        code = cg.main(
            ["--model", "gpt-5-mini", "--tool", "read_file",
             "--price-table", str(self.table)]
        )
        self.assertEqual(code, cg.EXIT_ALLOW)

    def test_cli_missing_table_errors(self):
        code = cg.main(
            ["--model", "gpt-5", "--tool", "read_file",
             "--price-table", "does-not-exist.json"]
        )
        self.assertEqual(code, cg.EXIT_ERROR)


if __name__ == "__main__":
    unittest.main(verbosity=2)
