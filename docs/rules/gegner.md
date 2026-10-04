# Regelwerk: Gegner und Wellen

Beschlossen von 🧑 im Workshop R4.2 am 2026-10-02 (Grundlage: [`archiv/ist-gegner-bosse.md`](archiv/ist-gegner-bosse.md); Rahmen: [`wirtschaft.md`](wirtschaft.md), [`stufen.md`](stufen.md), [`monarch.md`](monarch.md), [`buerger.md`](buerger.md)).
Je Regel: **Regel · Begründung · Verweis auf `data/` · Zielkorridor**. Werte sind **Startwerte**, Feintuning mit dem Balancing-Tester (B-099).
Zielkorridore gelten im Standardszenario **Normal, Wald-Start, 2 Spieler, Bot „sparsam“, je 100 Seeds**. Jede Regel gilt für 2+ Spieler. Bosse und Events: R4.3 (`bosse.md`).

## 1. Gegner je Stufe (Insel 1)

Je Stufe 2 Standard-Gegner und 1 Elite (Wald hat 3 Standard, 1 Elite, Höhle 2 Standard, 1 Elite; Mine 2 Standard, 1 Elite). **Neu:** Eisenstollen und Kristallhöhle erhalten eigene Arten (Namen und Werte sind Startwerte, Sprites später).

| Stufe | Standard | Elite | Nacht |
|---|---|---|---|
| Wald | Greed, Goblin (Wolf nachts) | Goblin Archer | Wolf |
| Höhle | Skelett, Fledermaus | Höhlentroll | Skelett |
| Mine | Zombie, Rattenschwarm | Minengeist | – |
| **Eisenstollen** (neu) | Lavaschleim, Eisenkäfer | Feuergeist | – |
| **Kristallhöhle** (neu) | Kristallspinne, Splitterwicht | Kristallwächter | – |

Werte der **neuen** Gegner (Startwerte; bestehende Gegner wie in `data/enemies.json`):

| Gegner | HP | Schaden | Tempo | Reichweite | Gold | Fähigkeit |
|---|---|---|---|---|---|---|
| Lavaschleim | 120 | 25 | 3 | 1 | wie Zombie-Klasse | – |
| Eisenkäfer | 90 | 18 | 5 | 1 | wie Wolf-Klasse | – |
| Feuergeist (Elite) | 150 | 35 | 3 | 8 | Elite-Klasse | Fernkampf mit Flammen-Fläche |
| Kristallspinne | 110 | 28 | 5 | 1 | wie Zombie-Klasse | – |
| Splitterwicht | 60 | 15 | 6 | 1 | wie Ratte | `swarm` |
| Kristallwächter (Elite) | 350 | 45 | 2 | 2 | Elite-Klasse | Flächenschlag (Radius 3) |

Gold-Drops je Gegnerart wie in `wirtschaft.md` § 1 (Werte in `data/enemies.json`). Daten: `data/enemies.json` (SIM legt die neuen Arten an), `data/biomes/ironhold.json` und `crystal.json` (Pools).

## 2. Traits und Fähigkeiten

| Trait / Fähigkeit | Beschluss | Wirkung |
|---|---|---|
| `aoe` | **umsetzen** | Flächenschlag: Radius 3 Units, alle 4 s (Höhlentroll, Kristallwächter) |
| `swarm` | **umsetzen** | tritt in Gruppen auf: ein Schwarm-Platz in der Welle spawnt 4 Gegner (Rattenschwarm, Splitterwicht) |
| `phases` | **umsetzen** | Phasenwechsel: alle 8 s für 2 s unverwundbar, ignoriert Mauern (Minengeist; Bosse in R4.3) |
| **Kiting** | **umsetzen** für Fernkämpfer | hält 8 Units Abstand zu Spielern und Truppen (Goblin Archer, Feuergeist) |
| `pack`, `stealsWood` | **streichen** | aus den Daten entfernt |
| `flying` | bleibt Teil von `ignoresWalls` | keine eigene Wirkung |
| `stealsGold`, `fleesAtHalfHp`, `ignoresWalls`, `ranged`, `prefers…` | unverändert | wie heute |

- **Angriffsrate je Gegner** in den Daten (`attacksPerSecond`, Standard 1); Ausnahmen setzen die Startwerte der Daten.
- Begründung: Jeder Gegner hat höchstens eine bis zwei umgesetzte Eigenheiten; der Elite hat genau eine Fähigkeit.

## 3. Wellen

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Wellenform unverändert:** Oberwelt eine Welle je Nacht; unten eine Welle je 100 % Aggressionspool; **2 Portale je Stufe, ab Tiefe 3 drei**. | Bewährter Rhythmus, Tempo unten bestimmt der Spieler. | `waves.json`, `biomes/*.json` › `portals.count` | – |
| **Wellenzähler je Stufe**; Tag und Nacht bleiben global. | Jede Stufe hat ihre eigene Steigerung. | `World.Wave` je Stufe | – |
| Größen laut Tabelle (Welle 1–5: 5–10 Standard; 6–10: 10–15 + 1–2 Elite; ab 11: 15–20 + 3–5 Elite), dazu Wellenfaktor je Spieleranzahl der Insel (Standard- und Elite-Zahl gleich, gerundet), Faktoren des Schwierigkeitsgrads und die Insel-Tabelle. | Bestehende Tabelle als Basis. | `waves.json` › `table` | Gegner einer Welle besiegt bis Tagesanbruch (Oberwelt) in ≥ 90 % |
| **Tagesanbruch:** Gegner fliehen nur in der Oberwelt; unten bleiben sie bis zum Tod. | Unten bestimmen Spieler das Tempo. | `cycle.go` | – |
| **Aggressionspool unverändert:** +1 %/min, +5 % je Kill, +1 % je gesammelter Ressource; bei 100 % eine Welle, Reset. Die Spieleranzahl wirkt nur über den Wellenfaktor. | Einfach; Tempo bleibt beim Spieler. | `biomes/cave.json`, `mine.json` (und neue Biome) | Wellen unten im Abstand 4–12 min Spielzeit (Startziel) |
| Zielkorridor Welle: letzter Gegner einer Welle tot **innerhalb 3 min nach Spawn** in ≥ 80 %. | Wellen sollen nicht ziehen. | – | ≥ 80 % (Kennzahl fehlt, B-099) |

## 4. Ziele der Gegner

- Das **Tor blockiert wie die Mauer** (eigene Bürger und Spieler passieren).
- Alle übrigen Gebäude (Heilplatz, Taverne, Lager, Schmiede, Rüstkammer, Werkstatt, Farm) sind **Ziele** für Gegner mit `prefersBuildings`; andere Gegner greifen sie nur an, wenn nichts anderes in Reichweite ist.
- **Landstreicher sind kein Ziel** (Q67, 2026-10-04): Bürger sterben nicht, ein Treffer auf 0 HP stuft sie zurück (`buerger.md` § 3).
- **Aufheben** (Q67, Q68, 2026-10-04): Gegner heben am Boden liegende **Münzen** und **Ausrüstung** auf. Ausrüstung **tragen sie zum Portal**, dort ist sie verloren (Ereignis `equipmentTaken`, Q69); wird der Träger vorher getötet, fällt sie wieder zu Boden.
- Begründung: Wirtschaft muss geschützt werden, ohne dass jeder Gegner sie sucht. Zielkorridor: zerstörte Gebäude je Welle (1–5): Median höchstens 1 (`materialien-gebaeude.md` § 4).

## 5. Offen und Annahmen

- **Startwerte** der neuen Gegner, die Eigenheiten (Radius 3, alle 4 s; Schwarm 4; Phasen 8 s/2 s; Kiting-Abstand 8), Portale 3 ab Tiefe 3 und die Zielkorridore sind Vorschläge des Agenten (🧑 hat den Satz mit „Vorschlag“ bestätigt, einzelne Zahlen nicht gesondert).
- Namen und Aussehen der neuen Gegner sind vorläufig (Sprites: B-010).
- Kennzahlen, die fehlen: `enemyKilled`, Schaden an Gebäuden je Welle, Zeit einer Welle (B-099).
- Bosse und Events: R4.3 (`bosse.md`).
