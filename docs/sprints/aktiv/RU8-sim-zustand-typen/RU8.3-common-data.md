# RU8.3 · Zustands-Typen: common, data, Level-Typen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru8
- **Abhängig von:** RU8.2
- **Tickets:** B-1014
- **Kriterien:** AC-02, AC-03

## Ziel

`common` und `data` der Go-Sim sowie die Level-Typen stehen in Rust.

## Kontext

Quellen `engine/sim/common.go` (230), `engine/sim/data.go` (307), Level-Typen aus `engine/level` (nur Typ-Definitionen, z. B. `Biome`, `Layout`) nach `rust/crates/k3c-level/src/types.rs`. Unveränderliche Wertesätze als `Arc<…>` (Vorentscheidung 6). Daten-Schemas aus `k3c-data` (RU3) wiederverwenden statt doppeln.

## Erlaubte Dateien

- `rust/crates/k3c-sim/src/state/**`
- `rust/crates/k3c-sim/src/lib.rs`
- `rust/crates/k3c-level/src/types.rs`
- `rust/crates/k3c-level/src/lib.rs`
- `rust/crates/k3c-level/Cargo.toml`

## Nicht-Ziele

Level-Generator (RU6). Änderungen an Go-Code.

## Schritte

1. Go-Dateien ganz lesen.
2. Port; Level-Typen als reine Daten.
3. Tests (JSON-Rundlauf).
4. `cargo test -p k3c-sim -p k3c-level`, `task rust:lint`.

## Fertig, wenn

- [ ] AC-02/AC-03 (Teil common, data, Level-Typen).
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
