# rust/crates/k3c-rng/

## Responsibility

Deterministischer Zufallsgenerator (mulberry32), Port von `engine/rng`, ohne fremde Crates. Level und Simulation dürfen nur diesen Generator verwenden.

## Design

- `Cargo.toml`: keine Abhängigkeiten.
- Quellcode in `src/` (siehe [`src/codemap.md`](src/codemap.md)).

## Flow

`Rng::new(seed)` leitet den Startwert per `hash_seed` aus einem Text ab; `next`, `int`, `weighted` ziehen daraus; `pick` und `shuffle` arbeiten auf einem `Rng`.

## Integration

- Konsumenten: `k3c-level`, `k3c-sim`, `k3c-room`, `k3c-game`.
