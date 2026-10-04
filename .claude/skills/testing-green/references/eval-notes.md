# Testing Green Eval Notes

Use this file during human review of `testing-green` outputs.

## What To Check

- The run starts from existing failing tests or a clearly described failing behavior.
- The implementation stays within the agreed task scope.
- The output favors the smallest viable change over speculative design work.
- The workflow keeps the suite green before suggesting refactor follow-up.

## Common Failure Modes

- The skill adds extra features that were not required to pass the tests.
- The implementation rewrites tests instead of fixing production code.
- The response jumps into refactoring before the Green phase is complete.
