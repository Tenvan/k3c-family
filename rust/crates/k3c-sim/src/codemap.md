# rust/crates/k3c-sim/src/

## Responsibility

Quelltext des Crates `k3c-sim`: die deterministische Simulation, Port von `engine/sim` (Go) nach Rust. Das Spiel
(`k3c-game`) und der Raum (`k3c-room`) rechnen damit; neue Mechaniken entstehen laut Entscheidung 013 nur hier.

## Design

- `lib.rs`: nur die Modulliste. Je Gruppe ein Unterordner, dazu `links`.
- `links.rs`: Import-Stelle der Gruppen `enemies` und `unit` (Port-Leitfaden § 7); re-exportiert Funktionen anderer Gruppen
  (`boss`, `crown`, `building`, `events`, `monarch`, `economy`, `world`, `island`, `hub`, `merchant`, `portal`, `dev::pure`),
  damit die Imports den Go-Dateien folgen. Go-Zeiger auf Gegner und Truppen sind dort Indizes (`ei`, `ti`).
- Unterordner: [boss](boss/codemap.md), [building](building/codemap.md), [campaign](campaign/codemap.md),
  [cargo](cargo/codemap.md), [combat](combat/codemap.md), [coop](coop/codemap.md), [crown](crown/codemap.md),
  [dev](dev/codemap.md), [economy](economy/codemap.md), [enemies](enemies/codemap.md), [events](events/codemap.md),
  [festival](festival/codemap.md), [hermit](hermit/codemap.md), [hub](hub/codemap.md), [island](island/codemap.md),
  [merchant](merchant/codemap.md), [monarch](monarch/codemap.md), [mounts](mounts/codemap.md), [portal](portal/codemap.md),
  [skills](skills/codemap.md), [state](state/codemap.md), [story](story/codemap.md), [unit](unit/codemap.md),
  [weather](weather/codemap.md), [world](world/codemap.md).

## Flow

Ein Tick einer Stufe läuft in `world::step::step` (Reihenfolge dort festgelegt); eine Insel ruft es je Stufe aus
`island::step_island` und hängt danach Fracht, Reise und Wechsel an. Die Reihenfolge der rng-Würfe ist Teil des Ergebnisses
(Entscheidung 012, Port-Orakel ist das Verhalten).

## Integration

- Konsumenten: `k3c-room` (Raum, Dev-Aktionen, Spielstand), `k3c-game` (Anzeige liest `World`, `boss::endboss::at_lair`).
- Abhängigkeiten: `k3c-data` (Daten aus `data/*.json`: Bosse, Gegner, Gebäude, Events, Fracht, Koop, Economy).
