# Regelwerk: Wirtschaft, Koop und Wellen

Beschlossen von 🧑 im Workshop R1.2 am 2026-10-02 (Grundlage: [`archiv/ist-abgleich.md`](archiv/ist-abgleich.md)).
Je Regel: **Regel · Begründung · Verweis auf `data/` · Zielkorridor**. Werte stehen in `data/*.json`, hier stehen Regeln und Ziele.
Zielkorridore prüft später der Balancing-Tester (B-099) mit dem Standardszenario **Wald, 2 Spieler, Bot „sparsam“, je 100 Seeds**, wenn nichts anderes steht.
Jede Regel gilt für 2+ Spieler (Couch und Online gemischt).

## 1. Gold und Material

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| Gold gehört jedem Spieler selbst, Baumaterial (Holz, Stein, Kupfer) gehört allen gemeinsam, und zwar **je Insel** (alle Stufen und Hubs einer Insel teilen einen Vorrat, `stufen.md` § 1). | Wie in K2C: persönliche Münzen, gemeinsamer Hub. Im Playtest prüfen (B-008). | `World.stock`, `Player.Gold` | – |
| Jeder Spieler startet mit 100 Gold und trägt höchstens 100. | Begrenzt Horten und hält das Geld in Bewegung. Gilt in **allen** Schwierigkeitsgraden (siehe 4). | `data/economy.json` › `purse` | Gold am Morgen: Median 20–70, nicht dauerhaft am Maximum (Normal) |
| Morgens bekommt jeder lebende Spieler 5 Gold. | Kleines Grundeinkommen, damit ein Spieler ohne Funde nicht festsitzt. Heutiger Wert, gilt in allen Graden. | `economy.json` › `dawnGoldPerPlayer` | im Korridor „Gold am Morgen“ |
| Münzen gibt man durch Halten von A (alle 0,25 s, Reichweite 2 Units); wer loslässt, bekommt die Münzen zurück. Fallen gelassene Münzen kann ein anderer Spieler nach 1,5 s aufheben. | Eine Taste für alles, Münzen wirken in der Welt. | `economy.json` › `payIntervalSeconds`, `payRangeUnits`, `pickupRangeUnits`, `dropPickupDelaySeconds` | – |
| Quellen von Gold: Truhen (10–25), Drops der Gegner, Tageseinkommen. Die Drops gelten **je Gegnerart** wie in `enemies.json`; die Werte im Spiel-Design sind nur Spannen (Standard etwa 3–25, Elite etwa 15–80). | Feinere Abstimmung je Gegner. | `economy.json` › `chestGold`; `enemies.json` › `gold` | Summe der Drops je Welle im Verhältnis zum Gold am Morgen (Messgröße wird mit B-099 festgelegt) |
| Gegner lassen mit 10 % Wahrscheinlichkeit 5 der Stufen-Ressource fallen. | Material kommt auch aus dem Kampf. | `economy.json` › `enemyResourceDrop` | – |
| Landstreicher werden mit einer Münze zu Bauern; ein Camp hat höchstens 2 Landstreicher und füllt nach 25 s auf. Bauern sammeln und bauen. | Rekrutierung bleibt eine Münze. | `economy.json` › `recruitCamp`; `data/troops.json` | – |
| Sammeln: Baum 10 Holz in 4 s, Fels 10 Stein in 6 s, Kupfererz 10 Kupfer in 8 s; das Markieren kostet 1–2 Münzen. | Bauern arbeiten nur auf Befehl und kosten damit Gold. | `economy.json` › `gatherables` | – |

## 2. Koop und Besitz

- **Wellen wachsen mit der Spieleranzahl.** Die Gegnerzahl einer Welle wird mit `1 + 0,5 × (Spieler − 1)` multipliziert und auf ganze Zahlen gerundet. Begründung: Mehr Spieler bringen mehr Gold, Truppen und Schlagkraft; die Nacht soll mit 4 Spielern nicht trivial werden. Daten: `data/waves.json` (neuer Faktor `perExtraPlayer: 0,5`, SIM legt das Feld an). Zielkorridor: dieselbe Überlebensquote wie in Abschnitt 4, **je Spieleranzahl 1–4**.
- Die Stufen einer Insel laufen alle weiter und sind pro Spieler frei begehbar; die Wellenstärke zählt die Spieler der **Insel** (siehe `stufen.md`). Was zählt als Spieler? Annahme (🧑 hat nicht gesondert bestätigt): Jeder gesteuerte, nicht freie Monarch im Raum, egal ob lokal oder online; ein freier Monarch (B-059) zählt nicht.
- Bauplätze und Truppen gehören dem Hub, das Material der Insel, nicht einem Spieler.

## 3. Tag, Nacht und Wellen

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| Wald: Tag 10 min, Dämmerung 1 min, Nacht 5 min; der globale Zyklus läuft auch unter Tage weiter. | Tempo wie K2C. | `data/biomes/forest.json` › `cycle` | – |
| Oberwelt: eine Welle je Nacht. Unter Tage kommt eine Welle, wenn der Aggressionspool 100 % erreicht (Details in `stufen.md`). | Spieler bestimmen unten das Tempo selbst. | `waves.json`, `biomes/cave.json`, `mine.json` | – |
| Wellengrößen laut Tabelle: Welle 1–5 5–10 Standard, ab 6 10–15 Standard + 1–2 Elite, ab 11 15–20 + 3–5 Elite (Normal, 1 Spieler). | Bisherige Tabelle bleibt Basis. | `waves.json` › `table` | Burg hält Nacht 1–5, siehe 4 |
| Gegnerwerte skalieren je Tiefe multiplikativ (HP ×1,5, Schaden ×1,3, Tempo ×1,1 je Tiefe). | Heutige Umsetzung (`Pow`). | `waves.json` › `depthScaling` | – |
| Greed stiehlt 5 Gold beim Kontakt mit einem Spieler. | Gold hat Risiko. | `waves.json` › `stealGold` | – |
| Vorbereitung auf die erste Nacht: Beide Mauern stehen, bevor Tag 1 endet. | Die erste Nacht muss mit Mauern zu schaffen sein (`hub.json` › `startTroops`). | `hub.json`, `buildings.json` | Erste Mauer steht vor Ende Tag 1 in ≥ 90 % der Seeds (Normal) |
| Truppen überleben Wellen mehrheitlich. | Verlust soll weh tun, aber nicht alles kosten. | `troops.json` | Verluste je Welle: Median höchstens die Hälfte der Truppen (Normal); Messgröße mit B-099 |

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

Gebäude und Material sind in `materialien-gebaeude.md` beschlossen (R2), Bürger, Berufe und Truppen-Limit in `buerger.md` (R3). Krieger, Elite-Upgrades, Tor, Farm, Kaserne und das Truppen-Limit bleiben im Regelwerk und in `data/`, sind aber im Code noch nicht umgesetzt. Bis dahin gilt: kein Truppen-Limit, kein Tor, keine Farm, keine Kaserne im Hub. Die Werte werden mit Regelwerk III (`gegner-truppen.md`) festgelegt, die Umsetzung kommt als SIM-Ticket in einem späteren Sprint.

## 6. Steuerung

- **Taste X (Controller) bzw. E (Tastatur) ist der Schlag des Monarchen** (R3.2, `monarch.md` § 4; ersetzt „bleibt frei für Skills“). Skill-Slots: LB, RB, LT, D-Pad hoch bzw. Q, R, T, Z; Skill-Menü D-Pad runter bzw. K.
- B bleibt unbelegt, View + Menu gemeinsam sind reserviert (`CLAUDE.md`).
- Das Skill-Menü liegt nicht auf View (`stufen.md` § 6). Das Debug-Panel und der Gradwechsel gehören nicht auf X; die Belegung steht mit dem Ticket für das Panel fest (nur Dev-Mode).

## 7. Offen / Annahmen

- **Gold-Beutel, Startgold und Tageseinkommen** bleiben bei den heutigen Werten; das folgt daraus, dass die Grade die Wirtschaft nicht ändern. 🧑 hat das nicht gesondert bestätigt und kann es mit den Messläufen von B-099 ändern.
- Zielkorridore für Verluste und Wirtschaftsfluss gelten als Startziele; die Kennzahlen dafür fehlen noch (siehe `archiv/ist-abgleich.md` › Messgrößen) und kommen mit B-099.
- **Vollmond und weitere Events:** beschlossen in `bosse.md` § 2.
- Das Standardszenario (Wald, 2 Spieler, Bot „sparsam“) gilt für alle Korridore; Korridore für Höhle und Mine folgen mit `stufen.md` (R1.3).
