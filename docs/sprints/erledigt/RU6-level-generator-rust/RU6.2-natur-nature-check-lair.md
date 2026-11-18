# RU6.2 · Level Teil 2: nature, nature_check, lair

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru6
- **Abhängig von:** RU6.1
- **Tickets:** B-1014
- **Kriterien:** AC-02

## Ziel

Natur-Werte (`ResolveNature`, `MergeNature`, `NeutralNature`, `assignVariants`), deren Prüfung (`CheckNatureValues`) und die Drachenhöhlen-Prüfung (`CheckLairFit`) rechnen in Rust wie in Go.

## Kontext

Leitfaden `docs/standards/rust-port.md` zuerst; Vorentscheidungen und Rust-Gerüst aus RU6.1 (`rules.rs`, `testutil.rs`, `planned.rs`) und `rust/crates/k3c-level/src/types.rs` (`NatureValues`, `Trait`, `WeightGroup`).

Quellen in `engine/level/`: `nature.go` (218, Test `nature_test.go`, 12 Tests; `GenerateWith` in 2 Tests, `Validate` in 1), `nature_check.go` (173), `lair.go` (41, fehlt in der Sprint-Tabelle, Test `lair_test.go`: 3 Tests, 4 Aufrufe `CheckLairFit`, 3 mit `Generate`/`GenerateWith`/`ValidateWith`). Ziel: `rust/crates/k3c-level/src/nature.rs`, `nature_tests.rs`, `nature_check.rs`, `lair.rs`, `lair_tests.rs`.

`nature_check.go` ist zu großen Teilen schon in `k3c_data::biome` (RU3: `check_nature`, `check_trait`, `check_targets`, `check_weights`, `check_options`, `has_resource`, alle private an `Biome`). Neu zu schreiben ist nur, was Go öffentlich macht und `k3c-data` nicht hat: `CheckNatureValues(v, strict)` und `checkGroupValues` (Zeilen 131 bis Ende) sowie `prefixed`. Die private `check_*`-Methoden in `k3c-data` dürfen **nicht** verändert werden (anderes Crate); braucht `nature_check.rs` eine davon, wird sie aus dem Go-Text in `nature_check.rs` als freie Funktion nachgebaut und im Ergebnis genannt (kein Eingriff in `k3c-data`).

`nature.go` Besonderheiten (selbst nachlesen): `MergeNature` und `CheckNatureValues` laufen in Go über `slices.Sorted(maps.Keys(…))`, also in Schlüsselreihenfolge: in Rust `BTreeMap` iterieren. `ResolveNature(b, seed, fixed)` würfelt mit `Rng::new(…)` aus dem Seed: Reihenfolge der `next`/`int`/`weighted`-Aufrufe unverändert lassen (Leitfaden § 5), auch wenn ein Wurf verworfen wird. `scaleCount(n, v, def)` ist ganzzahlig (`(2*n*v + def) / (2*def)`, Rundung zur Null wie in Go). `Biome.withNature(v)` liefert eine veränderte Kopie (`Biome: Clone`), `assignVariants` würfelt nicht (vor dem Portieren prüfen: Wenn doch, ist die Reihenfolge Teil des Ergebnisses). `Entity.Variant` setzt `assignVariants` in Reihenfolge der Entitäten.

## Erlaubte Dateien

- `rust/crates/k3c-level/src/**` (Module `nature`, `nature_check`, `lair` samt `*_tests.rs`; `testutil.rs` nur ergänzen)
- `rust/crates/k3c-level/src/lib.rs` (nur die `mod`-Zeilen)

## Nicht-Ziele

Änderungen an Go-Code und an `k3c-data`. `generate`/`validate` (RU6.4, RU6.5). `nature_golden_test.go` (läuft erst mit `generate`, RU6.5). Neue Crates.

## Schritte

1. Leitfaden, RU6.1-Ergebnis und die drei Go-Dateien samt `nature_test.go`, `lair_test.go` ganz lesen; `k3c_data::biome` nach vorhandenen Prüfungen durchsehen.
2. `nature.rs`, `nature_check.rs`, `lair.rs` portieren, nach jeder Datei `cargo build -p k3c-level` in `rust/`.
3. Tests portieren (`nature_tests.rs`, `lair_tests.rs`); Tests mit `GenerateWith`/`Generate`/`Validate*` bekommen `#[ignore = "wartet auf RU6.5"]`, alle anderen laufen.
4. `cargo test -p k3c-level nature`, `cargo test -p k3c-level lair`, `task rust:lint`.
5. Ergebnis mit `plan_section` eintragen (ignorierte Tests mit Namen), Status `fertig`, ein Commit mit Domäne im Titel.

## Fertig, wenn

- [ ] AC-02 (Teil nature, nature_check, lair): Tests aus `nature_test.go` und `lair_test.go` portiert, alle ohne `generate` grün (`cargo test -p k3c-level nature lair`).
- [ ] Jeder Aufruf von `Rng` steht in derselben Reihenfolge wie in Go (im Ergebnis als geprüft genannt).
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

Port von `nature`, `nature_check` (`check_nature_values`) und `lair` (`check_lair_fit`); `cargo test -p k3c-level`: 14 grün, 16 ignoriert; `task rust:lint` grün. Rust-Code ohne Tests: 313 Zeilen (nature 198, nature_check 63, lair 52).

Ignoriert (wartete auf RU6.5, brauchen `generate`; seit RU6.5 laufen sie): natur_gleicher_seed_gleiche_natur, natur_seeds_unterscheiden_sich, natur_walddichte_skaliert_baeume, natur_fester_wert_gilt, natur_monokultur, natur_feste_werte_abgelehnt, natur_neutraler_wert_aendert_nichts, natur_chunk_gewichte, natur_adern, natur_kristallfarben, bau_in_jedem_biom_500_seeds, bau_standard_ist_biom_feld.

Abweichungen: `nature_check.rs` baut `prefixed` und die Biom-Datenprüfung nicht nach (liegen in `k3c-data` und laufen beim Laden); `natur_datenpruefung` prüft sie über `load_biome` mit JSON. Gos Methoden an `Biome` (`withNature`, `traitOf`) sind freie Funktionen (`with_nature`, `trait_of`), weil `Biome` in `k3c-data` liegt. `LAIR_CAMP_GAP_UNITS` steht schon in `level.rs`. `assign_variants`: `go_float` allokiert je Entität (Markierung PERF(B-1079), einmal je Level).
