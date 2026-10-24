# enemies.json

- **Zweck:** Werte aller Gegnerarten nach Tiefe und Stufe (standard oder elite) samt Eigenschaften.
- **Gelesen von:** Simulation (`engine/sim/data.go`), Eigenschaften (`engine/sim/enemies_traits.go`), Bosse (`engine/sim/boss_data.go`)
- **Wichtigste Werte:** `greed.hp`, `greed.damage`, `wolf.speed`, `goblin.gold`, `caveTroll.aoe.radius`, `crystalGuardian.hp`
- **Regeln:** `docs/rules/gegner.md`

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `*.name` | Text | – | Anzeigename | Greed, Wolf, … |
| `*.depth` | Ganzzahl | Tiefe | Tiefe, ab der die Art vorkommt | 0 bis 4 |
| `*.tier` | Text | – | standard oder elite; Elite zählt in der Wellen-Tabelle gesondert | standard, elite |
| `*.hp` | Zahl | HP | Lebenspunkte vor Skalierung | 20 bis 350 |
| `*.damage` | Zahl | Schaden | Schaden je Treffer vor Skalierung | 5 bis 45 |
| `*.speed` | Zahl | Units/s | Lauftempo | 2 bis 7 |
| `*.range` | Zahl | Units | Angriffsreichweite | 1 bis 10 |
| `*.attacksPerSecond` | Zahl | 1/s | Angriffe je Sekunde | 1 |
| `*.gold` | Liste | Gold | Spanne [min, max] des Gold-Drops | [1, 2] bis [50, 80] |
| `*.traits` | Liste | – | Eigenschaften (z. B. flying, ignoresWalls, prefersTroops, kiting) | je Art |
| `*.grabRange` | Zahl | Units | Greif-Reichweite für Ausrüstung am Boden | 1,5 |
| `*.kiteDistance` | Zahl | Units | Haltedistanz bei Fernkämpfern | 8 |
| `*.swarmSize` | Ganzzahl | Stück | Gegner je Schwarm | 4 |
| `*.aoe.radius` | Zahl | Units | Radius des Flächenangriffs | 3 |
| `*.aoe.intervalSeconds` | Zahl | s | Abstand des Flächenangriffs | 4 |
| `mineGhost.phases.everySeconds` | Zahl | s | Minengeist wechselt in diesem Takt in die Phase | 8 |
| `mineGhost.phases.durationSeconds` | Zahl | s | Dauer der Phase | 2 |

## Wirkung auf das Balancing

Gegner-HP, Schaden und Tempo wirken direkt auf Burg-hält und zerstörte Gebäude; Gold-Drops füttern die Wirtschaft (Gold je Welle). Die Werte gelten vor der Skalierung nach Tiefe (`waves.json`, Welt-Datei) und Schwierigkeitsgrad (`difficulty.json`). Siehe `docs/rules/zielkorridore.md` § 1, § 3.

## Beispiele für Varianten

- `enemies.json:greed.hp=60` – zähere Greed, härtere Nächte.
- `enemies.json:wolf.speed=6` – schnellere Wölfe.
- `enemies.json:goblin.gold.0=10` – mehr Mindest-Gold je Goblin.
- `enemies.json:caveTroll.damage=25` – schwächere Höhlentrolle.
