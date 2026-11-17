# RU5.2 · rig Teil 2: frames, gait, keyframes

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
