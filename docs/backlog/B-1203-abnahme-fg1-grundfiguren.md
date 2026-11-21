# B-1203 · FG1 ist am PC abgenommen: Bürger, Meister-Gebäude, Kombinationen, Grafik

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** FG1
- **Projekt:** WIR
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint FG1 (B-1199 bis B-1201) macht aus Landstreicher und Bauer den Bürger (`citizen`), führt Bogner, Waffenschmied und Rüstungsschmied mit den Kombinationen Bürger → Handwerker, Bogenschütze, Soldat → Elite ein und liefert Grafik und Texte. Gen-, Sim- und Race-Läufe sowie die Sichtprüfung am PC stehen nach B-742 hier.

## Ziel

FG1 ist am PC gesehen und gemessen abgenommen; die offenen 50:50-Entscheidungen sind von Ralf entschieden.

## Beteiligte und Zielgruppen

Ralf (🧑) prüft am PC mit Pad und entscheidet; Claude fährt die Läufe.

## Anforderungen

- **AC-01** Sim nur über `sim_test`, blockierend gewartet, keine Regression.
- **AC-02** Balance: Preise und Zeiten der Ausrüstung (Schwert, Rüstung, Gürtel, Bogen) spielen sich ohne Stau; die Werte stehen als Code-Konstanten in `unit/hammer.rs` und `unit/warrior.rs` und kommen nach `data/` (Review-Befund).
- **AC-03** Gen: `check_run task:test` samt `gen:paper` und Grafik-Golden grün (neu: `truppen/handwerker`, `icons/icon-guertel|bogen|schwert|ruestung`).
- **AC-04** Race: `check_run task:check:race` grün.
- **AC-05** Sichtprüfung: Bürger holen Ausrüstung am Regal und ändern Aussehen (Handwerker mit Gürtel und Hammer) und Namen; Gebäudenamen Handwerksmeister, Bogner, Waffenschmied, Rüstungsschmied in de und en.
- **AC-06** Bekannte rote Tests wie in B-1190 AC-04 und B-1202 AC-07 geklärt.

## Nicht-Ziele

Neue Berufe außerhalb der fünf Kombinationen.

## Regeln und Einschränkungen

B-742, Sim-Sperre (Beschluss 2026-11-11), Entscheidung 013.

- **Bereich:** `rust/**`, `data/**`, `public/grafik/**`
- **Abhängig von:** B-1199, B-1200, B-1201
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Ein Bürger holt am Regal der Werkstatt den Gürtel: er trägt danach Gürtel und Hammer und heißt Handwerker.

## Ausnahme- und Fehlerfälle

Liegt keine Rüstung im Regal oder ist kein Kämpfer da, bleibt die Rüstung im Regal.

## Akzeptanzkriterien

- **AC-01** Sim über `sim_test` ohne Regression.
- **AC-02** Ausrüstungswerte aus dem Code nach `data/`, Balance am PC ohne Stau.
- **AC-03** Gen: `check_run task:test` samt `gen:paper` und Golden grün.
- **AC-04** Race: `check_run task:check:race` grün.
- **AC-05** Handwerker, Namen und Regal am PC sichtbar.
- **AC-06** Rote Tests geklärt.

## Offene Fragen

Entschieden von Ralf (2026-11-21, Dialog), als Umbau-Arbeit noch zu planen:

- Rüstung wird am Gebäude abgeholt (wie Bogen und Schwert), nur außerhalb der Gefahr; keine Fernwirkung mehr (`building/upgrades.rs` `armor_convert`).
- Werte (Rüstungspreis, Regal, Herstellzeiten) als optionale Felder nach `data/buildings.json`, Zahlen unverändert.
- Gegenstands-Icons im HUD: am Regal und im Holen-Hinweis.
- Bogner nur ganz oben (`topOnly`), Taverne bleibt Produktion.
- Besorger bekommen einen sichtbaren Weg über die Treppe.
- Truppen-Limit gemeinsam für die ganze Insel statt je Stufe.

Die Umbauten sind noch nicht geplant: Der Hauptrechner legt dafür Tickets und einen Sprint an.

## Notizen

Muster: B-1190, B-1202.
