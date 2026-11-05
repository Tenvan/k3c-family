# light.json

- **Zweck:** Licht in Nacht und Morgengrauen: Helligkeit von Vordergrund und Hintergrund, Lichtquellen und die Versätze der Fackel-Plätze.
- **Gelesen von:** Simulation (`engine/sim/torches.go`, Fackel-Versätze), Client (Lichtkarte, nur Darstellung)
- **Wichtigste Werte:** `nightBrightness`, `backgroundBrightness`, `torches`
- **Regeln:** B-472

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `nightBrightness` | Zahl | Anteil | Helligkeit des Vordergrunds in tiefster Nacht (0 = schwarz, 1 = Tag) | 0,2 |
| `backgroundBrightness` | Zahl | Anteil | Helligkeit des Hintergrunds (Parallax) in tiefster Nacht | 0,05 |
| `torches` | Objekt | Units | Versätze der Fackel-Plätze je Gebäudeart | wall, tower |

## Wirkung auf das Balancing

Keine: Helligkeit und Lichtquellen sind reine Darstellung im Client. Nur die Fackel-Versätze lädt die Simulation (Position der Plätze).

## Beispiele für Varianten

Nicht als Variante nutzbar: `sim.UseData` lädt diese Datei bei einer Variante nicht neu.
