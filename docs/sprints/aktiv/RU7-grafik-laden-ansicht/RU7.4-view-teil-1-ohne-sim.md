# RU7.4 · view Teil 1 ohne Sim: cell, crown, keys, stress

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru7
- **Abhängig von:** –
- **Tickets:** B-1014
- **Kriterien:** AC-03

## Ziel

Die Teile von `native/k3c-game/view` ohne Sim-Typen (Kamera und Zellen, Krone, Tastenbelegung, Stress-Grenzen) stehen in Rust.

## Kontext

Quellen: `native/k3c-game/view/cell.go` (121, Test), `crown.go` (7, Test braucht `sim.Crown*` → `#[ignore = "wartet auf RU8"]`), `keys.go` (33), `stress.go` (61), `stress_limits.go` (57), `doc.go` (68). Bestehend aus RU4.5: `rust/crates/k3c-game/src/view/{mod,input,profile,settings,devkeys}.rs`. `ebiten.Image` als Ziel → `&mut dyn Canvas`; die Zelle wird beim Zeichnen mit `Canvas::clip` beschnitten (RU4.1), Koordinaten bleiben Fensterkoordinaten. `UNIT_PX` = 32. Leitfaden § 1–3, § 8.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/view/mod.rs`
- `rust/crates/k3c-game/src/view/cell.rs`
- `rust/crates/k3c-game/src/view/cell_tests.rs`
- `rust/crates/k3c-game/src/view/crown.rs`
- `rust/crates/k3c-game/src/view/crown_tests.rs`
- `rust/crates/k3c-game/src/view/keys.rs`
- `rust/crates/k3c-game/src/view/stress.rs`
- `rust/crates/k3c-game/src/view/stress_limits.rs`

## Nicht-Ziele

Frame-Vertrag und alles mit Sim-Typen (RU7.5).

## Schritte

1. Go-Dateien und Tests ganz lesen.
2. `view/mod.rs`: Doc-Kommentar aus `doc.go` vollständig übernehmen, neue Module eintragen.
3. Port `cell.rs` (`WorldShare`, `Camera`, `Cell`, Lage der Zellen bei 1 und 2 Spielern), `crown.rs`, `keys.rs`, `stress.rs`, `stress_limits.rs`.
4. Tests `cell_tests.rs`, `crown_tests.rs` portieren.

## Fertig, wenn

- [ ] AC-03: `cargo test -p k3c-game view` grün; `cell_tests` deckt zwei Bildhälften (Zellen übereinander) und die Kamera-Abbildung Welt-x → Fenster-x ab.
- [ ] `task rust:lint` grün.

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
