# island-sizes.json

- **Zweck:** Benannte Inselgrößen als erlaubte Chunk-Anzahlen einer Stufe.
- **Gelesen von:** Simulation (`engine/sim/island_sizes.go`)
- **Wichtigste Werte:** `sizes.0.chunkCount`, `sizes.2.chunkCount`, `sizes.5.chunkCount`
- **Regeln:** `docs/rules/geplante-chunks.md` § 7

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `sizes.*.name` | Text | – | Name der Größe | sehr klein bis megagroß |
| `sizes.*.chunkCount` | Ganzzahl | Chunks | Chunk-Anzahl einer Stufe (gerade, Hub mittig; 16 ist das Minimum) | 16 bis 160 |

## Wirkung auf das Balancing

Größere Stufen verlängern die Wege zu Portalen und Rohstoffen und damit die Zeit zwischen Nächten; kleine sparen Weg, lassen aber wenig Raum. Die Größe je Insel und Tiefe setzt die Welt-Datei (`chunkCounts`).

## Beispiele für Varianten

Nicht als Variante nutzbar: `sim.UseData` lädt diese Datei bei einer Variante nicht neu, der Balancing-Lauf nutzt immer den Stand der Datei. Zum Ausprobieren die Datei ändern und den Lauf mit dem alten Bericht vergleichen.
