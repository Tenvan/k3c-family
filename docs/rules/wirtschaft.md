# Regelwerk: Wirtschaft, Koop und Wellen

Beschlossen von 🧑 im Workshop R1.2 am 2026-10-02 (Grundlage: [`archiv/ist-abgleich.md`](archiv/ist-abgleich.md)).
Je Regel: **Regel · Begründung · Verweis auf `data/` · Zielkorridor**. Werte stehen in `data/*.json`, hier stehen Regeln und Ziele.
Zielkorridore prüft später der Balancing-Tester (B-099) mit dem Standardszenario **Wald, 2 Spieler, Bot „sparsam“, je 100 Seeds**, wenn nichts anderes steht.
Jede Regel gilt für 2+ Spieler (Couch und Online gemischt).

## 1. Gold und Material

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| Gold gehört jedem Spieler selbst, Baumaterial (Holz, Stein, Kupfer, Eisen, Kristall; `materialien-gebaeude.md` § 1) gehört allen gemeinsam, und zwar **je Insel** (alle Stufen und Hubs einer Insel teilen einen Vorrat, `stufen.md` § 1). | Wie in K2C: persönliche Münzen, gemeinsamer Hub. Im Playtest prüfen (B-008). | `World.stock`, `Player.Gold` | – |
| Jeder Spieler startet mit 14 Gold und trägt höchstens 30 (Münzskala, § 8). | Begrenzt Horten und hält das Geld in Bewegung; der volle Beutel bezahlt genau das teuerste reguläre Bauwerk. Gilt in **allen** Schwierigkeitsgraden (siehe 4). | `data/economy.json` › `purse` | Gold am Morgen: Median 5–20, nicht dauerhaft am Maximum (Normal) |
| Morgens (`dawn`, bei Beginn des Morgengrauens, § 3) bekommt jeder lebende Spieler 2 Gold. | Kleines Grundeinkommen, damit ein Spieler ohne Funde nicht festsitzt. Gilt in allen Graden. | `economy.json` › `dawnGoldPerPlayer` | im Korridor „Gold am Morgen“ |
| Münzen gibt man durch Halten von A (alle 0,25 s, Reichweite 2 Units); wer loslässt, bekommt die Münzen zurück. Fallen gelassene Münzen kann ein anderer Spieler nach 1,5 s aufheben. | Eine Taste für alles, Münzen wirken in der Welt. | `economy.json` › `payIntervalSeconds`, `payRangeUnits`, `pickupRangeUnits`, `dropPickupDelaySeconds` | – |
| Quellen von Gold: Truhen (3–6), Drops der Gegner, Tageseinkommen. Die Drops gelten **je Gegnerart** wie in `enemies.json`; die Werte im Spiel-Design sind nur Spannen (Standard etwa 1–6, Elite etwa 4–12; Umrechnung § 8). | Feinere Abstimmung je Gegner. | `economy.json` › `chestGold`; `enemies.json` › `gold` | Summe der Drops je Welle im Verhältnis zum Gold am Morgen (Messgröße wird mit B-099 festgelegt) |
| Gegner lassen mit 10 % Wahrscheinlichkeit 5 der Stufen-Ressource fallen. | Material kommt auch aus dem Kampf. | `economy.json` › `enemyResourceDrop` | – |
| Landstreicher werden mit einer Münze zu Bauern; ein Camp hat höchstens 2 Landstreicher und füllt nach 25 s auf. Bauern sammeln und bauen. | Rekrutierung bleibt eine Münze. | `economy.json` › `recruitCamp`; `data/troops.json` | – |
| Sammeln: Baum 10 Holz in 4 s, Fels 10 Stein in 6 s, Kupfererz 10 Kupfer in 8 s; das Markieren kostet 1–2 Münzen. | Bauern arbeiten nur auf Befehl und kosten damit Gold. | `economy.json` › `gatherables` | – |

## 2. Koop und Besitz

- **Wellen wachsen mit der Spieleranzahl.** Die Gegnerzahl einer Welle wird mit `1 + 0,25 × (Spieler − 1)` multipliziert und auf ganze Zahlen gerundet. Begründung: Mehr Spieler bringen mehr Gold, Bürger und Schlagkraft; die Nacht soll mit 4 Spielern nicht trivial werden. Daten: `data/waves.json` (neuer Faktor `perExtraPlayer: 0,5`, SIM legt das Feld an). Zielkorridor: dieselbe Überlebensquote wie in Abschnitt 4, **je Spieleranzahl 1–4**.
- Die Stufen einer Insel laufen alle weiter und sind pro Spieler frei begehbar; die Wellenstärke zählt die Spieler der **Insel** (siehe `stufen.md`). Was zählt als Spieler? Annahme (🧑 hat nicht gesondert bestätigt): Jeder gesteuerte, nicht freie Monarch im Raum, egal ob lokal oder online; ein freier Monarch (B-059) zählt nicht.
- Bauplätze und Bürger gehören dem Hub, das Material der Insel, nicht einem Spieler.

## 3. Tag, Nacht und Wellen

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Vier Phasen:** Tag, Dämmerung, Nacht, Morgengrauen im Verhältnis 3:1:2:1, Einheit 2 min → **Tag 6, Dämmerung 2, Nacht 4, Morgengrauen 2 min** (14 min; Startwert, BR1 bestätigt; Q65, 2026-10-04). `dawn` (Gold je Spieler, Taverne, Händler) fällt bei **Beginn des Morgengrauens**: Die Nacht endet, Gegner ziehen ab, die Nacht-Events enden; der Übergang vom Morgengrauen zum Tag löst nichts aus. Der Tageszähler springt mit `dawn`, das Morgengrauen gehört schon zum neuen Tag. Die Welle startet mit Beginn der Nacht. Der globale Zyklus läuft auch unter Tage weiter. Speichern bei Tagesanbruch gibt es nicht (`docs/protocol.md` › Spielstand, B-186). | Tempo wie K2C; das Morgengrauen trennt das Ende der Nacht vom neuen Tag. | `data/biomes/forest.json` › `cycle` (`dayMinutes`, `twilightMinutes`, `nightMinutes`, `dawnMinutes`; fehlt eine Dauer oder ist sie nicht positiv, lehnt das Laden ab), `engine/sim/world_cycle.go` | – |
| Oberwelt: eine Welle je Nacht. Unter Tage kommt eine Welle, wenn der Aggressionspool 100 % erreicht (Details in `stufen.md`). | Spieler bestimmen unten das Tempo selbst. | `waves.json`, `biomes/cave.json`, `mine.json` | – |
| Wellengrößen laut Tabelle: Welle 1–5 5–10 Standard, ab 6 10–15 Standard + 1–2 Elite, ab 11 15–20 + 3–5 Elite (Normal, 1 Spieler). | Bisherige Tabelle bleibt Basis. | `waves.json` › `table` | Burg hält Nacht 1–5, siehe 4 |
| Gegnerwerte skalieren je Tiefe multiplikativ (HP ×1,5, Schaden ×1,3, Tempo ×1,1 je Tiefe). | Heutige Umsetzung (`Pow`). | `waves.json` › `depthScaling` | – |
| Greed stiehlt 2 Gold beim Kontakt mit einem Spieler. | Gold hat Risiko. | `waves.json` › `stealGold` | – |
| Vorbereitung auf die erste Nacht: Beide Mauern stehen, bevor Tag 1 endet. | Die erste Nacht muss mit Mauern zu schaffen sein (`hub.json` › `startTroops`). | `hub.json`, `buildings.json` | Erste Mauer steht vor Ende Tag 1 in ≥ 90 % der Seeds (Normal) |
| Truppen behalten in Wellen mehrheitlich ihre Ausrüstung (Verlust = Rückstufung zum Bauern, kein Tod; Q67, 2026-10-04). | Verlust soll weh tun, aber nicht alles kosten. | `troops.json` | Verluste je Welle: Median höchstens die Hälfte der Truppen (Normal); Messgröße mit B-099 |

## 4. Schwierigkeitsgrade (neu)

Fünf Grade: **Dev, Leicht, Normal, Hart, Ultra**. Sie ändern **nur Wellen und Gegner**; die Wirtschaft (Startgold, Beutel, Tageseinkommen, Kosten) ist in allen Graden gleich.

| Grad | Wellengröße | Gegner-HP | Gegner-Schaden | Zielkorridor: Burg hält Nacht 1–5 (2 Spieler, Bot „sparsam“) |
|---|---|---|---|---|
| Dev | ×0,5 | ×0,6 | ×0,5 | ≥ 99 % |
| Leicht | ×0,75 | ×0,8 | ×0,75 | ≥ 90 % |
| **Normal** | ×1 | ×1 | ×1 | **75–90 %** |
| Hart | ×1,25 | ×1,3 | ×1,25 | 50–70 % |
| Ultra | ×1,6 | ×1,6 | ×1,5 | 25–45 % |

Die Faktoren sind **Startwerte** (Basis: heutige Werte = Normal); der Balancing-Tester prüft sie, danach ändert REG nur noch Zahlen. Die Faktoren wirken zusätzlich zur Skalierung mit der Spieleranzahl (Abschnitt 2) und je Tiefe.

- **Live (Familie):** Der Grad wird beim **Anlegen des Raums** gewählt (Leicht bis Ultra, Standard Normal). **Dev** ist in Live nicht wählbar.
- **Dev-Mode (Entwicklungsphase):** Standardgrad **Dev**; alle fünf Grade lassen sich im **Debug-Panel** jederzeit umschalten. Der Wechsel wirkt ab der **nächsten Welle**, nie rückwirkend auf Gegner, die schon laufen oder warten.
- Der Grad gehört zum Raum und steht im Spielstand.
- Der Grad ist eine von drei **Raum-Optionen** (Grad, Ziel, Niederlage-Modus; siehe `stufen.md` § 5).
- Das Debug-Panel mit Aktionen ist neu (heute gibt es nur das lesende Overlay, B-093); die Dev-Aktionen laufen als B-080 über den Server.

## 5. Truppen und Gebäude: beschlossen, später gebaut

Gebäude und Material sind in `materialien-gebaeude.md` beschlossen (R2), Bürger, Berufe und Truppen-Limit in `buerger.md` (R3). Krieger, Elite-Upgrades, Tor, Baumschule, Kaserne und das Truppen-Limit bleiben im Regelwerk und in `data/`, sind aber im Code noch nicht umgesetzt. Bis dahin gilt: kein Truppen-Limit, kein Tor, keine Baumschule, keine Kaserne im Hub. Die Werte werden mit Regelwerk III (`gegner-truppen.md`) festgelegt, die Umsetzung kommt als SIM-Ticket in einem späteren Sprint.

## 6. Steuerung

- **Taste X (Controller) bzw. E (Tastatur) ist der Schlag des Monarchen** (R3.2, `monarch.md` § 4; ersetzt „bleibt frei für Skills“). Skill-Slots: LB, RB, LT, D-Pad hoch bzw. Q, R, T, Z; Skill-Menü D-Pad runter bzw. K.
- B bleibt unbelegt, View + Menu gemeinsam sind reserviert (`CLAUDE.md`).
- Das Skill-Menü liegt nicht auf View (`stufen.md` § 6). Das Debug-Panel und der Gradwechsel gehören nicht auf X; die Belegung steht mit dem Ticket für das Panel fest (nur Dev-Mode).

## 7. Offen / Annahmen

- **Gold-Beutel, Startgold und Tageseinkommen** folgen seit BAS der Münzskala (§ 8) und sind in allen Graden gleich; es sind Startwerte für Meilenstein 1 (BS3).
- Zielkorridore für Verluste und Wirtschaftsfluss gelten als Startziele; die Kennzahlen dafür fehlen noch (siehe `archiv/ist-abgleich.md` › Messgrößen) und kommen mit B-099.
- **Vollmond und weitere Events:** beschlossen in `bosse.md` § 2.
- Das Standardszenario (Wald, 2 Spieler, Bot „sparsam“) gilt für alle Korridore; Korridore für Höhle und Mine folgen mit `stufen.md` (R1.3).

## 8. Münzskala (BAS, B-635)

Beschlossen von 🧑 am 2026-10-26 (Chat, Projekt BAS, Defaults aus B-635). Goldpreise liegen auf einer kleinen, K2C-artigen Skala:
Man zählt Münzen, statt Beträge zu lesen.

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Skala:** Reguläre Goldpreise haben nur die Stufen **2, 3, 4, 6, 8, 10, 12, 15, 20, 25, 30**. Regulär sind Bauwerke (Mauer, Turm, Tor, Grundgebäude), ihre Ausbaustufen, die Ausbauschritte der Hub-Stufen und die Ausrüstung der Truppen. | Wenige, gut zählbare Beträge; der Abstand der Stufen wächst mit dem Preis. | `buildings.json` › `cost.gold`, `levels[].cost.gold`; `hub.json` › `levels[].steps[]`; `troops.json` | – |
| **Grenzen:** Die günstigste Mauer (Stufe 1) kostet **2**, das teuerste reguläre Bauwerk bzw. die höchste Ausbaustufe **30**. | Wunsch 🧑: Mauern fangen mit 2 Münzen an, die größten Bauwerke liegen bei 30. | wie oben | – |
| **Umrechnung** der alten Werte: neu = die nächstgelegene Stufe der Skala zu `2 · (alt / 5)^0,66`, bei Gleichstand die höhere. Beispiele: 5 → 2, 10 → 3, 20 → 4, 50 → 10, 100 → 15, 200 → 25, 300 → 30. Gleich teure Dinge bleiben gleich teuer, kein teureres wird billiger als ein bisher billigeres. | Eine Kurve statt Einzelentscheidungen; sie erhält die Reihenfolge und staucht die großen Beträge stärker als die kleinen. | – | – |
| **Einnahmen** folgen derselben Kurve, gerundet auf ganze Münzen, mindestens 1: Startgold 14, Tageseinkommen 2, Truhen 3–6, Drops Standard etwa 1–6 und Elite etwa 4–12, Greed stiehlt 2. Ausnahme ist der Beutel: höchstens 30. | Ausgaben und Einnahmen schrumpfen im selben Verhältnis, Tempo und Spannung bleiben. Der volle Beutel (30) bezahlt genau das teuerste reguläre Bauwerk. | `economy.json` › `purse`, `dawnGoldPerPlayer`, `chestGold`; `enemies.json` › `gold`; `waves.json` › `stealGold` | Gold am Morgen: Median 5–20 (§ 1) |
| **Kleinbeträge** außerhalb der Skala: Markieren (1–2 Münzen, `markCost`) und Rekrutieren (1 Münze) bleiben unverändert. | Weniger als eine Münze geht nicht; diese Beträge waren schon klein. | `economy.json` › `gatherables`, `veins`, `recruitCamp` | – |
| **Ausnahme-Preise:** Geplante und manuelle Chunks, Events, Bosse (Beute), Ruinen und Spielziele dürfen außerhalb der Skala liegen und bekommen später ein eigenes Balancing. Bis dahin werden sie mit derselben Kurve umgerechnet und in `notes` als „vorläufig (Ausnahme, B-635)“ markiert. | Sonderfälle sollen sich anders anfühlen dürfen; ihr Balancing kommt, wenn wir damit anfangen. | `bosses.json`, `events.json`, `ruins.json`, `goals.json`, `data/worlds/*.json` › `plannedChunks` | – |
| **Morgengold nach Tiefe** (B-645, Beschluss 🧑 2026-10-27, Option b): Das Tageseinkommen gilt für den Wald (Tiefe 0). Jede tiefere Stufe zahlt je Tiefe **6 Münzen** mehr (Höhle 8, Mine 14, Eisenstollen 20, Kristallhöhle 26); der Beutel (30) begrenzt weiter. | Unter Tage räumt jede Welle Mauern, Türme und Bauern ab, und nur der Morgen bringt Gold zum Wiederaufbau. Gemessen im Stufen-Test ohne Hilfen (Solo, Seed `k3c-dev`): Zuschlag 3–4 reicht nicht (Troll-König fällt nie), 6 und 8 bringen drei Minibosse bis Tag 20. | `economy.json` › `dawnGoldPerDepth` | – |
| **Schiff** (B-637, B-638): 3 Abschnitte, je Münzen auf der Skala plus Material der Insel — **Kiel** 10 Münzen + 80 Holz, **Rumpf** 20 Münzen + 60 Stein + 40 Kupfer, **Segel** 30 Münzen + 40 Eisen + 20 Kristall. Ein Abschnitt wird wie ein Gebäude bezahlt und von einem Bauern gebaut. Startwerte, BS3 übernimmt sie in `data/`, BS9 stimmt sie ab. | Das Schiff ist das teuerste Bauwerk der Insel und braucht Material aller Stufen; so zieht es durch die ganze Insel. | Schiffskosten in `data/` (BS3) | Schiff fertig: `zielkorridore.md` § 6 |
| **Prüfung:** Ein Daten-Test lehnt reguläre Goldpreise außerhalb der Skala ab und kennt die Ausnahmen als ausdrückliche Liste (BS3.1). | Die Skala bleibt beim späteren Feinschliff erhalten. | `engine/sim/*_data_test.go` | – |

Die konkreten Werte setzt BS3 in `data/`; bis dahin stehen dort noch die alten Beträge.
