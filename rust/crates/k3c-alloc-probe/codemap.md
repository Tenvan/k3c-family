# rust/crates/k3c-alloc-probe/

## Responsibility

Zählt Allokationen je Thread (RU42, B-1075), nur für Tests. Das Crate hat keine externen Abhängigkeiten und erbt die Workspace-Lints nicht, weil `unsafe` an der einen `GlobalAlloc`-Implementierung erlaubt ist.

## Design

- `Cargo.toml`: keine Abhängigkeiten; Quellcode in `src/` (siehe [`src/codemap.md`](src/codemap.md)).
- `src/lib.rs`: `CountingAlloc`, `Counts`, `now`, `measure`.

## Flow

Ein Test-Binary setzt `CountingAlloc` mit `#[global_allocator]`; `measure(f)` führt `f` aus und liefert das Ergebnis samt der Allokationen des aufrufenden Threads.

## Integration

- Abhängigkeiten: nur `std`.
- Konsument: `k3c-game` (als Abhängigkeit und Dev-Abhängigkeit, für Allokationstests).
