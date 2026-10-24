# hub.json

- **Zweck:** Layout und Ausbau des Hubs: Bauplätze, Hub-Stufen, Mauerlinien, Händler-Zahlziele, Insel-Startvorrat und Starttruppen.
- **Gelesen von:** Simulation (`engine/sim/hub_level.go`, `engine/sim/world_lines.go`), Level-Generator (`engine/level/lines.go`, `engine/level/planned_gen.go`), Raum (`engine/room/preview.go`)
- **Wichtigste Werte:** `levels.1.cost.gold`, `levels.1.buildSeconds`, `wallLines.wallUnits`, `islandStartStock.wood`, `startTroops.archer`, `travel.seconds`
- **Regeln:** `docs/rules/materialien-gebaeude.md` § 3, `docs/rules/stufen.md`

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `levels.*.cost` | Objekt | Gold und Material | Kosten des Burg-Ausbaus auf Hub-Stufe n (Stufe 1 ohne Kosten) | Stein 100 bis Kristall 250 |
| `levels.*.cost.gold` | Zahl | Gold | Goldanteil des Ausbaus | 50 bis 400 |
| `levels.*.buildSeconds` | Zahl | s | Bauzeit des Ausbaus | 20 bis 50 |
| `sites.*.kind` | Text | – | Gebäudeart am Bauplatz | workshop, farm, tavern, … |
| `sites.*.offsetUnits` | Zahl | Units | Position relativ zur Hub-Mitte | -52 bis 52 |
| `sites.*.hubLevel` | Ganzzahl | Hub-Stufe | Ab dieser Stufe bezahlbar | 1 bis 4 |
| `sites.*.fromDepth` | Ganzzahl | Tiefe | Nur ab dieser Tiefe vorhanden | 1 |
| `sites.*.needsDeeper` | Wahrheitswert | – | Nur wenn eine tiefere Stufe existiert | true |
| `islandSites.*.offsetUnits` | Zahl | Units | Position der Insel-Bauplätze (Lager) | -26 |
| `wallLines.wallUnits` | Liste | Units | Mauer-Abstand zur Hub-Mitte je Hub-Stufe | 44 bis 124 |
| `wallLines.fixedLines` | Ganzzahl | Linien | Anzahl fest platzierter Linien | 1 |
| `wallLines.jitterOutwardUnits` | Zahl | Units | Zufälliger Versatz nach außen je Seed | 4 |
| `wallLines.towerInsetUnits` | Zahl | Units | Turm innen von der Mauer | 8 |
| `wallLines.gateOutsetUnits` | Zahl | Units | Tor außen von der Mauer | 4 |
| `wallLines.gateHubLevel` | Ganzzahl | Hub-Stufe | Ab dieser Stufe gibt es ein Tor | 2 |
| `merchant.buyOffsetUnits` | Zahl | Units | Zahlziel Kaufen | 16 |
| `merchant.sellOffsetUnits` | Zahl | Units | Zahlziel Verkaufen | 20 |
| `merchant.onlyDepth` | Ganzzahl | Tiefe | Händler-Zahlziele nur in dieser Tiefe | 0 |
| `islandStartStock.wood` | Ganzzahl | Holz | Startvorrat einer neuen Insel | 100 |
| `castleRadiusUnits` | Zahl | Units | Radius der Burg | 5 |
| `homeRadiusUnits` | Zahl | Units | Heimat-Radius um den Hub | 12 |
| `startTroops.peasant` | Ganzzahl | Bauern | Bauern zu Spielbeginn | 1 |
| `startTroops.archer` | Ganzzahl | Bogenschützen | Bogenschützen zu Spielbeginn | 2 |
| `travel.rangeUnits` | Zahl | Units | Reichweite am Eingang bzw. an der Treppe | 3 |
| `travel.seconds` | Zahl | s | Wartezeit für den Stufenwechsel | 2 |

## Wirkung auf das Balancing

Hub-Kosten und Bauzeiten bestimmen das Tempo des Gesamtfortschritts (Stein, Kupfer, Eisen, Kristall der Reihe nach). `startTroops` und `islandStartStock` entscheiden, ob die erste Nacht mit Mauern zu schaffen ist (`castleHeld`, `firstWallBeforeDusk1`, `docs/rules/zielkorridore.md` § 1). Größere `wallUnits` verlängern die Verteidigungslinie und kosten mehr Material.

## Beispiele für Varianten

- `hub.json:levels.1.cost.gold=75` – teurerer erster Ausbau.
- `hub.json:levels.1.buildSeconds=15` – schnellerer erster Ausbau.
- `hub.json:islandStartStock.wood=140` – mehr Startholz, mehr Mauern in Nacht 1.
- `hub.json:startTroops.archer=3` – ein Bogenschütze mehr zu Beginn.
