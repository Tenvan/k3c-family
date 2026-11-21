# FG1 · SIM, CLI · Grundfiguren und Ausrüstung: Bürger, Handwerker, Bogenschütze, Soldat, Elite

- **Status:** aktiv
- **Projekt:** WIR
- **Domäne:** SIM, CLI
- **Reife:** bereit
- **Tickets:** B-1199, B-1200, B-1201
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21

## Ausgangslage

Grundfiguren heißen heute Landstreicher (`vagrant`) und Bauer (`peasant`); Schmiede und Rüstkammer geben Elite und einen Hub-HP-Bonus. Grill-Runde 2026-11-21 (Ralf) legt das neu fest, Details in B-1199 bis B-1201. Der Sprint ECO1 (B-1193/B-1194) liefert Werkzeuggürtel und Handwerker.

## Ziel

Nach dem Sprint gibt es die Kette Bürger → Handwerker, Bogenschütze, Soldat → Elite durch Ausrüstung (Gürtel, Bogen, Schwert, Rüstung) aus vier Meister-Gebäuden (Handwerksmeister, Bogner, Waffenschmied, Rüstungsschmied), mit Grafik und Texten im Standard. Am Ende sichtbar: Bürger holen Ausrüstung am Regal und ändern Aussehen und Namen.

## Beteiligte und Zielgruppen

Ralf entscheidet und nimmt am PC mit Pad ab; Hauptrechner: Sim, Second Worker: Grafik und Texte.

## Anforderungen

Siehe B-1199 (Rename citizen), B-1200 (Meister-Gebäude, Kombinationen), B-1201 (Grafik, Texte).

## Nicht-Ziele

Neue Berufe außerhalb der fünf Kombinationen; Balancing der Kosten (Abnahme-Ticket).

## Regeln und Einschränkungen

Neue Mechanik nur in Rust (Entscheidung 013); genau ein Beruf je Figur; der Bürger holt Ausrüstung selbst aus dem Regal; Hub-HP-Bonus der Rüstkammer entfällt; Waffenschmied ersetzt die Schmiede, Rüstungsschmied die Rüstkammer; alte Spielstände egal; Go-Golden nur mit `native_rules` aus.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-sim/**`, `rust/crates/k3c-data/**`, `rust/crates/k3c-game/src/actors/**`, `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/props/**`, `data/buildings.json`, `data/troops.json`, `data/paper.json`, `public/grafik/**`
- **Abhängig von:** ECO1
- **Braucht 🧑:** Abnahme
- **Sperre:** golden | spielstand

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** (B-1199) `citizen` ersetzt `vagrant` und `peasant`.
- **AC-02** (B-1200) Fünf Kombinationen mit Test je Kombination, Hub-HP-Bonus entfällt.
- **AC-03** (B-1201) Grafik und Texte de/en für Figuren, Gebäude, Gegenstände.

## Offene Fragen

keine (Name „Handwerksmeister“ entschieden von Ralf 2026-11-21, vermerkt in ECO1 B-1193). Die Freigabe wartet auf die Freigabe von ECO1.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| FG1.1 | `FG1.1-buerger-citizen.md` | Umsetzung | autonom | fertig |
| FG1.2 | `FG1.2-meister-kombinationen.md` | Umsetzung | autonom | offen |
| FG1.3 | `FG1.3-figuren-grafik-texte.md` | Umsetzung | autonom | offen |
| FG1.4 | `FG1.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
