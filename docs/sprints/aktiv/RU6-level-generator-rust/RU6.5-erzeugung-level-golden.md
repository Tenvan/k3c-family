# RU6.5 · Level Teil 5: level (Erzeugung) und Level-Golden bitgleich

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru6
- **Abhängig von:** RU6.4
- **Tickets:** B-1014
- **Kriterien:** AC-01, AC-02

## Ziel

`generate`/`generate_with` laufen in Rust; die sechs Level-Golden und `nature.json` zeigen das gleiche Ergebnis wie Go (Entscheidung 012): gleiche Chunks und Objekte (Art, Anzahl, Reihenfolge), Fließkomma innerhalb 1e-6 relativ (absolut 1e-9 nahe 0); Hinweise „bitgleich“ im Kontext lesen sich als diese Regel. Alle in RU6.1 bis RU6.4 ignorierten Tests laufen.

## Kontext

Leitfaden `docs/standards/rust-port.md` zuerst (§ 5 Determinismus, § 6 Tests); Gerüst und Vorentscheidungen aus RU6.1 bis RU6.4 (`level.rs` enthält bisher nur `Gen`, `center`, `free`, `HarborKind`).

Quelle: Rest von `engine/level/level.go` (364 Zeilen; Typen `Chunk`, `Entity`, `Layout` stehen seit RU8 in `types.rs`): `eventKinds`/`isEvent`, `Generate`, `GenerateWith`, `hubChunksOf`, `checkCount`, `frame`, `placePortals`, `eligible`, `lairX`, `placeEvents`, `farFromLair`, `safeCampSlots`, `entities`, `veins`, `inside`, `weights`. Ziel: `rust/crates/k3c-level/src/level.rs` (erweitern; wird sie länger als 400 Zeilen, an dem Abschnittskommentar vor `entities` teilen: `level_entities.rs`).

**Reihenfolge in `generate_with`** (Go Z. 107 bis 152) ist Teil des Ergebnisses und bleibt: `check_count` → `validate_plan_with` (Fehlertext `geplante Chunks: …`) → `resolve_nature` (Fehlertext `natur: …`) → `Gen` bauen (`Rng::new(&format!("{}:{}", b.id, seed))`, `b.with_nature(nature)`) → `has_lair` → `frame` → `reserve` → `place_portals` → `place_events` → Gewichte und `Rng::weighted` je freiem Chunk → `entities` → optional `lair`-Entität → **stabiles** Sortieren nach `x` (`sort.SliceStable`, Z. 141; in Rust `sort_by(|a, b| a.x.total_cmp(&b.x))`, stabil) → `assign_variants` → `place_sites` → `Layout` mit `free_ground`, `ruins_of`, `guards_of`, `backdrops`. `placeEvents` Z. 249 sortiert ebenfalls mit `sort.SliceStable` (Vergleich dort nachlesen, `sort_by`). `safeCampSlots` liefert in Go eine `map[int]bool`; in Rust `BTreeMap<i64, bool>`, nur Zugriff per Schlüssel. `Rng`-Aufrufe (`next`, `int`, `weighted`, `pick`, `shuffle`) alle an der Stelle und in der Reihenfolge von Go, auch bei verworfenen Ergebnissen. `int(math.Round(…))` in `hubChunksOf` → `(x).round() as i64` (`round` entspricht `math.Round`); `float64(float64(i)*g.cw) + g.cw/2` in `center` behält die Reihenfolge, die `float64(…)`-Hüllen fallen weg. `min(max(…), len(…))` Z. 344 ist ganzzahlig.

**Golden und Tests** (Pfade ab `k3c_det::golden::testdata()`; `golden::read`, `golden::tree`, `golden::diff`, Gleitkomma bitgleich, `null` = `[]` nur dort): `level_test.go` (256, 9 Tests, `TestGolden` je Biom: Datei `testdata/golden/level-<biom>.json` mit `levels[]`, je Level `seed`, Vergleich von `generate(b, seed)` gegen den Baum), `nature_golden_test.go` (51; `testdata/golden/nature.json` Feld `cases`, Fälle je Biom mit den Seeds `"golden"` und `"Käse🧀"` — Umlaut und Emoji verlangen `hash_seed` über UTF-8 wie Go, siehe `k3c-rng`), `planned_golden_test.go` (78, `testdata/golden/level-planned.json`), `camps_test.go`, `chunkcount_test.go` (173), `deco_test.go`, `hub_sites_test.go`, `lava_test.go`, `cycle_test.go` (schon in RU6.1), dazu die in RU6.1 bis RU6.4 mit `wartet auf RU6.5` markierten Tests (`backdrop_tests.rs`, `nature_tests.rs`, `lair_tests.rs`, `planned_gen_tests.rs`, `ruins_tests.rs`, `sites_tests.rs`): Marken entfernen. Die Aktualisier-Zweige der Go-Tests (`*golden.Update`, `-update`) werden nicht portiert; **kein Golden wird geschrieben**. `bench_test.go` enthält nur Benchmarks und wird nicht portiert (Messung macht RU39), Grund steht im Ergebnis.

Abweichungen suchen: Weicht ein Golden ab, gilt Go (Leitfaden § 1). Ersten abweichenden Pfad mit `golden::diff` ausgeben lassen, Ursache beheben (meist Reihenfolge eines Wurfs oder einer Rechenoperation); nie das Golden ändern. Kommt die Session dreimal nicht weiter: anhalten, Status `blockiert`, Stand ins Ergebnis.

## Erlaubte Dateien

- `rust/crates/k3c-level/src/**` (alle Module des Crates, nur Ignore-Marken und `unimplemented!`-Rumpf in `testutil.rs`/`pending.rs` entfernen; neu: `level.rs` erweitern, `level_tests.rs`, `golden_tests.rs`, `camps_tests.rs`, `chunkcount_tests.rs`, `deco_tests.rs`, `hub_sites_tests.rs`, `lava_tests.rs`, `level_nature_tests.rs`)
- `rust/crates/k3c-level/src/lib.rs` (nur die `mod`-Zeilen)

## Nicht-Ziele

Änderungen an Go-Code und `testdata/golden` (kein Neuschrieb, auch nicht mit Begründung). `bench_test.go`. Neue Crates. Bitgleichheit der Fließkommawerte zu Go (entfällt, Entscheidung 012).

## Schritte

1. Leitfaden, alle bisherigen Ergebnisse und `level.go` Z. 103 bis Ende ganz lesen, dazu die Tests dieser Session.
2. `level.rs` fertig portieren, nach jeder Funktion `cargo build -p k3c-level` in `rust/`.
3. `testutil.rs`: `generate(...)`-Rumpf füllen.
4. Tests portieren, danach **alle** Marken `wartet auf RU6.5` im Crate entfernen (`grep -rn "RU6.5" rust/crates/k3c-level`).
5. `cargo test -p k3c-level` (alle sechs Level-Golden und `nature.json` grün), `task rust:lint`.
6. Ergebnis mit `plan_section` eintragen (Tests je Datei, Zeilen je Session für AC-03, Grund für `bench_test.go`), Status `fertig`, ein Commit mit Domäne im Titel.

## Fertig, wenn

- [ ] AC-01: `level-cave`, `-crystal`, `-forest`, `-ironhold`, `-mine`, `-planned` und `nature.json` zeigen gleiches Ergebnis (Chunks und Objekte nach Art, Anzahl, Reihenfolge; Fließkomma innerhalb 1e-6 relativ; `cargo test -p k3c-level golden`), ohne Golden-Neuschrieb (`git diff --stat testdata/golden` leer).
- [ ] AC-02 (Teil level und Rest): Tests aus `level_test.go`, `camps_test.go`, `chunkcount_test.go`, `deco_test.go`, `hub_sites_test.go`, `lava_test.go`, `nature_golden_test.go`, `planned_golden_test.go` portiert und grün; keine Marke `wartet auf RU6.5` mehr.
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

Level Teil 5 in Rust: `level_gen.rs` (`generate`, `generate_with`, `frame`, `place_portals`, `place_events`, Camp-Sortierung, sichere Camp-Plätze) und `level_entities.rs` (Entitäten je Chunk, Adern) in `rust/crates/k3c-level`. Reihenfolge der rng-Aufrufe wie in Go (Prüfung der Anzahl, Planung, Natur, Rahmen, Reservierung, Portale, Ereignisse, Gewichte, Entitäten, Bau, stabile Sortierung nach x, Varianten, Plätze).

- Tests: alle Level-Tests von Go portiert (`camps`, `chunkcount`, `deco`, `hub_sites`, `lava`, `level`, `golden`, `nature`-Golden); die `ignore`-Marken aus RU6.1 bis RU6.4 sind entfernt, die zurückgestellten Generate-Tests laufen. `cargo test -p k3c-level`: 87 grün, Debug und Release.
- Golden (alle Biome, `level-planned.json`, `nature.json`) stimmt bitgleich, ohne Toleranz und ohne Änderung der Golden-Dateien.
- Clippy mit `-D warnings` grün, Dateien ≤ 400 Zeilen, Funktionen ≤ 60 Zeilen.
- Abweichung: `generate_with` bekommt den Regelsatz als `Arc<Rules>` übergeben (Go: Paketvariablen).
- Marker: `STAB(B-LVL)` in `level_entities.rs` (Fehler statt panic bei leerer Liste), `PERF(B-LVL)` in `nature.rs` (go_float-Text je Entität, einmal je Level). Ticket im Sprint RU43 folgt in RU6.6.
- Mehr Sims und Gen: keine gefahren.
