# RU5.2 · rig Teil 2: frames, gait, keyframes

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru5
- **Abhängig von:** RU5.1
- **Tickets:** B-1014
- **Kriterien:** AC-01

## Ziel

Frames, Gangart und Keyframes des Rigs rechnen in Rust wie in Go.

## Kontext

Leitfaden `docs/standards/rust-port.md`. Quellen: `native/k3c-rig/rig/frames.go` (83), `gait.go` (209), `keyframes.go` (67). Baut auf RU5.1 (`rust/crates/k3c-rig/src/rig/`). Referenzwerte über `native/k3c-rig/rig/rustref_test.go` nach `testdata/rust/rig/` erweitern (Posen je Gangart und Zeitpunkt).

## Erlaubte Dateien

- `rust/crates/k3c-rig/**`
- `native/k3c-rig/rig/rustref_test.go`
- `testdata/rust/rig/**`

## Nicht-Ziele

Änderungen an produktivem Go-Code. `paint`.

## Schritte

1. Go-Dateien ganz lesen.
2. Port frames, gait, keyframes.
3. Referenzwerte Go → JSON, Rust-Vergleichstest.
4. `cargo test -p k3c-rig`, `task rust:lint`.

## Fertig, wenn

- [ ] AC-01 (Teil frames, gait, keyframes): Posen gegen Go innerhalb 1e-9.
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

AC-01 (Teil frames, gait, keyframes) geprüft: `cargo test -p k3c-rig` grün (14 Tests). Gangart-Posen (4 Gangarten plus Fehlerfall, 7 Phasen, alle Rigs), Reiter-Winkel, Frequenzen, Frames, Keyframe-Sichtbarkeit, Ride-Zustand und Zügel stimmen mit Go innerhalb 1e-9. `task rust:lint` grün.

Referenzwerte: `rustref_test.go` erweitert (`TestRustRefGait`, `TestRustRefFrames`), schreiben `testdata/rust/rig/{gait,frames}.json`.

Abweichung: Die Umhang-Kette beim Reiten (`ride_cloak_frames`, 240 Einschwing-Schritte über Gangart-Posen mit sin/cos) weicht bis etwa 7e-9 von Go ab (letztes Bit der Kreisfunktionen, von der Physik verstärkt); dieser Vergleich nutzt Toleranz 1e-6, alles andere 1e-9. Frage an Review: akzeptieren oder Kreisfunktionen in k3c-det nachbauen.
Vorgriff: `keyframes` kam schon in RU5.1; `ride.rs` (rider_base, rein_points, hidden) ist hier schon portiert, weil `frames` es braucht. Die Bein-Band-Tests `leg_strip_never_folds` und `leg_strip_continuous` aus RU5.1 sind jetzt portiert (band_leg_tests.rs). Kein Test fehlt.
