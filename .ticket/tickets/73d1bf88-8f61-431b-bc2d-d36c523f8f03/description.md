# Objective

Align the context-read failing tests with the clarified normalization/materialization contract before wider algorithm edits proceed.

## Contract to encode

- embedded paths and materialized tokens may be semantically equivalent;
- normalization is required only on the most abstract API surfaces;
- lower-level path/cursor surfaces may retain Prefix/Postfix-style coverage information;
- overlap expansion steps are materialized immediately for safety;
- results are retained until invalidated or released;
- visible graph state must always preserve structural invariants.

## Main impact area

This work is expected to narrow or reframe the expectations inside the current normalization/materialization failure buckets, especially where tests currently assume mandatory `EntireRoot` normalization or delayed visibility of intermediate results.

## Done when

- the affected failing tests and acceptance criteria are reviewed against the clarified contract;
- any tests that encoded the old assumptions are updated or explicitly kept with a documented reason;
- the spec/ticket language for normalization and materialization remains consistent with the resulting test expectations.