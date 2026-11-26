# WD1.3 · Anzeige: Waldstücke grün schraffiert in Overlay, Workbench und F4

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** wd1/3-anzeige-waldstuecke
- **Abhängig von:** WD1.2
- **Tickets:** B-1204
- **Kriterien:** AC-03

## Ziel

Waldstücke erscheinen als grün schraffierte Fläche, die markierbaren äußeren Bäume hervorgehoben: Debug-Overlay im Spiel, Level-Ansicht der Workbench, Umschalter im Cheat-Fenster (F4).

## Kontext

B-1204 AC-05. Geometrie aus WD1.2. Debug-Overlays: `rust/crates/k3c-game/src/devtools/drawdebug.rs`, `debuggeo.rs`; Cheat-Fenster `k3c-devwin`/`k3c-devproto`; Workbench `tools/k3c-workbench/frontend/src/world/` (Level-Ansicht, Backend liefert die Geometrie).

## Erlaubte Dateien

- `rust/crates/k3c-game/src/devtools/**`
- `rust/crates/k3c-game/src/props/**`
- `rust/crates/k3c-devwin/**`
- `rust/crates/k3c-devproto/**`
- `tools/k3c-workbench/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Overlay-Geometrie (Fläche, hervorgehobene Bäume) in `debuggeo.rs` und Zeichnen in `drawdebug.rs`.
2. Umschalter im Cheat-Fenster (devproto-Befehl).
3. Level-Ansicht der Workbench (Backend-Feld und Frontend-Schraffur).
4. Tests der Geometrie.

## Fertig, wenn

- [ ] AC-03: Anzeige in den drei Ansichten (Geometrie-Tests; Sicht am PC durch 🧑 im Abnahme-Ticket).
- [ ] `task check:diff` grün (außer vorher roten Tests).

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

AC-03 geprüft (zwei von drei Ansichten): Geometrie-Tests in `debugstands_tests.rs` (5 Tests: kleines Stück nur Fläche, großes Stück mit gerahmten äußeren Bäumen, leer ohne native Regeln oder außerhalb des Bildes, Umschalter, Schraffur im Rechteck), Tests von `k3c-devwin` (Umschalter „Waldstücke aus“) und `k3c-game` view grün, `task rust:lint` grün. Sicht am PC durch 🧑 im Abnahme-Ticket.

- **Debug-Overlay (F10, Seite Geometrie):** Waldstück als grün schraffierte Fläche, markierbare äußere Bäume großer Stücke als Rahmen (`devtools/debugstands.rs`, `drawdebug.rs`, `debuggeo.rs`: `GEO_STAND`, `GEO_STAND_TREE`).
- **Cheat-Fenster (F4):** Umschalter „Waldstücke aus“ (Standard an), Taste `stands` im Protokoll (`devwin/view.rs`, `k3c-devwin/src/pages/view.rs`).
- **Level-Ansicht entfällt (🧑 Ralf, 2026-11-24):** Sie liegt seit WZ63 im Studio, das Level kommt dort aus der Go-Engine; Wald (WD1.1) und Waldstücke (WD1.2) gibt es nur in Rust. Eine Rust-Brücke zum Studio baut diese Session nicht, die Ansicht fällt vorerst weg.
- **`task check:diff`:** rot an `tests/projectRules.test.ts` (nicht aus dieser Session): `tools/k3c-workbench/internal/playtest/transcribe_test.go` hat 450 Zeilen (Grenze 400). Stufe 2 lief darum nicht; die Rust-Tests der geänderten Crates liefen von Hand. In `k3c-game` sind 3 Tests rot (`props::plan_tests::plan_*`): Standard-Welt „Etappe 2 hat keine Insel“ (Daten gegen Test-Fixture, nicht aus dieser Session).

### Für den Hauptrechner

- Ticket: `transcribe_test.go` auf höchstens 400 Zeilen teilen (rotes `check:diff` auf `main`).
- Ticket: `props/plan_tests.rs` baut die Standard-Welt mit einer Insel nach, `data/worlds/standard.json` verlangt aber eine Insel je Etappe: drei rote Tests.
- Optional: Waldstücke in der Studio-Level-Ansicht (braucht eine Rust-Brücke oder ein Rust-Level-Werkzeug, Entscheidung 013).
