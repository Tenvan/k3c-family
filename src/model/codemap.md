# src/model/

## Responsibility

Domain Model des Clients: Typen und Balancing-Daten, die der Client vom Go-Server kennt. Reine Daten und Typen ohne Logik; die Simulation läuft in `engine/sim/`.

## Design

- `types.ts`: Zustand und Ereignisse als Datentypen (`World`, `Player` mit `skills`, `slots`, `cooldowns`, `points`, `actions`, `PlayerAction`, `Troop`, `Job`, `ResourceNode`, `Site`/`SiteKind`, `Castle`, `Enemy` (mit `boss`, Phase und Warnkreis `WarnZone` für Bosse), `Projectile`, `Pickup`, `Merchant`, `Drop`, `HubUpgrade`, `GameEvent` als Union mit optionalem `stage` (u. a. `hit`, `kill`, `arrow`, `strike`, `coinPickup`, `buildProgress`, `revive`, `equipmentTaken`), `PlayerCommand`/`IDLE`, `Travel`, `CycleInfo`, `LevelLayout`/`Chunk`/`LevelEntity`, `World` mit optionalen Server-Feldern (`stockMax`, `hubLevel`, `danger`, `nightEvents`, `islandSwitch`, `eventsDropped`), `SaveGame`/`HubSave`/`isSaveGame`). Positionen in Units, Zeiten in Sekunden. `Stock` = Vorrat je `ResourceKind` (`iron` und `crystal` fehlen bei 0). `SiteKind` leitet sich aus den Schlüsseln von `data/buildings.json` ab.
- `data.ts`: typisierter Zugriff auf `data/*.json` (`BUILDINGS`, `TROOPS`, `ENEMIES`, `MONARCH`, `HUB`, `ECONOMY`, `WAVES`) per Cast; `ResourceKind` = `wood | stone | copper` plus `RareResource` (`iron | crystal`), `Cost` mit optionalem `gold`; die JSON ist einzige Quelle (auch für den Go-Server via `go:embed`).
- `biome.ts`: `BiomeConfig`, `BIOMES` (alle Dateien aus `data/biomes/*.json` per `import.meta.glob`, nach `depth` sortiert; `biome.test.ts` prüft Pflichtfelder und lückenlose Tiefen), `biomeForDepth` (wirft bei unbekannter Tiefe), `Range`, `EventChunkKind`.

## Flow

1. Beim Modulimport werden die JSON-Dateien aus `data/` per Vite gebündelt und als typisierte Konstanten exportiert.
2. `biome.ts` lädt `data/biomes/*.json` beim Import (Glob, eager).
3. `online/clientWorld.ts` baut aus Server-Snapshots (`applyState`) Objekte vom Typ `World`; `GameEvent`s kommen mit den Frames (`docs/protocol.md`).
4. Szenen und Audio lesen diese Typen/Konstanten nur (Zeichnen, Namen, Kosten, Ton-Zuordnung).

## Integration

- Konsumenten: `src/scenes/` (`GameScene`, `HudScene`, `worldRenderer`, `viewRules`, `radar`, `radarView`, `stageView`, `effects`, `effectRules`, `debugOverlay*`), `src/audio/` (`audioCore`, `events`), `src/online/` (`clientConnection`, `clientProtocol`, `clientWorld`, `protocol`), `src/core/saveStore.ts`, `src/tools/`.
- Abhängigkeiten: `data/*.json`, `data/biomes/*.json`; intern `types.ts` → `biome.ts`/`data.ts` (nur Typen).
