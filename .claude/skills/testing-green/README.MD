# testing-green (Skill)

## Zweck

Dieser Skill fuehrt die Green-Phase von TDD aus: minimale Implementierung schreiben,
damit bestehende failing tests gruen werden, ohne den Scope auszudehnen.

## Ablauf

1. Task- und Testkontext bestaetigen.
2. Failing Test gezielt nachvollziehen.
3. Die kleinstmoegliche Implementierung schreiben.
4. Tests erneut ausfuehren und auf gruen pruefen.
5. Refactor-Ideen nur notieren, nicht vorziehen.

## Struktur

- `SKILL.md`: Green-Workflow und Grenzen
- `references/eval-notes.md`: Review-Kriterien fuer Green-Ausgaben
- `evals/evals.json`: typische TDD-Green-Prompts
