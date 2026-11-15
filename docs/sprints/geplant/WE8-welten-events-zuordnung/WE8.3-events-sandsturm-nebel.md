# WE8.3 · Engine: Events sandstorm und fog (B-992)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/we8
- **Abhängig von:** WE8.2
- **Tickets:** B-992
- **Kriterien:** AC-03, AC-06

## Ziel

Die Engine kennt die Events `sandstorm` und `fog`: für eine Nacht an der Oberfläche geändertes Tempo und gekürzte Fernkampf-Reichweite (B-992).

## Kontext

Eigener Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`). Vorbild Blutmond (Faktor nur in der Event-Nacht): `engine/sim/events_moon.go`; Event `raid` aus WE8.2. Events kommen aus dem Wertesatz der Welt (B-982). Wirkung nur in Tiefe 0 (`engine/sim/island*.go` rechnet mehrere Stufen im selben Takt) und nur in der Phase Nacht (`world_cycle.go`). Reichweite und Tempo von Truppen, Türmen und Gegnern: Stellen mit Grep nach `Range` und `Speed` in `engine/sim` finden. Regeltext aus WE8.1 in `docs/rules/bosse.md` § 2; Wetter-Rahmen B-969. Zustand in `docs/protocol.md` (Sperre `protokoll`). Golden dürfen sich nicht ändern.

## Erlaubte Dateien

- `engine/sim/**`
- `engine/room/**`
- `data/events.json`
- `docs/rules/bosse.md`
- `docs/protocol.md`

## Nicht-Ziele

Taifun, Flut, Schneesturm (B-969), Anzeige (WE8.4), Einschalten in den Welten (WE8.5), Simulation, Balancing.

## Schritte

1. Tests zuerst: `sandstorm` ändert Tempo und Reichweite in der Event-Nacht in Tiefe 0, nicht am Tag, nicht in Tiefe 1; `fog` nur Verteidiger; Überlagerung; Determinismus.
2. Beide Events in `data/events.json` (aus), Laden und Prüfung (Faktoren > 0).
3. Ein gemeinsamer Zugriff „Faktor für Tempo/Reichweite in dieser Stufe jetzt“ statt verstreuter Abfragen.
4. Zustand nennt das aktive Event, `docs/protocol.md` nachziehen.
5. Review `k3c-reviewer` vor dem Commit.

## Fertig, wenn

- [ ] AC-03: B-992/AC-01, AC-02 als Tests grün; B-992/AC-04 nachgezogen.
- [ ] AC-06: Überlagerung und Determinismus getestet, `testdata/golden/` unverändert (B-992/AC-03).

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
