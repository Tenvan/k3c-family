# rust/crates/k3c-balance/src/

## Responsibility

Einstiegspunkt des Gerüst-Crates für den Balancing-Port.

## Design

- `lib.rs` (7 Zeilen): Doc-Kommentar „Balancing und Testläufe: Port von engine/balance“ und ein `#[cfg(test)]`-Test `crate_baut` ohne Prüfung.

## Flow

Keiner; nichts wird exportiert.

## Integration

- Abhängigkeiten laut `Cargo.toml`: `k3c-sim`, `k3c-data`.
- Konsument: `k3c-game`.
