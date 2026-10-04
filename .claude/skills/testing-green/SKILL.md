---
name: testing-green
version: 1.0.0
description: Implement minimal code to satisfy the agreed task and make failing tests pass without over-engineering. Use this skill for the Green phase of TDD when failing tests already describe the target behavior and the next step is the smallest implementation that turns the suite green.
---

# TDD Green Phase - Make Tests Pass Quickly

Write the minimal code necessary to satisfy the agreed task and make failing tests pass. Resist the urge to write more than required.

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
🚀 SKILL START │ testing-green
   Implement minimal code to satisfy the
   agreed task and make failing tests pass
   without over-engineering.
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

At the **end** of every execution (after `🎯 Workflow complete` / `🎯 Dry run complete`) output:

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ SKILL DONE  │ testing-green
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

On abort / unresolvable error:

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
❌ SKILL ABORTED │ testing-green – <reason>
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Task Context Integration

### Chat-Driven Implementation
- **Reference the chat context** - Keep the task agreed with the programmer in focus during implementation
- **Validate against acceptance criteria** - Ensure implementation meets the agreed definition of done
- **Track progress in the conversation** - Surface progress and blockers directly in chat
- **Stay in scope** - Implement only what's required by the current task or observable problem

### Implementation Boundaries
- **Task scope only** - Don't implement features not required by the current task
- **Future-proofing later** - Defer enhancements that are not needed for the current problem
- **Minimum viable solution** - Focus on core requirements from the chat or failing behaviour

## Core Principles

### Minimal Implementation
- **Just enough code** - Implement only what's needed to satisfy the task and make tests pass
- **Fake it till you make it** - Start with hard-coded returns based on known examples, then generalise
- **Obvious implementation** - When the solution is clear from the current context, implement it directly

### Speed Over Perfection
- **Green bar quickly** - Prioritise making tests pass over code quality
- **Ignore code smells temporarily** - Duplication and poor design will be addressed in refactor phase
- **Simple solutions first** - Choose the most straightforward implementation path from the available context
- **Defer complexity** - Don't anticipate requirements beyond the current task scope

### TypeScript Implementation Strategies
- **Start with constants** - Return hard-coded values from known examples initially
- **Progress to conditionals** - Add if/else logic as more scenarios are tested
- **Extract to functions** - Create simple helper functions when duplication emerges
- **Use basic collections** - Plain `Array`/`Map`/`Set`/`Record<K, V>` over custom data structures

## Execution Guidelines

1. **Review the current task** - Confirm implementation aligns with the chat context or current problem situation
2. **Run the failing test** - Confirm exactly what needs to be implemented
3. **Clarify the working assumptions from context** - Base the implementation on the agreed task, failing behaviour, and known edge cases
4. **Write minimal code** - Add just enough to satisfy the task and make the test pass
5. **Run all tests** - Ensure new code doesn't break existing functionality
6. **Do not modify the test** - Ideally the test should not need to change in the Green phase.
7. **Report progress in chat** - Summarise implementation status and blockers if needed

## Green Phase Checklist
- [ ] Implementation aligns with the agreed task
- [ ] All tests are passing (green bar)
- [ ] No more code written than necessary for the current task scope
- [ ] Existing tests remain unbroken
- [ ] Implementation is simple and direct
- [ ] Acceptance criteria from the chat or problem context satisfied
- [ ] Ready for refactoring phase
- [ ] Follow-up refactor ideas are deferred unless required for green
