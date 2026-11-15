# WE4.1 · Etappen und Zielwahl am Hafen in der Engine (B-961)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** sprint/we4
- **Abhängig von:** –
- **Tickets:** B-961
- **Kriterien:** AC-01

## Ziel

Etappen je Insel, Zielwahl am Hafen und Spielstand dazu liegen in der Engine, getestet (B-961).

## Kontext

Inselwechsel: `engine/sim/island_switch.go` (`NextDef`, `NextIsland`, `isLast`), Welt laden: `engine/sim/island_worlds.go` (`rawIsland.def`, Prüfungen), Raum tauscht Insel: `engine/room/`. Regeln `docs/rules/welten.md` § 6, `stufen.md` § 1. Sim-Code im eigenen Worktree (B-925).

## Erlaubte Dateien

- `engine/sim/**`
- `engine/room/**`
- `docs/rules/welten.md`
- `docs/rules/stufen.md`
- `docs/protocol.md`
- `data/worlds/sengoku.json`

## Nicht-Ziele

Anzeige im Spiel (WE4.2), Editor (WE4.3).

## Schritte

1. Feld `etappe` laden und prüfen (Standard Listenplatz + 1).
2. Abgeschlossene Inseln, Ziele, Wahl-Aktion, `NextIsland` mit Ziel.
3. Spielstand und Zustand/Ereignisse.
4. Doku und Sengoku-Etappen.

## Fertig, wenn

- [ ] AC-01, AC-02, AC-03, AC-04 aus B-961

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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
