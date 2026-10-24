# bosses.json

- **Zweck:** Minibosse je Stufe und Endboss mit Fähigkeiten, Phasen und Belohnung.
- **Gelesen von:** Simulation (`engine/sim/boss_data.go`, `engine/sim/events.go`)
- **Wichtigste Werte:** `bosses.0.hpFactor`, `bosses.0.damageFactor`, `bosses.0.wave`, `bosses.0.reward.gold`, `endBosses.0.hpFactor`, `endBosses.0.lair.triggerUnits`
- **Regeln:** `docs/rules/bosse.md`

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `bosses.*.id` | Text | – | Kennung des Minibosses | goblinLeader, trollKing, … |
| `bosses.*.depth` | Ganzzahl | Tiefe | Stufe, in der der Miniboss erscheint | 0 bis 4 |
| `bosses.*.wave` | Ganzzahl | Welle | Welle der Stufe, mit der er über das Portal kommt | 3 bis 5 |
| `bosses.*.base` | Text | – | Standardgegner als Bezug für HP und Schaden | goblin, skeleton, … |
| `bosses.*.hpFactor` | Zahl | Faktor | HP relativ zum Bezugsgegner | 8 |
| `bosses.*.damageFactor` | Zahl | Faktor | Schaden relativ zum Bezugsgegner | 2 |
| `bosses.*.speed` | Zahl | Units/s | Tempo vor Tiefen-Skalierung | 2 bis 3 |
| `bosses.*.range` | Zahl | Units | Angriffsreichweite | 1 bis 8 |
| `bosses.*.ability.type` | Text | – | Fähigkeit: summon, aoe, flameTrail, shardThrow | je Boss |
| `bosses.*.ability.intervalSeconds` | Zahl | s | Abstand der Fähigkeit | 1 bis 10 |
| `bosses.*.ability.radius` | Zahl | Units | Radius der Fähigkeit | 1,5 bis 3 |
| `bosses.*.ability.count` | Ganzzahl | Stück | Beschworene Gegner je Takt | 2 |
| `bosses.*.reward.gold` | Ganzzahl | Gold | Belohnung als Münzen | 100 |
| `bosses.*.reward.material` | Ganzzahl | Material | Belohnung in den Insel-Vorrat | 50 |
| `endBosses.*.hpFactor` | Zahl | Faktor | HP des Endbosses relativ zum Bezugsgegner | 30 |
| `endBosses.*.damageFactor` | Zahl | Faktor | Schaden relativ zum Bezugsgegner | 2 |
| `endBosses.*.lair.triggerUnits` | Zahl | Units | Abstand eines Spielers, der den Kampf auslöst | 15 |
| `endBosses.*.lair.noticeUnits` | Zahl | Units | Abstand, ab dem der Bau in der Aktionsliste steht | 40 |
| `endBosses.*.phases.*.belowHpPercent` | Zahl | % HP | Schwelle, ab der die Phase gilt | 60, 25 |
| `endBosses.*.phases.*.tempo` | Zahl | Faktor | Beschleunigung von Angriff, Fähigkeit und Lauftempo | 1,5 |
| `endBosses.*.reward.gold` | Ganzzahl | Gold | Belohnung des Endbosses | 500 |
| `endBosses.*.reward.material` | Ganzzahl | Material | Material-Belohnung | 250 |

## Wirkung auf das Balancing

Boss-HP und -Schaden (Faktor mal Bezugsgegner, dazu Tiefen- und Grad-Skalierung) bestimmen die Härte der Stufenwechsel; die Belohnungen speisen Wirtschaft und Skill-Punkte. Werte sind laut Datei vorläufig (B-099). Betroffen sind vor allem Zerstörungs- und Stufen-Korridore (`docs/rules/zielkorridore.md` § 3).

## Beispiele für Varianten

Nicht als Variante nutzbar: `sim.UseData` lädt diese Datei bei einer Variante nicht neu, der Balancing-Lauf nutzt immer den Stand der Datei. Zum Ausprobieren die Datei ändern und den Lauf mit dem alten Bericht vergleichen.
