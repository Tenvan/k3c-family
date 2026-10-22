# GR8.3 · Review GR8

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** gr8/3-review
- **Abhängig von:** GR8.2
- **Tickets:** B-329, B-331
- **Kriterien:** AC-01, AC-02

## Ziel

Review des Sprints GR8: alle Kriterien geprüft, PR geöffnet.

## Kontext

Review-Session nach `docs/arbeitsweise.md` › Review-Session; Diff `origin/develop...sprint/gr8`.

## Erlaubte Dateien

- Befundfixes in den Domänen des Sprints, `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. `task check`, `task check:go`, `task check:race`.
2. Diff lesen, nur schwere Befunde.
3. Abnahme (≤ 5 Zeilen), Version vorschlagen, Sprint nach `erledigt/`, PR öffnen.

## Fertig, wenn

- [ ] AC-01, AC-02: Nachweis in den Ergebnissen GR8.1 und GR8.2; Befunde behoben oder als Ticket.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
