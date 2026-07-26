## Problem
`tools/model-prices/__pycache__/cost_gate.cpython-314.pyc` is tracked in git (committed in 086f8b62 and updated in 9af02308). There is no `.pyc` / `__pycache__` entry in `.gitignore`, so compiled Python bytecode keeps getting committed as binary churn.

## Fix
- Add `__pycache__/` and `*.pyc` to `.gitignore` (root and/or tools/model-prices).
- `git rm --cached` the tracked `.pyc` so it stops tracking without deleting the local file.

## Acceptance
- No `.pyc` under version control (`git ls-files '*.pyc'` returns empty).
- `.gitignore` ignores `__pycache__/` and `*.pyc`.

Source: review of the graded cost-gate feature (spec 29ae5f6e).
