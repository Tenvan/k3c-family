# monarch.json

- **Zweck:** Monarch: Basiswerte, Angriff, Wiederbelebung, Skill-Punkte, Skills mit Wirkung und Linien-Presets.
- **Gelesen von:** Simulation (`engine/sim/monarch_data.go`, `engine/sim/monarch_passives.go`), Raum (`engine/room/skills.go`)
- **Wichtigste Werte:** `hitCoins`, `base.speed`, `attack.damage`, `tierPoints`, `skills.fireball.cooldown`
- **Regeln:** `docs/rules/monarch.md`

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `hitCoins` | Zahl | Münzen | Gold, das jeder Treffer kostet (kein HP mehr, KS3) | 1 |
| `base.damage` | Zahl | Schaden | Basis-Schaden | 10 |
| `base.speed` | Zahl | Units/s | Basis-Tempo | 5 |
| `sprintMultiplier` | Zahl | Faktor | Tempo-Faktor beim Sprinten | 1,8 |
| `acceleration` | Zahl | Units/s² | Beschleunigung | 8 |
| `mount.speedFactor` | Zahl | Faktor | Tempo-Faktor des Reittiers | 1,0 |
| `attack.damage` | Zahl | Schaden | Nahkampf-Schaden | 10 |
| `attack.range` | Zahl | Units | Nahkampf-Reichweite | 1,5 |
| `attack.cooldown` | Zahl | s | Abklingzeit des Angriffs | 0,7 |
| `tierPoints` | Liste | Punkte | Skill-Punkte je Stufe (Tier) bis zur Freischaltung | [0, 2, 4, 6] |
| `skillPointSources.*` | Ganzzahl | Punkte | Skill-Punkte je Quelle (hidden, miniboss, endboss, milestone, chestEvery) | 1 bis 3 |
| `skills.*.tier` | Ganzzahl | Tier | Stufe des Skills in seiner Linie | 1 bis 4 |
| `skills.*.cooldown` | Zahl | s | Abklingzeit eines aktiven Skills | 8 bis 180 |
| `skills.*.effect.damage` | Zahl | Schaden | Schaden aktiver Skills (Fireball, Meteor, Backstab) | 30 bis 200 |
| `skills.*.effect.amount` | Zahl | HP | Heilmenge der Heiler-Skills | 30 bis 50 |
| `skills.*.effect.radius` | Zahl | Units | Wirkradius des Skills | 3 bis 15 |
| `skills.*.effect.duration` | Zahl | s | Wirkdauer des Skills | 2 bis 10 |
| `presets.*.hp` | Zahl | HP | Start-HP des Linien-Presets | 90 bis 200 |
| `presets.*.damage` | Zahl | Schaden | Start-Schaden des Presets | 8 bis 12 |
| `presets.*.speed` | Zahl | Units/s | Start-Tempo des Presets | 5 bis 5,5 |
| `presets.*.defense` | Zahl | Punkte | Start-Rüstung des Presets | 4,5 bis 10 |

## Wirkung auf das Balancing

Gold je Treffer, Schaden und Skill-Abklingzeiten stärken oder schwächen die Verteidigung, besonders in den ersten Nächten (Korridor `castleHeld`). Passive sind laut Datei vorläufig (`provisional: true`). Mehr Skill-Punkte beschleunigen Freischaltungen. Siehe `docs/rules/zielkorridore.md` § 1.

## Beispiele für Varianten

- `monarch.json:hitCoins=2` – Treffer kosten doppelt so viel Gold.
- `monarch.json:base.speed=6` – schnellerer Monarch.
- `monarch.json:skills.fireball.cooldown=6` – Fireball häufiger.
- `monarch.json:attack.damage=12` – stärkerer Nahkampf.
