# hub.json

- **Zweck:** Layout und Ausbau des Hubs: Bauplätze, Hub-Stufen, Mauerlinien, Händler-Zahlziele, Insel-Startvorrat und Starttruppen.
- **Gelesen von:** Simulation (`engine/sim/hub_level.go`, `engine/sim/world_lines.go`), Level-Generator (`engine/level/lines.go`, `engine/level/planned_gen.go`), Raum (`engine/room/preview.go`)
- **Wichtigste Werte:** `levels.1.steps.0.cost.gold`, `levels.1.steps.2.gate.stone`, `sitePlacement.zone1WallUnits`, `islandStartStock.wood`, `startTroops.archer`, `travel.seconds`
- **Regeln:** `docs/rules/materialien-gebaeude.md` § 3, `docs/rules/stufen.md`

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `levels.*.steps` | Liste | – | Ausbauschritte 1 bis 3 auf Hub-Stufe n (Stufe 1 ohne Schritte, GEB17) | 3 Schritte |
| `levels.*.steps.*.cost` | Objekt | Gold und Material | Kosten eines Ausbauschritts (etwa 25/25/50 % der Stufe) | Stein 25 bis Kristall 125 |
| `levels.*.steps.*.cost.gold` | Zahl | Gold | Goldanteil des Schritts | 12 bis 200 |
| `levels.*.steps.*.buildSeconds` | Zahl | s | Bauzeit des Schritts | 5 bis 25 |
| `levels.*.steps.*.gate` | Objekt | Material | Material-Tor des letzten Schritts: 20 Material der eigenen neuen Hub-Stufe zusätzlich (Stufe 2 Stein, 3 Kupfer, 4 Eisen, 5 ohne; B-631) | 20 |
| `sites.*.kind` | Text | – | Gebäudeart am Bauplatz | workshop, farm, tavern, … |
| `sites.*.offsetUnits` | Zahl | Units | Position relativ zur Hub-Mitte | -52 bis 52 |
| `sites.*.hubLevel` | Ganzzahl | Hub-Stufe | Ab dieser Stufe bezahlbar | 1 bis 4 |
| `sites.*.fromDepth` | Ganzzahl | Tiefe | Nur ab dieser Tiefe vorhanden | 1 |
| `sites.*.needsDeeper` | Wahrheitswert | – | Nur wenn eine tiefere Stufe existiert | true |
| `islandSites.*.offsetUnits` | Zahl | Units | Position der Insel-Bauplätze (Lager) | -26 |
| `sitePlacement.zoneStartUnits` | Zahl | Units | Abstand der ersten Platz-Zone zur Hub-Mitte; Zone n spannt einen Chunk (B-652) | 40 |
| `sitePlacement.zone1WallUnits` | Zahl | Units | Fester Baugrund-Platz der Zone 1 je Seite: Mauer (wie die frühere Linie 1, B-652) | 44 |
| `sitePlacement.zone1TowerUnits` | Zahl | Units | Fester Baugrund-Platz der Zone 1 je Seite: Turm | 36 |
| `sitePlacement.zones` | Ganzzahl | Zonen | Zonen je Seite mit Mauer- und Turm-Plätzen; Zone n ab Hub-Stufe n sichtbar | 5 |
| `sitePlacement.perZoneMin` | Ganzzahl | Plätze | Mindestzahl Mauern und Türme je Zone und Seite | 1 |
| `sitePlacement.perZoneMax` | Ganzzahl | Plätze | Höchstzahl Mauern und Türme je Zone und Seite | 3 |
| `sitePlacement.towerNearShare` | Zahl | Anteil | Anteil der Türme, die neben einer Mauer der Zone stehen | 0,6 |
| `sitePlacement.towerNearMinUnits` | Zahl | Units | Kleinster Abstand eines nahen Turms zur Mauer (nicht unter dem Zahlziel-Abstand 4) | 4 |
| `sitePlacement.towerNearMaxUnits` | Zahl | Units | Größter Abstand eines nahen Turms zur Mauer | 10 |
| `sitePlacement.clearRadiusUnits` | Zahl | Units | Freiraum von Bäumen und Felsen, bevor ein Platz bezahlbar ist (Sim, BS12.2) | 6 |
| `sitePlacement.edgeMarginUnits` | Zahl | Units | Abstand der Plätze zum Level-Rand und zum nächsten Portal | 8 |
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

- `hub.json:levels.1.steps.0.cost.gold=20` – teurerer erster Ausbauschritt.
- `hub.json:levels.1.steps.2.buildSeconds=5` – schnellerer letzter Schritt zu Hub-Stufe 2.
- `hub.json:levels.1.steps.2.gate.stone=10` – kleineres Material-Tor zu Hub-Stufe 2.
- `hub.json:islandStartStock.wood=140` – mehr Startholz, mehr Mauern in Nacht 1.
- `hub.json:startTroops.archer=3` – ein Bogenschütze mehr zu Beginn.

**Hinweis Planer (B-652):** Der Hub-Planer des Studios prüft einen Kandidaten mit `sitePlacement.zone1WallUnits`, `zone1TowerUnits` und `outsideUnits`; der Level-Generator und die Sim lesen `sitePlacement` aber aus den eingebetteten Daten (`data/hub.json`). Eine geänderte Lage wirkt im Spiel und in der Vorschau erst nach dem Schreiben der Datei und dem Neustart.
