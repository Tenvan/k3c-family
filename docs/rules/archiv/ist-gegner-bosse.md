# Ist-Stand Gegner, Wellen, Bosse und Events (Vorbereitung R4.2 und R4.3)

Stand: 2026-10-02 (Session R4.1). Vergleich von `docs/game-design.md` mit dem Go-Code (`engine/sim/enemies.go`, `waves.go`, `cycle.go`) und `data/enemies.json`, `waves.json`, `biomes/*.json`.
Nur Tatsachen aus Code und Daten; was nicht geprüft wurde, steht als **ungeprüft**. Dieses Dokument beschließt nichts und ändert keinen Wert.
Beschlossene Rahmenregeln (hier nicht verhandelbar): Wellengröße × (1 + 0,5 je Zusatzspieler der Insel); fünf Schwierigkeitsgrade ändern Wellen und Gegner (Faktoren für Wellengröße, HP, Schaden); Gegnerskalierung je Insel mit eigener Tabelle; je Stufe ein Miniboss (1 Skill-Punkt), je Insel ein Endboss (3 Skill-Punkte); Vollmond-Wolf nur als Event-Idee; Insel 1 hat fünf Stufen (Wald, Höhle, Mine, Eisenstollen, Kristallhöhle); Drops je Gegnerart (`wirtschaft.md` § 1); Truppen siehe `buerger.md`.

## 1. Gegner

`data/enemies.json` (Angriffsrate bei allen Gegnern **1 pro Sekunde**, `waves.json` › `attacksPerSecond`):

| Gegner | Stufe | Rang | HP | Schaden | Tempo | Reichweite | Gold | Traits |
|---|---|---|---|---|---|---|---|---|
| Greed | Wald | Standard | 50 | 10 | 3 | 1 | 5–10 | stealsGold |
| Wolf | Wald (nachts) | Standard | 40 | 15 | 5 | 1 | 3–5 | prefersTroops, pack |
| Goblin | Wald | Standard | 60 | 12 | 4 | 1 | 8–12 | prefersBuildings, stealsWood, fleesAtHalfHp |
| Goblin Archer | Wald | Elite | 50 | 20 | 3 | 10 | 15–20 | ranged, prefersTowers |
| Skelett | Höhle (nachts) | Standard | 70 | 15 | 3 | 1 | 10–15 | – |
| Fledermaus | Höhle | Standard | 30 | 8 | 7 | 1 | 3–5 | flying, ignoresWalls, prefersMonarch |
| Höhlentroll | Höhle | Elite | 200 | 30 | 2 | 2 | 50–80 | aoe, prefersBuildings |
| Zombie | Mine | Standard | 100 | 20 | 2 | 1 | 15–25 | – |
| Rattenschwarm | Mine | Standard | 20 | 5 | 6 | 1 | 1–2 | swarm |
| Minengeist | Mine | Elite | 80 | 25 | 4 | 1 | 30–50 | ignoresWalls, prefersMonarch, phases |

**Eisenstollen und Kristallhöhle haben keine Gegner.** Das GDD nennt für Tiefe 3 und 4 nur Rohstoffe und Lava, keine Gegner (`game-design.md` › „Welt & Stufen“).

### 1.1 Traits: was der Code umsetzt

| Trait | Code (`enemies.go`) | Wirkung |
|---|---|---|
| `stealsGold` | ja | Kontakt mit einem Spieler: stiehlt 5 Gold (`waves.stealGold`), flieht zum Portal, das geklaute Gold fällt beim Tod aus |
| `fleesAtHalfHp` | ja | flieht bei halber HP |
| `ignoresWalls` | ja | läuft an Mauern vorbei |
| `ranged` | ja | Fernangriff mit Geschoss (Tempo 20), greift Ziele hinter der Mauer an |
| `prefersBuildings`, `prefersTowers`, `prefersTroops`, `prefersMonarch` | ja | wählt bevorzugte Ziele zuerst, sonst das nächste |
| `flying`, `pack`, `stealsWood`, `aoe`, `swarm`, `phases` | **nein** (nur Daten) | keine Wirkung; `flying` ist über `ignoresWalls` abgedeckt |

- **Zustände:** Move, Attack, Flee; kein Kiting, keine Spezialangriffe (GDD nennt Kiting und AoE für Elite). **Ziele:** Spieler, Truppen (außer Landstreicher), Mauer, Türme (nur Fernkämpfer), Burg. **Schaden an Spielern:** Verteidigung 5 mindert (`applyDamage`).
- Gegner laufen von den Portalen (2 je Stufe, abwechselnd links/rechts) zur Burg; **bei Tagesanbruch fliehen alle** zurück zu den Portalen und verschwinden (nur Oberwelt, `cycle.go` › `sendEnemiesHome`); **unter Tage bleiben sie** bis zum Tod.
- Kein Ereignis beim Tod eines Gegners (`removeDeadEnemies` meldet nichts); es gibt keine Zähler für getötete Gegner.

## 2. Wellen

| Thema | `game-design.md` / Daten | Ist im Code | Abweichung? |
|---|---|---|---|
| Größe | Welle 1–5: 5–10 Standard; 6–10: 10–15 + 1–2 Elite; ab 11: 15–20 + 3–5 Elite | `waves.json` › `table` identisch; Zahl zufällig im Bereich (`engine/rng`) | nein |
| Wellenzähler | – | `World.Wave` je Welt (je Stufe) und je Stufe gezählt; speichert der Hub-Spielstand | **offen**: je Stufe oder je Insel (Frage W3) |
| Zusammensetzung | Gegner aus dem Pool der Stufe | Pool `portal` (+ `night` nachts); Standard und Elite nach `tier`; Auswahl gleichverteilt (`planWave`) | nein |
| Portale | – | 2 je Stufe, mindestens 150 Units von der Burg; Gegner verteilt abwechselnd (`i % len`) | GDD schweigt |
| Spreizung | – | über 20 s (`spawnSpreadSeconds`) | GDD schweigt |
| Skalierung | „pro Stufe HP +50 %, Schaden +30 %, Tempo +10 %“ | `depthScaling` potenziert je Tiefe | beschlossen: Insel-Tabelle statt Formel (`stufen.md`) |
| Rhythmus Oberwelt | eine Welle je Nacht | `startWave` beim Wechsel zur Nacht | nein |
| Unter Tage | Aggressionspool | +1 %/min, +5 % je Kill, +1 % je gesammelter Ressource; bei 100 % eine Welle, Reset | nein |
| Spieleranzahl, Schwierigkeitsgrad | beschlossen | **nicht im Code** (kein Faktor je Spieler, kein Grad) | als SIM-Ticket B-101 |
| Nacht-Gegner unter Tage | „Skelette nachts“ | `night`-Pool wird nur genutzt, wenn der Auslöser (Aggression 100 %) nachts liegt (`w.Cycle.Phase`) | **ungeprüft**: Skelette tauchen nur zufällig nachts auf |
| Mine | Zombie, Ratten, Minengeist | `night` leer | nein |

## 3. Bosse und Events

- **Bosse:** nichts im Code und nichts in den Daten. Das GDD kennt „Elite“ als stärkere Standardgegner (Höhlentroll, Goblin Archer, Minengeist), aber keinen Miniboss oder Endboss.
- **Events:** Das GDD nennt „Wölfe bei Vollmond“. Kein Mond im Code (`rg -i moon` ohne Treffer); Wölfe kommen nachts als Teil der Welle.
- **Belohnung beschlossen:** Miniboss 1 Skill-Punkt, Endboss 3 (`monarch.md` § 3); der Endboss macht den Weg zur nächsten Insel frei (`stufen.md`).
- **Bauplätze im Hub:** Gegner greifen Mauern, Türme (nur Fernkämpfer), die Burg und Spieler/Truppen an; Tor, Heilplatz, Taverne, Lager u. a. sind noch keine Ziele definiert (Frage G8).

## 4. Kennzahlen für Zielkorridore

Aus `ist-abgleich.md` § 6 verfügbar: Ereignisse `wave` (`count`), `destroyed`, `playerDown`, `goldStolen`, `castleFallen`, `World.Enemies` (Bestand), `World.Aggression`. **Fehlen** (Vorschlag für B-099): `enemyKilled` (Art, Ort) oder ein Zähler, Schaden an Burg und Gebäuden je Welle, Zeit bis zur ersten toten Welle, Bossereignisse (`bossSpawned`, `bossDefeated`), Zeit einer Welle von Spawn bis zum Ende.

| Beschlussgröße | Kennzahl für den Zielkorridor | Verfügbar? |
|---|---|---|
| Burg hält Nacht 1–5 | `castleFallen` | ja |
| Gegner je Welle | `wave` (`count`) | ja |
| Gegner besiegt je Welle | `enemyKilled` | nein |
| Schaden an Gebäuden je Welle | Zerstörungen (`destroyed`), HP-Verlust | Zerstörungen ja, HP-Verlust nein |
| Miniboss/Endboss besiegt bis Tag X | `bossDefeated` | nein |
| Zeit der Welle | Spawn bis letzter Gegner tot | nein |

## 5. Fragen für die Workshops

Je Frage mit Optionen und Empfehlung; 🧑 entscheidet einzeln.

**R4.2 Gegner und Wellen**

- **G1 · Pools für Eisenstollen und Kristallhöhle:** (a) eigene Gegnerarten (Eisenstollen: Feuer-/Metall-Wesen wie Lavaschleim, Eisengolem, Feuergeist als Elite; Kristallhöhle: Kristallspinne, Splitter-Golem, Kristallwächter als Elite), (b) bestehende Gegner stärker, nur umgefärbt, (c) später. Empfehlung (a) mit je 2 Standard und 1 Elite.
- **G2 · Traits ohne Code** (`pack`, `stealsWood`, `aoe`, `swarm`, `phases`, `flying`): umsetzen, ändern oder streichen? Empfehlung: `aoe` und `swarm` umsetzen (Höhlentroll, Rattenschwarm), `phases` für Minengeist und Bosse, `stealsWood` und `pack` streichen, `flying` als Teil von `ignoresWalls` lassen.
- **G3 · Elite-Fähigkeiten:** Kiting (Fernkampf hält Abstand), Flächenangriff, Spezialangriffe: welche je Elite? Empfehlung: ein Fähigkeitsmerkmal je Elite (Goblin Archer kitet, Höhlentroll Flächenschlag, Minengeist Phasen).
- **G4 · Angriffsrate:** heute 1/s für alle; Rate je Gegner in den Daten (`attacksPerSecond`)? Empfehlung: ja.
- **G5 · Wellenform:** Eine Welle je Nacht in der Oberwelt, eine Welle je 100 % Aggression unten, 2 Portale je Stufe: bleibt das, oder mehr Portale/zwei Wellen pro Nacht ab Tag N? Empfehlung: unverändert, Portale 3 ab Tiefe 3.
- **G6 · Wellenzähler und Skalierung:** Wellennummer je Stufe (heute) oder je Insel (alle Stufen teilen Tage/Wellen)? Empfehlung: je Stufe, damit jede Stufe ihre eigene Steigerung hat; Tagesbeginn global.
- **G7 · Tagesanbruch:** Gegner fliehen nur in der Oberwelt (heute). Sollen sie auch in Höhle und Mine bei Tagesanbruch fliehen, oder dort bis zum Tod bleiben? Empfehlung: dort bleiben (Aggressionspool ist das Tempo).
- **G8 · Ziele der Gegner:** Welche Gebäude sind Ziele (Tor, Heilplatz, Taverne, Lager, Schmiede, Rüstkammer, Werkstatt, Farm)? Blockiert das Tor wie die Mauer? Empfehlung: Tor wie Mauer, übrige Gebäude als Ziele nur für `prefersBuildings`.
- **G9 · Aggressionspool:** +1 %/min, +5 % je Kill, +1 % je gesammelter Ressource unverändert (und skaliert der Pool mit der Spieleranzahl)? Empfehlung: unverändert, Spieleranzahl über den Wellenfaktor.
- **G10 · Gegnerzahl-Skalierung:** Der Wellenfaktor je Spieler gilt für Standard- und Elite-Zahl gleich? Empfehlung: ja.

**R4.3 Bosse und Events**

- **B1 · Boss-Art:** Miniboss je Stufe als stärkere Variante eines Elite (Aufbau aus vorhandenen Gegnern) oder eigene Figur je Stufe? Endboss immer eigene Figur? Empfehlung: Miniboss eigene Figur je Stufe (Thema der Stufe), Endboss eigene Figur mit Phasen.
- **B2 · Auslöser:** (a) kommt mit einer bestimmten Welle (z. B. Miniboss in Welle 5), (b) liegt im Level in seinem Bau und wird ausgelöst, wenn ein Spieler dort ankommt, (c) wird beschworen (z. B. am Portal mit Material). Empfehlung (a) für Minibosse, (b) für den Endboss.
- **B3 · Werte:** Miniboss ungefähr 8-fache HP eines Standardgegners der Stufe und doppelter Schaden, Endboss ungefähr 30-fache HP, drei Phasen? Empfehlung: ja als Startwerte.
- **B4 · Fähigkeiten:** je Boss 1 bis 3 Fähigkeiten (z. B. Beschwörung, Flächenschlag, Rückzug)? Empfehlung: Miniboss 1, Endboss 3.
- **B5 · Belohnung:** Skill-Punkte (1/3, beschlossen) plus Gold, Material oder eine einmalige Truhe? Empfehlung: Gold 100 (Miniboss) / 500 (Endboss) plus Material der Stufe.
- **B6 · Nach dem Boss:** Besiegter Boss kehrt nie zurück oder erscheint bei Spielstand-Neuanfang (Stufenverlust)? Empfehlung: kehrt nie zurück, der Sieg bleibt im Spielstand.
- **B7 · Endboss und Inselwechsel:** Boot/Portal erscheint nach dem Sieg; das Ziel „Endboss“ löst Sieg aus (beschlossen). Gibt es einen Zeitdruck (Boss verschwindet nach N Tagen) oder wartet er? Empfehlung: wartet.
- **B8 · Boss im Koop:** Skaliert der Boss mit der Spieleranzahl der Insel (HP × Faktor)? Empfehlung: ja, wie der Wellenfaktor.
- **E1 · Events:** Vollmond-Wolf (Auslöser z. B. jede 7. Nacht, verstärkte Wolfswelle, Belohnung Gold) und weitere (Blutmond, Händler-Überfall)? Empfehlung: Vollmond als einziges Start-Event, die anderen verschoben.

## 6. Gliederung für `gegner.md` und `bosse.md`

1. `gegner.md`: Pools je Stufe, Werte und Traits, Elite-Fähigkeiten, Wellen (Tabelle, Portale, Rhythmus, Wellenzähler, Aggressionspool), Ziele, Zielkorridore. 2. `bosse.md`: Minibosse, Endboss, Auslöser, Werte, Fähigkeiten, Belohnung, Koop, Events, Zielkorridore. 3. Offen und Annahmen.
