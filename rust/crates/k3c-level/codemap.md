# rust/crates/k3c-level/

## Responsibility

Level-Generator, Port von `engine/level`: plant und erzeugt deterministisch aus Biom und Seed ein Level (Chunks, Objekte, Burg, Portale, Ruinen, Wachen, Natur) und prüft es. Die Reihenfolge der Zufallswürfe ist Teil des Ergebnisses und bleibt wie in Go.

## Design

- `Cargo.toml`: `k3c-rng`, `k3c-data`, `k3c-det`, `serde`, `serde_json`.
- Quellcode in `src/` (siehe [`src/codemap.md`](src/codemap.md)); Tests liegen als `*_tests.rs` neben dem Code, dazu ein Vergleich mit Go (`vergleich_go_tests.rs`, `golden_tests.rs`).

## Flow

`Rules` aufbauen → `generate`/`generate_with(biome, seed, options, rules)` → `Layout` → `validate`.

## Integration

- Abhängigkeiten: `k3c-rng`, `k3c-data`, `k3c-det`.
- Konsumenten: `k3c-sim`, `k3c-room` (Vorschau-Welten), `k3c-game`.
