# balance-targets.json

- **Zweck:** Zielkorridore des Balancing-Testers: Seeds, Toleranz und die gemessenen Kennzahlen mit Unter- und Obergrenze.
- **Gelesen von:** Balancing-Tester der Workbench (`tools/k3c-workbench/internal/balance/`), nicht die Simulation
- **Wichtigste Werte:** `margin.sharePp`, `margin.medianFraction`, `seeds`, `targets.0.lower`, `targets.0.upper`, `targets.2.upper`
- **Regeln:** `docs/rules/zielkorridore.md`

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `version` | Ganzzahl | – | Version des Dateiformats | 1 |
| `margin.sharePp` | Zahl | Prozentpunkte | Toleranz bei Kennzahlen vom Typ share | 5 |
| `margin.medianFraction` | Zahl | Anteil | Toleranz bei Kennzahlen vom Typ median | 0,1 |
| `seedMode` | Text | – | `dev`: ohne Seed-Angabe nur der erste Seed (`k3c-dev`, Projekt BAS bis zur Abnahme der Werte); `all` oder leer: alle (B-634) | `dev`, `all` |
| `seeds` | Liste | Seeds | Seeds, über die der Tester jede Variante misst; `k3c-dev` steht vorn (B-634) | `k3c-dev`, 1 bis 100 |
| `targets.*.measure` | Text | – | Kennzahl (castleHeld, firstWallBeforeDusk1, destroyedPerWave) | je Ziel |
| `targets.*.name` | Text | – | Anzeigename des Ziels | je Ziel |
| `targets.*.kind` | Text | – | share (Anteil der Läufe) oder median | share, median |
| `targets.*.players` | Ganzzahl | Spieler | Spielerzahl der Messung | 2 |
| `targets.*.bot` | Text | – | Bot, der spielt | saver |
| `targets.*.depth` | Ganzzahl | Tiefe | Gemessene Stufe | 0 |
| `targets.*.days` | Ganzzahl | Tage | Länge des Messlaufs | 5 |
| `targets.*.lower` | Zahl | % bzw. Wert | Untergrenze des Korridors; null = keine | 75 bis 90 |
| `targets.*.upper` | Zahl | % bzw. Wert | Obergrenze des Korridors | 1 bis 100 |
| `targets.*.rule` | Text | – | Verweis auf die Regelstelle | docs/rules/zielkorridore.md § 1 |

## Wirkung auf das Balancing

Die Datei gibt vor, was als ausgewogen gilt: Burg hält Nacht 1–5 zu 75–90 %, erste Mauer vor Ende der hellen Phase zu 90–100 %, höchstens ein zerstörtes Gebäude je Welle (Median). Engere Korridore oder weniger Seeds machen den Tester strenger bzw. ungenauer; die Simulation liest die Datei nicht.

## Beispiele für Varianten

Nicht als Variante nutzbar (Ordner bzw. nur Tester).
