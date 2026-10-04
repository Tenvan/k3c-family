---
name: testing-red
version: 1.0.0
description: Guide test-first development by writing failing tests that describe desired behavior before implementation. Use this skill for the Red phase of TDD when the next step is to express a missing behavior as one clear failing test before production code changes begin.
---

# TDD Red Phase - Write Failing Tests First

Focus on writing clear, specific failing tests that describe the desired behaviour from the current task context before any implementation exists.

## TypeScript-Testing-Konventionen (orga)

- Tests neben den Quelldateien halten.
- `*.spec.ts` für Unit-/Integrationstests, `*.e2e.test.ts` für E2E-Tests, `*.test.tsx` für React-Komponenten, wo diese Konvention gilt.
- Typischer Stack: Jest, RTL, MSW, Playwright. Projektspezifische Ergänzungen gehören in die jeweilige Projekt-Testing-Rule.
- Klare Arrange-Act-Assert-Struktur bevorzugen; externe Abhängigkeiten bewusst mocken, ohne Integrationsgrenzen versehentlich zu verstecken.
- Für E2E-Daten session-scoped Kennungen oder eine gleichwertige Isolation verwenden, um Kollisionen zu vermeiden.
- Coverage-Schwellen in der Projektkonfiguration definieren statt in prose-lastigen Rule-Texten.
- Zentrale Fehler-Helper nutzen und direkten Zugriff auf `error.message` vermeiden, wenn ein Projekt-Helper existiert.
- Editor-Testtool bevorzugen, wenn möglich; sonst die projektbezogene Jest-Konfiguration nutzen.

## Skill Output (Entry / Exit)

Output these banners as **fenced code blocks** so line breaks are preserved in all rendering environments (chat, CLI, terminal).

At the **start** of every execution output:

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🚀 SKILL START │ testing-red
   Guide test-first development by writing
   failing tests that describe desired
   behavior before implementation.
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

At the **end** of every execution (after `🎯 Workflow complete` / `🎯 Dry run complete`) output:

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ SKILL DONE  │ testing-red
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

On abort / unresolvable error:

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
❌ SKILL ABORTED │ testing-red – <reason>
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Task Context Integration

### Current Context Analysis

- **Use the current conversation** to understand the requested change and expected behaviour
- **Inspect the failing behaviour** from tests, logs, errors, or code paths when available
- **Understand the full context** from the current problem situation and the programmer's guidance

### Task Context Analysis

- **Requirements extraction** - Parse the requested behaviour and acceptance criteria from the chat
- **Edge case identification** - Review current errors, logs, or examples for boundary conditions
- **Definition of Done** - Use the agreed task outcome as the test validation target
- **Programmer context** - Consider the programmer's current focus and constraints

## Core Principles

### Test-First Mindset

- **Write the test before the code** - Never write production code without a failing test
- **One test at a time** - Focus on a single behaviour or requirement from the current task
- **Fail for the right reason** - Ensure tests fail due to missing implementation, not syntax errors
- **Be specific** - Tests should clearly express what behaviour is expected from the current context

### Test Quality Standards

- **Descriptive test names** - Use clear, behaviour-focused naming like `it('returns a validation error when the email is invalid')`
- **AAA Pattern** - Structure tests with clear Arrange, Act, Assert sections
- **Single assertion focus** - Each test should verify one specific outcome from the task criteria
- **Edge cases first** - Consider boundary conditions implied by the current problem situation

### TypeScript Test Patterns

- Use **Jest** or **Vitest** (`describe`/`it`/`expect`) — both share the same core API, pick whichever the project already runs
- Prefer factory functions (or `@faker-js/faker`) over inline literals for test data generation
- Implement **`it.each`/`test.each`** for multiple input scenarios from known examples
- Use **`expect.extend`** for domain-specific custom matchers required by the task
- Mock external calls with `jest.mock`/`vi.mock`, or **MSW** for HTTP boundaries — never real network/DB access in unit tests

## Execution Guidelines

1. **Analyse the current context** - Break down the chat request or current problem into testable behaviours
2. **Identify the first missing behaviour** - Choose the most basic scenario that demonstrates the gap
3. **Align on working assumptions from context** - Base the test on the current conversation, observed failures, and known edge cases
4. **Write the simplest failing test** - Start with the most basic scenario. NEVER write multiple tests at once. You will iterate on RED, GREEN, REFACTOR cycle with one test at a time
5. **Verify the test fails** - Run the test to confirm it fails for the expected reason
   - **If the test passes immediately:** The behaviour already exists. Either the test is redundant (discard it) or it is not testing what you intended (revise the test to target the actual missing behaviour).
6. **Link test to the task** - Reflect the requested behaviour in test names and comments

## Red Phase Checklist

- [ ] Current task context retrieved and analysed
- [ ] Test clearly describes expected behaviour from the current task
- [ ] Test fails for the right reason (missing implementation)
- [ ] Test name describes behaviour without relying on external ticket metadata
- [ ] Test follows AAA pattern
- [ ] Edge cases from the current problem situation considered
- [ ] No production code written yet
