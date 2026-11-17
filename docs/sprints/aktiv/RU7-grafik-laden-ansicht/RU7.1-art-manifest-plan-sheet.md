# RU7.1 · art Teil 1: manifest, plan, budget, sheet

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru7
- **Abhängig von:** –
- **Tickets:** B-1014
- **Kriterien:** AC-01, AC-02

## Ziel

Manifest, Level-Plan, Texturbudget und Sheet aus `native/k3c-game/art` stehen in Rust; der RU2-Lader nutzt das portierte Manifest.

## Kontext

Quellen: `native/k3c-game/art/manifest.go` (93), `plan.go` (85), `budget.go` (30), `sheet.go` (65); Testdaten `native/k3c-game/art/testdata/grafik` (Rust-Tests lesen sie direkt, Pfad ab `CARGO_MANIFEST_DIR` nach `../../../native/k3c-game/art/testdata`). Bestehend aus RU2: `rust/crates/k3c-game/src/art/{mod,manifest,paper,tests}.rs` (vereinfachter Lader, die Demo nutzt `Library::open/assets/load`). Vorgaben: RU7-README › Vorentscheidungen (`SheetFrame { tex, src }`, `Sheet::draw(canvas, i, Affine)` → `sprite_tf` aus RU4.2). Leitfaden § 1–3, § 8.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/art/**`
- `rust/crates/k3c-game/src/demo/**`
- `rust/crates/k3c-game/tests/fixtures/**`

## Nicht-Ziele

`library.go` (RU7.2). Änderungen an `k3c-platform`.

## Schritte

1. Die vier Go-Dateien ganz lesen, dazu die Stellen in `library.go`, die sie aufrufen.
2. `art/manifest.rs` durch den Port von `manifest.go` ersetzen (`Anim`, `Manifest`, `read_manifest`, `anim_names`, `resolve`, `group_assets`); den RU2-Lader in `art/mod.rs` darauf umstellen, ohne seine öffentliche API zu ändern.
3. `art/plan.rs` (`SheetSpec`, `Plan`, `plan_level`, `plan_sheet`), `art/budget.rs` (`Budget`), `art/sheet.rs` (`Sheet`, `SheetFrame`, `frame`, `frame_at`, `draw`, `upload(spec, pix, canvas)`).
4. Tests: Teile aus `art_test.go`, die nur diese Dateien prüfen, in `manifest_tests.rs`/`plan_tests.rs`; die RU2-Tests in `art/tests.rs` bleiben grün.

## Fertig, wenn

- [x] AC-01: `cargo test -p k3c-game art` grün, Manifest und Plan gegen `native/k3c-game/art/testdata/grafik`.
- [x] AC-02: `Budget` rechnet für einen Plan dieselben Bytes wie Go (Test mit festen Zahlen aus der Testgrafik).
- [x] Demo `--headless --steps 600` Code 0; `task rust:lint` grün.

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

- AC-01 geprüft: `cargo test -p k3c-game art` grün (16 Tests); `manifest_tests.rs` (TestGroupAssets + Manifest-Lesen) und `plan_tests.rs` (TestPlanLevel, …Budget, …Missing, …MissingSheet, …BadSize, TestSheetFrameIndex, dazu `upload`) gegen `native/k3c-game/art/testdata/grafik`. RU2-Lader (`Library::open/assets/load`) läuft auf dem portierten Manifest, `art/tests.rs` grün.
- AC-02 geprüft: `plan_level_budget` – 2 Bilder, (12×6+16×6)×4 Byte, Text „2 Bilder, 0.0 MiB RGBA“, `mib()` = 5 wie Go.
- Demo `--headless --steps 600` Code 0 (Grafik aus dem Hauptcheckout, `--grafik`); `task check:diff` grün (fmt, clippy -D warnings, cargo test k3c-game).
- **Abweichung (korrigiert):** Der Commit `6950a6c60` nutzte für `Sheet::draw` einen Platzhalter (`art/pending.rs::sprite_tf` mit `unimplemented!`) und einen eigenen `Affine`, weil RU4.2 als fehlend galt. Das war falsch: RU4 war mit `k3c_platform::Affine` und `Canvas::sprite_tf` schon gemergt (#667). Die Korrektur steht im Folge-Commit `067c62f43`: `Sheet::draw` ruft `canvas.sprite_tf`, `pending.rs` enthält nur noch `load_rigs` (bis RU7.6 ein `Missing`-Fehler statt `unimplemented!`, wie Go bei fehlenden Rigs).
- `errors.Join` → `ArtError::Joined`, `errors.Is(ErrMissing)` → `ArtError::is_missing()`.
- Review (Sonnet): keine schweren Befunde. Offen, bewusst wie Go belassen: Überlauf bei absurd großen Manifest-Werten (`plan.rs` `frames*fw`, `Budget::add`) paniken in Debug statt zu wrappen; OS-Fehlertexte weichen im Wortlaut vom Go-`open …` ab. Zweites Review (Standards und Spec) fand zusätzlich den Doppel-Free in `LevelLoad::step` (Rust und Go, behoben in `c2a32d0e2` und `067c62f43`, je mit Test).
