# RU5.3 · rig Teil 3: load, matrix, order, ride

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
