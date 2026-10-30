# buildings.json

- **Zweck:** Werte aller Gebäude: HP, Bauzeit, Kosten und Stufen 1 bis 5 mit gleicher Platzierung.
- **Gelesen von:** Simulation (`engine/sim/data.go`), Level-Generator (`engine/level/ruins.go`), Gebäude-Logik (`engine/sim/building_*.go`)
- **Wichtigste Werte:** `wall.levels.0.cost.gold`, `wall.levels.0.hp`, `tower.levels.0.cost.wood`, `wall.levels.1.cost.stone`, `stairsDown.costByDepth.1.stone`, `tower.archerSlots`, `barracks.troopLimit.base`, `nursery.plantation.growSeconds`
- **Regeln:** `docs/rules/materialien-gebaeude.md`

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `*.hp` | Zahl | HP | Lebenspunkte des Gebäudes (Stufe 1) | 100 bis 1000 |
| `*.buildSeconds` | Zahl | s | Bauzeit eines Bauern | 6 bis 20 |
| `*.unlockDepth` | Ganzzahl | Tiefe | Ab dieser Stufe (Tiefe) baubar | 0 oder 1 |
| `*.cost.gold` | Zahl | Gold | Goldkosten des Gebäudes (Stufe 1) | 5 bis 100 |
| `*.cost.wood` | Zahl | Holz | Holzkosten | 20 bis 50 |
| `*.cost.stone` | Zahl | Stein | Steinkosten | 30 bis 100 |
| `*.levels.*.cost` | Objekt | Gold und Material | Kosten des Ausbaus auf Stufe n (Mauer, Turm, Tor und Hub-Gebäude); Material je Stufe: Holz, Stein, Kupfer, Eisen, Kristall | Gold wächst von 5 bis 300 |
| `*.levels.*.cost.gold` | Zahl | Gold | Goldanteil der Stufe | 5 bis 300 |
| `*.levels.*.hp` | Zahl | HP | HP der Stufe | Mauer 300, 600, 1000, 1600, 2500; Tor 250, 500, 850, 1350, 2100 |
| `*.levels.*.buildSeconds` | Zahl | s | Bauzeit der Stufe | 6 bis 40 |
| `stairsDown.costByDepth` | Liste | Gold und Material | Kosten der Treppe runter je Biom-Tiefe 0 bis 4: 100 Material der eigenen Stufe + 50 Gold (Holz, Stein, Kupfer, Eisen; Tiefe 4 leer); leer oder fehlt = `cost` (B-631) | 100 Material, 50 Gold |
| `stairsUp.costByDepth` | Liste | Gold und Material | Kosten der Treppe hoch je Biom-Tiefe 0 bis 4: 100 Material der Stufe darüber + 50 Gold (Tiefe 0 leer); leer oder fehlt = `cost` | 100 Material, 50 Gold |
| `stairsUp.costByDepth.*.gold` | Zahl | Gold | Goldanteil der Treppe je Tiefe | 50 |
| `tower.archerSlots` | Ganzzahl | Schützen | Bogenschützen-Plätze am Turm | 2 |
| `tower.rangeBonus` | Zahl | Units | Reichweitenbonus für Schützen im Turm | 3 |
| `tower.spell.fromLevel` | Ganzzahl | Stufe | Ab dieser Stufe ein Zauberturm | 5 |
| `tower.spell.damage` | Zahl | Schaden | Flächenschaden des Turmzaubers | 40 |
| `tower.spell.radiusUnits` | Zahl | Units | Radius um das Ziel | 3 |
| `tower.spell.rangeUnits` | Zahl | Units | Reichweite | 13 |
| `tower.spell.intervalSeconds` | Zahl | s | Abstand der Schüsse | 1,5 |
| `workshop.bowRack` | Ganzzahl | Bögen | Bogen-Vorrat am Platz | 3 |
| `workshop.swordRack` | Ganzzahl | Schwerter | Schwert-Vorrat | 3 |
| `workshop.craftSeconds.*` | Zahl | s | Herstellungszeit je Stück (Bogen, Schwert) | 10 |
| `workshop.offers.*.kind` | Text | – | Angebot am Zahlziel (Handwerker, Schwert, Bergmann, Baumeister) | craftsman, sword, miner, builder |
| `workshop.offers.*.dx` | Zahl | Units | Versatz des Angebots zum Gebäude | -4 bis 12 |
| `nursery.plantation.slots` | Ganzzahl | Plätze | Baum-Plätze der Plantage | 6 |
| `nursery.plantation.growSeconds` | Zahl | s | Nachwuchszeit eines Baums | 30 |
| `nursery.plantation.stripUnits` | Zahl | Units | Breite des Streifens um die Baumschule | 3 |
| `barracks.troopLimit.base` | Ganzzahl | Kämpfer | Kämpfer-Limit je Hub ohne Kaserne | 10 |
| `barracks.troopLimit.perBuilding` | Ganzzahl | Kämpfer | Zusatzlimit je Kaserne | 10 |
| `tavern.vagrants.perDawn` | Ganzzahl | Landstreicher | Neue Landstreicher je Morgen | 1 |
| `tavern.vagrants.max` | Ganzzahl | Landstreicher | Obergrenze an der Taverne | 2 |
| `healer.heal.hpPerSecond` | Zahl | HP/s | Heilung im Radius | 5 |
| `healer.heal.radiusUnits` | Zahl | Units | Heilradius | 6 |
| `smithy.craftSeconds.*` | Zahl | s | Herstellungszeit der Elite-Truppen | 20 |
| `smithy.offers.*.dx` | Zahl | Units | Versatz des Elite-Angebots | 4 |
| `armory.craftSeconds.armor` | Zahl | s | Herstellungszeit einer Rüstungsstufe | 30 |
| `armory.armor.*.hubLevel` | Ganzzahl | Hub-Stufe | Ab dieser Hub-Stufe verfügbar | 4, 5 |
| `armory.armor.*.hpBonus` | Zahl | Anteil | HP-Bonus auf alle Kämpfer des Hubs | 0,2 und 0,4 |
| `armory.armor.*.cost.iron` | Zahl | Eisen | Eisenkosten der Rüstungsstufe | 100 |

## Wirkung auf das Balancing

Mauer-, Turm- und Truppenwerte bestimmen direkt, ob die Burg die Nächte hält (Korridor `castleHeld`) und wie viele Gebäude je Welle zerstört werden (`destroyedPerWave`). Höhere Mauer-HP oder niedrigere Kosten senken Verluste, verschieben aber das Material-Gleichgewicht. Die Ausbau-Kosten der Stufen 2 bis 5 steuern, wie früh Stein, Kupfer, Eisen und Kristall gebraucht werden. Siehe `docs/rules/zielkorridore.md` § 1 und § 3.

## Beispiele für Varianten

- `buildings.json:wall.levels.0.cost.gold=8` – teurere erste Mauer.
- `buildings.json:wall.levels.0.hp=400` – stabilere erste Mauer.
- `buildings.json:tower.levels.0.cost.wood=40` – billigerer Turm.
- `buildings.json:wall.levels.1.hp=600` – stabilere Steinmauer (die äußerste Mauer ist das Tor).
- `buildings.json:stairsUp.costByDepth.1.wood=80` – billigere Treppe hoch in der Höhle.
- `buildings.json:barracks.troopLimit.base=14` – höheres Kämpfer-Limit ohne Kaserne.
