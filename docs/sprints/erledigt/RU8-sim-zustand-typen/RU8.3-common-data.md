# RU8.3 · Zustands-Typen: common, data, Level-Typen

- **Status:** fertig
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

AC-02/AC-03 (Teil common, data, Level-Typen) geprüft. `state/common.rs` portiert alle Funktionen aus `common.go` (approach, sign, int_ptr, shielded, is_dangerous, outer_wall, site_by_id, troop_by_id, node_by_id, is_worker, apply_damage, hit_troop, apply_damage_by, damage_player, cause_or_other, absorb_by_shield, free_hit, destroy_site) in Go-Reihenfolge; dazu die Nachschlage-Methoden `World::troop/node/site(_mut)` (Vorentscheidung 2). Go-Zeiger auf Spieler und Truppen sind Indizes (`pi`, `ti`), Plätze werden per ID gesucht (`destroy_site(w, id)`). Funktionen späterer Sprints stehen als Platzhalter in `state/pending.rs` (crowned, drop_crown, hit_coins, hit_event, troop_damage, each_passive, invulnerable, is_wave_enemy, damage_merchant, protected_night, is_ruin, values; Konstante MERCHANT_ID); die Sprint-Nummern in den Texten sind aus der Projekt-Tabelle abgeleitet, ungeprüft. `state/data.rs`: Die Lese-Typen und das Laden von `data.go` gibt es seit RU3 in `k3c-data`; sie werden wiedergegeben, ergänzt um first_day_night, has_depth, check_waves. Gos Paket-Variablen und `UseData` entfallen (kein globaler Zustand, Wertesatz = `Arc<WorldValues>` aus `k3c_data::load_all`). Level-Typen standen schon aus RU8.2 in `k3c-level/src/types.rs`. Tests: 8 neue für common (1 `ignore = "wartet auf RU18"`: destroy_site braucht values und is_ruin), 5 für data; Go hat für common.go und data.go keine Testdatei. `cargo test -p k3c-sim -p k3c-level`, `task rust:lint`, `task check:diff` grün.
