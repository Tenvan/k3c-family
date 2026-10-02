# Regelwerk: Materialien, Hub-Ausbau und Gebäude

Beschlossen von 🧑 im Workshop R2.2 am 2026-10-02 (Grundlage: [`ist-material-gebaeude.md`](ist-material-gebaeude.md); Rahmen: [`wirtschaft.md`](wirtschaft.md), [`stufen.md`](stufen.md)).
Je Regel: **Regel · Begründung · Verweis auf `data/` · Zielkorridor**. Werte sind **Startwerte**, Feintuning mit dem Balancing-Tester (B-099, B-015).
Zielkorridore gelten im Standardszenario **Normal, Wald-Start, 2 Spieler, Bot „sparsam“, je 100 Seeds**. Jede Regel gilt für 2+ Spieler.

## 1. Materialien

Fünf Materialien, eines je Hub-Stufe. Das Baumaterial gehört der **Insel** (alle Stufen und Hubs teilen einen Vorrat, `stufen.md` § 1), begrenzt durch Burg und Lager; Gold gehört dem Spieler.

| Material | Hub-Stufe | Quelle | Verwendung |
|---|---|---|---|
| **Holz** | 1 | Wald (Bäume) | Grundgebäude, Holzmauer und -turm, Bogen und Schwert |
| **Stein** | 2 | Höhle (Felsen), auch Fels-Chunks im Wald | Steinmauer und -turm, Tor, Kaserne, Lager, Taverne, Treppen |
| **Kupfer** | 3 | Mine (Kupfererz) | Kupfermauer und -turm, Schmiede (Elite-Upgrades), Heilplatz |
| **Eisen** | 4 | Stufe 4 von Insel 1 (Eisenstollen, mit Lava) | Eisenmauer und -turm, Rüstkammer |
| **Kristall** | 5 | Stufe 5 von Insel 1 (Kristallhöhle) | Kristallmauer, Zaubertum |

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Insel 1 hat fünf Stufen** (Wald, Höhle, Mine, Eisenstollen, Kristallhöhle); die ersten drei werden zuerst gebaut, Eisenstollen und Kristallhöhle kommen später als Inhalt. Der Endboss liegt in der tiefsten Stufe. | Jede Tiefe erschließt das nächste Material. | `data/biomes/*.json`, `data/islands.json` | – |
| **Lager-Maximum je Rohstoff:** 300 je Hub (die Burg ist das Basislager) plus 300 je gebautem **Lager**; der Insel-Vorrat bleibt gemeinsam, seine Kapazität ist die Summe aller Hubs und Lager der Insel. **Nichts wächst nach:** Bäume, Felsen und Erz sind endlich. | Material wird planbar und knapp; Lager sind ein Ausbauziel. | `data/economy.json` › `storage` (SIM legt an), `World.stock` begrenzt | Anteil der Zeit, in der ein Rohstoff am Maximum steht: höchstens 20 % (Kennzahl fehlt, B-099) |
| **Arbeiter bringen gesammeltes Material zum nächsten Lager oder zur Burg ihres Hubs.** Ist das Maximum voll, **bleibt das Material liegen** (der Arbeiter wartet), nichts geht verloren. | Das Tragen ist sichtbar und kostet Zeit; Lager nahe an den Ressourcen helfen. | `engine/sim/units.go` (`carry`, SIM-Ticket) | – |
| Der **Busch** bleibt Dekoration ohne Funktion. | Kein Nutzen, keine Regel nötig. | `data/biomes/forest.json` | – |
| Nur Bauern sammeln, nur markierte Ressourcen (Markierung kostet 1–2 Gold), je Objekt eine Menge (10) in 4–8 s; Gegner lassen mit 10 % je 5 der Hauptressource der Stufe fallen. | Wie `wirtschaft.md` § 1. | `data/economy.json` | – |
| Die Siegvariante „alles abbauen“ (`stufen.md` § 3) ist erfüllbar, weil Rohstoffe endlich sind. | Folge von „kein Nachwachsen“. | – | – |

## 2. Hub-Ausbau (Hub-Stufen 1 bis 5)

Jeder Hub (jede Stufe einer Insel) hat eine **Ausbaustufe 1 bis 5**. Stufe n schaltet frei: Mauer- und Turm-Stufe n und die Gebäude der Stufe n (§ 3). Der Ausbau braucht **Gold plus das Material der neuen Stufe** aus dem Insel-Vorrat; Stufe 1 ist der Anfang.

| Ausbau auf Stufe | Kosten | Zielkorridor |
|---|---|---|
| 2 | 100 Stein + 50 Gold | erreicht vor Tag 5: 60–85 % |
| 3 | 150 Kupfer + 100 Gold | erreicht vor Tag 10: 40–70 % |
| 4 | 200 Eisen + 200 Gold | Korridor mit B-099 (Insel 1 Stufe 4 noch nicht gebaut) |
| 5 | 250 Kristall + 400 Gold | Korridor mit B-099 |

Begründung: Fortschritt über Tiefe und Material, kein reines Goldsparen. Daten: neu `data/hub.json` › `levels` (SIM legt an).

## 3. Gebäude je Hub-Stufe

Plätze sind **fest je Gebäude** wie heute (`data/hub.json` › `sites`, jeder Platz ein Gebäude, neue Plätze je Gebäude kommen in die Daten); es gibt kein Bau-Menü. Mauer und Turm werden **am selben Platz** auf die nächste Stufe ausgebaut.

| Hub-Stufe | Gebäude |
|---|---|
| 1 (Holz) | Burg, Holzmauer ×2, Holzturm ×2, Werkstatt, Farm |
| 2 (Stein) | Steinmauer, Steinturm (Ausbau), Tor, Kaserne, Lager, Taverne, Treppe hoch, Treppe runter |
| 3 (Kupfer) | Kupfermauer, Kupferturm (Ausbau), Schmiede, Heilplatz |
| 4 (Eisen) | Eisenmauer, Eisenturm (Ausbau), Rüstkammer |
| 5 (Kristall) | Kristallmauer, Zaubertum (Turm-Stufe 5) |

Zusatzgebäude: **Lager** (Stufe 2), Taverne (Stufe 2), Heilplatz (Stufe 3).

### 3.1 Mauern und Türme (Stufen 1 bis 5)

| Stufe | Material | Mauer: Kosten | Mauer: HP | Mauer: Bauzeit | Turm: HP |
|---|---|---|---|---|---|
| 1 | Holz | 20 Holz + 5 Gold | 300 | 6 s | 200 |
| 2 | Stein | 30 Stein + 10 Gold | 600 | 9 s | 400 |
| 3 | Kupfer | 40 Kupfer + 20 Gold | 1000 | 12 s | 700 |
| 4 | Eisen | 50 Eisen + 40 Gold | 1600 | 16 s | 1100 |
| 5 | Kristall | 60 Kristall + 80 Gold | 2500 | 20 s | 1700 |

Turm: Kosten etwa das 2,5-Fache der Mauer, gerundet (der bestehende Turm 50 Holz + 20 Gold bleibt Stufe 1); 2 Bogenplätze, +3 Reichweite. Stufe 5 ist der **Zaubertum**: Flächenschaden statt Bogen. Daten: `data/buildings.json` (SIM legt Stufen an).
Zielkorridor: Erste Mauer vor Ende Tag 1 in ≥ 90 %; erster Turm vor Ende Tag 2 in ≥ 70 %.

### 3.2 Wirkungen der übrigen Gebäude

| Gebäude | Wirkung | Kosten und HP |
|---|---|---|
| **Burg** | Hub-Kern; fällt sie, wirkt der Niederlage-Modus (`stufen.md` § 4) | HP 1000 |
| **Mauer** | blockiert Gegner (außer `ignoresWalls`) | siehe 3.1 |
| **Tor** | eigene Truppen und Spieler passieren, Gegner nicht | Startwerte wie heute (30 Holz + 10 Gold, 250 HP), ab Stufe 2 (Stein) |
| **Werkstatt** | Bogen und Schwert, je bis 3 im Waffenregal | wie heute (40 Holz + 15 Gold, 150 HP) |
| **Farm** | +5 Holz je Tagesanbruch für die Insel | wie heute (30 Holz + 10 Gold, 100 HP) |
| **Kaserne** | Truppen-Limit +10 (Basis 10, einfach gebaut) | wie heute (60 Stein + 30 Gold, 200 HP) |
| **Lager** | +300 Kapazität je Rohstoff für die Insel; Arbeiter bringen Material hierher oder zur Burg | Startwert: 50 Stein + 20 Gold, HP 200, Bauzeit 8 s |
| **Taverne** | 1 Landstreicher je Tag im Hub | Startwert: 60 Stein + 30 Gold, HP 150 |
| **Heilplatz** | heilt Truppen und Spieler in Reichweite | Startwert: 50 Kupfer + 30 Gold, HP 150 |
| **Schmiede** | Elite-Upgrades (Werte in Regelwerk III) | Startwert: 80 Kupfer + 50 Gold, HP 250 |
| **Rüstkammer** | Rüstung und Waffen-Upgrade für alle Truppen (Werte in Regelwerk III) | Startwert: 100 Eisen + 100 Gold, HP 350 |
| **Treppen** | Verbindung zur Stufe darüber/darunter, je 1 je Hub, ab Hub-Stufe 2 | 100 Stein + 50 Gold, HP 500, 20 s |

Die Startwerte für Taverne, Heilplatz, Schmiede und Rüstkammer sind **Vorschläge des Agenten** (🧑 hat die Wirkung beschlossen, nicht die Zahlen) und werden mit B-099 geprüft.

## 4. Bau-Ablauf, Zerstörung, Reparatur

| Regel | Begründung | Zielkorridor |
|---|---|---|
| Ablauf wie heute: Gold zahlen → Material wird automatisch aus dem Insel-Vorrat abgebucht → ein Bauer baut. Ausbau einer Mauer-Stufe läuft genauso am selben Platz. | Eine Taste, Material kommt aus dem gemeinsamen Vorrat. | Wartezeit „bezahlt bis gebaut“: Median ≤ 60 s (Kennzahl fehlt, B-099) |
| **Reparatur:** Bauern reparieren beschädigte Gebäude zwischen den Wellen **kostenlos** (Anteil der Bauzeit). | Beschädigte Mauern sollen sich erholen. | Zerstörte Gebäude je Welle 1–5: Median höchstens 1 |
| **Zerstörung:** Wird ein Gebäude zerstört, ist der Platz leer; **Gold und Material sind verloren**; bei Mauern und Türmen geht die Stufe verloren (neu ab Holzstufe), die Hub-Stufe bleibt. | Verlust tut weh, der Hub-Fortschritt nicht. | – |
| Wartet ein Bauplatz auf Bauer oder Material, wird das in der Welt angezeigt (CLI-Ticket). | Spieler sollen wissen, was fehlt. | – |
| **Schwierigkeitsgrade ändern Kosten und HP nicht** (`wirtschaft.md` § 4). | Wirtschaft bleibt in allen Graden gleich. | – |

## 5. Offen und Annahmen

- Die Zahlen für **Stufen 4 und 5** (Eisen, Kristall) und die Stufen selbst werden erst messbar, wenn Insel 1 sie enthält; ihre Zielkorridore folgen mit B-099.
- Startwerte für Taverne, Heilplatz, Schmiede, Rüstkammer (§ 3.2) und die Turm-Kosten (2,5-faches der Mauer) sind Vorschläge ohne gesonderte Bestätigung.
- Elite-Upgrades (Schmiede) und Rüstung/Waffen (Rüstkammer): Werte und Wirkung gehören zu Regelwerk III (`gegner-truppen.md`). Bürger-Fortschritt gehört zu B-110.
- Die Wirkung „Farm: +5 Holz je Tagesanbruch“ und „Taverne: 1 Landstreicher je Tag“ sind Startwerte (Wirtschaftsbalance, B-099).
- Annahme: Die Kapazität der Insel ist die Summe aus 300 je Hub und 300 je Lager (🧑 hat „Burg als Basislager, Lager erweitern“ gewählt, die Summenbildung über Hubs ist nicht gesondert bestätigt); Startwerte für das Lager (50 Stein + 20 Gold, 8 s) sind ein Vorschlag.
- Stufe 4 und 5 hängen an `stufen.md` § 1 (Insel 1 mit fünf Stufen; Endboss in der tiefsten): `stufen.md` und `game-design.md` sind angeglichen.
