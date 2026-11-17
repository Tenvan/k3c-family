# RU8.5 · Spielstand: save, save_economy, Rundlauf JSON

- **Status:** fertig
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

AC-04 geprüft: `rust/crates/k3c-sim/tests/golden_state.rs` lädt jeden Snapshot (`snapshots[i].world`) aller 13 Sim-Golden und beider Kampagnen-Golden (über 500 Zustände) als `World`, schreibt ihn und vergleicht als JSON-Baum (`k3c_det::golden::diff`, Zahlen bitgleich); dazu laufen die zwei Kampagnen-Spielstände (`save`) durch `SaveGame`. Beide Tests grün, Golden nur gelesen.
Port: `save.rs` (SaveGame, HubSave, PlayerSave, TroopSave, SiteSave, key, keys, without, node_key, pickup_key, hub_save, site_save, parse_save), `save_apply.rs` (apply_hub, apply_site, apply_hub_level, apply_upgrade, set), `save_economy.rs` (CraftSave, PlotSave, craft_save, apply_craft, site_by_key, apply_plots, troops_save, apply_troops, validate_economy), `fehler.rs` (`enum Fehler`). Platzhalter in `pending.rs`: spawn_vagrant, make_fighter, is_soldier, is_fighter, hub_steps, guards_cleared_keys, count_fetching, apply_guards, apply_portals, plot_x (Sprint-Nummern ungeprüft zugeordnet).
Abweichungen: (1) `Campaign.ToSave` und `FromSave` fehlen, sie hängen an `Campaign` (RU14), kein Ticket nötig. (2) `parse_save` bekommt `biomes` und `defaults` als Parameter statt Paket-Variablen; `apply_craft` bekommt den Wertesatz statt der Welt; `craft_save` die Welt (Baum = Knoten-ID). (3) `nature` mit `omitzero`: leere Maps fehlen wie nil-Maps. (4) Die JSON-Fehlertexte von `serde_json` weichen von Go ab (nur der Teil nach `spielstand: `). Neue Cargo-Abhängigkeiten: keine.
Tests grün (`cargo test -p k3c-sim`: 41 + 2, 11 `ignore`): 9 Unit-Tests in `save_tests.rs`. `ignore` mit Grund: spielstand_wirtschaft_rundlauf, spielstand_wirtschaft_fixture, spielstand_unbekannte_truppenart_fehler (alle `wartet auf RU21`: Insel, ToSave, FromIslandSave, Fixtures). Aus RU8.4 weiter offen: 8 `ignore` (RU14/RU18). `task rust:lint` und `task check:diff` grün.
