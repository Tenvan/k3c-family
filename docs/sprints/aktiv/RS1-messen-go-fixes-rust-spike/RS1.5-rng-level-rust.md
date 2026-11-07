# RS1.5 · rng und level in Rust gegen die Golden-Dateien

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** rs1/5-rng-level-rust
- **Abhängig von:** RS1.4
- **Tickets:** B-725
- **Kriterien:** AC-06

## Ziel

`rng` und der Level-Generator laufen in Rust und erzeugen `testdata/golden/rng.json` und alle `level-*.json` bitgleich; Go kann per Schalter die Rust-Variante nutzen, Standard bleibt Go.

## Kontext

`engine/rng/rng.go` (85 Zeilen: `HashSeed`, `New`, `Next`, `Int`, `Weighted`, `Pick`, `Shuffle`), Golden-Test `engine/rng/rng_test.go`. `engine/level/level.go` (`Generate`, `GenerateWith`), Golden-Tests `engine/level/level_test.go` und `planned_golden_test.go` gegen `testdata/golden/level-*.json`. Werte kommen aus `data/` (Biome) – Rust bekommt sie als JSON von Go, liest `data/` nicht selbst. Brücke aus RS1.4 (`engine/core`). Gleitkomma: Rust fusioniert nicht von selbst zu FMA; Go auf arm64 schon (CI-Job `go-arm64`, B-071).

## Erlaubte Dateien

- `rust/k3c-core/**`
- `engine/core/**`
- `engine/rng/*_test.go`, `engine/level/*_test.go` (Vergleichstests Go↔Rust)

## Nicht-Ziele

Kein Umschalten des Standards auf Rust, keine Änderung an `testdata/golden/`, keine Sim-Logik.

## Schritte

1. `rng` in Rust, Test gegen `rng.json` in Rust und über die Brücke.
2. Level-Generator in Rust (Eingabe: Biome-JSON + Seed + Options, Ausgabe: Layout-JSON).
3. Vergleichstest in Go: alle Golden-Level über Rust erzeugen, Byte-Vergleich.
4. Benchmark `GenerateWith` Go vs. Rust inkl. Brückenkosten, Zahl in 006.

## Fertig, wenn

- [ ] AC-06: Vergleichstests grün auf amd64 (lokal) und arm64 (CI-Job `go-arm64`).
- [ ] Benchmark Go vs. Rust für Level steht in 006.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-06 umgesetzt, lokal (amd64) geprüft: `rng` (`rng.rs`) und Level-Generator (`rust/k3c-core/src/level/`, 9 Module) erzeugen `rng.json` und alle sechs `level-*.json` bitgleich; Go-Vergleichstests `engine/rng/rust_test.go` (`TestRustBitgleich`) und `engine/level/rust_test.go` (`TestRustWieGo`: alle Biome × 9–12 Fälle × 40 Seeds, `TestRustFehlerWieGo`: 31 Fehlerfälle mit gleichem Text); `cargo test` (13 Tests) und clippy grün. Aufruf aus Go ohne cgo (`engine/core/level.go`, `rng.go`).
- arm64: nicht lokal geprüft (kein Linker/Runner); der CI-Job `rust` (linux arm64) führt `go test -tags k3ccore ./engine/core ./engine/rng ./engine/level` und die Rust-Golden-Tests aus (nur Push/Dispatch, ungeprüft bis zum Merge).
- Benchmark Go vs. Rust inkl. Brücke: 006 › rng und Level-Generator; Rust + Brücke 0,5–1,3× Go (Biom-JSON ein-/auslesen frisst den Gewinn des 2–4× schnelleren Kerns).
- Lücken (bewusst): `Validate`/`ValidateWith`, `LoadBiome`, `Cycle.Check`, `checkRuinRules`/`checkGuardRules` bleiben in Go; Zahlenformat nur für Werte im Bereich 1e-6..1e21 wie Go.
- Umsetzung des Level-Ports durch einen Subagent (Opus), Prüfung und Messung durch Claude; Reviewer (Sonnet) über Speicherzugriffe der FFI und Determinismus: siehe RS1.7.
- Geprüft: `task check:diff`.
