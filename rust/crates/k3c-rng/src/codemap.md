# rust/crates/k3c-rng/src/

## Responsibility

Der Generator selbst und seine kleinen Hilfen.

## Design

- `lib.rs`: `hash_seed(&str) -> u32`, `Rng` (`new`, `next` als `f64`, `int(lo, hi)`, `weighted(&[Weight]) -> String` mit `Weight { key, w }`), `pick` (Panik bei leerer Liste, wie TS) und `shuffle` (Fisher-Yates von hinten). Die Gewichtung rechnet ohne fusionierte Multiplikation (Go rundet gegen FMA, B-071).
- `lib_tests.rs`: Tests.

## Flow

Seed-Text → `hash_seed` → Zustand → `next` → `int`/`weighted`/`pick`/`shuffle`.

## Integration

- Konsumenten: `k3c-level`, `k3c-sim`, `k3c-room`, `k3c-game`.
