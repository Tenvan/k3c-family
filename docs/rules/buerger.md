# Regelwerk: Bürger, Berufe, Upgrades und Truppen-Limit

Beschlossen von 🧑 im Workshop R3.3 am 2026-10-02 (Grundlage: [`ist-monarch-buerger.md`](ist-monarch-buerger.md); Rahmen: [`monarch.md`](monarch.md), [`wirtschaft.md`](wirtschaft.md), [`materialien-gebaeude.md`](materialien-gebaeude.md)).
Je Regel: **Regel · Begründung · Verweis auf `data/` · Zielkorridor**. Werte sind **Startwerte**, Feintuning mit dem Balancing-Tester (B-099).
Zielkorridore gelten im Standardszenario **Normal, Wald-Start, 2 Spieler, Bot „sparsam“, je 100 Seeds**. Alles Material kommt aus dem Insel-Vorrat, Gold zahlen die Spieler. Jede Regel gilt für 2+ Spieler.

## 1. Figuren

| Figur | Entstehung | Aufgabe | Werte (`data/troops.json`) |
|---|---|---|---|
| **Landstreicher** | im Camp (max. 2, Nachwuchs 25 s) und in der Taverne (1 je Tag) | wandert; eine Münze macht ihn zum Bauern | HP 30, Tempo 1,5 |
| **Bauer** | Landstreicher + 1 Gold | sammelt, baut, repariert, holt Waffen; bei Gefahr in der Burg | HP 40, Tempo 3 |
| **Bogenschütze** | Bauer + Bogen (Werkstatt) | Posten auf Turm oder hinter der äußersten Mauer, schießt | HP 50, Schaden 15, Reichweite 10 |
| **Krieger** | Bauer + Schwert (Werkstatt) | Nahkampf an der Frontlinie | HP 100, Schaden 20, Reichweite 1 |
| **Elite-Bogenschütze / -Krieger** | Upgrade in der Schmiede | wie oben, stärker | HP 75 / 150, Schaden 23 / 30 |
| **Bergmann** | Bauer, Ausbildung 20 Gold | an Adern und Fels, +50 % Abbaurate | wie Bauer |
| **Baumeister** | Bauer, Ausbildung 20 Gold | baut und repariert +50 % schneller | wie Bauer |
| **Handwerker** | Bauer, Ausbildung 30 Gold, in Werkstatt, Schmiede oder Rüstkammer (1–2 je Gebäude) | stellt Waffen, Upgrades und Rüstung her, je Handwerker +50 % Tempo | wie Bauer |
| **Händler** | kommt alle 3 Tage vorbei (häufiger mit Taverne), bleibt einen Tag | tauscht Material gegen Gold und umgekehrt (10 Material = 5 Gold) | – (Daten bei SIM) |

Begründung: Bürger sind Arbeiter und Verteidiger; Spezialisierungen der Bauern statt vieler neuer Figuren. Handwerker und Händler erweitern Wirtschaft und Truppenaufbau. Berufe sind auf die Arbeit festgelegt, **Umschulung kostet erneut**. Daten: `data/troops.json` › neue Figuren und Berufe (SIM legt an).

## 2. Kosten (Startwerte)

| Posten | Kosten | Wo |
|---|---|---|
| Landstreicher → Bauer | 1 Gold | Camp, Taverne |
| Bogen, Schwert | je 50 Holz + 20 Gold | Werkstatt (bis 3 im Waffenregal) |
| Elite-Bogenschütze | 100 Stein + 50 Gold | Schmiede (Hub-Stufe 3) |
| Elite-Krieger | 100 Kupfer + 50 Gold | Schmiede |
| Rüstungs-Upgrade | 100 Eisen + 100 Gold je Stufe, +20 % HP für alle Truppen | Rüstkammer (Hub-Stufe 4) |
| Bergmann, Baumeister | je 20 Gold | Werkstatt |
| Handwerker | 30 Gold | Werkstatt, Schmiede, Rüstkammer |
| Händler-Tausch | 10 Material = 5 Gold | beim Händler |

Daten: `data/troops.json` (Kosten), `data/buildings.json`. Die Kosten der Gebäude selbst: `materialien-gebaeude.md`.

## 3. Entwicklung, Limit, Verhalten

| Regel | Begründung | Zielkorridor |
|---|---|---|
| **Kein Level und keine aktiven Skills für Bürger.** Entwicklung nur über Upgrades der Gebäude (Elite in der Schmiede, Rüstung in der Rüstkammer) und über Berufe. | Wie beim Monarchen: weniger Mechanik. | Zeitpunkt des ersten Elite-Upgrades: vor Tag 10 in 40–70 % (Kennzahl fehlt, B-099) |
| **Truppen-Limit je Hub:** Basis **10**, Kaserne **+10**; es zählen nur **Kämpfer** (Bogenschützen, Krieger, auch Elite). Bauern, Berufe, Handwerker, Händler und Landstreicher zählen nicht. | Adern und Handwerk bleiben besetzbar; jede Stufe hat ihre eigene Besatzung. | Kämpfer je Hub zu Tagesbeginn: Tag 3 ≥ 4, Tag 6 ≥ 8 in ≥ 70 % (aus `World.Troops`) |
| **Verhalten wie heute:** Bauern (auch Bergleute an Adern) fliehen bei Gefahr in die Burg, Adern ruhen; Kämpfer halten Posten und schießen oder kämpfen in Reichweite. Kein aktiver Vorstoß. | Einfach und verständlich; die Welle bestimmt der Spieler durch Aufbau. | – |
| **Heilung:** nur am **Heilplatz** (Truppen und Spieler in Reichweite) und durch den Heiler-Skill; **keine Regeneration**. | Der Heilplatz hat Gewicht. | Verluste je Welle: Median höchstens 25 % der Kämpfer (Kennzahl fehlt, B-099) |
| **Verlust:** Gefallene Truppen sind verloren; Ersatz über Taverne und Rekrutierung. Beim Burgfall wirkt der Niederlage-Modus (`stufen.md` § 4). | Verlust hat Gewicht, Ersatz ist möglich. | – |

## 4. Offen und Annahmen

- **Alle Startwerte** (Ausbildungskosten, +50 %-Boni, Händler-Rate und Rhythmus, Rüstungs-Upgrade, Korridore) sind Vorschläge des Agenten; 🧑 hat die Kosten mit „Vorschlag“ bestätigt, Händler-Rhythmus und Boni nicht gesondert.
- Händler: genaue Erscheinung (Figur, Ort, Umtausch in beide Richtungen, Preise je Material) legt das SIM-Ticket mit B-099 fest; die Regel „kommt alle 3 Tage vorbei“ ist ein Startwert.
- Offen: Handwerker-Anzahl je Gebäude bei Gebäuden mit mehreren Plätzen; Zusammenspiel Berufe und Adern (2 Bauern je Ader: Bergmann zählt als einer); Bürger der Bosse-Stufen (Regelwerk III).
- Das Aktionen-Overlay (`monarch.md` § 4) zeigt auch die Bürger-Aktionen (Ausbilden, Tauschen, Upgrade).
