# RU6.3 · Level Teil 3: planned_gen, planned_totals, guards, ruins

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru6
- **Abhängig von:** RU6.2
- **Tickets:** B-1014
- **Kriterien:** AC-02

## Ziel

Der Zustand der Erzeugung (`Gen`, `Options`) steht, dazu geplante Chunks (`planned_gen`, `planned_totals`), Wächter (`guards`) und Ruinen (`ruins`) rechnen in Rust wie in Go.

## Kontext

Leitfaden `docs/standards/rust-port.md` zuerst; Gerüst aus RU6.1/RU6.2 (`rules.rs`, `planned.rs` mit `Geometry`/`Placed`, `nature.rs`).

Quellen in `engine/level/`: `planned_gen.go` (135, Test `planned_gen_test.go`, 6 Tests, 5 mit `Generate*`), `planned_totals.go` (88), `guards.go` (96, Test `guards_test.go`: 6 Tests, nur `ValidatePlan`, laufen also), `ruins.go` (158, Test `ruins_test.go`, 11 Tests, 4 mit `GenerateWith`). Ziel: `rust/crates/k3c-level/src/planned_gen.rs`, `planned_gen_tests.rs`, `planned_totals.rs`, `guards.rs`, `guards_tests.rs`, `ruins.rs`, `ruins_tests.rs` und **`level.rs`** als Rumpf.

**`level.rs` Rumpf:** Go hält `type gen struct` samt `center` und `free` in `level.go` (Zeilen 69 bis 101). Diese Session legt `level.rs` mit genau diesem Teil an (`pub(crate) struct Gen` mit allen Feldern: `b`, `r` als `Rng`, `seed`, `kinds: Vec<String>` ("" = frei), `cw`, `hub`, `count`, `sites`, `hub_start`, `hub_chunks`, `planned: BTreeMap<i64, PlannedChunk>`, `fixed: BTreeMap<String, i64>`, `side_fixed: [i64; 2]`, `planned_res`, dazu `rules: Arc<Rules>`), die Methoden `center`, `free`, und `HarborKind`; die Datei bekommt in RU6.5 die übrigen Funktionen. Die Methoden von `planned_gen.go`, `guards.go`, `ruins.go`, `sites.go` sind in Go `func (g *gen) …`; in Rust `impl Gen { … }` in der jeweiligen Datei (mehrere `impl`-Blöcke sind erlaubt), Felder `pub(crate)`.

`Options` (Go: `planned_gen.go` Z. 12 bis 36: `ChunkCount`, `Planned`, `HubSites`, `Nature`, `Lair`-Schalter, `hasLair`) lebt in `planned_gen.rs`. `chunkStream(c)` baut einen eigenen `Rng` je Chunk aus dem Chunk-Seed: genau dieselbe Seed-Zeichenkette wie Go (`fmt.Sprintf`-Format ganz abschreiben, Gleitkomma- und Ganzzahl-Formatierung prüfen). `reserve`, `plannedEntities`, `clearOfRuin` (bis zu `clearTries = 32` Versuche, Würfe in fester Reihenfolge) würfeln: Aufrufreihenfolge unverändert. Sortierungen und Schleifen über Maps: `fixed`, `planned` sind in Rust `BTreeMap`; Go iteriert sie nicht (Regel `engine/internal/rules/map_range_test.go`), also nur Zugriff per Schlüssel.

`guards.go`/`ruins.go`: `GuardRules`, `RuinRules` kommen aus `Rules` (`k3c-data`); `RuinHP(hp)` ist öffentlich (wird vom Spiel gerufen) und rechnet in `float64`; `ruins.go` Z. 115 nutzt `math.Max` (→ `k3c_det::fmax`). `ruinsOf` und `guardsOf` würfeln nicht (vor dem Portieren prüfen; würfeln sie doch, gilt Leitfaden § 5). `checkTotals`/`checkRoom`/`eligibleFree` in `planned_totals.go` gehören zu `ValidatePlanWith` aus RU6.1: Falls RU6.1 dafür einen Platzhalter `planned_totals`-Aufruf braucht, steht er in `planned.rs` als `pub(crate)`-Aufruf von `planned_totals::check_totals`; diese Session legt die Funktion dort ab und entfernt den Platzhalter.

## Erlaubte Dateien

- `rust/crates/k3c-level/src/**` (Module `planned_gen`, `planned_totals`, `guards`, `ruins`, `level` (nur Rumpf) samt `*_tests.rs`; `planned.rs`, `testutil.rs` nur ergänzen)
- `rust/crates/k3c-level/src/lib.rs` (nur die `mod`-Zeilen)

## Nicht-Ziele

Änderungen an Go-Code. `Generate*`, `placePortals`, `placeEvents`, `entities`, `veins`, `frame` (RU6.5), `sites` und `validate` (RU6.4). Neue Crates.

## Schritte

1. Leitfaden, RU6.1/RU6.2-Ergebnis, die vier Go-Dateien samt Tests und `level.go` Z. 61 bis 101 lesen.
2. `level.rs` (Rumpf) und `planned_gen.rs` (mit `Options`) anlegen, dann `planned_totals.rs`, `guards.rs`, `ruins.rs`; die Platzhalter aus `pending.rs` (RU6.1, Vorentscheidung 8: `check_totals`, `check_guards`, `check_buildings`, `check_ruins_across`) durch die echten Funktionen ersetzen und löschen; nach jeder Datei `cargo build -p k3c-level` in `rust/`.
3. Tests portieren (`planned_gen_tests.rs`, `guards_tests.rs`, `ruins_tests.rs`); Tests mit `Generate*` bekommen `#[ignore = "wartet auf RU6.5"]`; die mit `wartet auf RU6.3` markierten Tests aus RU6.1 (`planned_tests.rs`) laufen jetzt, Marken entfernen.
4. `cargo test -p k3c-level`, `task rust:lint`.
5. Ergebnis mit `plan_section` eintragen (ignorierte Tests mit Namen), Status `fertig`, ein Commit mit Domäne im Titel.

## Fertig, wenn

- [ ] AC-02 (Teil planned_gen, planned_totals, guards, ruins): Tests aus `planned_gen_test.go`, `guards_test.go`, `ruins_test.go` portiert; die ohne `Generate*` grün (`cargo test -p k3c-level planned guards ruins`); die Tests aus `planned_test.go` laufen ohne Marke `RU6.3`.
- [ ] `pending.rs` enthält keinen Platzhalter mehr mit Marke `RU6.3`.
- [ ] `chunk_stream` erzeugt dieselbe Seed-Zeichenkette wie Go (eigener Test mit festem Beispiel, Wert aus der Go-Quelle abgeleitet).
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
