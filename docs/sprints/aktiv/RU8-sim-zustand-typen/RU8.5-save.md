# RU8.5 · Spielstand: save, save_economy, Rundlauf JSON

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru8
- **Abhängig von:** RU8.4
- **Tickets:** B-1014
- **Kriterien:** AC-04

## Ziel

Ein Zustand aus einem Sim-Golden lädt in Rust und ergibt geschrieben denselben JSON-Baum.

## Kontext

Quellen `engine/sim/save.go` (351), `save_economy.go` (145, T). Golden unter `testdata/golden/` nur lesen (Golden-Lesen aus RU3, `k3c-det`/Test-Helfer). Vergleich als JSON-Baum (`serde_json::Value`), nicht als Bytes. Ausnahme der Sim-Sperre aus B-1014 gilt nur für Tests.

## Erlaubte Dateien

- `rust/crates/k3c-sim/src/state/**`
- `rust/crates/k3c-sim/src/lib.rs`
- `rust/crates/k3c-sim/tests/**`

## Nicht-Ziele

Golden neu schreiben. Änderungen an Go-Code.

## Schritte

1. Go-Dateien und Test lesen.
2. Port save, save_economy samt Test.
3. Rundlauf-Test über alle Golden mit Sim-Zustand.
4. `cargo test -p k3c-sim`, `task rust:lint`.

## Fertig, wenn

- [ ] AC-04: Rundlauf-Test grün für alle Sim-Golden mit Zustand.
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
