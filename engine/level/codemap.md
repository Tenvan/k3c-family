# engine/level/

## Responsibility

Domain Service: prozeduraler Level-Generator samt Validator, Go-Port des früheren TS-Generators (`src/world/levelGenerator.ts`, entfernt mit SP09). Erzeugt aus Biom und Seed ein deterministisches `Layout` (Chunks und Entities).

## Design

- `Generate(b Biome, seed string) (Layout, error)` in `level.go`; interner Builder `gen` mit Schritten `frame` → `placePortals` → `placeEvents` (Camps auf Abstand zu den Linien, sonst `swapCampsWithChests`) → `entities` → `veins` (Adern zuletzt, damit bestehende Seeds gleich bleiben).
- Layout: `[Rand/Ausgang] … [Portal] … [Chunks] [HUB] [Chunks] … [Portal] … [Ausgang/Rand]`; Typen `Chunk`, `Entity`, `Layout` (mit `Lines`; JSON-Form wie `LevelLayout` in TS).
- `biome.go`: `Biome`, `Cycle` (dayNight / aggressionPool, nur für `engine/sim`), `Veins` (Anzahl und Art der Adern), `Lava` (Schaden je Sekunde und Breite; Wirkung in `engine/sim/lava.go`), `Range`, `LoadBiome(id)` liest `data/biomes/<id>.json`; `Ordered[V]` bewahrt die JSON-Schlüsselreihenfolge (TS `Object.entries`).
- `lines.go`: `WallLine` (Seite, Index, Mauer-, Turm- und Tor-Position in Units) und `generateLines(biomeID, seed, hub)` aus `data/hub.json › wallLines` (feste Linien plus Versatz nach außen aus eigenem `rng`-Strom `<biom>:<seed>:sites`); `clearOfLines` hält Platzierungen frei von den Linien.
- `validate.go`: `Validate(l, b) []string` (leere Liste = gültig) mit `validateChunks`, `validatePortalsOutsideLines`, `validateEntities`.
- Determinismus: Reihenfolge der `rng`-Aufrufe ist Vertrag (Golden-Files `testdata/golden/level-*.json`); `float64(…)`-Rundung gegen FMA.

## Flow

1. `LoadBiome(id)` → `Biome`.
2. `Generate`: `rng.New(seed)`, `frame` legt Chunk-Zahl, Hub, Ausgang fest.
3. `placePortals` (links/rechts mit Mindestabstand), `placeEvents` (Truhen, Camps nahe Hub), `entities` (Burg, Portale, Ressourcen, Skill-Punkte, Positionen via `inside`).
4. `generateLines` setzt die Mauerlinien beider Seiten in `Layout.Lines`.
5. Optional `Validate(layout, biome)` prüft Spielbarkeit.

## Integration

- Abhängigkeiten: `k3c/engine/rng`, `k3c/data` (embed der Biome, `hub.json`).
- Konsumenten: `engine/sim` (`CreateWorld`, Biome/Cycle), `engine/room`, `engine/net` (Level-Endpoint), `tools/k3c-dev/internal/enginetools` (`level_generate`).
