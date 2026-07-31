## Validation

### Correctness Criteria

| Criterion | How to Verify |
|---|---|
| All 4 skill documents exist | `ls docs/skills/0*.md` returns 4 files |
| README.md index exists | `cat docs/skills/README.md` shows index with links |
| Template structure followed | Each doc has all 7 sections from the template |
| ASCII art renders correctly | View each doc in a terminal at 80 columns |
| CLI examples are runnable | Execute all `context-cli` commands in "Try It Yourself" sections |
| CLI examples produce expected output | Compare actual vs documented output |
| Repetition/long-context example is included | Skill 4 contains one worked example with repeated multi-line prefixes across 3+ segments |
| Terminology consistency | Grep for key terms across skill docs and HIGH_LEVEL_GUIDE.md files |
| Cross-references work | All `[links](targets)` resolve to existing files |
| No internal jargon leaks | Skill docs should not reference agent guides, plans, or internal implementation details |

### CLI Smoke Test

Run this sequence to verify the core examples work end-to-end:

```bash
# Setup
context-cli workspace create skill-test

# Skill 1 verification
context-cli read skill-test "abc"
context-cli inspect skill-test "abc"
# EXPECT: token with 3 atoms, width=3

# Skill 2 verification
context-cli read skill-test "abcabc"
context-cli inspect skill-test "abcabc"
# EXPECT: pattern ["abc", "abc"]

# Skill 3 verification
context-cli read skill-test "hel"
context-cli read skill-test "lo"
context-cli read skill-test "hello"
context-cli inspect skill-test "hello"
# EXPECT: pattern ["hel", "lo"]

# Skill 4 verification
context-cli inspect skill-test "abcabc"
# EXPECT: multiple child patterns

# Cleanup
context-cli workspace delete skill-test
```

---

