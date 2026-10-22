# SK2.4 · Review SK2

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** sk2/4-review
- **Abhängig von:** SK2.2, SK2.3
- **Tickets:** B-504, B-503
- **Kriterien:** alle

## Ziel

Alle autonom prüfbaren Kriterien von SK2 sind geprüft, die Abnahme steht in der Sprint-README.

## Kontext

Diff von SK2.2 (SIM) und SK2.3 (CLI). Review-Session: `task check`, `task check:go`, `task check:race` nacheinander, Reviewer-Agent getrennt (SIM).

## Erlaubte Dateien

- `docs/sprints/`, `docs/backlog/`
- Fixes unter 30 Zeilen in den Dateien von SK2.2 und SK2.3

## Nicht-Ziele

Neue Funktionen, TV-Abnahme (SK2.5).

## Schritte

1. Prüfläufe nacheinander.
2. Reviewer-Agent.
3. Abnahme (höchstens fünf Zeilen), offene Mensch-Sessions nennen.

## Fertig, wenn

- [ ] AC-02 und AC-03 (Test-Teil) mit Nachweis; AC-01 und TV-Teil bei 🧑.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
