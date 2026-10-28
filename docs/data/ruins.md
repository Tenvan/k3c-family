# ruins.json

- **Zweck:** Ruinen in geplanten Chunks: erlaubte Gebäudearten und Start-HP-Anteil.
- **Gelesen von:** Level-Generator (`engine/level/ruins.go`)
- **Wichtigste Werte:** `hpShare`, `kinds`
- **Regeln:** `docs/rules/geplante-chunks.md` § 8

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `hpShare` | Zahl | Anteil | Start-HP einer Ruine als Anteil der Gebäude-HP aus `buildings.json` (aufgerundet) | 0,25 |
| `kinds` | Liste | – | Nutzgebäude, die als Ruine geplant werden dürfen (nie Burg, Mauer, Turm, Tor, Treppen) | workshop, storage, nursery, … |

## Wirkung auf das Balancing

Ein höherer `hpShare` macht den Wiederaufbau billiger bzw. die Ruinen stabiler; die Liste bestimmt, welche Gebäude schon früh in der Welt zu finden sind. Wirkung auf die Burg-Korridore ist gering.

## Beispiele für Varianten

Nicht als Variante nutzbar: `sim.UseData` lädt diese Datei bei einer Variante nicht neu, der Balancing-Lauf nutzt immer den Stand der Datei. Zum Ausprobieren die Datei ändern und den Lauf mit dem alten Bericht vergleichen.
