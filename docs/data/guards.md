# guards.json

- **Zweck:** Wachen in geplanten Chunks: Aggro-Radius, Anzahl je Chunk und Mindestabstand.
- **Gelesen von:** Level-Generator (`engine/level/guards.go`)
- **Wichtigste Werte:** `aggroRadiusUnits`, `maxPerChunk`, `minGapUnits`
- **Regeln:** `docs/rules/geplante-chunks.md` § 9

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `aggroRadiusUnits` | Zahl | Units | Radius um den Heimatpunkt der Wache, in dem sie Spieler und Truppen angreift | 10 |
| `maxPerChunk` | Ganzzahl | Wachen | Höchstzahl Wachen je Chunk | 4 |
| `minGapUnits` | Zahl | Units | Mindestabstand zweier Wachen eines Chunks | 1,5 |

## Wirkung auf das Balancing

Mehr Wachen oder ein größerer Aggro-Radius machen geplante Chunks gefährlicher und bremsen das Erkunden außerhalb der Burg; auf Burg-hält wirkt das nur indirekt.

## Beispiele für Varianten

Nicht als Variante nutzbar: `sim.UseData` lädt diese Datei bei einer Variante nicht neu, der Balancing-Lauf nutzt immer den Stand der Datei. Zum Ausprobieren die Datei ändern und den Lauf mit dem alten Bericht vergleichen.
