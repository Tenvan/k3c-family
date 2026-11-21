# rust/crates/k3c-det/src/

## Responsibility

Port der Go-`math`-Funktionen mit amd64-Verhalten (FMA-Pfad) und der lesenden Teile von `engine/internal/golden`.

## Design

- `lib.rs`: exportiert `fmax`, `fmin`, `exp`, `log`, `pow` und das Modul `golden`.
- `bits.rs`: `nan`, `inf`, `frexp`, `ldexp`, `modf` wie Gos `math` (crate-intern).
- `dim.rs`: `fmax`/`fmin` mit den Go-Regeln für ±Inf, NaN und ±0.
- `exp.rs`: `exp` wie `exp_amd64.s` (FMA); `log.rs`: `log` wie `log_amd64.s`; `pow.rs`: `pow` wie `pow.go` mit den Sonderfällen in Go-Reihenfolge.
- `golden.rs`: `testdata`, `read` (auch gzip über `flate2`), `tree`, `diff`, `diff_tol` mit `Tol` und `SIM_TOL`, `go_float` (Go-Textform eines `f64`). Golden schreibt nur Go, hier gibt es kein `-update`.
- `golden_tests.rs`, `vectors_tests.rs`: Tests (Vektoren aus Go).

## Flow

`pow` → `exp`/`log`/`bits`; Golden: Datei lesen → `serde_json::Value` → `diff`/`diff_tol` liefert den Unterschied als Text.

## Integration

- Konsumenten: `k3c-data`, `k3c-level` (u. a. `go_float`), `k3c-sim`, `k3c-game`.
- Abhängigkeiten: `flate2`, `serde`, `serde_json`.
