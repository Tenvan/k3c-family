# testing-red (Skill)

## Zweck

Dieser Skill fuehrt die Red-Phase von TDD aus: eine klare failing test-Beschreibung fuer
das naechste Verhalten schreiben, bevor Produktionscode angepasst wird.

## Ablauf

1. Chat- und Fehlerkontext analysieren.
2. Das naechste fehlende Verhalten identifizieren.
3. Einen einzelnen failing test formulieren.
4. Verifizieren, dass der Test aus dem richtigen Grund fehlschlaegt.
5. Erst danach an die Green-Phase uebergeben.

## Struktur

- `SKILL.md`: Red-Workflow und Testprinzipien
- `references/eval-notes.md`: Review-Kriterien fuer Red-Ausgaben
- `evals/evals.json`: typische TDD-Red-Prompts
