"""
gen_repo_map.py — Generate .agent/repo_map.toon

Compact workspace structural map for low-token agent orientation.
Agents should read repo_map.toon before any exploratory file scanning.

Usage:
    python3 .agent/gen_repo_map.py

Refresh triggers:
    - After adding or removing workspace crates (Cargo.toml members change)
    - After adding agent instructions or prompts
    - Via pre-commit hook (optional): add a call to this script in .githooks/pre-commit
"""
import os
import re
from pathlib import Path

ROOT = Path(os.environ.get("REPO_ROOT", ".")).resolve()
OUTPUT = ROOT / ".agent" / "repo_map.toon"


def get_workspace_members():
    cargo_toml = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    members_match = re.search(r"members\s*=\s*\[(.*?)\]", cargo_toml, re.DOTALL)
    if not members_match:
        return []
    raw = members_match.group(1)
    return re.findall(r'"([^"]+)"', raw)


def get_crate_name(member_path):
    cargo_toml_path = ROOT / member_path / "Cargo.toml"
    if not cargo_toml_path.exists():
        return None
    content = cargo_toml_path.read_text(encoding="utf-8")
    m = re.search(r'^\s*name\s*=\s*"([^"]+)"', content, re.MULTILINE)
    return m.group(1) if m else None


def get_top_level_dirs():
    dirs = []
    for entry in sorted(ROOT.iterdir()):
        if entry.is_dir() and not entry.name.startswith(".") and entry.name not in ("target",):
            dirs.append(entry.name)
    return dirs


def get_git_hooks():
    hooks = []
    for hook_dir in [ROOT / ".githooks", ROOT / ".github" / "hooks"]:
        if hook_dir.exists():
            for f in sorted(hook_dir.iterdir()):
                if f.is_file():
                    hooks.append(str(f.relative_to(ROOT)).replace("\\", "/"))
    return hooks


def get_agent_files():
    """Return instruction and prompt files under .agents/."""
    files = []
    agents_dir = ROOT / ".agents"
    if agents_dir.exists():
        for category in ["instructions", "prompts", "skills"]:
            subdir = agents_dir / category
            if subdir.exists():
                for f in sorted(subdir.iterdir()):
                    if f.is_file():
                        files.append(str(f.relative_to(ROOT)).replace("\\", "/"))
    return files


def main():
    members = get_workspace_members()
    crates = []
    for m in members:
        name = get_crate_name(m)
        if name:
            crates.append((name, m))

    top_dirs = get_top_level_dirs()
    hooks = get_git_hooks()
    agent_files = get_agent_files()

    lines = []
    lines.append("# repo_map.toon — compact workspace structural map")
    lines.append("# Refresh: python3 .agent/gen_repo_map.py")
    lines.append("# Usage: read this before opening source files for structural orientation")
    lines.append("")

    lines.append("## workspace")
    lines.append(f"root={ROOT}")
    lines.append("")

    lines.append("## top-level-dirs")
    for d in top_dirs:
        lines.append(f"  {d}/")
    lines.append("")

    lines.append("## crates  (name  path)")
    for name, path in sorted(crates):
        lines.append(f"  {name}  {path}")
    lines.append("")

    lines.append("## agent-guidance")
    lines.append("  AGENTS.md                        global agent rules")
    lines.append("  .agents/instructions/token-efficiency.instructions.md  bounded-read and compact-output rules")
    if (ROOT / "CHEAT_SHEET.md").exists():
        lines.append("  CHEAT_SHEET.md                   API patterns and gotchas")
    lines.append("")

    lines.append("## agent-files")
    for f in agent_files:
        lines.append(f"  {f}")
    lines.append("")

    lines.append("## hooks")
    for h in hooks:
        lines.append(f"  {h}")
    lines.append("")

    lines.append("## key-tools")
    lines.append("  target/debug/ticket.exe   ticket-cli (state machine, board, deps)")
    lines.append("  target/debug/spec.exe     spec-cli")
    lines.append("  target/debug/peek         bounded file inspection (--start/--end/--grep/--count)")
    lines.append("  rtk <cmd>                 token-optimized CLI proxy (auto-compress output)")
    lines.append("")

    lines.append("## bounded-inspection-pattern")
    lines.append("  peek <file> --count           # 1. learn size")
    lines.append("  peek <file> --grep <pattern>  # 2. locate target line")
    lines.append("  peek <file> --start N --end M # 3. bounded read")
    lines.append("  peek <file> --all             # 4. escape hatch (token-expensive, use sparingly)")
    lines.append("")

    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"Written {OUTPUT} ({len(crates)} crates, {len(agent_files)} agent files)")


if __name__ == "__main__":
    main()
