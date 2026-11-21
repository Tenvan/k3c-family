# LA3.1 · Bauplatz-Abstände nach Gebäudebreite (B-1182)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** hauptrechner
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/la3
- **Abhängig von:** –
- **Tickets:** B-1182
- **Kriterien:** AC-01

## Ziel

Feste Bauplätze richten ihren Abstand nach der Größe der Gebäude, sodass Gebäude sich nicht mehr überlappen.

## Kontext

Bauplatz-Positionen stehen fest in `data/hub.json` und werden in `k3c-sim` berechnet; gezeichnet werden sie in `rust/crates/k3c-game/src/props/plan_sites.rs` und `props/plan.rs` (Reihenfolge: decor, back_walls, sites_pass(true), castle, entities, nodes, sites_pass(false)). Gebäudebreiten je Welt aus `data/sizes.json` (Größenstufen, LA1.1) und den Papierschnitt-Maßen. Ticket B-1182. Neue Mechanik nur in Rust (Entscheidung 013), die Go-Engine bleibt eingefroren: Golden und Go-Tests (`engine/`, `testdata/golden/`) werden nie zugunsten von Rust geändert. Verändert eine Änderung an `data/hub.json` das Go-Verhalten (Go-Golden rot), dann die Abstände stattdessen in Rust berechnen und `data/hub.json` unverändert lassen. Sperre: `golden|spielstand`.

## Erlaubte Dateien

- `rust/crates/k3c-sim/src/**`
- `rust/crates/k3c-data/src/**`
- `rust/crates/k3c-game/src/props/**`
- `data/hub.json`, `data/buildings.json`, `data/sizes.json` (nur wenn das Go-Golden grün bleibt)
- `testdata/rust/**` (nur Rust-Golden, mit Begründung im Commit)

## Nicht-Ziele

Keine Balance-Änderung, keine Simulationsläufe.

## Schritte

1. Gebäudebreiten je Typ und Welt aus `data/sizes.json` und den Assets bestimmen.
2. Abstandsregel für Bauplätze in `k3c-sim`: Mitte zu Mitte mindestens halbe Breite beider Nachbarn plus Mindestlücke; deterministisch, 2 Spieler.
3. Alte Spielstände: keine Migration (🧑 2026-11-21: egal). Spielstand-Version nur erhöhen, wenn das Format sich ändert.
4. Test: kein Paar benachbarter Bauplätze überlappt, für alle festen Welten und Hub-Stufen.
5. Nur Rust-Golden bewusst neu schreiben (Begründung im Commit); Go-Golden unverändert.
6. `cargo test -p k3c-sim -p k3c-game --release`, `task rust:lint`, einmal `task check:diff`; im Ergebnis nennen, was 🧑 im Spiel prüfen soll (Hub in allen festen Welten, Gebäude nebeneinander).

## Fertig, wenn

- [ ] AC-01: Kein Paar benachbarter Bauplätze überlappt in der Breite; Test über alle festen Welten grün.
- [ ] Go-Golden und Go-Tests unverändert grün (`task check:go` ohne Lint).
- [ ] Tests der geänderten Crates, `task rust:lint` und `task check:diff` grün.

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

AC-01 umgesetzt und geprüft: `k3c_data::load_game` (`hub_layout.rs`) spreizt die Hub-Plätze nach den Breiten aus `data/sizes.json › bauplaetze` (Lücke 1,5 Units, Treppen bleiben stehen) und schiebt Turm und Mauer der Zone 1 hinter das äußerste Gebäude (Turm ≈ 53, Mauer ≈ 57, Zahlabstand 4). Das Spiel lädt damit (`local/worlds.rs`), `load_all` und `data/hub.json` bleiben roh wie für Go. Tests: `hub_layout_tests.rs` (kein Paar überlappt, Turm und Mauer hinter den Gebäuden, Änderung nur nach außen, stabil), `k3c-sim hub/layout_tests.rs` (`validate_hub_data` ohne Befund auf den gespreizten Daten). Go-Golden unverändert; `testdata/rust/data-tree.json.gz` mit `go test ./engine/internal/detvec -write` neu geschrieben, weil es `sizes.json` roh enthält. Review (k3c-reviewer): zwei Befunde (Treppenregel, Zone 1), behoben. Entscheidung 🧑: Option Hub-Layout in Rust, alte Spielstände egal.

Rote Tests, nicht aus dieser Session: `alloc_je_bild`, `e2e_flow`, `insel_nach_1000_schritten_wie_go` (auf sauberem main ebenfalls rot), `eingebettete_welten_gueltig` und `alle_welt_dateien_aus_data_sind_ladbar` (lokale, nicht eingecheckte `data/worlds/rom.json`). `task check:diff`: nur `golangci-lint` fehlt lokal.

Für 🧑 am PC: Hub in Standard-Welt auf Stufe 4 (alle Gebäude) ansehen, ob nichts überlappt und Turm und Mauer am Rand stehen; Level-Layout der Zone 1 hat sich in Rust verschoben.
