# RS1.5 · rng und level in Rust gegen die Golden-Dateien

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
