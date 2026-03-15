#!/usr/bin/env python3
"""
apply_ts_export_macro.py
========================
Replaces every ts-gen cfg_attr pair *together with all preceding attributes
and the item they decorate* with a single ``ts_export! { ... }`` invocation.

The macro is defined in crates/context-api/src/lib.rs and expands back to the
two cfg_attr lines, so no behaviour changes — the export_to path string moves
from N call-sites down to exactly one place.

Before
------

    #[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
    #[cfg_attr(feature = "ts-gen", derive(ts_rs::TS))]
    #[cfg_attr(
        feature = "ts-gen",
        ts(
            export,
            export_to = "../../../../packages/context-types/src/generated/"
        )
    )]
    #[serde(rename_all = "snake_case")]
    pub enum Foo { ... }

After
-----

    ts_export! {
        #[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
        #[serde(rename_all = "snake_case")]
        pub enum Foo { ... }
    }

Algorithm
---------
We scan the token stream line-by-line to find *attribute blocks* — contiguous
runs of lines that start with ``#[`` (or are continuation lines of a
multi-line attribute).  When an attribute block contains the two ts-gen
cfg_attr lines we:

  1. Remove those two cfg_attr lines from the block.
  2. Collect all remaining attributes in the block.
  3. Find the end of the item that follows (brace-balanced or semicolon).
  4. Emit  ``ts_export! {\\n<attrs>\\n<item>\\n}``

Usage
-----
    python3 scripts/apply_ts_export_macro.py [--dry-run] [file ...]

If no files are given the script processes the default set of context-api
source files.  Pass --dry-run to print diffs without writing.
"""

from __future__ import annotations

import argparse
import difflib
import re
import sys
from pathlib import Path

# ---------------------------------------------------------------------------
# Files to process when none are given on the command line
# ---------------------------------------------------------------------------

DEFAULT_FILES = [
    "crates/context-api/src/types.rs",
    "crates/context-api/src/commands/mod.rs",
    "crates/context-api/src/commands/export_import.rs",
    "crates/context-api/src/log_parser.rs",
    "crates/context-api/src/error.rs",
]

# ---------------------------------------------------------------------------
# use-import injection
# ---------------------------------------------------------------------------

# Sentinel we insert once per file so ts_export! resolves inside the crate
_USE_DECL = "use crate::ts_export;\n"


# We look for the first `use ` statement line and insert our import right
# before it.  If no `use` statement is found we insert after the module-doc
# block (i.e. after the last `//!` line).
def _inject_use(lines: list[str]) -> list[str]:
    """Return a copy of *lines* with ``use crate::ts_export;`` added."""
    # Already present?
    if any(_USE_DECL.strip() in ln for ln in lines):
        return lines

    # Find first `use ` line
    for i, ln in enumerate(lines):
        if ln.lstrip().startswith("use "):
            return lines[:i] + [_USE_DECL] + lines[i:]

    # Fallback: append after the last //! doc line
    last_doc = -1
    for i, ln in enumerate(lines):
        if ln.startswith("//!"):
            last_doc = i
    insert_at = last_doc + 1 if last_doc >= 0 else 0
    return lines[:insert_at] + [_USE_DECL] + lines[insert_at:]


# ---------------------------------------------------------------------------
# types.rs module-doc update
# ---------------------------------------------------------------------------

_OLD_TYPES_DOC_SNIPPET = (
    "//! All `export_to` paths use the value of [`crate::TS_EXPORT_DIR`]:\n"
    '//! `"../../../packages/context-types/src/generated/"`.  ts-rs resolves this\n'
    "//! relative to `<CARGO_MANIFEST_DIR>/bindings/`, so three `../` segments\n"
    "//! reach the workspace root.  Search for **`TS_EXPORT_DIR`** to locate every\n"
    "//! usage site across the codebase.\n"
)

_NEW_TYPES_DOC_SNIPPET = (
    "//! All types use the [`ts_export!`] macro which stamps out the two\n"
    "//! `cfg_attr` lines and hard-codes the `export_to` path in exactly one\n"
    "//! place (`lib.rs`).  The path value matches [`crate::TS_EXPORT_DIR`].\n"
    "//! Search for **`ts_export!`** to find every usage site.\n"
)


def _update_types_doc(lines: list[str]) -> list[str]:
    src = "".join(lines)
    updated = src.replace(_OLD_TYPES_DOC_SNIPPET, _NEW_TYPES_DOC_SNIPPET)
    if updated == src:
        return lines
    return updated.splitlines(keepends=True)


# ---------------------------------------------------------------------------
# Helpers – attribute-block parsing
# ---------------------------------------------------------------------------

# Matches the opening of any outer attribute line
_ATTR_START_RE = re.compile(r"^(\s*)\#\[")

# Matches the first cfg_attr line (derive ts_rs::TS)
_TS_DERIVE_RE = re.compile(
    r'^\s*\#\[cfg_attr\(feature\s*=\s*"ts-gen",\s*derive\(ts_rs::TS\)\)\]'
)

# Matches a single-line ts(...) cfg_attr
_TS_EXPORT_SINGLE_RE = re.compile(
    r"^\s*\#\[cfg_attr\(\s*$"  # opening line only
    r"|"
    r'^\s*\#\[cfg_attr\(feature\s*=\s*"ts-gen",\s*ts\(export,'
)


def _is_attr_line(line: str) -> bool:
    """Return True if *line* starts an outer attribute (#[...)."""
    return bool(_ATTR_START_RE.match(line))


def _attr_block_end(lines: list[str], start: int) -> int:
    """
    Given that lines[start] is the first line of an attribute (starts with
    optional whitespace then ``#[``), return the index of the first line
    *after* this attribute (i.e. the exclusive end of the attribute text).

    Handles multi-line attributes by counting unmatched ``[`` / ``]``.
    """
    depth = 0
    i = start
    while i < len(lines):
        for ch in lines[i]:
            if ch == "[":
                depth += 1
            elif ch == "]":
                depth -= 1
        i += 1
        if depth == 0:
            break
    return i


def _collect_attr_block(
    lines: list[str], start: int
) -> tuple[list[tuple[int, int]], int]:
    """
    Starting at *start*, collect all consecutive attributes.

    Returns:
        attrs  – list of (line_start, line_end_exclusive) for each attribute
        cursor – index of the first non-attribute line after the block
    """
    attrs: list[tuple[int, int]] = []
    i = start
    while i < len(lines):
        line = lines[i]
        if _ATTR_START_RE.match(line):
            end = _attr_block_end(lines, i)
            attrs.append((i, end))
            i = end
        else:
            break
    return attrs, i


def _attr_text(lines: list[str], span: tuple[int, int]) -> str:
    return "".join(lines[span[0] : span[1]])


def _is_ts_gen_derive_attr(text: str) -> bool:
    """True if this attribute is the ``#[cfg_attr(feature = "ts-gen", derive(ts_rs::TS))]`` line."""
    return bool(_TS_DERIVE_RE.match(text))


def _is_ts_gen_export_attr(text: str) -> bool:
    """True if this attribute is the ``#[cfg_attr(... ts(export, export_to = ...))]`` block."""
    return 'feature = "ts-gen"' in text and "export_to" in text


# ---------------------------------------------------------------------------
# Helpers – item-end finding
# ---------------------------------------------------------------------------


def _find_item_end(lines: list[str], start: int) -> int:
    """
    Given that lines[start] is the first line of an item definition (pub
    struct / pub enum / etc.), return the index of the first line *after* the
    item (exclusive end).

    Handles brace-balanced items and semicolon-terminated items at depth 0.
    """
    depth = 0
    in_line_comment = False
    in_block_comment = False
    in_str = False
    in_char = False

    for i in range(start, len(lines)):
        line = lines[i]
        j = 0
        while j < len(line):
            ch = line[j]
            nch = line[j + 1] if j + 1 < len(line) else ""

            if in_line_comment:
                # line comment ends at newline; we process line-by-line so
                # just break out of the inner loop
                break

            if in_block_comment:
                if ch == "*" and nch == "/":
                    in_block_comment = False
                    j += 2
                    continue
                j += 1
                continue

            if not in_str and not in_char:
                if ch == "/" and nch == "/":
                    in_line_comment = True
                    break
                if ch == "/" and nch == "*":
                    in_block_comment = True
                    j += 2
                    continue

            if in_str:
                if ch == "\\" and nch:
                    j += 2
                    continue
                if ch == '"':
                    in_str = False
                j += 1
                continue

            if in_char:
                if ch == "\\" and nch:
                    j += 2
                    continue
                if ch == "'":
                    in_char = False
                j += 1
                continue

            if ch == '"':
                in_str = True
                j += 1
                continue

            if ch == "'":
                in_char = True
                j += 1
                continue

            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    return i + 1  # include the line with the closing brace
            elif ch == ";" and depth == 0:
                return i + 1

            j += 1

        in_line_comment = False  # reset for next line

    return len(lines)


# ---------------------------------------------------------------------------
# Core transformation
# ---------------------------------------------------------------------------


def _collect_preceding_doc(
    out: list[str],
) -> tuple[list[str], list[str]]:
    """
    Pop any trailing ``///`` doc-comment lines (and blank lines between them
    and the attribute block) off the end of *out* and return them as a
    separate list so they can be moved inside the macro call.

    Returns:
        (trimmed_out, doc_lines)  where doc_lines preserves original order.
    """
    # Work backwards: skip blank lines, then collect /// lines
    tmp = list(out)
    trailing_blank: list[str] = []

    while tmp and tmp[-1].strip() == "":
        trailing_blank.insert(0, tmp.pop())

    doc_lines: list[str] = []
    while tmp and tmp[-1].lstrip().startswith("///"):
        doc_lines.insert(0, tmp.pop())

    if doc_lines:
        # Keep blank lines that were between the doc comment and the attribute
        # block — they go inside the macro too.
        return tmp, doc_lines + trailing_blank
    else:
        # No doc comment found — put the blanks back
        return out, []


def transform(src: str, is_types_rs: bool = False) -> str:
    """
    Replace every ts-gen cfg_attr pair (plus the preceding doc comment, all
    surrounding attributes, and the following item) with a
    ``ts_export! { ... }`` invocation, and inject ``use crate::ts_export;``
    so the macro resolves inside the crate.
    """
    lines = src.splitlines(keepends=True)
    if is_types_rs:
        lines = _update_types_doc(lines)
    lines = _inject_use(lines)
    out: list[str] = []
    i = 0

    while i < len(lines):
        line = lines[i]

        # Only start scanning when we see the beginning of an attribute block
        if not _ATTR_START_RE.match(line):
            out.append(line)
            i += 1
            continue

        # Collect the full contiguous attribute block starting here
        attrs, item_start = _collect_attr_block(lines, i)

        # Check whether this block contains both ts-gen cfg_attr lines
        ts_derive_idx: int | None = None
        ts_export_idx: int | None = None

        for idx, span in enumerate(attrs):
            text = _attr_text(lines, span)
            if ts_derive_idx is None and _is_ts_gen_derive_attr(text):
                ts_derive_idx = idx
            elif (
                ts_export_idx is None
                and ts_derive_idx is not None
                and _is_ts_gen_export_attr(text)
            ):
                ts_export_idx = idx

        if ts_derive_idx is None or ts_export_idx is None:
            # Not a ts-gen attribute block — emit as-is and move on
            out.extend(lines[i:item_start])
            i = item_start
            continue

        # Pull any preceding /// doc comment out of `out` so it moves inside
        # the macro call (keeps rustdoc happy).
        out, doc_lines = _collect_preceding_doc(out)

        # Determine the indentation from the first attribute line
        first_line = lines[attrs[0][0]]
        indent = len(first_line) - len(first_line.lstrip())
        indent_str = first_line[:indent]

        # Skip any blank lines between the attribute block and the item
        actual_item_start = item_start
        while actual_item_start < len(lines) and lines[actual_item_start].strip() == "":
            actual_item_start += 1

        # Find the end of the item
        item_end = _find_item_end(lines, actual_item_start)

        # Build the list of attributes to keep (everything except the two ts-gen ones)
        keep_indices = {
            idx
            for idx in range(len(attrs))
            if idx != ts_derive_idx and idx != ts_export_idx
        }
        kept_attrs_text = "".join(
            _attr_text(lines, attrs[idx]) for idx in sorted(keep_indices)
        )

        # The item body (all lines from actual_item_start to item_end)
        item_text = "".join(lines[actual_item_start:item_end])

        # Indent everything inside the macro by 4 extra spaces
        def add_indent(text: str) -> str:
            result = []
            for ln in text.splitlines(keepends=True):
                if ln.strip():
                    result.append("    " + ln)
                else:
                    result.append(ln)
            return "".join(result)

        # doc comment lines already carry their own indentation; just add 4 spaces
        doc_text = "".join("    " + ln for ln in doc_lines)
        inner = add_indent(kept_attrs_text + item_text)

        # Ensure inner content ends with exactly one newline before closing brace
        inner = inner.rstrip("\n")

        macro_call = f"{indent_str}ts_export! {{\n{doc_text}{inner}\n{indent_str}}}\n"
        out.append(macro_call)

        i = item_end

    return "".join(out)


# ---------------------------------------------------------------------------
# CLI plumbing
# ---------------------------------------------------------------------------


def print_diff(original: str, updated: str, filename: str) -> None:
    diff = difflib.unified_diff(
        original.splitlines(keepends=True),
        updated.splitlines(keepends=True),
        fromfile=f"a/{filename}",
        tofile=f"b/{filename}",
    )
    sys.stdout.writelines(diff)


def process_file(path: Path, dry_run: bool) -> bool:
    src = path.read_text(encoding="utf-8")
    is_types_rs = path.name == "types.rs" and "context-api" in str(path)
    updated = transform(src, is_types_rs=is_types_rs)

    if src == updated:
        print(f"  (no changes) {path}")
        return False

    if dry_run:
        print(f"  [dry-run] {path}")
        print_diff(src, updated, str(path))
        return True

    path.write_text(updated, encoding="utf-8")
    print(f"  updated    {path}")
    return True


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Replace ts-gen cfg_attr pairs with ts_export! macro calls."
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Print diffs without writing files.",
    )
    parser.add_argument(
        "files",
        nargs="*",
        help="Source files to process (default: context-api sources).",
    )
    args = parser.parse_args(argv)

    script_dir = Path(__file__).parent
    workspace_root = script_dir.parent

    files = (
        [workspace_root / f for f in DEFAULT_FILES]
        if not args.files
        else [Path(f) for f in args.files]
    )

    changed = 0
    errors = 0
    for f in files:
        if not f.exists():
            print(f"  ERROR: file not found: {f}", file=sys.stderr)
            errors += 1
            continue
        try:
            if process_file(f, dry_run=args.dry_run):
                changed += 1
        except Exception as exc:  # noqa: BLE001
            print(f"  ERROR processing {f}: {exc}", file=sys.stderr)
            errors += 1

    noun = "file" if changed == 1 else "files"
    action = "would change" if args.dry_run else "changed"
    print(f"\n{action} {changed} {noun}.")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
