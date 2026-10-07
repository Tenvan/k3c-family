# Zielkorridore (Kennzahlen für das Balancing)

- **Stand:** Entwurf von Agent (F1.2, 2026-10-03), **Bestätigt von 🧑 am 2026-10-03** im Workshop F1.4 (alle Zahlen in § 1–3 unverändert)
- **Revision:** 1

Alle Kennzahlen mit Zielkorridor aus den Regelwerken in einer Tabelle (Beschluss Q02: der Agent schlägt Startwerte vor, 🧑 bestätigt; Pass/Fail je Kennzahl über 100 feste Seeds). **Alle Zahlen in § 1–3 sind von 🧑 bestätigt** (Datum im Kopf). Sie sind Startwerte; die Balancing-Runden B-155 und B-156 passen sie an, jede Änderung erhöht die Revision.

**Standardszenario** (wenn die Zeile nichts anderes nennt): Insel 1, Wald-Start, Grad Normal, 2 Spieler, Bot „sparsam“, 100 Seeds (`wirtschaft.md`, Kopf).
**Lesart:** Ein Anteil in % ist der Anteil der Seeds, in denen die Bedingung erfüllt ist; ein Median oder eine Zeit gilt über alle Seeds des Szenarios. „–“ heißt: keine Grenze in diese Richtung.
**Messbar mit:** `sim_run` = ein Lauf in k3c-dev (ein Seed, feste Eingaben, kein Bot) liefert die Größe schon heute; `B-099` = braucht den Balancing-Tester (Bots, 100 Seeds), Prüfung als `task balance` mit B-157; `Bench` = Go-Benchmark bzw. Lasttest.

## 1. Pflicht-Kennzahlen (B-134)

| Kennzahl | Szenario | Untergrenze | Obergrenze | Quelle | Messbar mit |
|---|---|---|---|---|---|
| Burg hält Nacht 1–5 (kein `castleFallen` bis Tagesanbruch nach Nacht 5) | Standard | 75 % | 90 % | `wirtschaft.md` § 4 | B-099 (`castleFallen`, Tag aus `World.Cycle`) |
| Gold am Morgen je Spieler, Median | Standard | 20 Gold | 70 Gold | `wirtschaft.md` § 1 | sim_run (`Player.Gold` bei `dawn`); Median über Seeds B-099 |
| Erste Mauer gebaut vor Ende der hellen Phase von Tag 1 (≤ 10 min Spielzeit, `data/biomes/forest.json` › `cycle.dayMinutes`) | Standard | 90 % | 100 % | `wirtschaft.md` § 3, `materialien-gebaeude.md` § 3.1 | sim_run (Ereignis `built`, Zeit aus dem Tick); Anteil B-099 |
| Höhle erreicht vor Beginn von Tag 6 | Standard | 80 % | 100 % | `stufen.md` § 3 | B-099 (Ereignis `arrived`, Tag) |
| Abstand der Wellen unter Tage | Mine, Normal, 2 Spieler, Ressourcen gesammelt, 100 Seeds | 4 min | 12 min | `stufen.md` § 2, `gegner.md` § 3 | sim_run (Ereignis `wave`, Zeit); Verteilung B-099 |
| Tick-Dauer p99 | 2 Räume × 3 Spieler, 3 Stufen aktiv, 30 Hz | – | 10 ms | `stufen.md` § 1 (Ziel aus B-042) | Bench (`BenchmarkIslandStep3Stages4Players` in `engine/sim/island_bench_test.go`; Messlauf am Pi mit LT1, B-175) |

## 2. Schwierigkeitsgrade: Burg hält Nacht 1–5

| Kennzahl | Szenario | Untergrenze | Obergrenze | Quelle | Messbar mit |
|---|---|---|---|---|---|
| Burg hält Nacht 1–5 | Standard, Grad Dev | 99 % | 100 % | `wirtschaft.md` § 4 | B-099 |
| Burg hält Nacht 1–5 | Standard, Grad Leicht | 90 % | 100 % | `wirtschaft.md` § 4 | B-099 |
| Burg hält Nacht 1–5 | Standard, Grad Hart | 50 % | 70 % | `wirtschaft.md` § 4 | B-099 |
| Burg hält Nacht 1–5 | Standard, Grad Ultra | 25 % | 45 % | `wirtschaft.md` § 4 | B-099 |

## 3. Weitere Kennzahlen aus den Regelwerken

| Kennzahl | Szenario | Untergrenze | Obergrenze | Quelle | Messbar mit |
|---|---|---|---|---|---|
| Erster Turm gebaut vor Ende von Tag 2 | Standard | 70 % | 100 % | `materialien-gebaeude.md` § 3.1 | B-099 (`built`) |
| Holz am Tagesbeginn ≥ 100, ab Tag 3 | Standard | 70 % | 100 % | `materialien-gebaeude.md` § 1 | sim_run (`World.Stock` bei `dawn`); Anteil B-099 |
| Hub-Stufe 2 erreicht vor Tag 5 | Standard | 60 % | 85 % | `materialien-gebaeude.md` § 2 | B-099 |
| Hub-Stufe 3 erreicht vor Tag 10 | Standard | 40 % | 70 % | `materialien-gebaeude.md` § 2 | B-099 |
| Zerstörte Gebäude je Welle 1–5, Median | Standard | – | 1 | `materialien-gebaeude.md` § 4, `gegner.md` § 4 | sim_run (`destroyed`); Median B-099 |
| Gegner einer Welle besiegt bis Tagesanbruch (Oberwelt) | Standard | 90 % | 100 % | `gegner.md` § 3 | B-099 (`World.Enemies`, `dawn`) |
| Kämpfer je Hub zu Tagesbeginn: Tag 3 ≥ 4 und Tag 6 ≥ 8 | Standard | 70 % | 100 % | `buerger.md` § 3 | sim_run (`World.Troops`); Anteil B-099 |
| Skill-Punkte im Pool nach Tag 10 ≥ 10 | Standard | 70 % | 100 % | `monarch.md` § 3 | B-099 (`World.SkillPoints`) |
| Miniboss Wald besiegt bis Tag 8 | Standard | 60 % | 85 % | `bosse.md` § 1, `stufen.md` § 3 | B-099 (Bosse erst mit K2) |
| Miniboss Höhle bis Tag 14, Mine bis Tag 20 besiegt | Standard | 50 % | 80 % | `bosse.md` § 1 | B-099 (Bosse erst mit K2) |
| Endboss besiegt bis Tag 25 | Standard | 30 % | 55 % | `bosse.md` § 1, `stufen.md` § 3 | B-099 (Bosse erst mit K2) |
| Burg hält Vollmond und Blutmond | Standard | 70 % | 100 % | `bosse.md` § 2 | B-099 (Events erst mit K3) |

## 4. Fehlende Messgrößen

Diese Korridore stehen in den Regelwerken, ihre Messgröße liefert der Simulator noch nicht (`archiv/ist-abgleich.md` § 6). Sie bekommen mit dem Balancing-Tester B-099 eine Messgröße und erst dann eine Zeile in der Tabelle:

- Verluste je Welle: Median **höchstens 25 % der Kämpfer** (`buerger.md` § 3; entschieden von 🧑 am 2026-10-03 im Workshop F1.4, der abweichende Wert „höchstens die Hälfte der Truppen“ in `wirtschaft.md` § 3 wird mit B-185 angeglichen); kein Verlust-Ereignis, nur Differenz des Bestands. **Begriff seit Q67 (2026-10-04):** Kämpfer sterben nicht; ein Verlust ist ein Kämpfer, der seine Ausrüstung verliert und zum Bauern zurückgestuft wird (Ereignis `disarmed`, Q69). Ob die Messung den Bestand oder `disarmed` zählt und ob 25 % bleibt, ist offen (B-099).
- Letzter Gegner einer Welle tot innerhalb der Frist ≥ 80 % (`gegner.md` § 3).
- Boss-Kampfdauer Median 60–180 s (`bosse.md` § 1).
- Erstes Elite-Upgrade vor Tag 10 in 40–70 % (`buerger.md` § 3).
- Anteil der Zeit, in der ein Rohstoff am Lager-Maximum steht: höchstens 20 % (`materialien-gebaeude.md` § 1).
- Wartezeit „bezahlt bis gebaut“: Median ≤ 60 s (`materialien-gebaeude.md` § 4; Ausgaben sind kein Ereignis).
- Anteil des Monarchen am Gesamtschaden einer Welle: höchstens 20 % (`monarch.md` § 1).
- `playerDown` je Welle Median höchstens 0,5; Anteil Wiederbelebungen an Toden ≥ 30 % (`monarch.md` § 5).
- Summe der Drops je Welle im Verhältnis zum Gold am Morgen (`wirtschaft.md` § 1; Größe noch nicht festgelegt).
- Material am Morgen je Rohstoff (`stufen.md` § 1; Korridor noch nicht festgelegt).
- Hub-Stufe 4 und 5 (`materialien-gebaeude.md` § 2; Insel 1 enthält diese Stufen noch nicht).

## Balancing-Runde Wirtschaft (BR1)

Messung vor der Runde (BR1.1, 2026-10-07, Commit `69fce4a`, Stand `develop` `edb2a8f` mit K2.1a–K2.2a). Die Zahlen in § 1–3 bleiben unverändert; hier stehen nur die Lesart und die Messung.

### Korridore der Wirtschaft

Zur Wirtschaft zählen die Zeilen aus § 1–3 mit Quelle `wirtschaft.md`, `materialien-gebaeude.md` oder `buerger.md`. Die Zeitgrenzen hängen an Tag und Phase, nicht an Minuten. Umgerechnet auf den Tageszyklus nach Q65 (Tag 6, Dämmerung 2, Nacht 4, Morgengrauen 2 min, 14 min je Tag) bzw. auf den heutigen Code bis B-213 (10/1/5 min, 16 min je Tag) ergibt das:

| Kennzahl | Korridor | Grenze nach Q65 | Grenze heute (bis B-213) | Quelle |
|---|---|---|---|---|
| Burg hält Nacht 1–5, Normal | 75–90 % | Tagesanbruch nach Nacht 5 = 68 min | 80 min | § 1, `wirtschaft.md` § 4 |
| Burg hält Nacht 1–5, Dev / Leicht / Hart / Ultra | ≥ 99 % / 90–100 % / 50–70 % / 25–45 % | wie Normal | wie Normal | § 2 |
| Gold am Morgen je Spieler, Median | 20–70 Gold, nicht dauerhaft am Maximum | bei `dawn` | bei `dawn` | § 1, `wirtschaft.md` § 1 |
| Erste Mauer gebaut vor Ende der hellen Phase von Tag 1 | 90–100 % | 6 min | 10 min | § 1, `materialien-gebaeude.md` § 3.1 |
| Erster Turm gebaut vor Ende von Tag 2 (helle Phase) | 70–100 % | 20 min | 26 min | § 3, `materialien-gebaeude.md` § 3.1 |
| Holz am Tagesbeginn ≥ 100, ab Tag 3 | 70–100 % | Beginn Tag n = (n−1) × 14 min | (n−1) × 16 min | § 3, `materialien-gebaeude.md` § 1 |
| Hub-Stufe 2 erreicht vor Tag 5 | 60–85 % | 56 min | 64 min | § 3, `materialien-gebaeude.md` § 2 |
| Hub-Stufe 3 erreicht vor Tag 10 | 40–70 % | 126 min | 144 min | § 3, `materialien-gebaeude.md` § 2 |
| Zerstörte Gebäude je Welle 1–5, Median | ≤ 1 | – | – | § 3, `materialien-gebaeude.md` § 4 |
| Kämpfer je Hub zu Tagesbeginn: Tag 3 ≥ 4 und Tag 6 ≥ 8 | 70–100 % | Beginn Tag 3 = 28 min, Tag 6 = 70 min | 32 / 80 min | § 3, `buerger.md` § 3 |

Nicht zur Wirtschaft (Quelle `stufen.md`, `gegner.md`, `monarch.md`, `bosse.md`): Höhle erreicht, Wellenabstand unter Tage, Tick-Dauer, Gegner bis Tagesanbruch, Skill-Punkte, Bosse, Events (BR2).

### Messung vor der Runde

Befehle (Standardszenario Wald, Normal, 2 Spieler, Bot `saver`, Seeds 1–100):

- `task balance`: Bericht `reports/balance-20261007-145615.md` (5 Tage)
- `./bin/k3c-balance.exe --seeds 100 --days 10 --out …`: Rohmetriken je Lauf, 10 Tage
- `./bin/k3c-balance.exe --curves --seeds 100`: Grad-Kurven, `reports/sensitivity-20261007-150222.md`

| Kennzahl | Messwert | Pass/Fail (vorläufig) | Anmerkung |
|---|---|---|---|
| Burg hält Nacht 1–5, Normal | 4 % | Fail | Nächte 1–4: 99 %. In Welle 5 hält die Burg nur in 4 % (Miniboss Wald, `bosses.json › goblinLeader`, Welle 5); Kampf, nicht Wirtschaft (B-346) |
| Burg hält Nacht 5 je Grad: Dev / Leicht / Hart / Ultra | 5 % / 5 % / 59 % / 26 % | Fail / Fail / Pass / Pass | Dev und Leicht scheitern wie Normal an Welle 5; Hart hält Nacht 5 besser als Normal (B-346) |
| Gold am Morgen je Spieler, Median | 100 (Maximum), ab Tag 2 in 99 % am Maximum | Fail | Messgröße Gold zur Dämmerung (`goldAtDusk` / 2) statt bei `dawn`; Bot `saver` gibt nach den Mauern kein Gold aus (B-347) |
| Erste Mauer vor Ende der hellen Phase Tag 1 | 100 %, Median 28 s | Pass | auch nach Q65 (6 min) sicher |
| Erster Turm vor Ende Tag 2 | nicht messbar | – | Der Bot baut keinen Turm (`firstBowTick` nur in 4 Läufen); keine Messgröße für `built` Turm (B-347) |
| Holz am Tagesbeginn ≥ 100, ab Tag 3 | 1–11 % (Tag 3–5), Median 70–80 | Fail | Messgröße Vorrat zur Dämmerung; ohne Farm wächst kein Holz nach (Bot baut keine Farm, B-347) |
| Hub-Stufe 2 / 3 | nicht messbar | – | keine Messgröße, Bot baut nicht aus (B-347) |
| Zerstörte Gebäude je Welle 1–5, Median | 0 | Pass | |
| Kämpfer je Hub Tag 3 / Tag 6 | 3 / 0 (Median) | Fail (vorläufig) | `troopsAtStart` der Welle; nur die 3 Start-Truppen, nach Welle 5 keine; Bot rekrutiert nicht (B-347) |

### Vorschläge (noch nicht beschlossen, nicht in `data/`)

| Datei › Pfad | alt → neu | Kennzahlen (vorher → erwartet) | Begründung |
|---|---|---|---|
| `economy.json › purse.startGold` | 100 → 60 | Gold am Morgen Median 100 → ≈ 60; erste Mauer bleibt 100 % (5 Gold) | Am Maximum fehlt der Anreiz zum Ausgeben; 60 Gold zahlen zwei Mauern, einen Turm und die erste Rekrutierung. Messung mit `--vary economy.json:purse.startGold` in BR1.3 |
| `economy.json › dawnGoldPerPlayer` | 5 → 5 (unverändert) | – | Erst nach dem Umbau des Bots bewerten; bei vollem Beutel wirkt das Einkommen nicht |
| `hub.json › islandStartStock.wood` | 100 → 100 (unverändert) | Holz ≥ 100 ab Tag 3 | Ursache ist die fehlende Farm im Bot, nicht der Startvorrat; nach B-347 neu messen |
| `bosses.json › goblinLeader.hpFactor` | 8 → (BR2) | Burg hält Nacht 1–5: 4 % → Ziel 75–90 % | Kampfwert, gehört in BR2 (B-346); ohne diese Änderung ist der Wirtschafts-Korridor „Burg hält“ nicht erreichbar |

Ob ein Wert kippt, entscheidet erst die Messung mit dem verbesserten Bot (B-347). Die Vorschläge gehen an 🧑 im Spieleabend 2 (BR1.2).

### Vorbereitung B-015: Begründung von HP und Kosten

| Gebäude | Kosten / HP (`buildings.json`) | Begründung (Vorschlag) |
|---|---|---|
| Burg | – / 1000 | Hält eine Normal-Welle 1–4 ohne Mauer-Durchbruch (gemessen 99 %); erst Welle 5 mit Miniboss bricht durch |
| Mauer 1–5 | 20 Holz + 5 Gold … 60 Kristall + 80 Gold / 300 … 2500 | Stufe 1 aus dem Startvorrat bezahlbar (2 × 20 Holz von 100), erste Mauer nach 28 s; HP etwa ×1,6–2 je Stufe, passend zu Gegner-HP ×1,5 je Tiefe |
| Turm 1–5 | 50 Holz + 20 Gold … 150 Kristall + 200 Gold / 200 … 1700 | Turm 1 + zwei Mauern = 90 Holz ≤ Startvorrat; HP unter der Mauer derselben Stufe, weil er hinter ihr steht |
| Tor | 30 Holz + 10 Gold / 250 | Weniger HP als die Mauer, weil es eigene Leute durchlässt; billig genug für die äußerste Linie |
| Werkstatt | 40 Holz + 15 Gold / 150 | Erstes Ausgabeziel nach den Mauern; geringe HP, steht innen |
| Farm | 30 Holz + 10 Gold / 100 | Amortisiert sich in 2,5 min (≈ 12 Holz/min); ungeschützt zwischen Linie 1 und 2, daher billig |
| Lager | 50 Stein + 20 Gold / 200 | Erst mit Stein bezahlbar (Hub-Stufe 2), +300 Kapazität |
| Kaserne, Taverne | 60 Stein + 30 Gold / 200, 150 | Stufe-2-Ausbau, teurer als das Lager, weil Truppen-Limit und Landstreicher dauerhaft wirken |
| Heilplatz | 50 Kupfer + 30 Gold / 150 | Kupfer-Einstieg, Wirkung immer an; HP gering, steht innen |
| Schmiede | 80 Kupfer + 50 Gold / 250 | Elite-Upgrades sind ein großer Kampfsprung; teuerster Kupferbau |
| Rüstkammer | 100 Eisen + 100 Gold / 350 | Stufe 4, Wirkung auf alle Kämpfer |
| Treppen | 100 Stein + 50 Gold / 500 | Kosten wie Hub-Stufe 2, damit der Weg nach unten eine Entscheidung ist; hohe HP, weil der Verlust die Stufe abschneidet |

Gemessen sind bisher nur Burg, Mauer 1 und die Zerstörung je Welle; alle übrigen Begründungen prüft BR1.3 nach B-347.
