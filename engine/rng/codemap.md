# engine/rng/

## Responsibility

Utility-Layer: deterministischer Pseudozufallsgenerator (mulberry32), bitgleich zum TS-Original. Einzige erlaubte Zufallsquelle für Level und Simulation.

## Design

- `Rng` (`rng.go`): Zustand eines Generators je Seed; `HashSeed` (FNV-1a über UTF-16-Codeeinheiten wie `charCodeAt`), `New(seed string)`.
- Methoden: `Next` ([0,1)), `Int(lo, hi)` inklusive, `Weighted([]Weight)` (geordnete Liste statt map, weil TS in Einfügereihenfolge zieht).
- Generische Helfer `Pick[T]` und `Shuffle[T]` (Fisher-Yates von hinten wie TS).
- Verifiziert gegen `testdata/golden/rng.json`; die Reihenfolge der Aufrufe ist Teil des Vertrags.

## Flow

1. Aufrufer: `r := rng.New(seed)`.
2. `r.Next()` verändert den 32-Bit-Zustand und liefert die nächste Zahl; `Int`, `Weighted`, `Pick`, `Shuffle` bauen darauf auf.
3. Gleicher Seed → gleiche Folge, unabhängig von Plattform.

## Integration

- Keine Abhängigkeiten außer Stdlib.
- Konsumenten: `engine/level` (Generator), `engine/sim` (`planWave`, Spawns), `cmd/k3c-load`.
- Regel: kein `math/rand`, kein `Math.random()` im Client für Spiel-Logik.
