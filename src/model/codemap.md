# src/model/

## Responsibility

Domain Model des Clients: Typen und Balancing-Daten, die der Client vom Go-Server kennt. Reine Daten und Typen ohne Logik; die Simulation läuft in `engine/sim/`.

## Design

- `types.ts`: Zustand und Ereignisse als Datentypen (`World`, `Player`, `Troop`, `Enemy`, `Site`, `Castle`, `GameEvent` als Union mit optionalem `stage`, `PlayerCommand`/`IDLE`, `Travel`). Positionen in Units, Zeiten in Sekunden.
- `data.ts`: typisierter Zugriff auf `data/*.json` (`BUILDINGS`, `TROOPS`, `ENEMIES`, `MONARCH`, `HUB`, `ECONOMY`, `WAVES`) per Cast; die JSON ist einzige Quelle (auch für den Go-Server via `go:embed`).
- `biome.ts`: `BiomeConfig`, `BIOMES` (forest, cave, mine aus `data/biomes/*.json`), `biomeForDepth`.

## Flow

1. Beim Modulimport werden die JSON-Dateien aus `data/` per Vite gebündelt und als typisierte Konstanten exportiert.
2. `online/clientWorld.ts` baut aus Server-Snapshots (`applyState`) Objekte vom Typ `World`; `GameEvent`s kommen mit den Frames.
3. Szenen und Audio lesen diese Typen/Konstanten nur (Zeichnen, Namen, Kosten, Ton-Zuordnung).

## Integration

- Konsumenten: `src/scenes/` (`GameScene`, `HudScene`, `worldRenderer`, `viewRules`, `radar`, `radarView`, `stageView`, `effects`, `effectRules`, `debugOverlay*`), `src/audio/` (`audioCore`, `events`), `src/online/` (`clientConnection`, `clientProtocol`, `clientWorld`, `protocol`), `src/core/saveStore.ts`, `src/tools/` (`leveltest`, `levelView`).
- Abhängigkeiten: `data/*.json`, `data/biomes/*.json`; intern `types.ts` → `biome.ts`/`data.ts` (nur Typen).
