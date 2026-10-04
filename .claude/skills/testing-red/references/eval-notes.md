# Testing Red Eval Notes

Use this file during human review of `testing-red` outputs.

## What To Check

- The run derives the missing behavior from the current task context.
- Only one clear failing test is introduced at a time.
- The test failure is expected to come from missing implementation, not broken setup.
- The output avoids writing production code during the Red phase.

## Common Failure Modes

- Multiple tests are added in one step.
- The generated test is vague or not behavior-focused.
- Production code changes are suggested before confirming a failing test.
