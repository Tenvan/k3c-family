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

Diese Korridore stehen in den Regelwerken, ihre Messgröße liefert der Simulator noch nicht (`ist-abgleich.md` § 6). Sie bekommen mit dem Balancing-Tester B-099 eine Messgröße und erst dann eine Zeile in der Tabelle:

- Verluste je Welle: Median **höchstens 25 % der Kämpfer** (`buerger.md` § 3; entschieden von 🧑 am 2026-10-03 im Workshop F1.4, der abweichende Wert „höchstens die Hälfte der Truppen“ in `wirtschaft.md` § 3 wird mit B-185 angeglichen); kein Verlust-Ereignis, nur Differenz des Bestands.
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
