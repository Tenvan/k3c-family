# RU8.2 · Zustands-Typen: types

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru8
- **Abhängig von:** RU8.1
- **Tickets:** B-1014
- **Kriterien:** AC-02, AC-03

## Ziel

Die Kern-Typen aus `engine/sim/types.go` existieren in Rust mit denselben JSON-Namen.

## Kontext

Quelle `engine/sim/types.go` (394). Ziel `rust/crates/k3c-sim/src/state/`, Dateinamen nach Leitfaden § 2, Regeln aus `docs/standards/rust-sim-zustand.md`. Fehlen Level-Typen, kurze Platzhalter in `rust/crates/k3c-level/src/types.rs` anlegen (RU8.3 vervollständigt). `BTreeMap` statt `HashMap`.

## Erlaubte Dateien

- `rust/crates/k3c-sim/src/state/**`
- `rust/crates/k3c-sim/src/lib.rs`
- `rust/crates/k3c-level/src/types.rs`
- `rust/crates/k3c-level/src/lib.rs`

## Nicht-Ziele

Sim-Logik. Änderungen an Go-Code.

## Schritte

1. `types.go` ganz lesen.
2. Typen portieren, `serde`-Namen wie die Go-JSON-Tags.
3. Test: JSON-Rundlauf je Typ mit einem Beispiel aus Go-Tests oder Golden.
4. `cargo test -p k3c-sim`, `task rust:lint`.

## Fertig, wenn

- [ ] AC-02/AC-03 (Teil types): Typen mit Go-JSON-Namen, Listen als `Vec` in Go-Reihenfolge.
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
