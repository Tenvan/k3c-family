# RU8.4 · Kleine Typen: action_view, node_mark, purse, site_view, torches

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru8
- **Abhängig von:** RU8.3
- **Tickets:** B-1014
- **Kriterien:** AC-02

## Ziel

Die kleinen Zustands-Typen mit ihren Tests stehen in Rust.

## Kontext

Quellen `engine/sim/action_view.go` (39), `node_mark.go` (35, T), `purse.go` (31, T), `site_view.go` (28, T), `torches.go` (33, T). Tests werden mitportiert.

## Erlaubte Dateien

- `rust/crates/k3c-sim/src/state/**`
- `rust/crates/k3c-sim/src/lib.rs`

## Nicht-Ziele

Änderungen an Go-Code.

## Schritte

1. Go-Dateien und Tests lesen.
2. Port samt Tests.
3. `cargo test -p k3c-sim`, `task rust:lint`.

## Fertig, wenn

- [ ] AC-02 (Teil): Typen und portierte Tests grün.
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
