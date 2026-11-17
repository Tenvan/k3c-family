# RU4.6 · Eingabe-Port Teil 1: bindings, tracker

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru4
- **Abhängig von:** RU4.5
- **Tickets:** B-1014
- **Kriterien:** AC-03

## Ziel

Die reine Auswertung eines Geräts (Rohzustand → Befehl und Menü-Aktionen) und die Belegung je Profil laufen in Rust wie in Go.

## Kontext

Quellen: `native/k3c-game/input/bindings.go` (205, Tests `bindings_test.go`, `profile_test.go`), `input/tracker.go` (248, Test `tracker_test.go`). Ziel `rust/crates/k3c-game/src/input/` (Leitfaden § 2). Verträge aus RU4.5 (`crate::view::{Action, Command, DeviceID, Input, …}`), Tasten und Pad-Typen aus `k3c_platform` (`Key`, `PadButton`, `PadAxis`). B ist `ActionBack` in Menüs, im Spiel unbelegt (`view/doc.go` › Ablauf).

## Erlaubte Dateien

- `rust/crates/k3c-game/src/input/**`
- `rust/crates/k3c-game/src/lib.rs`

## Nicht-Ziele

`input/source.go` (RU4.7). Änderungen an `view` oder `k3c-platform`; fehlt dort etwas: Ticket.

## Schritte

1. Go-Dateien und Tests ganz lesen.
2. `input/mod.rs`, `bindings.rs`, `tracker.rs`; `lib.rs`: `pub mod input;`.
3. Tests `bindings_tests.rs`, `profile_tests.rs`, `tracker_tests.rs` portieren.
4. `cargo test -p k3c-game input`.

## Fertig, wenn

- [ ] AC-03: `cargo test -p k3c-game input` grün; alle Tests aus `bindings_test.go`, `profile_test.go`, `tracker_test.go` portiert, darunter B → `ActionBack`.
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

- `k3c-game/src/input/`: `bindings.rs` (Belegung beider Profile, Pad-Aktionen, Layouts), `tracker.rs` (Wiederholung, Stick-Hysterese, Home-Akkord, erster Takt ignoriert gehaltene Tasten), Tests aus `bindings_test.go`, `tracker_test.go`, `profile_test.go` (gleiche Namen in snake_case); gemeinsame Helfer in `test_util.rs`.
- `cargo test -p k3c-game input` grün, Clippy grün. Review (Sonnet) gegen Go: verhaltensgleich, nur leichte Notizen.
- Abweichungen: Tracker-`KeyState` ist das Bitfeld der Plattform; `step` hängt an `&mut Vec<Action>` an; Layout als `i64`, Tabelle `LAYOUT_TABLE`.
