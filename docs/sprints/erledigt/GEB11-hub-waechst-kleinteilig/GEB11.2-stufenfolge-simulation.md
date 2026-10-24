# GEB11.2 · Neue Stufenfolge in Daten und Simulation

- **Status:** verworfen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** geb11/2-stufenfolge-simulation
- **Abhängig von:** GEB11.1
- **Tickets:** B-458
- **Kriterien:** AC-02

## Ziel

Entfällt bei „nur Grafik“ (Sprint-README › Ausnahme- und Fehlerfälle).

## Kontext

Eine neue Stufenfolge im Server (mehr Hub-Stufen, Freischaltungen, Schilde) ändert `data/hub.json` › `levels`, `engine/sim/hub_level.go`, die Linien-Regel in `engine/sim/world_lines.go`, Golden und Balancing. Das ist eine kritische Entscheidung für 🧑 (Vorlage in der Sprint-README › Offene Fragen).

## Erlaubte Dateien

– (verworfen)

## Nicht-Ziele

Jede Änderung an `engine/` und `data/`.

## Schritte

1. Keine; Session verworfen.

## Fertig, wenn

- [ ] AC-02 entfällt begründet (Ergebnis).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Verworfen (2026-10-22, autonom, Claude). Grund: Entscheidung „nur Grafik“ (GEB11.1); eine neue Stufenfolge im Server ist eine kritische Entscheidung (Regeln, Server, Protokoll, Spielstand, Golden, Balancing) und wird 🧑 in einem eigenen Sprint vorgelegt (Vorlage: Sprint-README › Offene Fragen). AC-02 entfällt damit begründet: Simulation und Daten bleiben unverändert, `task check:go` und `task balance` haben nichts Neues zu prüfen.
