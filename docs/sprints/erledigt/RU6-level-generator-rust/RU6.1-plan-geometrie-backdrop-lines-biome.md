# RU6.1 · Level Teil 1: Gerüst, backdrop, lines, planned, biome

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru6
- **Abhängig von:** RU3, RU39
- **Tickets:** B-1014
- **Kriterien:** AC-02

## Ziel

Das Gerüst des Level-Generators steht in `k3c-level`: Regelsatz, Hintergründe, Mauerlinien, Plan-Geometrie und Plan-Prüfung (`ValidatePlan`) rechnen wie in Go. Die Reihenfolge der Sessions weicht von der Go-Dateireihenfolge ab (Grund: jede Session soll ohne Platzhalter bauen, siehe Sprint-README › Offene Fragen).

## Kontext

Leitfaden `docs/standards/rust-port.md` zuerst, dann `docs/standards/rust-sim-zustand.md` (Entscheidungen 2, 6, 11). Crate `rust/crates/k3c-level`: RU8 legte `lib.rs` und `types.rs` an (Chunk, Entity, Layout, WallSite, Ruin, Guard, NatureValues, PlannedObject, PlannedBuilding, PlannedEnemy, PlannedChunk; `Biome`, `Cycle`, `Trait`, `WeightGroup` aus `k3c_data::biome`). RU3 liegt fertig in `k3c-data` und wird benutzt, nicht neu geschrieben: Biom-Laden samt `checkPortals`, `Cycle.Check`, `checkNature` (`k3c_data::biome::load`, private `check_*`), `Ordered`/`Entry` (`k3c_data::ordered`), `GuardRules`, `RuinRules`, `HubData`/`HubSite`/`SitePlacementData`, alles gesammelt in `k3c_data::data::Data` (`load_all(&Source)`).

Quellen in `engine/level/`: `backdrop.go` (34, Test `backdrop_test.go`, 7 Aufrufe `backdrops`, 2 Tests rufen `Generate`), `lines.go` (77, Hub-Plätze und `payGapUnits` aus `data/hub.json`/`economy.json`; Go liest sie beim Paketstart über `readData`), `planned.go` (276, Test `planned_test.go`: `geometry`, `placed`, `spotKey`, `ValidatePlan`, `ValidatePlanFor`, `ValidatePlanWith`, `checkChunk`, `checkObjects`, `checkPlaced`, `checkPortal`, `checkVein`), `biome.go` (172; fast ganz in `k3c_data::biome`, nur Wiedergabe und Hilfen; Tests `biome_portals_test.go` (3 Aufrufe `checkPortals`) und `cycle_test.go` (3 Aufrufe `Check`) gehören hierher).

Ziel: `rust/crates/k3c-level/src/` mit `backdrop.rs`, `backdrop_tests.rs`, `lines.rs`, `planned.rs`, `planned_tests.rs`, `biome.rs` (Wiedergabe `pub use k3c_data::biome::…` plus die Tests `biome_portals_tests.rs`, `cycle_tests.rs` gegen die Daten-Lader), neu und nur in Rust: `rules.rs`, `testutil.rs`, `pending.rs`.

**Vorentscheidungen (gelten für den ganzen Sprint):**
1. Gos Paket-Variablen (`guardRules`, `ruinRules`, `ruinBuildings`, `hubSites`, `payGapUnits`, `sitePlacement`) gibt es in Rust nicht (wie `state/data.rs` in RU8). `rules.rs` baut `pub struct Rules { guard: GuardRules, ruin: RuinRules, ruin_buildings: BTreeMap<String, BuildingInfo>, hub_sites: Vec<HubSiteRef>, pay_gap_units: f64, site_placement: SitePlacementData }` mit `Rules::from_data(&Data) -> Result<Rules, Fehler>`; `BuildingInfo` ist das Go-`buildingInfo` aus `ruins.go` (Feld-Liste dort lesen). Die Funktionen bekommen `&Rules` oder halten `Arc<Rules>` (Lebenszeit-Parameter an öffentlichen Typen sind verboten). `Default*`-Funktionen von Go (`DefaultGuardRules`, `DefaultRuinRules`, `DefaultSitePlacement`) werden Zugriffe auf `Rules`.
2. `Generate(b, seed)` heißt `generate(b: &Biome, seed: &str, rules: &Rules)`, `GenerateWith(b, seed, o)` heißt `generate_with(b, seed, o, rules)`; entsprechend `validate`, `validate_with`, `validate_plan*`.
3. Private Go-Typen (`geometry`, `placed`, `spotKey`, `gen`) werden `pub(crate)`; das Modul `planned` besitzt `Geometry`, `Placed`.
4. `Ordered[V]` ist `k3c_data::ordered::Ordered<V>` (Reihenfolge der Schlüssel wie in der JSON-Datei); nie in eine `BTreeMap` umwandeln, wo Go in Datei-Reihenfolge läuft.
5. Tests, die erst mit `generate`/`validate` (RU6.5) laufen, werden mit portiert und tragen `#[ignore = "wartet auf RU6.5"]`; RU6.5 entfernt die Marken. Jede Marke steht mit Testname im Ergebnis.
6. `testutil.rs` (`#[cfg(test)]`): Gos Test-Hilfen aus `level_test.go` (`biomeIDs`, `biome`, `generate`) als `biome_ids()`, `biome(id)`, `rules()` (lädt `Data` über `Source::from_env`/Pfad wie `rust/crates/k3c-data/src/source_tests.rs`), `generate(...)` erst mit `unimplemented!("RU6.5")`-Rumpf.
7. Gleitkomma: `math.Max/Min` auf `float64` und eingebautes `min/max` auf `float64` werden `k3c_det::fmax/fmin` (Leitfaden § 5); in diesen Dateien gibt es keinen Würfel-Aufruf.
8. `ValidatePlanWith` (`planned.go` Z. 160 bis 175) ruft `checkBuildings`, `checkGuards`, `checkRuinsAcross` (RU6.3: `ruins.go`, `guards.go`) und `checkTotals` (RU6.3: `planned_totals.go`). Diese Session legt dazu `pending.rs` mit Platzhaltern gleicher Signatur an (`unimplemented!("RU6.3: check_totals")`, Leitfaden § 7); RU6.3 löscht sie. Tests von `planned_test.go`, die diese Pfade erreichen (also jeder erfolgreiche `ValidatePlan`-Aufruf), tragen `#[ignore = "wartet auf RU6.3"]`; Tests, die vorher mit einer Fehlermeldung aussteigen (`checkChunk`, `checkObjects`, `checkPlaced`), laufen.

## Erlaubte Dateien

- `rust/crates/k3c-level/src/**` (Module `backdrop`, `lines`, `planned`, `biome`, `rules`, `testutil` samt `*_tests.rs`)
- `rust/crates/k3c-level/src/lib.rs` (nur die `pub mod`-/`mod`-Zeilen)
- `rust/crates/k3c-level/Cargo.toml` (nur Abhängigkeiten auf vorhandene Workspace-Crates)

## Nicht-Ziele

Änderungen an Go-Code und `testdata/`. `generate`, `nature`, `sites`, `validate`, `ruins`, `guards` (spätere Sessions). Neue Crates. Verbesserungen über den Go-Stand hinaus. `bench_test.go` (Benchmarks, nicht portiert; Grund im Ergebnis von RU6.5).

## Schritte

1. Leitfaden und `rust-sim-zustand.md` lesen, dann die vier Go-Dateien und ihre Tests ganz (`backdrop`, `lines`, `planned`, `biome`).
2. `rules.rs`, `testutil.rs` und `pending.rs` anlegen (Vorentscheidungen 1, 2, 6, 8); `lib.rs` um die `mod`-Zeilen ergänzen.
3. `backdrop.rs`, `lines.rs`, `planned.rs`, `biome.rs` portieren, nach jeder Datei `cargo build -p k3c-level` in `rust/`.
4. Tests portieren: `backdrop_tests.rs`, `planned_tests.rs`, `biome_portals_tests.rs`, `cycle_tests.rs`; Tests mit `Generate` ignorieren (Vorentscheidung 5), Tests, die nur `unimplemented!`-Platzhalter erreichen, mit `#[ignore = "wartet auf RU6.3"]`.
5. `cargo test -p k3c-level`, `task rust:lint`.
6. Ergebnis mit `plan_section` eintragen (ignorierte Tests mit Namen und Grund), Status `fertig`, ein Commit mit Domäne im Titel.

## Fertig, wenn

- [ ] AC-02 (Teil backdrop, lines, planned, biome): Tests aus `backdrop_test.go`, `planned_test.go`, `biome_portals_test.go`, `cycle_test.go` portiert; alle bis auf die mit `Generate` grün (`cargo test -p k3c-level`).
- [ ] `Rules::from_data` lädt die Daten fehlerfrei, `rules()` liefert im Test denselben Satz für alle Biome.
- [ ] `task rust:lint` grün, Dateien ≤ 400 Zeilen, Funktionen ≤ 60 Zeilen, kein `HashMap`.

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

AC-02 (Teil backdrop, lines, planned, biome) geprüft: `cargo test -p k3c-level` 11 grün, 4 ignoriert; `task rust:lint` grün (mit RU42-Lints).

Rust-Code ohne Tests: rund 480 Zeilen (backdrop, biome, error, level, lines, pending, planned, rules).

Ignoriert: `level_backdrops` (wartet auf RU6.5, braucht generate); `validate_plan_gueltig`, `validate_plan_lehnt_ab`, `validate_plan_mine_mit_freiem_platz` (wartet auf RU6.3, erreichen die Platzhalter in `pending.rs`).

Abweichungen: `level.rs` entsteht schon hier (nur `is_event`, `hub_chunks_of`, `check_count`, `HARBOR_KIND`, `EVENT_KINDS`), weil die Plan-Prüfung sie braucht. Neu `error.rs` (`Fehler`). `portal_werte_sind_pflicht` prüft `checkPortals` über den Lader `load_biome` mit verändertem JSON, da die Prüfung in `k3c-data` privat ist. `testmodule` per `#[path]`. Platzhalter tragen `allow(clippy::unimplemented)` (RU42, § 11). `check_totals` bekommt zusätzlich `&Rules`.
