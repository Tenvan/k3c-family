# RU7.1 · art Teil 1: manifest, plan, budget, sheet

- **Status:** offen
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

- [ ] AC-01: `cargo test -p k3c-game art` grün, Manifest und Plan gegen `native/k3c-game/art/testdata/grafik`.
- [ ] AC-02: `Budget` rechnet für einen Plan dieselben Bytes wie Go (Test mit festen Zahlen aus der Testgrafik).
- [ ] Demo `--headless --steps 600` Code 0; `task rust:lint` grün.

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
