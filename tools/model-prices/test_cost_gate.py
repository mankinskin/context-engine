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
    {"provider_id": "anthropic", "model_id": "claude-haiku", "output_mtok": 1.0},
]

FIXTURE_ROLLUP = {
    "schema_version": 1,
    "report": {
        "tools": [
            {"tool_name": "read_file", "call_count": 10, "cost": 80},
            {"tool_name": "grep_search", "call_count": 3, "cost": 50},  # insufficient
        ]
    },
}


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
    """Legacy evaluate() tests using evaluate_legacy()."""

    def test_expensive_model_token_heavy_delegates(self):
        d = cg.evaluate_legacy("claude-opus-4-1", "read_file", models=FIXTURE_MODELS)
        self.assertEqual(d["decision"], "delegate")
        self.assertTrue(d["orchestrator"])
        self.assertIn("delegate", (d.get("guidance") or "").lower())
        self.assertIn("runSubagent", d.get("guidance") or "")

    def test_expensive_model_light_tool_allows(self):
        d = cg.evaluate_legacy("o3", "runSubagent", models=FIXTURE_MODELS)
        self.assertEqual(d["decision"], "allow")
        self.assertTrue(d["orchestrator"])

    def test_boundary_sonnet_allows_token_heavy(self):
        # out=15 is NOT strictly greater than X=15 -> not an orchestrator.
        d = cg.evaluate_legacy("claude-sonnet-4-5", "read_file", models=FIXTURE_MODELS)
        self.assertEqual(d["decision"], "allow")
        self.assertFalse(d["orchestrator"])

    def test_cheap_model_token_heavy_allows(self):
        d = cg.evaluate_legacy("gpt-5-mini", "grep_search", models=FIXTURE_MODELS)
        self.assertEqual(d["decision"], "allow")
        self.assertFalse(d["orchestrator"])

    def test_unknown_model_conservative_delegates_token_heavy(self):
        d = cg.evaluate_legacy("mystery-model", "semantic_search", models=FIXTURE_MODELS)
        self.assertEqual(d["decision"], "delegate")
        self.assertTrue(d["orchestrator"])

    def test_unknown_model_opt_out_allows(self):
        d = cg.evaluate_legacy(
            "mystery-model",
            "semantic_search",
            models=FIXTURE_MODELS,
            unknown_model_orchestrates=False,
        )
        self.assertEqual(d["decision"], "allow")


class CliTests(unittest.TestCase):
    def setUp(self):
        self.table = _fixture_file()

    def tearDown(self):
        self.table.unlink(missing_ok=True)

    def test_cli_delegate_exit_code(self):
        code = cg.main(
            ["--model", "claude-opus-4-1", "--tool", "read_file",
             "--price-table", str(self.table), "--format", "json", "--legacy"]
        )
        self.assertEqual(code, cg.EXIT_DELEGATE)

    def test_cli_allow_exit_code(self):
        code = cg.main(
            ["--model", "gpt-5-mini", "--tool", "read_file",
             "--price-table", str(self.table), "--legacy"]
        )
        self.assertEqual(code, cg.EXIT_ALLOW)

    def test_cli_missing_table_errors(self):
        code = cg.main(
            ["--model", "gpt-5", "--tool", "read_file",
             "--price-table", "does-not-exist.json", "--legacy"]
        )
        self.assertEqual(code, cg.EXIT_ERROR)


class BaseBudgetTests(unittest.TestCase):
    def test_linear_inverse(self):
        cal = cg.ModelBudgetCalibration()
        # Haiku (1): high budget
        haiku = cg.base_budget(1.0, cal)
        self.assertGreaterEqual(haiku, 90)
        self.assertLessEqual(haiku, 100)
        # Sonnet (15): mid budget
        sonnet = cg.base_budget(15.0, cal)
        self.assertGreaterEqual(sonnet, 70)
        self.assertLessEqual(sonnet, 80)
        # Opus-4-5 (25): lower
        opus = cg.base_budget(25.0, cal)
        self.assertGreaterEqual(opus, 50)
        self.assertLessEqual(opus, 65)
        # Opus-4-1 (75): near zero (above budget_zero_price=60)
        opus_old = cg.base_budget(75.0, cal)
        self.assertEqual(opus_old, 0)
        # Unknown: conservative 0
        self.assertEqual(cg.base_budget(None, cal), 0)


class ToolCostTests(unittest.TestCase):
    def test_static_fallback(self):
        cal = cg.ModelBudgetCalibration()
        self.assertEqual(cg.tool_cost("runSubagent", None, cal), 0)  # always allowed
        self.assertEqual(cg.tool_cost("read_file", None, cal), 75)  # token heavy (budget at X=15)
        self.assertEqual(cg.tool_cost("some_unknown_tool", None, cal), 1)  # light

    def test_from_rollup(self):
        cal = cg.ModelBudgetCalibration()
        self.assertEqual(cg.tool_cost("read_file", FIXTURE_ROLLUP, cal), 80)  # from rollup
        self.assertEqual(cg.tool_cost("grep_search", FIXTURE_ROLLUP, cal), 75)  # insufficient -> fallback (75)
        self.assertEqual(cg.tool_cost("runSubagent", FIXTURE_ROLLUP, cal), 0)  # always allowed bypass


class GrantTests(unittest.TestCase):
    def test_valid_grant(self):
        with tempfile.TemporaryDirectory() as tmpdir:
            grants_dir = Path(tmpdir)
            grant_path = grants_dir / "sess1.json"
            grant_path.write_text(
                json.dumps({"grant_id": "sess1", "offset": 30, "model": "claude-sonnet-4-5"}),
                encoding="utf-8",
            )
            self.assertEqual(cg.load_grant("sess1", "claude-sonnet-4-5", grants_dir), 30)
            self.assertEqual(cg.load_grant("sess1", "other-model", grants_dir), 0)  # model mismatch
            self.assertEqual(cg.load_grant("missing", "claude-sonnet-4-5", grants_dir), 0)  # missing

    def test_expired_grant(self):
        with tempfile.TemporaryDirectory() as tmpdir:
            grants_dir = Path(tmpdir)
            grant_path = grants_dir / "expired.json"
            grant_path.write_text(
                json.dumps({"grant_id": "expired", "offset": 50, "expires_at": "2020-01-01T00:00:00Z"}),
                encoding="utf-8",
            )
            self.assertEqual(cg.load_grant("expired", "any-model", grants_dir), 0)

    def test_malformed_grant(self):
        with tempfile.TemporaryDirectory() as tmpdir:
            grants_dir = Path(tmpdir)
            grant_path = grants_dir / "bad.json"
            grant_path.write_text("{not valid json", encoding="utf-8")
            self.assertEqual(cg.load_grant("bad", "any-model", grants_dir), 0)


class GradedEvaluateTests(unittest.TestCase):
    def test_allow_light_tool(self):
        d = cg.evaluate("claude-haiku", "some_unknown_tool", models=FIXTURE_MODELS)
        self.assertEqual(d.decision, "allow")
        self.assertGreaterEqual(d.base_budget, 90)
        self.assertEqual(d.tool_cost, 1)

    def test_delegate_heavy_tool(self):
        d = cg.evaluate("claude-sonnet-4-5", "read_file", models=FIXTURE_MODELS)
        # Sonnet budget ~75 == heavy cost 75 -> allow at boundary
        self.assertEqual(d.decision, "allow")

    def test_delegate_heavy_tool_expensive_model(self):
        d = cg.evaluate("claude-opus-4-5", "read_file", models=FIXTURE_MODELS)
        # Opus-4-5 budget ~58 < heavy cost 75 -> delegate
        self.assertEqual(d.decision, "delegate")
        self.assertIsNotNone(d.guidance)

    def test_with_grant_offset(self):
        with tempfile.TemporaryDirectory() as tmpdir:
            grants_dir = Path(tmpdir)
            grant_path = grants_dir / "boost.json"
            grant_path.write_text(
                json.dumps({"grant_id": "boost", "offset": 30}),
                encoding="utf-8",
            )
            # Sonnet base ~75 + offset 30 = 105 > 75 heavy tool cost -> allow
            d = cg.evaluate(
                "claude-sonnet-4-5",
                "read_file",
                grant_id="boost",
                models=FIXTURE_MODELS,
                grants_dir=grants_dir,
            )
            self.assertEqual(d.decision, "allow")
            self.assertEqual(d.offset, 30)

    def test_always_allowed_bypass(self):
        d = cg.evaluate("claude-opus-4-1", "runSubagent", models=FIXTURE_MODELS)
        self.assertEqual(d.decision, "allow")
        self.assertEqual(d.tool_cost, 0)

    def test_rust_python_parity(self):
        # Same scenario in both Rust and Python
        d = cg.evaluate("claude-haiku", "some_unknown_tool", models=FIXTURE_MODELS)
        self.assertEqual(d.decision, "allow")
        self.assertGreaterEqual(d.base_budget, 90)  # high budget for cheap model


class HeavyFallbackBoundaryTests(unittest.TestCase):
    def test_heavy_fallback_cost(self):
        cal = cg.ModelBudgetCalibration()
        # With defaults (budget_zero_price=60, X=15), heavy_fallback_cost = 75
        self.assertEqual(cg.heavy_fallback_cost(cal), 75)

    def test_models_with_sufficient_budget_allow_heavy(self):
        heavy_tool = "read_file"
        # Models with output_mtok 1, 10, 15 → Allow (budget >= 75)
        d = cg.evaluate("claude-haiku", heavy_tool, models=FIXTURE_MODELS)  # 1 → budget ~98
        self.assertEqual(d.decision, "allow")
        
        d = cg.evaluate("gpt-5", heavy_tool, models=FIXTURE_MODELS)  # 10 → budget ~83
        self.assertEqual(d.decision, "allow")
        
        d = cg.evaluate("claude-sonnet-4-5", heavy_tool, models=FIXTURE_MODELS)  # 15 → budget 75 (at boundary)
        self.assertEqual(d.decision, "allow")
        self.assertEqual(d.base_budget, 75)
        self.assertEqual(d.tool_cost, 75)

    def test_models_with_insufficient_budget_delegate_heavy(self):
        heavy_tool = "read_file"
        # Models with output_mtok 25, 50, 75 → Delegate (budget < 75)
        d = cg.evaluate("claude-opus-4-5", heavy_tool, models=FIXTURE_MODELS)  # 25 → budget ~58
        self.assertEqual(d.decision, "delegate")
        self.assertLess(d.base_budget, 75)
        
        d = cg.evaluate("o3", heavy_tool, models=FIXTURE_MODELS)  # 40 → budget ~33
        self.assertEqual(d.decision, "delegate")
        self.assertLess(d.base_budget, 75)
        
        d = cg.evaluate("claude-opus-4-1", heavy_tool, models=FIXTURE_MODELS)  # 75 → budget 0
        self.assertEqual(d.decision, "delegate")
        self.assertEqual(d.base_budget, 0)

    def test_unknown_model_delegates_heavy(self):
        heavy_tool = "read_file"
        d = cg.evaluate("unknown-model", heavy_tool, models=FIXTURE_MODELS)
        self.assertEqual(d.decision, "delegate")
        self.assertEqual(d.base_budget, 0)

    def test_light_tool_allows_most_models(self):
        light_tool = "some_light_tool"
        # Light tool (cost 1) → Allow for all models with budget >= 1
        d = cg.evaluate("claude-haiku", light_tool, models=FIXTURE_MODELS)
        self.assertEqual(d.decision, "allow")
        
        d = cg.evaluate("gpt-5", light_tool, models=FIXTURE_MODELS)
        self.assertEqual(d.decision, "allow")
        
        d = cg.evaluate("claude-sonnet-4-5", light_tool, models=FIXTURE_MODELS)
        self.assertEqual(d.decision, "allow")
        
        d = cg.evaluate("claude-opus-4-5", light_tool, models=FIXTURE_MODELS)
        self.assertEqual(d.decision, "allow")
        
        d = cg.evaluate("o3", light_tool, models=FIXTURE_MODELS)
        self.assertEqual(d.decision, "allow")
        
        # Model at/above budget_zero_price (75 > 60) → budget 0, cost 1 → delegate
        d = cg.evaluate("claude-opus-4-1", light_tool, models=FIXTURE_MODELS)
        self.assertEqual(d.decision, "delegate")
        self.assertEqual(d.base_budget, 0)

    def test_always_allowed_bypasses_all_checks(self):
        always_tool = "runSubagent"
        # AlwaysAllowed → Always allow regardless of model
        d = cg.evaluate("claude-haiku", always_tool, models=FIXTURE_MODELS)
        self.assertEqual(d.decision, "allow")
        
        d = cg.evaluate("claude-opus-4-1", always_tool, models=FIXTURE_MODELS)
        self.assertEqual(d.decision, "allow")
        
        d = cg.evaluate("unknown-model", always_tool, models=FIXTURE_MODELS)
        self.assertEqual(d.decision, "allow")


if __name__ == "__main__":
    unittest.main(verbosity=2)
