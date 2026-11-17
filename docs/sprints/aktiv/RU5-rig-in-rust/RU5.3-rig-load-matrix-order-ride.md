# RU5.3 · rig Teil 3: load, matrix, order, ride

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru5
- **Abhängig von:** RU5.2
- **Tickets:** B-1014
- **Kriterien:** AC-01

## Ziel

Alle Rig-Dateien unter `native/k3c-rig/assets` laden in Rust; Knochen-Transformationen je Pose stimmen mit Go.

## Kontext

Leitfaden `docs/standards/rust-port.md`. Quellen: `native/k3c-rig/rig/load.go` (244), `matrix.go` (49), `order.go` (73), `ride.go` (40), dazu `expected_test.go` (portieren). Baut auf RU5.1, RU5.2. Referenz: `rustref_test.go` schreibt für jede Rig-Datei und Pose die Knochen-Matrizen nach `testdata/rust/rig/`.

## Erlaubte Dateien

- `rust/crates/k3c-rig/**`
- `native/k3c-rig/rig/rustref_test.go`
- `testdata/rust/rig/**`

## Nicht-Ziele

Änderungen an produktivem Go-Code. `paint`.

## Schritte

1. Go-Dateien und `expected_test.go` ganz lesen.
2. Port load, matrix, order, ride; `expected_test.go` portieren.
3. Vergleich aller Assets und Posen gegen Go-Referenz.
4. `cargo test -p k3c-rig`, `task rust:lint`.

## Fertig, wenn

- [ ] AC-01: alle Rig-Dateien laden; Knochen-Transformationen je Pose innerhalb 1e-9 zu Go.
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

AC-01 geprüft: `cargo test -p k3c-rig` grün (19 Tests). Alle 32 Rig-Dateien unter `native/k3c-rig/assets` laden in Rust; Indizes, Eltern, Topo-Reihenfolge, Pose-Faktoren, Zeichenreihenfolge, Bein-Indizes und Keyframe-Teile stimmen mit Go (`load.json`). Knochen-Matrizen je Rig und Pose (Figur-Posen, Gangarten), Platz-Matrizen, Inverse und `hidden` stimmen innerhalb 1e-9 (`world.json`). `expected_test.go` ist portiert (`expected_tests.rs`: `animations_match_paper`, `ride_matches_paper`, Toleranz 0,01 gegen assets/soll) und grün. `task rust:lint` grün.

Referenz: `rustref_test.go` erweitert (`TestRustRefLoad`, `TestRustRefWorld`).
Vorgriff aus RU5.1/RU5.2: `load`, `matrix`, `order`, `ride` waren schon portiert; diese Session fügte Tests hinzu (Vergleich, expected, `load_fehler` für fehlende Datei, kaputtes JSON, unbekannte Eltern, Kreis, Band/Platz an unbekanntem Teil). Keine neuen Tickets, kein Test fehlt.
