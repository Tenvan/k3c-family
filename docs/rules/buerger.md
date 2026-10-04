# Regelwerk: Bürger, Berufe, Upgrades und Truppen-Limit

Beschlossen von 🧑 im Workshop R3.3 am 2026-10-02 (Grundlage: [`archiv/ist-monarch-buerger.md`](archiv/ist-monarch-buerger.md); Rahmen: [`monarch.md`](monarch.md), [`wirtschaft.md`](wirtschaft.md), [`materialien-gebaeude.md`](materialien-gebaeude.md)).
Je Regel: **Regel · Begründung · Verweis auf `data/` · Zielkorridor**. Werte sind **Startwerte**, Feintuning mit dem Balancing-Tester (B-099).
Zielkorridore gelten im Standardszenario **Normal, Wald-Start, 2 Spieler, Bot „sparsam“, je 100 Seeds**. Alles Material kommt aus dem Insel-Vorrat, Gold zahlen die Spieler. Jede Regel gilt für 2+ Spieler.
**Begriffe** (Q63, 2026-10-04): **Bürger** sind alle Figuren des Hubs, **Truppen** nur die Kämpfer (Bogenschützen, Krieger, Elite). Code-Namen wie `World.Troops` (alle Hub-Figuren) bleiben als Ausnahme.

## 1. Figuren

| Figur | Entstehung | Aufgabe | Werte (`data/troops.json`) |
|---|---|---|---|
| **Landstreicher** | im Camp (max. 2, Nachwuchs 25 s) und an der Taverne (bei jedem `dawn` einer, solange dort weniger als 2 stehen, Wanderradius 6; Q30, 2026-10-04) | wandert; eine Münze macht ihn zum Bauern; wird nicht angegriffen (Q67, 2026-10-04) | HP 30, Tempo 1,5 |
| **Bauer** | Landstreicher + 1 Gold | sammelt, baut, repariert, holt Waffen; bei Gefahr in der Burg | HP 40, Tempo 3 |
| **Bogenschütze** | Bauer + Bogen (Werkstatt) | Posten auf Turm oder hinter der äußersten Mauer, schießt | HP 50, Schaden 15, Reichweite 10 |
| **Krieger** | Bauer + Schwert (Werkstatt) | Nahkampf an der Frontlinie: Posten hinter der **äußersten gebauten Sperre** – mit Tor zwischen Mauer und Tor, ohne Tor hinter der Mauer; Posten sind keine Bauplätze, sie leiten sich von der äußersten Linie ab, innere Linien bekommen keine Krieger (Q46, Q54, 2026-10-04; ändert Q38); Seitenverteilung wie bei Bogenschützen (Startwert; Q38) | HP 100, Schaden 20, Reichweite 1 |
| **Elite-Bogenschütze / -Krieger** | Upgrade in der Schmiede | wie oben, stärker | HP 75 / 150, Schaden 23 / 30 |
| **Bergmann** | Bauer, Ausbildung 20 Gold | an Adern, Fels und Kupfererz, +50 % Abbaurate; zählt als einer der höchstens 2 Bauern an einer Ader (Q35, 2026-10-04) | wie Bauer |
| **Baumeister** | Bauer, Ausbildung 20 Gold | baut und repariert +50 % schneller | wie Bauer |
| **Handwerker** | Bauer, Ausbildung 30 Gold, in Werkstatt, Schmiede oder Rüstkammer (1–2 je Gebäude) | stellt Waffen, Upgrades und Rüstung her, je Handwerker +50 % Tempo | wie Bauer |
| **Händler** | einer je Insel, im Hub der Tiefe 0 bei Hub-Mitte +8; kommt bei `dawn` alle 3 Tage, mit Taverne auf der Insel alle 2 Tage, bleibt einen Tag (Q36, 2026-10-04) | tauscht Material gegen Gold und umgekehrt (10 Material = 5 Gold) an den Zahlzielen „Kaufen“ und „Verkaufen“; ein Material je Besuch, per `w.rng` aus den freigeschalteten (Q36, 2026-10-04) | – (Daten bei SIM) |

Begründung: Bürger sind Arbeiter und Verteidiger; Spezialisierungen der Bauern statt vieler neuer Figuren. Handwerker und Händler erweitern Wirtschaft und Truppenaufbau. Berufe sind auf die Arbeit festgelegt, **Umschulung kostet erneut**. Daten: `data/troops.json` › neue Figuren und Berufe (SIM legt an).

## 2. Kosten (Startwerte)

| Posten | Kosten | Wo |
|---|---|---|
| Landstreicher → Bauer | 1 Gold | Camp, Taverne |
| Bogen, Schwert | je 50 Holz + 20 Gold | Werkstatt (bis 3 im Waffenregal) |
| Elite-Bogenschütze | 100 Stein + 50 Gold | Schmiede (Hub-Stufe 3) |
| Elite-Krieger | 100 Kupfer + 50 Gold | Schmiede |
| Rüstungs-Upgrade | 100 Eisen + 100 Gold je Stufe; **2 Stufen** (ab Hub-Stufe 4 und 5), +20 % / +40 % auf die Basis-HP; wirkt sofort auf alle Kämpfer (`MaxHP` und `HP` steigen gleich, keine Vollheilung); Bauern und Landstreicher ohne Rüstung (Q37, 2026-10-04) | Rüstkammer (Hub-Stufe 4) |
| Bergmann, Baumeister | je 20 Gold | Werkstatt |
| Handwerker | 30 Gold | Werkstatt, Schmiede, Rüstkammer |
| Händler-Tausch | 10 Material = 5 Gold | beim Händler |

**Herstellungszeiten** (Startwerte; Q35, 2026-10-04): Bogen und Schwert 10 s, Elite-Upgrade 20 s, Rüstungsstufe 30 s; je Handwerker +50 % Tempo (höchstens 2). Jedes Angebot hat ein eigenes Zahlziel als Anhang mit festem `dx` am Gebäude, das mit dem Bau entsteht; das Schwert-Zahlziel liegt an der Werkstatt bei `dx +4` (Q34, Q52, Q53, 2026-10-04; `materialien-gebaeude.md` § 3).

Daten: `data/troops.json` (Kosten), `data/buildings.json`. Die Kosten der Gebäude selbst: `materialien-gebaeude.md`.

## 3. Entwicklung, Limit, Verhalten

| Regel | Begründung | Zielkorridor |
|---|---|---|
| **Kein Level und keine aktiven Skills für Bürger.** Entwicklung nur über Upgrades der Gebäude (Elite in der Schmiede, Rüstung in der Rüstkammer) und über Berufe. | Wie beim Monarchen: weniger Mechanik. | Zeitpunkt des ersten Elite-Upgrades: vor Tag 10 in 40–70 % (Kennzahl fehlt, B-099) |
| **Truppen-Limit je Hub:** Basis **10**, Kaserne **+10**; es zählen nur **Kämpfer** (Bogenschützen, Krieger, auch Elite). Bauern, Berufe, Handwerker, Händler und Landstreicher zählen nicht. Geprüft **beim Waffe-Holen** an genau einer Stelle (Bogen und Schwert): bei vollem Limit bleibt die Waffe im Regal; Landstreicher werden weiter zu Bauern (Q29, 2026-10-04). | Adern und Handwerk bleiben besetzbar; jede Stufe hat ihre eigene Besatzung. | Kämpfer je Hub zu Tagesbeginn: Tag 3 ≥ 4, Tag 6 ≥ 8 in ≥ 70 % (aus `World.Troops`) |
| **Verhalten wie heute:** Bauern (auch Bergleute an Adern) fliehen bei Gefahr in die Burg, Adern ruhen; Kämpfer halten Posten und schießen oder kämpfen in Reichweite. Kein aktiver Vorstoß. | Einfach und verständlich; die Welle bestimmt der Spieler durch Aufbau. | – |
| **Heilung:** nur am **Heilplatz** (Truppen, also Kämpfer, und Spieler in Reichweite, immer, auch im Kampf; Startwerte 5 HP/s, Radius 6; Q32, 2026-10-04) und durch den Heiler-Skill; **keine Regeneration**. | Der Heilplatz hat Gewicht. | Verluste je Welle: Median höchstens 25 % der Kämpfer (Kennzahl fehlt, B-099) |
| **Verlust-Kaskade statt Tod** (Q67, 2026-10-04; wie im Vorbild Kingdom Two Crowns): Bürger sterben nicht. Ein Treffer, der die HP auf 0 bringt, kostet die nächste Schicht: Bürger **mit Ausrüstung** (Kämpfer, auch Elite; Bergmann, Baumeister, Handwerker) lassen die **Ausrüstung zu Boden fallen** und werden **Bauer** (bleiben rekrutiert, volle HP, können sich Ausrüstung neu holen, zählen nicht mehr zum Truppen-Limit). Ein **Bauer** lässt seine **Münze** fallen (Gegner oder Monarch können sie aufheben) und wird **Landstreicher**, der zum Camp läuft. **Landstreicher werden nicht angegriffen.** Beim Burgfall wirkt der Niederlage-Modus (`stufen.md` § 4). Heute entfernt der Code Bürger bei HP ≤ 0 (`engine/sim/world.go`); das ist nicht mehr das Ziel. | Verlust hat Gewicht, lässt sich aber zurückholen. | Verluste je Welle: siehe Heilung (zählt künftig Rückstufungen, `zielkorridore.md`) |
| **Ausrüstung am Boden** (Q68, Q69, 2026-10-04): Bürger heben sie auf wie eine Waffe im Regal (Abholauftrag). Ein **Gegner** kann sie aufheben und **zum Portal tragen**, dann ist sie verloren; wird er vorher getötet, fällt sie wieder zu Boden. **Elite-Waffen** fallen **mit ihrer Stufe**. | Der Verlust lässt sich abwenden. | – |
| **Ereignisse** (Q69, 2026-10-04; ersetzt Q39, Q64): `troopLost` entfällt. **`disarmed`** (Felder `kind`, `x`, `cause`), wenn ein Bürger seine Ausrüstung verliert; **`equipmentTaken`**, wenn ein Gegner Ausrüstung wegträgt. Ein Ereignis für „Bauer verliert Münze“ ist nicht beschlossen (offen). | Ereignisse heißen nach dem, was geschieht. | – |

## 4. Offen und Annahmen

- **Alle Startwerte** (Ausbildungskosten, +50 %-Boni, Händler-Rate und Rhythmus, Rüstungs-Upgrade, Korridore) sind Vorschläge des Agenten; 🧑 hat die Kosten mit „Vorschlag“ bestätigt, Händler-Rhythmus und Boni nicht gesondert.
- Händler: Ort, Ablauf, Kurs und Rhythmus sind beschlossen (Q36, 2026-10-04); die Figur zeigt B-126.
- Offen: Handwerker-Anzahl je Gebäude bei Gebäuden mit mehreren Plätzen; Bürger der Bosse-Stufen (Regelwerk III). Geklärt: Der Bergmann zählt als einer der 2 Bauern an einer Ader (Q35, 2026-10-04).
- Das Aktionen-Overlay (`monarch.md` § 4) zeigt auch die Bürger-Aktionen (Ausbilden, Tauschen, Upgrade).
