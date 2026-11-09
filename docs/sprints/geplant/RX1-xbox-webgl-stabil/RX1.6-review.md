# RX1.6 · Review RX1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** rx1/6-review
- **Abhängig von:** RX1.5
- **Tickets:** B-762, B-763, B-764, B-765
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06, AC-07, AC-11, AC-12, AC-13, AC-14, AC-15, AC-16

## Ziel

Alle autonom prüfbaren Kriterien von RX1 sind geprüft, die Abnahme steht in der Sprint-README.

## Kontext

Review-Session am Sprint-Ende (wie PM1.3). Sessions RX1.1, RX1.2 und RX1.5 sind erledigt; RX1.4 hat die Xbox-Werte geliefert.

## Erlaubte Dateien

- `docs/sprints/geplant|aktiv/RX1-*/README.md` (Abnahme), Befund-Fixes unter etwa 30 Zeilen
- neue Tickets unter `docs/backlog/`

## Nicht-Ziele

Keine neuen Features; keine Sim-/Balance-/Gen-Läufe außer im Abnahme-Ticket.

## Schritte

1. Diff gegen `develop` lesen, Kriterien AC-01 bis AC-07 prüfen.
2. `task check`, `task check:go`, `task check:race` nacheinander.
3. Befunde beheben oder als Ticket anlegen; genau ein Abnahme-Ticket für Sim/Balance/Gen, falls nötig.
4. Abschnitt Abnahme in der Sprint-README füllen (höchstens fünf Zeilen).

## Fertig, wenn

- [ ] AC-01 bis AC-07: geprüft (Tests grün, Diff gelesen).
- [ ] Abnahme in der README, offene Punkte (AC-09, AC-10) auf RX1.7 verwiesen.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
