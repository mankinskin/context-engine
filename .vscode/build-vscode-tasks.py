"""Merge `.vscode/tasks.d/*.jsonc` into `.vscode/tasks.json`.

VS Code only reads a single `tasks.json`, so we keep the source-of-truth
modular under `.vscode/tasks.d/` and run this script to produce the
canonical merged file. Edit the part-files, never `tasks.json` directly.

Usage:
    python .vscode/build-vscode-tasks.py

Exit codes:
    0 — success (tasks.json written or already up-to-date)
    1 — parse error or duplicate label / id
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
TASKS_DIR = REPO_ROOT / ".vscode" / "tasks.d"
OUT_FILE = REPO_ROOT / ".vscode" / "tasks.json"

HEADER = (
    "// ╔══════════════════════════════════════════════════════════════════════╗\n"
    "// ║  GENERATED FILE — DO NOT EDIT BY HAND                                ║\n"
    "// ║  Source: .vscode/tasks.d/*.jsonc                                     ║\n"
    "// ║  Regenerate with: python .vscode/build-vscode-tasks.py               ║\n"
    "// ╚══════════════════════════════════════════════════════════════════════╝\n"
)

# Strip JSONC comments and trailing commas while preserving string
# literals (which may contain `//` like `"http://..."` or `/*`).
_TRAILING_COMMA = re.compile(r",(\s*[}\]])")


def _strip_jsonc(text: str) -> str:
    """Remove JSONC features so json.loads can parse the result.

    String-aware: walks characters and only strips comments outside of
    `"..."` literals, so URLs and other content with `//` survive.
    """
    out: list[str] = []
    i = 0
    n = len(text)
    in_string = False
    while i < n:
        ch = text[i]
        if in_string:
            out.append(ch)
            if ch == "\\" and i + 1 < n:
                # Preserve escaped char verbatim (e.g. \" inside a string).
                out.append(text[i + 1])
                i += 2
                continue
            if ch == '"':
                in_string = False
            i += 1
            continue
        if ch == '"':
            in_string = True
            out.append(ch)
            i += 1
            continue
        if ch == "/" and i + 1 < n:
            nxt = text[i + 1]
            if nxt == "/":
                # Line comment: skip to (but keep) the newline.
                j = text.find("\n", i + 2)
                if j == -1:
                    break
                i = j
                continue
            if nxt == "*":
                # Block comment: skip to terminator.
                j = text.find("*/", i + 2)
                if j == -1:
                    break
                i = j + 2
                continue
        out.append(ch)
        i += 1
    return _TRAILING_COMMA.sub(r"\1", "".join(out))


def main() -> int:
    if not TASKS_DIR.is_dir():
        print(f"error: {TASKS_DIR} does not exist", file=sys.stderr)
        return 1

    parts = sorted(TASKS_DIR.glob("*.jsonc"))
    if not parts:
        print(f"error: no *.jsonc files under {TASKS_DIR}", file=sys.stderr)
        return 1

    all_tasks: list[dict] = []
    all_inputs: list[dict] = []
    seen_labels: dict[str, str] = {}
    seen_input_ids: dict[str, str] = {}

    for part in parts:
        try:
            obj = json.loads(_strip_jsonc(part.read_text(encoding="utf-8")))
        except json.JSONDecodeError as e:
            print(f"error: {part.relative_to(REPO_ROOT)}: {e}", file=sys.stderr)
            return 1

        for task in obj.get("tasks", []) or []:
            label = task.get("label")
            if not label:
                print(
                    f"error: {part.relative_to(REPO_ROOT)}: task without label",
                    file=sys.stderr,
                )
                return 1
            if label in seen_labels:
                print(
                    f"error: duplicate task label '{label}' in "
                    f"{part.relative_to(REPO_ROOT)} (also in {seen_labels[label]})",
                    file=sys.stderr,
                )
                return 1
            seen_labels[label] = str(part.relative_to(REPO_ROOT))
            all_tasks.append(task)

        for inp in obj.get("inputs", []) or []:
            iid = inp.get("id")
            if not iid:
                print(
                    f"error: {part.relative_to(REPO_ROOT)}: input without id",
                    file=sys.stderr,
                )
                return 1
            if iid in seen_input_ids:
                print(
                    f"error: duplicate input id '{iid}' in "
                    f"{part.relative_to(REPO_ROOT)} (also in {seen_input_ids[iid]})",
                    file=sys.stderr,
                )
                return 1
            seen_input_ids[iid] = str(part.relative_to(REPO_ROOT))
            all_inputs.append(inp)

    merged = {"version": "2.0.0", "tasks": all_tasks, "inputs": all_inputs}
    body = json.dumps(merged, indent=2, ensure_ascii=False) + "\n"
    OUT_FILE.write_text(HEADER + body, encoding="utf-8")

    print(
        f"wrote {OUT_FILE.relative_to(REPO_ROOT)} "
        f"({len(all_tasks)} tasks, {len(all_inputs)} inputs from {len(parts)} parts)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
