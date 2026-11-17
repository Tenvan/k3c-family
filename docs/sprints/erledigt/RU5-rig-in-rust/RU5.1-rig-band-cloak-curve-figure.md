# RU5.1 · rig Teil 1: band, cloak, curve, figure

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru5
- **Abhängig von:** –
- **Tickets:** B-1014
- **Kriterien:** AC-01

## Ziel

Kurven, Bänder, Umhang und Figur des Rigs rechnen in Rust wie in Go.

## Kontext

Leitfaden `docs/standards/rust-port.md`. Quellen: `native/k3c-rig/rig/band.go` (186, Test `band_test.go`), `cloak.go` (120), `curve.go` (41), `figure.go` (151). Ziel: `rust/crates/k3c-rig/src/rig/` mit `mod.rs`, Dateinamen nach Leitfaden § 2. `f64` in Go-Reihenfolge; `sin`/`cos` aus std erlaubt. Referenzwerte für AC-01: ein Go-Test `native/k3c-rig/rig/rustref_test.go` schreibt mit `-update` JSON nach `testdata/rust/rig/`; Rust liest sie im Test (Toleranz 1e-9). Kein produktiver Go-Code wird geändert.

## Erlaubte Dateien

- `rust/crates/k3c-rig/**`
- `native/k3c-rig/rig/rustref_test.go`
- `testdata/rust/rig/**`

## Nicht-Ziele

Änderungen an produktivem Go-Code. `paint`. Neue Crates (nur `serde`/`serde_json` aus dem Workspace).

## Schritte

1. Go-Dateien und `band_test.go` ganz lesen, Leitfaden lesen.
2. Port nach `rust/crates/k3c-rig/src/rig/` (band, cloak, curve, figure), Test `band_test.go` portieren.
3. `rustref_test.go` anlegen (Referenzwerte, falls hier schon sinnvoll), Rust-Vergleichstest.
4. `cargo test -p k3c-rig`, `task rust:lint`.

## Fertig, wenn

- [ ] AC-01 (Teil band, cloak, curve, figure): `cargo test -p k3c-rig` grün, Werte gegen Go innerhalb 1e-9.
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

AC-01 (Teil band, cloak, curve, figure) geprüft: `cargo test -p k3c-rig` grün (8 Tests), Werte gegen Go innerhalb 1e-9 (curve, band inkl. Bein-Stützpunkte und tangent_limits, figure-Posen/Frequenzen/Fehlertexte, cloak). `task rust:lint` (clippy -D warnings) grün.

Referenzwerte: `native/k3c-rig/rig/rustref_test.go` (`-update`) schreibt `testdata/rust/rig/{curve,band,figure}.json`.

Abweichungen vom Plan (Vorgriff, weil der Ladevorgang und figure sie brauchen): `matrix`, `load` (Typen, Load, link), `order`, `keyframes` komplett sowie in `gait` nur Konstanten, `LegIndex`, `leg_indices`. RU5.2 ergänzt den Rest von `gait` und `frames`; RU5.3 `ride` und `expected_test`.
Nicht portiert, mit Grund: `TestLegStripNeverFolds` und `TestLegStripContinuous` brauchen `GaitPose` (RU5.2), kommen dort. `Cargo.lock` ändert sich (serde, serde_json für k3c-rig).
