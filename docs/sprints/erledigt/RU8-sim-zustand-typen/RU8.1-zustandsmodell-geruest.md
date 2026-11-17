# RU8.1 · Zustandsmodell festschreiben, Crate-Gerüst

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru8
- **Abhängig von:** –
- **Tickets:** B-1014
- **Kriterien:** AC-01

## Ziel

Das Zustandsmodell steht in `docs/standards/rust-sim-zustand.md`; Crate `k3c-sim` hat das Gerüst `state/`.

## Kontext

Die zwölf Vorentscheidungen stehen in der Sprint-README RU8 › Vorentscheidungen; sie werden übernommen, nicht neu entschieden. Je Fall (Verweis, Reihenfolge, Löschen, Teilung, Vorrat-Tausch, zwei Dinge zugleich ändern) ein kurzes Rust-Beispiel neben dem Go-Gegenstück. Leitfaden `docs/standards/rust-port.md`.

## Erlaubte Dateien

- `docs/standards/rust-sim-zustand.md`
- `rust/crates/k3c-sim/src/lib.rs`
- `rust/crates/k3c-sim/src/state/**`
- `rust/crates/k3c-sim/Cargo.toml`

## Nicht-Ziele

Typen portieren (RU8.2 ff.). Neue Crates außer Workspace-Abhängigkeiten (`serde`, `serde_json`, `k3c-rng`, `k3c-data`).

## Schritte

1. Sprint-README RU8 und Leitfaden lesen.
2. `rust-sim-zustand.md` schreiben (Vorentscheidungen + Beispiele).
3. `lib.rs`, `state/mod.rs` anlegen; `cargo build -p k3c-sim`, `task rust:lint`.

## Fertig, wenn

- [ ] AC-01: Dokument mit einem Beispiel je Fall.
- [ ] `cargo build -p k3c-sim` und `task rust:lint` grün.

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

AC-01 geprüft: `docs/standards/rust-sim-zustand.md` mit den zwölf Vorentscheidungen und je einem Go/Rust-Beispiel (Verweis, Reihenfolge, Löschen, Teilung, Vorrat-Tausch, zwei Dinge zugleich, Rückverweis, JSON-Felder). `cargo build -p k3c-sim`, `task rust:lint` und `task check:diff` grün. Crate-Gerüst: `lib.rs` (`pub mod state`), `state/mod.rs`; `Cargo.toml` hat serde und serde_json aus dem Workspace. Abweichungen: keine.
