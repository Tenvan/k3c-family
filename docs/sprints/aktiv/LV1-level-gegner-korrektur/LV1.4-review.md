# LV1.4 · Review LV1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** lv1/4-review
- **Abhängig von:** LV1.1, LV1.2, LV1.3
- **Tickets:** B-291, B-262, B-200, B-189, B-217, B-260
- **Kriterien:** alle

## Ziel

Alle Kriterien von LV1 sind geprüft, die Abnahme steht in der Sprint-README.

## Kontext

Diff von LV1.1 bis LV1.3 (SIM). `task check`, `task check:go`, `task check:race` nacheinander; Reviewer-Agent getrennt; Golden-Diffs lesen.

## Erlaubte Dateien

- `docs/sprints/`, `docs/backlog/`
- Fixes unter 30 Zeilen in den Dateien von LV1.1 bis LV1.3

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. Prüfläufe.
2. Reviewer.
3. Abnahme.

## Fertig, wenn

- [ ] AC-01 bis AC-06 mit Nachweis oder Ticket.

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
