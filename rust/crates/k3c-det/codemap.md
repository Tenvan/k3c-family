# rust/crates/k3c-det/

## Responsibility

Determinismus-Hilfen: Gleitkomma-Funktionen bitgleich zu Go auf amd64 und der Golden-Vergleich. Die Rust-Bordmittel `powf`, `exp`, `ln`, `f64::max` und `f64::min` sind in den Sim-Crates verboten (Doc in `src/lib.rs`), stattdessen gelten die Funktionen hier.

## Design

- `Cargo.toml`: `flate2` (fest `=1.1.11`), `serde`, `serde_json` (mit `float_roundtrip`).
- Quellcode in `src/` (siehe [`src/codemap.md`](src/codemap.md)).

## Flow

Sim, Level und Daten rufen `k3c_det::exp/log/pow/fmax/fmin` statt der Standardbibliothek; Tests lesen Golden-Dateien und vergleichen sie als JSON-Baum.

## Integration

- Abhängigkeiten: `flate2`, `serde`, `serde_json`; keine internen Crates.
- Konsumenten: `k3c-data`, `k3c-level`, `k3c-sim`, `k3c-game`.
