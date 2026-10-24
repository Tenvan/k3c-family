# biomes/*.json

- **Zweck:** Je Tiefe ein Biom: Länge, Chunk-Gewichte, Ressourcen, Ereignis-Chunks, Gegner, Tag-Nacht-Zyklus und Aggro-Pool.
- **Gelesen von:** Level-Generator (`engine/level/biome.go`), Simulation (`engine/sim/data.go`), Netz (`engine/net/level.go`)
- **Wichtigste Werte:** `lengthUnits.min`, `portals.count`, `chunkWeights`, `resourcesPerChunk`, `enemies.portal`, `cycle.type`
- **Regeln:** `docs/rules/welten.md`, `docs/rules/stufen.md`

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `id` | Text | – | Kennung des Bioms (forest, cave, mine, ironhold, crystal) | forest bis crystal |
| `depth` | Ganzzahl | Tiefe | Tiefe der Stufe | 0 bis 4 |
| `lengthUnits.min` | Ganzzahl | Units | Mindestlänge der Stufe (ohne `chunkCounts` der Welt) | 400 bis 900 |
| `lengthUnits.max` | Ganzzahl | Units | Höchstlänge der Stufe | 480 bis 1100 |
| `chunkWidthUnits` | Ganzzahl | Units | Breite eines Chunks | 50 |
| `hubWidthUnits` | Ganzzahl | Units | Breite des Hubs | 100 |
| `exitSide` | Text | – | Seite des Ausgangs (left, right) | left, right |
| `portals.count` | Ganzzahl | Portale | Anzahl Gegner-Portale | 2 |
| `portals.minDistanceFromHubUnits` | Ganzzahl | Units | Mindestabstand der Portale zum Hub | 150 |
| `chunkWeights.*` | Ganzzahl | Gewicht | Gewicht der Chunk-Art bei der Zufallsziehung | 1 bis 5 |
| `eventChunks.*.min` | Ganzzahl | Chunks | Mindestzahl an Ereignis-Chunks (Truhe, Rekrutierlager) | 1 |
| `eventChunks.*.max` | Ganzzahl | Chunks | Höchstzahl an Ereignis-Chunks | 1 bis 3 |
| `skillPoints.min` | Ganzzahl | Punkte | Skill-Punkte-Fundorte, Minimum | 3 |
| `skillPoints.max` | Ganzzahl | Punkte | Skill-Punkte-Fundorte, Maximum | 5 |
| `resourcesPerChunk.*.*` | Liste | Stück | Spanne [min, max] je Ressource und Chunk-Art | [0, 2] bis [14, 20] |
| `primaryResource` | Text | – | Hauptrohstoff der Stufe | wood, stone, copper, iron, crystal |
| `veins.count` | Ganzzahl | Adern | Anzahl Adern der Stufe | 2 |
| `veins.kind` | Text | – | Art der Ader (Schlüssel aus `economy.json` › veins) | stoneVein, copperVein, … |
| `lava.damagePerSecond` | Zahl | HP/s | Schaden der Lava (nur Eisenfeste) | 5 |
| `lava.widthUnits` | Zahl | Units | Breite der Lavabahnen | 8 |
| `lair` | Wahrheitswert | – | Bau des Endbosses (nur Kristallhöhle) | true |
| `enemies.portal` | Liste | – | Gegner-Arten, die aus den Portalen kommen | z. B. greed, goblin, goblinArcher |
| `enemies.night` | Liste | – | Gegner-Arten der Nacht im Hub | wolf, skeleton, … |
| `cycle.type` | Text | – | dayNight (Wald) oder aggressionPool (Tiefen) | dayNight, aggressionPool |
| `cycle.dayMinutes` | Zahl | min | Länge des Tages (dayNight) | 10 |
| `cycle.nightMinutes` | Zahl | min | Länge der Nacht (dayNight) | 5 |
| `cycle.twilightMinutes` | Zahl | min | Länge der Dämmerung (dayNight) | 1 |
| `cycle.percentPerMinute` | Zahl | % je min | Zuwachs des Aggro-Pools nach Zeit (aggressionPool) | 1 |
| `cycle.percentPerKill` | Zahl | % je Kill | Zuwachs je Kill | 5 |
| `cycle.percentPerGather` | Zahl | % je Abbau | Zuwachs je Abbau | 1 |
| `palette.*` | Text | Farbe | Farben der Darstellung (sky, far, near, ground) | #8ecae6, … |

## Wirkung auf das Balancing

Länge und Chunk-Gewichte bestimmen Wege und Material je Stufe, `cycle` den Takt der Nächte (Wald: Tag/Nacht, Tiefen: Aggro-Pool), `enemies` welche Arten kommen. Das Tempo der Nächte und die Gegner-Mischung wirken auf Burg-hält und zerstörte Gebäude (`docs/rules/zielkorridore.md` § 1, § 3). Die Palette hat keine Wirkung auf das Balancing.

## Beispiele für Varianten

Nicht als Variante nutzbar (Ordner bzw. nur Tester).
