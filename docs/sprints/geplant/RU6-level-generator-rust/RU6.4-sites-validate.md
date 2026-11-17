# RU6.4 · Level Teil 4: sites, validate

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru6
- **Abhängig von:** RU6.3
- **Tickets:** B-1014
- **Kriterien:** AC-02

## Ziel

Hub-Plätze, Baumschulen und Plätze an den Mauerlinien (`sites`) sowie die Prüfung eines fertigen Levels (`validate`) rechnen in Rust wie in Go.

## Kontext

Leitfaden `docs/standards/rust-port.md` zuerst; Gerüst und Vorentscheidungen aus RU6.1 bis RU6.3 (`rules.rs`, `level.rs` mit `Gen`, `planned.rs`, `lines.rs`, `planned_gen.rs` mit `Options`).

Quellen in `engine/level/`: `sites.go` (275, Test `sites_test.go`, 7 Tests, 9 Aufrufe `Generate`, 3 `OnFreeGround`) und `validate.go` (191; eigene Tests gibt es nicht, `Validate*` wird in `level_test.go`, `chunkcount_test.go`, `lair_test.go`, `nature_test.go`, `planned_golden_test.go` geprüft, diese laufen erst in RU6.5). Ziel: `rust/crates/k3c-level/src/sites.rs`, `sites_tests.rs`, `validate.rs`.

`sites.go`: `placeSites`, `placeNurseries`, `freeNear`, `placeZone` würfeln über `g.r` (`Gen.r: Rng`); den Würfel-Aufruf **an derselben Stelle** der Reihenfolge lassen, auch wenn der Platz danach verworfen wird (`zonePlacer.free`/`pick`/`nearWall`/`add`). `taken *[]float64` (Zeiger auf eine Liste, die mehrere Aufrufe füllen) wird `&mut Vec<f64>` und als Parameter durchgereicht; `zonePlacer` ist `pub(crate)`, hält nur Werte und bekommt `taken` je Aufruf (Lebenszeit-Parameter an Typen sind verboten). `blockedXs`, `sideLimit` (`math.Min` → `k3c_det::fmin`), `lavaAt`, `freeGround(sites)` und `Layout.OnFreeGround` (`impl Layout` in `sites.rs`) sind reine Rechnung. `DefaultSitePlacement()` wird `rules.site_placement`.

`validate.go`: `Validate`, `ValidateWith`, `validateLength`, `validateChunks`, `validatePortalsOutsideLines`, `validateLair`, `validateEntities`, `validateVeins` liefern `Vec<String>` mit **denselben Meldungstexten** wie Go (Tests und das Spiel vergleichen sie; `fmt.Sprintf`-Formate Zeichen für Zeichen abschreiben; `%v`/`%g` auf `float64` mit `k3c_det::golden::go_float`, nicht mit `{}`). `max(…)`/`min(…)` auf `int` in Z. 185/186 bleiben ganzzahlig (`i64::max`).

## Erlaubte Dateien

- `rust/crates/k3c-level/src/**` (Module `sites`, `validate` samt `*_tests.rs`; `level.rs`, `lines.rs`, `testutil.rs` nur ergänzen)
- `rust/crates/k3c-level/src/lib.rs` (nur die `mod`-Zeilen)

## Nicht-Ziele

Änderungen an Go-Code. `Generate*` und die übrigen Funktionen von `level.go` (RU6.5). Neue Crates.

## Schritte

1. Leitfaden, bisherige Ergebnisse, `sites.go`, `validate.go`, `sites_test.go` ganz lesen; `level_test.go` nach den `Validate`-Tests durchsehen (diese Meldungstexte sind das Soll).
2. `sites.rs` und `validate.rs` portieren, nach jeder Datei `cargo build -p k3c-level` in `rust/`.
3. `sites_tests.rs` portieren; Tests mit `Generate` bekommen `#[ignore = "wartet auf RU6.5"]`, `OnFreeGround`-Tests auf handgebauten `Layout`-Werten laufen.
4. Bei jedem `fmt`-Format mit Gleitkomma `go_float` (`rust/crates/k3c-det/src/golden.rs`) verwenden und im Ergebnis nennen, wo.
5. `cargo test -p k3c-level`, `task rust:lint`.
6. Ergebnis mit `plan_section` eintragen (ignorierte Tests mit Namen), Status `fertig`, ein Commit mit Domäne im Titel.

## Fertig, wenn

- [ ] AC-02 (Teil sites, validate): Tests aus `sites_test.go` portiert; die ohne `Generate` grün (`cargo test -p k3c-level sites`).
- [ ] Alle Meldungstexte von `validate.go` stimmen Zeichen für Zeichen mit Go überein (im Ergebnis als geprüft genannt).
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
