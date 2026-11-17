# RU7.2 · art Teil 2: library, font, outline; Demo auf die Go-API

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru7
- **Abhängig von:** RU7.1
- **Tickets:** B-1014
- **Kriterien:** AC-01, AC-02

## Ziel

`art::Library` ist ein Port von `library.go` (Level, schrittweises Laden, Budget, Freigeben, Schrift) samt `font.go` und `outline.go`; die Demo lädt ihre Grafik darüber.

## Kontext

Quellen: `native/k3c-game/art/library.go` (270), `font.go` (108), `outline.go` (133, Test `outline_test.go`), Tests `art_test.go`, `config_test.go`. Vorgaben: RU7-README › Vorentscheidungen (Hochladen mit `&mut dyn Canvas`, Outline ohne Offscreen-Bild, `paper.rs` → `demo/assets.rs`). Plattform aus RU4: `Canvas::font`, `text_font`, `text_width`, `free_texture`, `sprite_tf`. Rig-Bibliothek: `Config.rig_dir` nicht leer → Platzhalter in `art/pending.rs` nach Leitfaden § 7 (`unimplemented!("RU7.6: rigs")`); `DefaultConfig` setzt `rig_dir` erst in RU7.6, bis dahin leer. Demo heute: `rust/crates/k3c-game/src/demo/{mod,world,scene}.rs` nutzt `Library::open/assets/load`.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/art/**`
- `rust/crates/k3c-game/src/demo/**`
- `rust/crates/k3c-game/tests/fixtures/**`

## Nicht-Ziele

Rig-Teil (RU7.6). `scales`, `seamap`, `voyage`, `worldimages` (RU7.3).

## Schritte

1. Go-Dateien und Tests ganz lesen.
2. `art/library.rs`: `Config`, `default_config`, `Library::new`, `set_levels(want, canvas)`, `begin_levels`, `LevelLoad { step(n, canvas), abort, progress }`, `plan_levels`, `diff_sheets`, `budget_of`, `sheet(asset, anim)`, `levels`, `budget`; Budget-Log mit Emoji (📂) wie Go.
3. `art/font.rs` (`Font` auf `Canvas::text_font`, Rückfall Pixel-Schrift mit Warnung, `StrictFont` als Fehler), `art/outline.rs` (direktes Zeichnen an den Versätzen).
4. RU2-Lader aus `art/mod.rs` entfernen; `art/paper.rs` nach `demo/assets.rs` verschieben; Demo auf `Library::new` + `set_levels` (ein Level mit allen Demo-Assets) + `sheet(asset, anim)` umstellen.
5. Tests portieren (`library_tests.rs`, `font_tests.rs`, `outline_tests.rs`); Outline-Cache-Tests, die wegfallen, im Ergebnis nennen.

## Fertig, wenn

- [ ] AC-01: `cargo test -p k3c-game art` grün; Tests aus `art_test.go` und `config_test.go` portiert.
- [ ] AC-02: Test: Levelwechsel gibt nicht mehr gebrauchte Sheets frei (`NullCanvas` zählt `free_texture`), `LevelLoad::step` lädt in Schritten wie Go.
- [ ] AC-01: `cargo run --release -p k3c-game -- demo --driver rust/demo/reiten.json` zeigt dieselbe Szene wie vorher (Screenshot durch Claude); `--headless --steps 600` Code 0.
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
