# BAL9.4 · Review BAL9

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** bal9/4-review
- **Abhängig von:** BAL9.1, BAL9.2, BAL9.3
- **Tickets:** B-656, B-657, B-654
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Review des Sprints BAL9: alle Kriterien geprüft, Befunde behoben oder als Ticket, Sprint-Abnahme ausgefüllt.

## Kontext

Sessions BAL9.1 (castleHeld im Fenster), BAL9.2 (Bot-Performance, byte-gleich), BAL9.3 (Meilenstein-Lauf 8 Seeds). Review nach `docs/arbeitsweise.md` › Review.

## Erlaubte Dateien

- Dateien der Sessions BAL9.1 bis BAL9.3 für kleine Fixes
- Sprint-README und Tickets über `plan_*`

## Nicht-Ziele

Neue Mechaniken, neue Werte.

## Schritte

1. Diff seit Start-Commit `29493645` lesen, Ergebnisse der Sessions gegen AC-01 bis AC-03 prüfen.
2. `task check`, `task check:go`, `task check:dev`, `task check:race` (nacheinander; ohne gcc als übersprungen nennen).
3. Abnahme der Sprint-README füllen, Tickets B-654, B-656, B-657 schließen oder Rest als Ticket.

## Fertig, wenn

- [ ] AC-01, AC-02, AC-03 mit Nachweis aus den Session-Ergebnissen.
- [ ] Prüfungen grün, Abnahme ausgefüllt.

## Prüfen

```bash
task check
task check:go
task check:dev
task check:race
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
