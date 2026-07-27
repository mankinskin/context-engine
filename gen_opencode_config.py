"""Generate opencode.json from Eden AI's model catalog."""

import json
import requests
from pathlib import Path

MODELS_URL = "https://api.edenai.run/v3/models"
BASE_URL = "https://api.edenai.run/v3"

models = requests.get(MODELS_URL).json()["data"]

def to_per_million(v):
    return round(v * 1_000_000, 6) if v else None

model_entries = {}
for m in models:
    caps = m.get("capabilities", {})
    if not caps.get("supports_function_calling"):
        continue  # opencode requires tool calling

    p = m.get("pricing", {})
    cost = {k: to_per_million(p.get(v)) for k, v in {
        "input": "input_cost_per_token",
        "output": "output_cost_per_token",
        "cache_read": "cache_read_input_token_cost",
        "cache_write": "cache_creation_input_token_cost",
    }.items() if p.get(v)}

    model_entries[m["id"]] = {
        **({"cost": cost} if cost else {}),
        **({"reasoning": True} if caps.get("supports_reasoning") else {}),
    }

config = {
    "$schema": "https://opencode.ai/config.json",
    "provider": {
        "edenai": {
            "npm": "@ai-sdk/openai-compatible",
            "name": "Eden AI",
            "options": {"baseURL": BASE_URL},
            "models": model_entries,
        }
    },
}

output = Path.home() / ".config" / "opencode" / "opencode.json"
output.parent.mkdir(parents=True, exist_ok=True)
output.write_text(json.dumps(config, indent=2))

print(f"Written {len(model_entries)} models to {output}")