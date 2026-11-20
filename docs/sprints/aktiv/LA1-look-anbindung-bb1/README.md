# LA1 · CLI · Look-Anbindung BB1: Waldwand, Leitmotive und Maßstab-Nachträge im Rust-Spiel

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-1146, B-1147, B-1148, B-1149, B-1150, B-1151, B-1163, B-1164, B-1167, B-947, B-1160, B-1175, B-1176, B-1161
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 Ralf, 2026-11-20

## Ausgangslage

BB1 hat Zielgrößen und neue Assets geliefert (B-1038 bis B-1042), RU55 den Maßstab (Monarch etwa 12 % der Ansicht) und RU54 satte Farben im Rust-Spiel. Nach RU36.2 (🧑 2026-11-20) passen die Größen zwischen Monarch und den anderen Figuren nicht, und die Stimmung ist noch zu weit vom K2C-Vorbild entfernt.

## Ziel

Das Rust-Spiel hat definierte Größenstufen mit dem Monarch auf dem Mount als Bezugswert, die Look-Anbindung aus BB1 (Waldwand, Leitmotive, Schiff, Pflanzung) und eine Stimmung nahe am K2C-Vorbild; der Kompass im Hub zeigt die Biome. Am Ende sichtbar: am PC in einer und zwei Ansichten neben den K2C-Screenshots, Tag und Nacht.

## Beteiligte und Zielgruppen

🧑 gibt die Spec frei, nimmt die Größen und die Stimmung am PC und TV ab und legt an den K2C-Screenshots fest, was an der Stimmung fehlt. Agenten setzen um.

## Anforderungen

- Größenstufen (`B-1163`): eine Tabelle in den Daten, Monarch auf dem Mount ist der Bezugswert, alle anderen Figuren und Gebäude sind Vielfache; Abgleich an den K2C-Screenshots.
- Stimmung (`B-1164`): zuerst Abgleich mit 🧑 (was fehlt: Farbe, Licht, Dunst, Tiefe, Details), dann Umsetzung in Sessions.
- Kompass im Hub (`B-1167`).
- BB1-Nachträge im Rust-Spiel: `B-1146` bis `B-1151`.

## Nicht-Ziele

Neue Welt-Dateien und Weltwahl (`B-1165`), Wald-Dichte im Level-Generator (`B-1166`), Regeln und Balancing, neue Wetter-Mechanik.

## Regeln und Einschränkungen

Port ohne Regeländerung (Entscheidung 012: Ziel ist das Verhalten, nicht Bit-Gleichheit); Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; Assets nur über MCP `assets_*` (Entscheidung 005). Gen, Bildvergleiche und Race laufen in den Sessions mit `Lane: zweit`, sonst im Abnahme-Ticket (B-742). Golden werden nie zugunsten von Rust neu geschrieben. LA1 läuft parallel zu RU36.3 (Hauptrechner: Schiff, Seekarte, Cheat-Fenster); die Bereiche liegen in anderen Dateien, Konflikte löst der Second Worker beim Merge von `origin/main`.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `data/sizes.json`, `data/look.json`, `data/light.json`, `data/hub.json`, `data/paper.json`, `rust/crates/k3c-game/src/art/**`, `rust/crates/k3c-game/src/actors/**`, `rust/crates/k3c-game/src/props/**`, `rust/crates/k3c-game/src/fx/**`, `rust/crates/k3c-game/src/still/**`, `rust/crates/k3c-data/**`, `public/grafik/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Sperre:** –

## Beispiele

Monarch auf dem Mount = 1 M; ein Bürger steht bei 0,55 bis 0,65 M, das Schiff bei etwa 3 M. Nacht-Szene am Strand zeigt Farbe, Licht und Dunst neben der K2C-Referenz.

## Ausnahme- und Fehlerfälle

Ein Wert der Stufentabelle verlangt neue Grafik: Ticket statt eigener Zeichnung. Die Stimmung braucht mehr als eine Session: Rest als Ticket, 🧑 entscheidet über eine Folge-Session.

## Akzeptanzkriterien

- **AC-01** (B-1163) `data/sizes.json` nennt jede Stufe als Vielfaches des Monarchen auf dem Mount; das Rust-Spiel zeichnet danach; Tests prüfen die Verhältnisse.
- **AC-02** (B-1146 bis B-1151) Die BB1-Nachträge sind im Rust-Spiel angebunden; Texturbudget der Bäume um 15 MiB gesenkt.
- **AC-03** (B-1167) Der Kompass im Hub zeigt die Biome der Insel.
- **AC-04** (B-1164) `docs/messung/stimmung-abgleich.md` nennt je Szene die fehlenden Merkmale der Stimmung in Rangfolge, abgenommen von 🧑.
- **AC-05** (B-1164) Die Punkte mit Rang 1 bis 3 aus AC-04 sind behoben, Vorher-Nachher-Bilder liegen im Ergebnis.
- **AC-06** (B-947, B-1160, B-1175, B-1176) Der Mauerzug im Hub ist lückenlos (Test für die Kachelung), im Burgbereich geschlossen, die Mauer der Stufe 1 sieht aus wie ein Zaun oder eine Bretterwand, und im Hub steht kein einfarbiges Platzhalter-Rechteck mehr.
- **AC-07** (B-1161) Alle Gebäude mit Fackelplatz tragen nachts Fackeln, am Tag nur Halter; der Monarch hat nachts ein folgendes Licht.

## Offene Fragen

- 🧑: Welche Merkmale der K2C-Screenshots sind für die Stimmung entscheidend (Reihenfolge)? Wird in der ersten Session von B-1164 mit 🧑 festgehalten.
- 🧑: Soll LA1 sofort aktiviert werden (Spec freigeben)? Reihenfolge der Sessions: 1. Größenstufen (B-1163), 2. Stimmung (B-1164), 3. BB1-Nachträge, 4. Kompass (B-1167), 5. Review.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| LA1.1 | `LA1.1-groessenstufen.md` | Umsetzung | autonom | fertig |
| LA1.2 | `LA1.2-bb1-nachtraege.md` | Umsetzung | autonom | fertig |
| LA1.3 | `LA1.3-kompass-biome.md` | Umsetzung | autonom | offen |
| LA1.4 | `LA1.4-stimmung-abgleich.md` | Workshop | Mensch | offen |
| LA1.5 | `LA1.5-stimmung-umsetzung.md` | Umsetzung | autonom | offen |
| LA1.6 | `LA1.6-review.md` | Review | autonom | offen |
| LA1.7 | `LA1.7-mauern.md` | Umsetzung | autonom | fertig |
| LA1.8 | `LA1.8-fackeln-monarch-licht.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
