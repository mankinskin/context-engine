## Gap

`.agents/instructions/orchestration/compact-output.instructions.md` is 28 lines with `applyTo: "**/*.sh,**/*.ps1"`. Under `Compact-by-Default Output` at line 6, line 17 says `Prefer rtk <cmd> over bare <cmd> - rtk filters/compresses output automatically.` Line 18 provides only a missing-binary fallback. No exception exists for a command that emits a stream of file paths consumed by another command.

## Session Evidence

In the restructuring session, `rtk` altered path-stream output and polluted filenames, breaking a copy/relocation pipeline.

## Required Corrected State

Add an explicit exception: commands whose output is a stream of file paths consumed by another command must run bare.