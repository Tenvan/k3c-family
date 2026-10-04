# B-206 · Alle Bauplätze sind feste Punkte aus Daten und Level-Seed, Mauerlinien schalten je Seite nacheinander frei

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** W0
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bauplätze sind heute feste Offsets aus `data/hub.json` › `sites` ab der Seed-abhängigen Hub-Mitte (`engine/sim/world.go`, Zeile 46–51): je Seite eine Mauer (±44) und ein Turm (±36), dazu Werkstatt, Treppen und Lager. Der Spielstand ordnet Plätze über `kind@x` zu (`engine/sim/save.go`, Zeile 74–80 und 249–263). `outerWall`, `blockingWall` und `freeTower` arbeiten schon mit der „äußersten gebauten Mauer“. Der Level-Generator zieht Ressourcen, Portale und Camps aus dem Strom `rng.New(b.ID+":"+seed)` (`engine/level/level.go`, Zeile 68). Q26 („Hub wächst, Mauer wandert“) ist am 2026-10-04 durch feste Bauplätze ersetzt (Q43).

## Ziel

Alles wird an festen Bauplätzen gebaut, wie im Vorbild Kingdom Two Crowns: Hub-Plätze, Mauerlinien mit Turm- und Tor-Platz, Farm-Weltplatz und Angebots-Anhänge stehen fest in Daten und Seed, kein Platz bewegt sich. Spieler erweitern ihre Verteidigung Linie für Linie nach außen; Spielstände und Golden-Level bleiben stabil.

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen, beide Seiten); Entwickler SIM (W0), danach W1 bis W4, die das Layout nutzen; 🧑 hat das Modell beschlossen (Q43, Q47–Q59).

## Anforderungen

- **Platz-Klassen** (Q43): Hub-Platz (Offset zur Hub-Mitte, `data/hub.json`), Mauerlinie (Mauer- und Turm-Platz), Tor-Platz, Farm-Weltplatz, Angebots-Anhang. Kein Platz bewegt sich.
- **Mauerlinien** (Q49): 5 Linien je Seite bei ±44/64/84/104/124 (Startwerte), Turm 8 Units innen, Tor 4 Units außen; Linie 1 = heutige Mauer ±44 und Turm ±36. Alle Linien liegen unter dem Portal-Mindestabstand 150.
- **Streuung** (Q50): Linie 1 und alle Hub-Plätze sind fest; Linien 2–5 streuen je Seed **nur nach außen um 0 bis +4 Units** (Q56; ganzzahlig, eigener Wurf je Seite und Linie) über einen **eigenen RNG-Strom** (z. B. `…:sites`), der Level-Strom wird nicht zusätzlich verbraucht. Farm (±52) ↔ Turm 2 und Tor k ↔ Turm k+1 bleiben dadurch immer ≥ 4 Units auseinander.
- **Validierung und Camps** (Q57): Der Generator validiert nur Portale (außerhalb der äußersten Linie 5 inkl. Streuung). Camps dürfen innerhalb der Linien liegen (Golden-Seeds ±75) und werden nur **gemessen** (Ergebnis unter Notizen); Ressourcen, Truhen und Skill-Punkte zwischen den Linien sind erlaubt, die ≥ 4-Units-Regel gilt nur für Plätze und Zahlziele aus den Daten.
- **Freischaltung** (Q48): Linie k ist bezahlbar ab Hub-Stufe k, sobald die **Mauer** der Linie k−1 derselben Seite gebaut ist (Material egal); jede Seite für sich. Zerstörung (Q58): Eine bereits gebaute Linie k bleibt gültig (auch Tor und Turm); nur das Bezahlen neuer Linien k+1… wartet, bis die Mauer k−1 repariert ist.
- **Hub-Stufe** (Q59): W0 führt `World.HubLevel` (Start 1) nur für die Linien- und Tor-Regel ein; die allgemeine Sperre der Hub-Gebäude kommt mit W1.1, bis dahin sind neue Hub-Gebäude ohne Wirkung bezahlbar.
- **Tor** (Q47, Q54): je Linie ein fester Tor-Platz, bezahlbar nur an der äußersten gebauten Linie; wird außen eine neue Linie gebaut, bleiben Turm und Tor innen stehen und wirken weiter.
- **Hub-Plätze** (Q51, Q55): dürfen zwischen Linie 1 und 2 liegen (ungeschützt, bis Linie 2 steht); Treppen +16/+24, Händler +8/+12 (nur Tiefe 0) bleiben Hub-Plätze.
- **Farm** (Q51): ein fester Weltplatz je Seite zwischen Linie 1 und 2.
- **Angebots-Anhänge** (Q52, Q53): Zahlziele mit festem `dx` am Gebäude in `data/buildings.json`, entstehen mit dem Bau; Schwert an der Werkstatt `dx +4`.
- Deterministisch (`engine/rng`), Werte nur in `data/`, gilt für 2+ Spieler.

## Nicht-Ziele

Hub-Ausbau, Mauer- und Turm-Stufen (B-112, W1); Gebäude-Wirkungen (B-116); Anzeige freier und gesperrter Plätze (B-207); Protokoll (B-208, W5); Client-Typen (B-209); Krieger-Posten (B-122, leiten sich von der äußersten Linie ab, Q46); mehrere Inseln (B-103).

## Regeln und Einschränkungen

`docs/rules/materialien-gebaeude.md` § 3 (Platz-Klassen, Tabelle der Linien); Beschlüsse Q43–Q59 im Fragenkatalog. Golden-Level dürfen sich nicht ändern (Ressourcen, Portale, Camps). Spielstand: `kind@x` bleibt stabil, alte Stände (Mauer ±44, Turm ±36) laden unverändert. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`.

## Beispiele

Seed A, Hub-Stufe 2, Linie 1 links gebaut → Linie 2 links ist bezahlbar, Linie 2 rechts nicht (rechts fehlt Linie 1). Seed A und Seed B → Linie 1 bei ±44 in beiden, Linie 3 bei 84 plus 0 bis 4 Units je Seed verschieden, Ressourcen und Portale gleich wie vor der Änderung.

## Ausnahme- und Fehlerfälle

Tor an einer inneren Linie → nicht bezahlbar. Mauer der Linie k−1 zerstört, Linie k steht noch → Linie k bleibt gültig, nur neue Linien k+1… sind nicht bezahlbar, bis die Mauer repariert ist (Q58). Gestreute Linie träfe einen Hub-Platz oder Anhang → Layout-Test schlägt fehl (Daten anpassen, nicht im Code ausweichen).

## Akzeptanzkriterien

- **AC-01** Test: Für 100 Seeds liegen je Seite 5 Linien mit Turm 8 Units innen und Tor 4 Units außen vor; Linie 1 steht immer bei ±44 (Turm ±36, Tor ±48), Linien 2–5 liegen 0 bis 4 Units außerhalb ihres Startwerts, alle unter 150, Abstand Farm ↔ Turm 2 und Tor k ↔ Turm k+1 immer ≥ 4 Units; gleicher Seed ergibt dasselbe Layout.
- **AC-02** Test: Ressourcen, Portale und Camps der Golden-Level sind unverändert (Golden-Level-Test ohne Aktualisierung grün); Portale liegen außerhalb der äußersten Linie 5 inkl. Streuung, die Lage der Camps zu den Linien ist gemessen und in den Notizen festgehalten (Q57).
- **AC-03** Test: Linie k einer Seite ist erst mit Hub-Stufe k und gebauter Linie k−1 derselben Seite bezahlbar; die andere Seite bleibt davon unberührt. Ist die Mauer der Linie k−1 zerstört, bleibt eine gebaute Linie k gültig, Linie k+1 ist erst nach der Reparatur bezahlbar (Q58). `World.HubLevel` (Start 1) steuert nur diese Regel, Hub-Gebäude sind bis W1.1 nicht gesperrt (Q59).
- **AC-04** Test: Ein Tor ist nur an der äußersten gebauten Linie bezahlbar; nach dem Bau einer äußeren Linie stehen Turm und Tor innen weiter und wirken.
- **AC-05** Test: Alle Hub-Plätze, Farm-Weltplätze und `dx`-Anhänge liegen frei (kein Zahlziel überlappt ein anderes) für alle 100 Seeds.
- **AC-06** Test: Ein Spielstand der aktuellen Version mit Mauer ±44 und Turm ±36 lädt an dieselben Plätze (`kind@x`); `task check:go` grün.

## Offene Fragen

keine (Q56–Q59 am 2026-10-04 geklärt).

## Notizen

Beschlüsse Q43–Q55 vom 2026-10-04, zweite Runde (`docs/fragenkatalog.md`, Block 6). Ersetzt die Breiten- und Offset-Planung aus Q26; die Bauzeiten aus Q26 bleiben (B-116). Q56–Q59 (zweite Runde, Chat 2026-10-04) präzisieren Q48 und Q50. Camp-Messung (Q57): Ergebnis trägt W0.2 nach.
