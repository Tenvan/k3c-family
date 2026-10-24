# troops.json

- **Zweck:** Werte der Truppen und Bürger: HP, Tempo, Schaden, Reichweite, Kosten und Berufe.
- **Gelesen von:** Simulation (`engine/sim/data.go`), Wirtschaft (`engine/sim/economy.go`), Berufe (`engine/sim/unit_professions.go`), Upgrades (`engine/sim/building_upgrades.go`)
- **Wichtigste Werte:** `archer.damage`, `archer.cost.gold`, `warrior.hp`, `eliteArcher.damage`, `peasant.professions.miner.cost.gold`, `vagrant.recruitCost.gold`
- **Regeln:** `docs/rules/buerger.md`, `docs/rules/materialien-gebaeude.md`

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `*.hp` | Zahl | HP | Lebenspunkte | 30 bis 150 |
| `*.speed` | Zahl | Units/s | Lauftempo | 1,5 bis 3,5 |
| `*.damage` | Zahl | Schaden | Schaden je Treffer | 0 bis 30 |
| `*.range` | Zahl | Units | Angriffsreichweite (Bogen 10 bis 12) | 0 bis 12 |
| `*.attacksPerSecond` | Zahl | 1/s | Angriffe je Sekunde | 1 bis 1,2 |
| `*.upgradeFrom` | Text | – | Vorstufe, aus der die Truppe entsteht | peasant, archer, warrior |
| `*.cost.gold` | Zahl | Gold | Goldkosten des Upgrades | 20 bis 50 |
| `*.cost.wood` | Zahl | Holz | Holzkosten (Bogenschütze, Krieger) | 50 |
| `*.cost.stone` | Zahl | Stein | Steinkosten (Elite-Bogenschütze) | 100 |
| `*.cost.copper` | Zahl | Kupfer | Kupferkosten (Elite-Krieger) | 100 |
| `vagrant.recruitCost.gold` | Zahl | Gold | Preis für die Anwerbung zum Bauern | 1 |
| `warrior.postUnits` | Zahl | Units | Abstand des Postens vor der äußersten Sperre | 0,5 |
| `peasant.professions.*.cost.gold` | Zahl | Gold | Ausbildungspreis des Berufs | 20 bis 30 |
| `peasant.professions.*.workFactor` | Zahl | Faktor | Arbeitstempo des Berufs | 1,5 |
| `peasant.professions.*.resources` | Liste | – | Rohstoffe, die der Bergmann abbaut | stone bis crystal |
| `peasant.professions.craftsman.maxPerBuilding` | Ganzzahl | Handwerker | Höchstzahl je Gebäude | 2 |

## Wirkung auf das Balancing

Kämpferwerte bestimmen, wie gut die Burg die Wellen hält; Elite-Werte gelten ab Schmiede und Hub-Stufe 3. Höhere Kosten bremsen das Truppenwachstum, ein höheres `workFactor` beschleunigt Sammeln und Bauen. Betroffen: `castleHeld` und `destroyedPerWave` (`docs/rules/zielkorridore.md` § 1, § 3).

## Beispiele für Varianten

- `troops.json:archer.damage=18` – stärkere Bogenschützen.
- `troops.json:archer.cost.gold=25` – teurerer Bogenschütze.
- `troops.json:warrior.hp=120` – zähere Krieger.
- `troops.json:peasant.professions.miner.workFactor=2` – schnellere Bergleute.
