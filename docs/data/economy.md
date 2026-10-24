# economy.json

- **Zweck:** Gold, Aufheben, Sammeln, Adern, Lager, Rekrutierlager und Händler der Wirtschaft.
- **Gelesen von:** Simulation (`engine/sim/data.go`), Level-Generator (`engine/level/lines.go`), Händler (`engine/sim/merchant.go`), Adern (`engine/sim/world_vein.go`)
- **Wichtigste Werte:** `purse.startGold`, `dawnGoldPerPlayer`, `chestGold`, `gatherables.tree.workSeconds`, `veins.copperVein.workSeconds`, `storage.basePerHub`
- **Regeln:** `docs/rules/wirtschaft.md`, `docs/rules/materialien-gebaeude.md`

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `purse.startGold` | Zahl | Gold | Startgold je Monarch | 100 |
| `purse.maxGold` | Zahl | Gold | Obergrenze des Geldbeutels | 100 |
| `payRangeUnits` | Zahl | Units | Reichweite, in der ein Zahlziel bezahlt wird | 2 |
| `payIntervalSeconds` | Zahl | s | Abstand zwischen zwei eingezahlten Münzen | 0,25 |
| `pickupRangeUnits` | Zahl | Units | Aufheb-Reichweite für Münzen am Boden | 0,8 |
| `dropPickupDelaySeconds` | Zahl | s | Sperre, bevor abgeworfene Münzen wieder aufgehoben werden | 1,5 |
| `dawnGoldPerPlayer` | Zahl | Gold | Gold je Spieler bei Tagesanbruch, wenn die Burg hielt | 5 |
| `chestGold` | Liste | Gold | Spanne [min, max] einer Truhe | [10, 25] |
| `enemyResourceDrop.chance` | Zahl | Anteil 0–1 | Wahrscheinlichkeit, dass ein Gegner Material fallen lässt | 0,1 |
| `enemyResourceDrop.amount` | Ganzzahl | Material | Menge eines Material-Drops | 5 |
| `gatherables.*.resource` | Text | – | Rohstoff, den Baum, Fels oder Kupfererz liefert | wood, stone, copper |
| `gatherables.*.amount` | Ganzzahl | Material | Ertrag je Abbau | 10 |
| `gatherables.*.workSeconds` | Zahl | s | Arbeitszeit eines Bauern je Abbau | 4 bis 8 |
| `gatherables.*.markCost` | Ganzzahl | Gold | Preis der Markierung zum Abbauen | 1 bis 2 |
| `veins.*.resource` | Text | – | Rohstoff der Ader | stone, copper, iron, crystal |
| `veins.*.amount` | Ganzzahl | Material | Ertrag je Abbau | 5 bis 15 |
| `veins.*.workSeconds` | Zahl | s | Arbeitszeit je Abbau; Adern sind deutlich langsamer als Bäume | 20 bis 40 |
| `veins.*.markCost` | Ganzzahl | Gold | Preis der Markierung | 1 bis 2 |
| `veins.*.maxWorkers` | Ganzzahl | Bauern | Wie viele Bauern gleichzeitig an einer Ader arbeiten | 2 |
| `storage.basePerHub` | Ganzzahl | Material je Rohstoff | Lagerkapazität je Hub ohne Lager | 300 |
| `storage.perStorage` | Ganzzahl | Material je Rohstoff | Zusatzkapazität je gebautem Lager | 300 |
| `recruitCamp.maxVagrants` | Ganzzahl | Landstreicher | Obergrenze am Rekrutierlager | 2 |
| `recruitCamp.respawnSeconds` | Zahl | s | Nachwuchszeit eines Landstreichers | 25 |
| `recruitCamp.wanderUnits` | Zahl | Units | Wanderradius der Landstreicher | 6 |
| `merchant.hp` | Zahl | HP | Lebenspunkte des Händlers | 100 |
| `merchant.everyDays` | Ganzzahl | Tage | Abstand der Händler-Besuche | 3 |
| `merchant.everyDaysWithTavern` | Ganzzahl | Tage | Abstand mit gebauter Taverne | 2 |
| `merchant.stayDays` | Ganzzahl | Tage | Aufenthaltsdauer | 1 |
| `merchant.material` | Ganzzahl | Material | Material je Handelsschritt | 10 |
| `merchant.gold` | Ganzzahl | Gold | Gold je Handelsschritt | 5 |
| `merchant.materials` | Liste | – | Handelbare Rohstoffe | wood bis crystal |

## Wirkung auf das Balancing

Mehr Startgold oder Tagesgold bringt die erste Mauer früher (Korridor `firstWallBeforeDusk1`, Burg hält Nacht 1–5). Kürzere `workSeconds` und höhere `amount` beschleunigen Material und damit Ausbau und Hub-Stufen; zu schnell entwertet es die Adern auf tieferen Stufen. `storage.*` begrenzt, wie viel Vorrat für teure Hub-Stufen angespart werden kann. Betroffen sind vor allem die Zielkorridore zu Wirtschaft und Burg-hält (`docs/rules/zielkorridore.md` § 1, § 3).

## Beispiele für Varianten

- `economy.json:purse.startGold=120` – mehr Startgold, erste Mauer früher.
- `economy.json:dawnGoldPerPlayer=8` – mehr Tagesgold, schnellerer Aufbau.
- `economy.json:veins.copperVein.workSeconds=30` – Kupferadern schneller abbaubar.
- `economy.json:storage.basePerHub=200` – knapperes Lager, Vorrat muss früher ausgegeben werden.
