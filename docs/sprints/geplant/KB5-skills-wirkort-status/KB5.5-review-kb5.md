# KB5.5 · Review KB5

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** kb5/5-review-kb5
- **Abhängig von:** KB5.4
- **Tickets:** B-674, B-675, B-676
- **Kriterien:** –

## Ziel

Review des Sprints KB5 auf dem Sprint-Branch mit vollem Prüfsatz.

## Kontext

Sessions KB5.1 bis KB5.4; Protokolländerungen (Wirk-Ereignis, Status im Snapshot). Ablauf: `docs/arbeitsweise.md` › Review-Session.

## Erlaubte Dateien

- Nur Korrekturen in den Dateien der Sessions KB5.1 bis KB5.4.

## Nicht-Ziele

- Neue Funktionen.

## Schritte

1. `task check`, `task check:go`, `task check:race` nacheinander.
2. Diff lesen, Kriterien gegen Nachweise prüfen.
3. Abnahme in höchstens fünf Zeilen.

## Fertig, wenn

- [ ] Prüfsatz grün, jedes Kriterium mit Nachweis.

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
