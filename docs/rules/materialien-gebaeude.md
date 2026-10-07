# Regelwerk: Materialien, Hub-Ausbau und Gebäude

Beschlossen von 🧑 im Workshop R2.2 am 2026-10-02 (Grundlage: [`archiv/ist-material-gebaeude.md`](archiv/ist-material-gebaeude.md); Rahmen: [`wirtschaft.md`](wirtschaft.md), [`stufen.md`](stufen.md)).
Je Regel: **Regel · Begründung · Verweis auf `data/` · Zielkorridor**. Werte sind **Startwerte**, Feintuning mit dem Balancing-Tester (B-099, B-015).
Zielkorridore gelten im Standardszenario **Normal, Wald-Start, 2 Spieler, Bot „sparsam“, je 100 Seeds**. Jede Regel gilt für 2+ Spieler.

## 1. Materialien

Fünf Materialien, eines je Hub-Stufe. Das Baumaterial gehört der **Insel** (alle Stufen und Hubs teilen einen Vorrat, `stufen.md` § 1), begrenzt durch Burg und Lager; Gold gehört dem Spieler.

| Material | Hub-Stufe | Quelle | Verwendung |
|---|---|---|---|
| **Holz** | 1 | Wald (Bäume), **Farm-Plantage** (wächst nach) | Grundgebäude, Holzmauer und -turm, Bogen und Schwert |
| **Stein** | 2 | Höhle (**Adern**), endliche Felsen im Wald und in der Mine | Steinmauer und -turm, Tor, Kaserne, Lager, Taverne, Treppen |
| **Kupfer** | 3 | Mine (**Adern**), endliches Erz im Level | Kupfermauer und -turm, Schmiede (Elite-Upgrades), Heilplatz |
| **Eisen** | 4 | Eisenstollen (**Adern**) | Eisenmauer und -turm, Rüstkammer |
| **Kristall** | 5 | Kristallhöhle (**Adern**) | Kristallmauer, Zaubertum |

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Insel 1 hat fünf Stufen** (Wald, Höhle, Mine, Eisenstollen, Kristallhöhle); die ersten drei werden zuerst gebaut, Eisenstollen und Kristallhöhle kommen später als Inhalt. Der Endboss liegt in der tiefsten Stufe. | Jede Tiefe erschließt das nächste Material. | `data/biomes/*.json`, `data/islands.json` | – |
| **Lager-Maximum je Rohstoff:** 300 je Hub (die Burg ist das Basislager) plus 300 je gebautem **Lager**; der Insel-Vorrat bleibt gemeinsam, seine Kapazität ist die Summe aller Hubs und Lager der Insel.  | Material wird planbar und knapp; Lager sind ein Ausbauziel. | `data/economy.json` › `storage` (SIM legt an), `World.stock` begrenzt | Anteil der Zeit, in der ein Rohstoff am Maximum steht: höchstens 20 % (Kennzahl fehlt, B-099) |
| **Startvorrat:** Eine neue Insel startet mit **100 Holz** im Insel-Vorrat (Startwert, in allen Schwierigkeitsgraden gleich); Spielstände bringen ihren eigenen Vorrat mit. Alle Gebäude der Hub-Stufe 1 kosten weiter Holz plus Gold, es gibt keine reinen Gold-Gebäude. | Ohne Vorrat ist vor dem ersten Bau ein Holzfäller-Umweg nötig; 100 Holz bezahlen beide Mauern (40) und einen Turm (50). Beschlossen von 🧑 am 2026-10-03 (B-177). | `data/hub.json` › `islandStartStock` | Erste Mauer vor Ende Tag 1 in ≥ 90 % (wie § 3.1); Höhe mit B-099/B-155 prüfen |
| **Arbeiter bringen gesammeltes Material zum nächsten Lager oder zur Burg ihres Hubs.** Ist das Maximum voll, **bleibt das Material liegen** (der Arbeiter wartet), nichts geht verloren. | Das Tragen ist sichtbar und kostet Zeit; Lager nahe an den Ressourcen helfen. | `engine/sim/units.go` (`carry`, SIM-Ticket) | – |
| Der **Busch** bleibt Dekoration ohne Funktion. | Kein Nutzen, keine Regel nötig. | `data/biomes/forest.json` | – |
| Nur Bauern sammeln, nur markierte Ressourcen (Markierung kostet 1–2 Gold); ein Baum oder Fels liefert 10 Einheiten in 4–8 s und ist danach verbraucht; Gegner lassen mit 10 % je 5 der Hauptressource der Stufe fallen. **Adern** werden **einmal markiert**, die Markierung bleibt; Kosten = `markCost` des Materials: Stein 1, Kupfer 2, Eisen 2, Kristall 2 Gold (Eisen und Kristall Startwert). **Plantage-Bäume** brauchen keine Markierung (Q25, 2026-10-04). | Wie `wirtschaft.md` § 1: Level-Objekte, Truhen und Drops sind der **endliche Startvorrat**. Die Ader verschwindet nie; die gebaute Farm ist schon die Entscheidung. | `data/economy.json` › `gatherables`, `veins` | – |
| **Holz wächst über Farmen nach:** Jede Farm ist eine **Baumplantage** mit 6 Plätzen; je Platz wächst alle 30 s ein Baum (10 Holz) nach (etwa 12 Holz/min je Farm). Farm-Stufen erhöhen Plätze oder Tempo (Werte mit B-099). | Holz ist die Basis und soll nicht enden, solange eine Farm steht. | `data/buildings.json` › `farm` (SIM legt `plantation` an) | Holz am Tagesbeginn ≥ 100 ab Tag 3 in ≥ 70 % |
| **Adern liefern Stein, Kupfer, Eisen und Kristall unendlich** (neues Level-Objekt je Mine-Stufe; **2 Adern je Stufe**). Der Vorrat ist unendlich, die **Abbaurate** steuert: höchstens 2 Bauern gleichzeitig je Ader; Zielrate bei 2 Bauern: **Stein 60/min, Kupfer 45/min, Eisen 35/min, Kristall 25/min** je Ader. Das Lager-Maximum begrenzt das Horten. | Material soll in den tiefen Stufen sicher fließen, aber Zeit und Bauern kosten; Menge steuert die Rate, nicht die Fundmenge. | `data/economy.json` › `veins` (SIM legt an), `data/biomes/*.json` | Zeit bis Hub-Stufe 2 und 3 siehe § 2; Zeit am Lager-Maximum ≤ 20 % (Kennzahl fehlt, B-099) |
| **Breite der Stufen: nach unten schmaler, dafür dichter.** Startwerte (Units): Wald 900–1100, Höhle 700–900, Mine 550–700, Eisenstollen 480–560, Kristallhöhle 400–480 (die beiden letzten Vorschläge, noch nicht gebaut). Gemessen (100 Seeds): Wald Mittel 1035, Höhle 832, Mine 647. | Kurze Wege, mehr Druck und weniger Rechenzeit in den tiefen Stufen, die gleichzeitig laufen (SP11, Pi 3); Material kommt aus Adern statt aus der Länge. | `data/biomes/*.json` › `lengthUnits` | Dichte unter Tage nicht abnehmend, ≥ 2,8 endliche Objekte je 100 Units (Startwert; Q28, 2026-10-04; `stufen.md` § 1) |
| Die Siegvariante „alles abbauen“ (`stufen.md` § 3) zählt nur die **endlichen Objekte** (Bäume, Felsen, Erz im Level); Adern sind unendlich und zählen nicht, die Plantage auch nicht. | Folge aus Adern und Plantage. | – | – |

## 2. Hub-Ausbau (Hub-Stufen 1 bis 5)

Jeder Hub (jede Stufe einer Insel) hat eine **Ausbaustufe 1 bis 5**. Stufe n schaltet frei: Mauer- und Turm-Stufe n und die Gebäude der Stufe n (§ 3). Der Ausbau braucht **Gold plus das Material der neuen Stufe** aus dem Insel-Vorrat; Stufe 1 ist der Anfang. Bezahlt wird an der **Burg** (Hub-Mitte, wird Zahlziel), ein Bauer baut (Q44, 2026-10-04).

| Ausbau auf Stufe | Kosten | Bauzeit (Startwert, Q44) | Zielkorridor |
|---|---|---|---|
| 2 | 100 Stein + 50 Gold | 20 s | erreicht vor Tag 5: 60–85 % |
| 3 | 150 Kupfer + 100 Gold | 30 s | erreicht vor Tag 10: 40–70 % |
| 4 | 200 Eisen + 200 Gold | 40 s | Korridor mit B-099 (Insel 1 Stufe 4 noch nicht gebaut) |
| 5 | 250 Kristall + 400 Gold | 50 s | Korridor mit B-099 |

Begründung: Fortschritt über Tiefe und Material, kein reines Goldsparen. Daten: neu `data/hub.json` › `levels` (SIM legt an).

## 3. Gebäude je Hub-Stufe

**Alles wird an festen Bauplätzen gebaut**, wie im Vorbild Kingdom Two Crowns (Q43, 2026-10-04). Kein Platz bewegt sich; jeder Platz trägt ein Gebäude und wird am selben Ort auf die nächste Stufe ausgebaut. Es gibt **kein Bau-Menü**, die Taste Y bleibt frei (Q34/Q06, 2026-10-04). Umsetzung: B-206 (Sprint W0).

| Platz-Klasse | Lage | Freischaltung | Quelle |
|---|---|---|---|
| **Hub-Platz** | fester Offset zur Hub-Mitte (`data/hub.json`): Burg (Hub-Mitte, Zahlziel des Hub-Ausbaus), Werkstatt, Lager, Kaserne, Taverne, Heilplatz, Schmiede, Rüstkammer, Treppen (+16/+24), Händler (+8/+12, nur Tiefe 0) | Hub-Stufe des Gebäudes (Tabelle unten) | Q43, Q55 |
| **Mauerlinie** | je Seite 5 Linien mit Mauer-Platz und eigenem Turm-Platz (8 Units innen) | Linie k ab Hub-Stufe k, sobald die **Mauer** der Linie k−1 derselben Seite gebaut ist (Material egal); jede Seite für sich | Q48, Q58, Q49 |
| **Tor-Platz** | je Linie einer, Mauer +4 Units außen | bezahlbar nur an der äußersten gebauten Linie der Seite (Tor ab Hub-Stufe 2) | Q47 |
| **Farm-Weltplatz** | ein fester Weltplatz je Seite zwischen Linie 1 und 2 | Hub-Stufe 1 | Q51 |
| **Angebots-Anhang** | Zahlziel mit festem `dx` am Gebäude (`data/buildings.json`), z. B. Schwert an der Werkstatt `dx +4` | entsteht mit dem Bau des Gebäudes | Q52, Q53 |

**Mauerlinien** (Startwerte; Q49, Q50, 2026-10-04): Linie 1 = heutige Mauer ±44 und Turm ±36 (alte Spielstände bleiben kompatibel), fest. Linien 2–5 liegen bei ±64/84/104/124 und **streuen je Seed nur nach außen um 0 bis +4 Units** (ganzzahlig, eigener Wurf je Seite und Linie; Q56) über einen **eigenen RNG-Strom** (z. B. `…:sites`), damit Ressourcen, Portale, Camps und Golden-Level unverändert bleiben. Alle Linien liegen unter dem Portal-Mindestabstand 150.

| Linie | Mauer | Turm (8 innen) | Tor (4 außen) | Streuung je Seed | bezahlbar ab Hub-Stufe |
|---|---|---|---|---|---|
| 1 | ±44 | ±36 | ±48 | fest | 1 |
| 2 | ±64 | ±56 | ±68 | 0…+4 | 2 |
| 3 | ±84 | ±76 | ±88 | 0…+4 | 3 |
| 4 | ±104 | ±96 | ±108 | 0…+4 | 4 |
| 5 | ±124 | ±116 | ±128 | 0…+4 | 5 |

Farm (±52) ↔ Turm 2 und Tor k ↔ Turm k+1 bleiben dadurch immer ≥ 4 Units auseinander (Q56). **Zerstörung** (Q58): Maßgeblich ist die Mauer der Linie k−1; wird sie zerstört, bleibt eine bereits gebaute Linie k gültig (auch Tor und Turm), nur das Bezahlen neuer Linien k+1… wartet, bis die Mauer k−1 repariert ist. **Camps** (Q57) dürfen innerhalb der Linien liegen (Golden-Seeds: ±75); validiert werden nur Portale (außerhalb der äußersten Linie 5 inkl. Streuung), Ressourcen und Truhen zwischen den Linien sind erlaubt, die ≥ 4-Units-Regel gilt nur für Plätze und Zahlziele aus den Daten. Bis W1.1 führt W0 `World.HubLevel` (Start 1) nur für die Linien- und Tor-Regel ein (Q59).

Hub-Plätze dürfen zwischen Linie 1 und Linie 2 liegen; sie sind ungeschützt, bis Linie 2 steht (Q51). Wird außen eine neue Linie gebaut, bleiben Turm und Tor innen stehen und wirken weiter als zweite Sperre (Q54). Krieger-Posten sind keine Bauplätze, sie leiten sich von der äußersten gebauten Sperre ab (Q46, `buerger.md` § 1).

**Auswahl am Platz** (Q34, Q52, 2026-10-04): Bietet ein Gebäude mehrere Angebote (Werkstatt, Schmiede, Rüstkammer), hat jedes Angebot ein **eigenes Zahlziel** als Anhang mit festem `dx` am Gebäude, keine neue Taste; alle `dx` liegen frei (Test). Ausgebildet bzw. aufgewertet wird der nächste freie Bauer bzw. Bogenschütze/Krieger.

| Hub-Stufe | Gebäude |
|---|---|
| 1 (Holz) | Burg, Holzmauer ×2, Holzturm ×2, Werkstatt, Farm |
| 2 (Stein) | Steinmauer, Steinturm (Ausbau), Tor, Kaserne, Lager, Taverne, Treppe hoch, Treppe runter |
| 3 (Kupfer) | Kupfermauer, Kupferturm (Ausbau), Schmiede, Heilplatz |
| 4 (Eisen) | Eisenmauer, Eisenturm (Ausbau), Rüstkammer |
| 5 (Kristall) | Kristallmauer, Zaubertum (Turm-Stufe 5) |

Zusatzgebäude: **Lager** (Stufe 2), Taverne (Stufe 2), Heilplatz (Stufe 3).

### 3.1 Mauern und Türme (Stufen 1 bis 5)

| Stufe | Material | Mauer: Kosten | Mauer: HP | Mauer: Bauzeit | Turm: Kosten | Turm: HP | Turm: Bauzeit |
|---|---|---|---|---|---|---|---|
| 1 | Holz | 20 Holz + 5 Gold | 300 | 6 s | 50 Holz + 20 Gold (wie heute) | 200 | wie heute |
| 2 | Stein | 30 Stein + 10 Gold | 600 | 9 s | 75 Stein + 25 Gold | 400 | 15 s |
| 3 | Kupfer | 40 Kupfer + 20 Gold | 1000 | 12 s | 100 Kupfer + 50 Gold | 700 | 20 s |
| 4 | Eisen | 50 Eisen + 40 Gold | 1600 | 16 s | 125 Eisen + 100 Gold | 1100 | 27 s |
| 5 | Kristall | 60 Kristall + 80 Gold | 2500 | 20 s | 150 Kristall + 200 Gold | 1700 | 33 s |

Turm: Kosten und Bauzeiten der Stufen 2–5 sind Startwerte (Q45, 2026-10-04), Ausbau am selben Platz; 2 Bogenplätze, +3 Reichweite. Stufe 5 ist der **Zaubertum**: Flächenschaden statt Bogen, als eigener Schuss mit den Startwerten 40 Schaden, Radius 3, Reichweite 13, alle 1,5 s; die Schützen steigen beim Ausbau ab und zählen weiter als Kämpfer (Q31, 2026-10-04). Daten: `data/buildings.json` (SIM legt Stufen an).
Zielkorridor: Erste Mauer vor Ende Tag 1 in ≥ 90 %; erster Turm vor Ende Tag 2 in ≥ 70 %.

### 3.2 Wirkungen der übrigen Gebäude

| Gebäude | Wirkung | Kosten und HP |
|---|---|---|
| **Burg** | Hub-Kern; fällt sie, wirkt der Niederlage-Modus (`stufen.md` § 4) | HP 1000 |
| **Mauer** | blockiert Gegner (außer `ignoresWalls`) | siehe 3.1 |
| **Tor** | eigene Bürger und Spieler passieren, Gegner nicht; je Linie ein fester Tor-Platz 4 Units außen vor der Mauer (Linie 1: ±48), bezahlbar nur an der äußersten gebauten Linie (Q47, 2026-10-04); für Gegner wie eine Mauer (Hindernis und Angriffsziel), nicht für den Posten der Bogenschützen (`outerWall`) (Q27, 2026-10-04); ein inneres Tor bleibt stehen und wirkt weiter (Q54) | Startwerte wie heute (30 Holz + 10 Gold, 250 HP), ab Stufe 2 (Stein) |
| **Werkstatt** | Bogen und Schwert, je bis 3 im Waffenregal; Schwert-Zahlziel als Anhang `dx +4` (Q53, 2026-10-04) | wie heute (40 Holz + 15 Gold, 150 HP) |
| **Farm** | Baumplantage: 6 Plätze, je Platz alle 30 s ein Baum (10 Holz), siehe § 1; fester Weltplatz je Seite zwischen Linie 1 und 2 (Q51, 2026-10-04) | wie heute (30 Holz + 10 Gold, 100 HP) |
| **Kaserne** | Truppen-Limit +10 (Basis 10, einfach gebaut); Regel und Prüfung beim Waffe-Holen: `buerger.md` § 3 | wie heute (60 Stein + 30 Gold, 200 HP) |
| **Lager** | +300 Kapazität je Rohstoff für die Insel; Arbeiter bringen Material hierher oder zur Burg | Startwert: 50 Stein + 20 Gold, HP 200, Bauzeit 8 s |
| **Taverne** | bei jedem `dawn` ein Landstreicher an der Taverne, solange dort weniger als 2 stehen; Wanderradius 6; eigene Werte in `data/` (Q30, 2026-10-04) | Startwert: 60 Stein + 30 Gold, HP 150, Bauzeit 12 s (Q26, gilt nach Q43) |
| **Heilplatz** | heilt Truppen (Kämpfer) und Spieler in Reichweite, **immer** (auch im Kampf); Startwerte 5 HP/s, Radius 6 um den Platz (Q32, 2026-10-04) | Startwert: 50 Kupfer + 30 Gold, HP 150, Bauzeit 12 s (Q26, gilt nach Q43) |
| **Schmiede** | Elite-Upgrades (Werte in `buerger.md`) | Startwert: 80 Kupfer + 50 Gold, HP 250, Bauzeit 16 s (Q26, gilt nach Q43) |
| **Rüstkammer** | Rüstung und Waffen-Upgrade für alle Kämpfer (Werte in `buerger.md`) | Startwert: 100 Eisen + 100 Gold, HP 350, Bauzeit 16 s (Q26, gilt nach Q43) |
| **Treppen** | Verbindung zur Stufe darüber/darunter, je 1 je Hub, ab Hub-Stufe 2 | 100 Stein + 50 Gold, HP 500, 20 s |

Die Startwerte für Taverne, Heilplatz, Schmiede und Rüstkammer sind **Vorschläge des Agenten** (🧑 hat die Wirkung beschlossen, nicht die Zahlen) und werden mit B-099 geprüft.

## 4. Bau-Ablauf, Zerstörung, Reparatur

| Regel | Begründung | Zielkorridor |
|---|---|---|
| Ablauf wie heute: Gold zahlen → Material wird automatisch aus dem Insel-Vorrat abgebucht → ein Bauer baut. Ausbau einer Mauer- oder Turm-Stufe läuft genauso, **am selben Platz**; der Hub-Ausbau wird an der Burg bezahlt (Q43, Q44, 2026-10-04). | Eine Taste, Material kommt aus dem gemeinsamen Vorrat. | Wartezeit „bezahlt bis gebaut“: Median ≤ 60 s (Kennzahl fehlt, B-099) |
| **Reparatur:** Bauern reparieren beschädigte Gebäude zwischen den Wellen **kostenlos** (Anteil der Bauzeit). | Beschädigte Mauern sollen sich erholen. | Zerstörte Gebäude je Welle 1–5: Median höchstens 1 |
| **Zerstörung:** Wird ein Gebäude zerstört, ist der Platz leer; **Gold und Material sind verloren**; bei Mauern und Türmen geht die Stufe verloren (neu ab Holzstufe), die Hub-Stufe bleibt. | Verlust tut weh, der Hub-Fortschritt nicht. | – |
| Wartet ein Bauplatz auf Bauer oder Material, wird das in der Welt angezeigt (CLI-Ticket). | Spieler sollen wissen, was fehlt. | – |
| **Schwierigkeitsgrade ändern Kosten und HP nicht** (`wirtschaft.md` § 4). | Wirtschaft bleibt in allen Graden gleich. | – |

## 5. Offen und Annahmen

- Die Zahlen für **Stufen 4 und 5** (Eisen, Kristall) und die Stufen selbst werden erst messbar, wenn Insel 1 sie enthält; ihre Zielkorridore folgen mit B-099.
- Startwerte für Taverne, Heilplatz, Schmiede, Rüstkammer (§ 3.2) sind Vorschläge ohne gesonderte Bestätigung; Turm-Kosten und -Bauzeiten (§ 3.1) und die Bauzeiten des Hub-Ausbaus (§ 2) hat 🧑 als Startwerte beschlossen (Q44, Q45, 2026-10-04).
- Elite-Upgrades (Schmiede) und Rüstung/Waffen (Rüstkammer): Werte und Wirkung stehen in `buerger.md` (R3.3). Bürger-Fortschritt gehört zu B-110.
- Die Wirkung „Farm: Plantage mit 6 Plätzen, je Platz alle 30 s ein Baum (10 Holz)“ (§ 1) und „Taverne: 1 Landstreicher je `dawn`, höchstens 2“ sind Startwerte (Wirtschaftsbalance, B-099).
- Beschlüsse vom 2026-10-04 (Fragenkatalog Q25–Q42): Bauzeiten von Taverne, Heilplatz (12 s), Schmiede und Rüstkammer (16 s) sind beschlossen; Zaubertum- und Heilplatz-Werte sind Startwerte. Das Modell „Hub wächst“ aus Q26 ist durch feste Bauplätze ersetzt (Q43–Q55, 2026-10-04, zweite Runde); Linien-Lagen und Streuung (§ 3) sind Startwerte, die Offsets der neuen Hub-Plätze legt W0 an (B-206).
- Das Rate-Modell (Plantage, Adern, Raten) und die Breiten der Eisenstollen und Kristallhöhle sind **Startwerte** (🧑 hat Plantage + Adern beschlossen; Adernzahl 2, Raten und 6 Plätze/30 s hat er mit „Vorschlag“ übernommen). Die Kosten aus R2.2 bleiben, **jeder Hub baut die ganze Liste**.
- Gemessene Level-Mengen (100 Seeds, Mittel): Wald 39 Bäume (≈ 390 Holz), 3 Felsen; Höhle 23 Felsen (≈ 230 Stein); Mine 6 Felsen, 3,4 Kupfererz (≈ 34 Kupfer, p10 = 0); damit tragen die Level allein die Kosten nicht, deshalb Plantage und Adern.
- Annahme: Die Kapazität der Insel ist die Summe aus 300 je Hub und 300 je Lager (🧑 hat „Burg als Basislager, Lager erweitern“ gewählt, die Summenbildung über Hubs ist nicht gesondert bestätigt); Startwerte für das Lager (50 Stein + 20 Gold, 8 s) sind ein Vorschlag.
- Messung vor BR1 (2026-10-07) und vorbereitete Begründung von HP und Kosten je Gebäude (B-015): `zielkorridore.md` › „Balancing-Runde Wirtschaft (BR1)“; beschlossen wird in BR1.2/BR1.3.
- Stufe 4 und 5 hängen an `stufen.md` § 1 (Insel 1 mit fünf Stufen; Endboss in der tiefsten): `stufen.md` und `game-design.md` sind angeglichen.
