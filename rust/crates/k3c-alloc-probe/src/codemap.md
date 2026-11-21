# rust/crates/k3c-alloc-probe/src/

## Responsibility

Ein zählender Allocator, der an `System` weiterreicht und je Thread Allokationen und Bytes mitschreibt.

## Design

- `lib.rs`: `CountingAlloc` (`unsafe impl GlobalAlloc`; `alloc`, `alloc_zeroed` und `realloc` zählen je eine Allokation, `dealloc` nicht). Die Zähler `ALLOCS` und `BYTES` sind `thread_local!` mit `const`-Init ohne Destruktor und werden per `try_with` gelesen und erhöht, damit der Zugriff selbst nie allokiert und beim Thread-Abbau nichts bricht.
- `Counts { allocs, bytes }`; `now()` liest den Stand des aufrufenden Threads; `measure(f)` bildet die Differenz vor und nach `f`.
- Jeder `unsafe`-Block trägt einen `// SAFETY:`-Kommentar.

## Flow

`measure(f)` → `now()` vorher → `f()` → `now()` nachher → `(Ergebnis, Counts-Differenz)`.

## Integration

- Konsument: `k3c-game` (Tests mit `#[global_allocator]`). Ohne gesetzten globalen Allocator bleiben die Zähler auf 0.
