# rust/crates/k3c-balance/

## Responsibility

Vorgesehener Port von `engine/balance` (Balancing und Testläufe). Im aktuellen Stand ist das Crate ein Gerüst: `lib.rs` enthält nur den Doc-Kommentar und einen Test `crate_baut`.

## Design

- `Cargo.toml`: Abhängigkeiten `k3c-sim` und `k3c-data`, Workspace-Lints.
- Quellcode in `src/` (siehe [`src/codemap.md`](src/codemap.md)).

## Flow

Kein Datenfluss; es gibt keine öffentlichen Typen oder Funktionen.

## Integration

- Abhängigkeiten: `k3c-sim`, `k3c-data` (im Quellcode noch nicht benutzt).
- Konsument: `k3c-game` (Abhängigkeit).
