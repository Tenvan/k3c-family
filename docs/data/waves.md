# waves.json

- **Zweck:** Wellengröße nach Wellennummer, Verteilung der Spawns und Skalierung für Welten ohne Insel.
- **Gelesen von:** Simulation (`engine/sim/data.go`), Insel-Daten (`engine/sim/island_data.go`)
- **Wichtigste Werte:** `table.0.standard`, `table.1.elite`, `spawnSpreadSeconds`, `perExtraPlayer`, `depthScaling.hp`, `stealGold`
- **Regeln:** `docs/rules/gegner.md`, `docs/rules/stufen.md`

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `table.*.fromWave` | Ganzzahl | Welle | Ab dieser Welle gilt die Zeile | 1, 6, 11 |
| `table.*.standard` | Liste | Gegner | Spanne [min, max] Standardgegner je Welle | [5, 10] bis [15, 20] |
| `table.*.elite` | Liste | Gegner | Spanne [min, max] Elite-Gegner je Welle | [0, 0] bis [3, 5] |
| `spawnSpreadSeconds` | Zahl | s | Zeitraum, über den die Spawns einer Welle verteilt werden | 20 |
| `perExtraPlayer` | Zahl | Anteil | Zuschlag auf die Wellengröße je zusätzlichem Spieler | 0,5 |
| `depthScaling.hp` | Zahl | Basis je Tiefe | HP-Faktor, potenziert mit der Tiefe (nur Welten ohne Insel) | 1,5 |
| `depthScaling.damage` | Zahl | Basis je Tiefe | Schaden-Faktor je Tiefe | 1,3 |
| `depthScaling.speed` | Zahl | Basis je Tiefe | Tempo-Faktor je Tiefe | 1,1 |
| `stealGold` | Ganzzahl | Gold | Gold, das ein Dieb-Gegner raubt | 5 |

## Wirkung auf das Balancing

Die Tabelle ist der stärkste Hebel auf Burg-hält Nacht 1–5 und zerstörte Gebäude je Welle. `perExtraPlayer` skaliert für 2 Spieler, `spawnSpreadSeconds` glättet den Ansturm. Inseln verwenden die Tabelle der Welt-Datei (`worlds.md`), nicht `depthScaling` hier. Siehe `docs/rules/zielkorridore.md` § 1, § 3.

## Beispiele für Varianten

- `waves.json:table.0.standard.1=12` – größere Wellen in Nacht 1 bis 5.
- `waves.json:perExtraPlayer=0.3` – weniger Zuschlag für den zweiten Spieler.
- `waves.json:spawnSpreadSeconds=30` – Ansturm länger gestreckt.
- `waves.json:depthScaling.hp=1.7` – mehr HP je Tiefe.
