# RU8.4 · Kleine Typen: action_view, node_mark, purse, site_view, torches

- **Status:** fertig
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

AC-02 (Teil) geprüft: `action_view.rs` (train_at, trade_at), `node_mark.rs` (node_markable, mark_node, marked_by_of, apply_marked_by), `purse.rs` (purse_size, purse_max_of, purse_skill), `site_view.rs` (site_hub_level_of = Go `SiteHubLevel`, site_zone, site_visible), `torches.rs` (torch_xs_of, torch_xs; die Versätze kommen als Parameter statt aus der Paket-Variable `torchOffsets`, Quelle `k3c_data::light::Light::torches`). `save.rs` enthält vorab nur `key` (von node_mark gebraucht, RU8.5 ergänzt die Datei). Neue Platzhalter in `pending.rs`: find_pay_target samt PayTarget/PayOffer, profession_of, stock_field, resource_full, gather_of, skill_ref, purse_max, site_hub_level, level_site (alle RU18, ungeprüft zugeordnet; level_site eher mit der Welt-Linien-Gruppe).
Tests (`views_tests.rs`): grün 5: fackel_plaetze (Go `TestFackelPlaetze` mit `testdata/torches.json`), knoten_markierer_fehlt_im_alten_stand (ohne Kampagne, über marked_by_of/apply_marked_by), marked_by_of_ohne_markierer_ist_leer, key_rundet_wie_to_fixed, fackel_versaetze_aus_light_json. `ignore` mit Namen und Grund (8): fackeln_der_welt, knoten_markierer_in_snapshot_und_spielstand, site_hub_level_je_art, site_visible_bis_naechste_stufe (alle `wartet auf RU14`: quietWorld, Kampagne), purse_sizes, purse_respec_drops_new_gold, site_hub_level_passt_zu_bezahlbar (`wartet auf RU18`: Insel, Skills, sitePayable). Die ignorierten Rümpfe sind leer; ihre Go-Fälle werden mit dem genannten Sprint portiert. `cargo test -p k3c-sim`, `task rust:lint`, `task check:diff` grün.
