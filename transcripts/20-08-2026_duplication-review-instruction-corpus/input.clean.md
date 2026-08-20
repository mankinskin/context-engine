# Duplication Review Agent

Create an agent for a structured duplication review of the entire instruction corpus.

## Goal

The agent's primary goal is to identify all duplicated or semantically very similar passages and collect them in a report. It should extract the most important duplicated ideas and document where they occur.

## Workflow

1. Compare all files with one another by iterating through every pair.
2. Classify similarity findings into categories such as exact duplicates, near-duplicates, and other similarity levels where appropriate.
3. Collect longer text passages that are very similar and store them together in one file.
4. After all pairs have been compared, evaluate the collected findings again.
5. Group related findings, simplify them where possible, and determine which ideas are expressed in multiple places across the corpus.

## Output

Write the results into a dedicated file created for this review. The file should document the most important duplicated ideas and all of their occurrences.

## Workspace

Use a dedicated folder in the repository as the workspace for this review, similar to the `transcripts` folder.