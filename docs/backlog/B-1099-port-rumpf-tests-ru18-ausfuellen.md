# B-1099 · Rumpf-Tests der Rust-Sim (leer oder unimplemented) mit echten Rümpfen ausfüllen

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU52
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Beim Aktivieren der `wartet auf RU18`-Tests (RU18.3) zeigte `cargo test -p k3c-sim -- --ignored` Tests, die nur wegen eines leeren Rumpfs (`fn x() {}`) grün sind, dazu Tests mit `unimplemented!("RU18: ...")` im Rumpf. Das sind Go-Tests, die nie portiert wurden. Leer: crown::review_tests (fortified_aura_fuer_truppen, last_stand_nur_mit_letzter_muenze), crown::tests (treffer_kostet_muenze_krone_faellt_ohne_gold, ohne_krone_nur_laufen, kronenraub_ist_game_over, kronenraub_stuft_mit_insel_over, burgfall_beendet_das_spiel_nicht), portal::tests (angegriffenes_portal_spawnt_verteidiger, dieb_ohne_portal_laesst_beute_fallen, portal_simulation_ist_deterministisch, letztes_portal_*, sichere_stufe_bleibt_nach_dem_laden), state::views_tests (fackeln_der_welt, knoten_markierer_in_snapshot_und_spielstand, purse_sizes, purse_respec_drops_new_gold), state::save_tests (3 Stück, RU21). Unimplemented: building::ruin_tests (8), outpost_tests (dev_castle_fall...), repair_tests (spielstand_hub_und_platz_stufen), spell_tower_tests (zaubertum_schuetzen_steigen_ab), upgrades_tests (upgrade_elite_krieger), island::carry_tests (3), island::storage_tests (island_storage_five_materials).

## Ziel

Jeder dieser Tests prüft die Regel seines Go-Tests; kein Test ist ohne Rumpf grün.

## Beteiligte und Zielgruppen

Agenten der Review-Session RU18.6 und RU22.

## Anforderungen

- Rümpfe aus den Go-Tests portieren; wo Voraussetzungen fehlen (RU18.4, RU18.5, RU19, RU10), `ignore` mit der genauen Voraussetzung lassen.
- Kein leerer Rumpf ohne `ignore`.

## Nicht-Ziele

Änderungen an Go-Code, Regel- oder Wertänderungen.

## Regeln und Einschränkungen

Port-Leitfaden `docs/standards/rust-port.md` § 6; Entscheidung 012.

- **Bereich:** `rust/crates/k3c-sim/src/crown/**`, `rust/crates/k3c-sim/src/portal/**`, `rust/crates/k3c-sim/src/state/views_tests.rs`, `rust/crates/k3c-sim/src/building/**`, `rust/crates/k3c-sim/src/island/carry_tests.rs`, `rust/crates/k3c-sim/src/island/storage_tests.rs`
- **Abhängig von:** RU18.5
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** –

## Beispiele

`purse_sizes` prüft die Beutelgröße je Skill-Stand wie `TestPurseSizes` (purse_test.go).

## Ausnahme- und Fehlerfälle

nicht relevant: reine Testarbeit.

## Akzeptanzkriterien

- **AC-01** `grep -rn 'fn [a-z_0-9]*() {}' rust/crates/k3c-sim/src` findet unter den Tests aus der Ausgangslage keinen leeren Rumpf mehr; `cargo test -p k3c-sim -- --ignored` zeigt keinen dieser Tests grün ohne echten Rumpf.

## Offene Fragen

keine

## Notizen

Gefunden in RU18.3. Zwei fehlerhaft portierte Tests (`start_handwerker`, `pool_drei_portale_in_der_welt` nahmen `quiet_world` statt `newWorld`) und `gegner_ignores_walls_laeuft_an_der_mauer_vorbei` (ohne `sturdy`) sind dort schon korrigiert.
