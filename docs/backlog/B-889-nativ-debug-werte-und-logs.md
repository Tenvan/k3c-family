# B-889 · Das native Spiel zeigt Debug-Werte der Sim (F10) und loggt Zustandswechsel und Zusammenfassungen

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU61
- **Projekt:** RSU
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-14
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Das native Spiel zeigt mit F2 nur Bildrate und Taktzeit. Zustand der Sim (Tageszeit, Vorrat je Spieler, Objektanzahl, ausgewähltes Objekt) ist nur über Logs des Raums oder gar nicht zu sehen; Logs fassen Kampf und Wirtschaft nicht zusammen.

## Ziel

Das F10-Debug-Overlay (B-888) hat eine durchschaltbare Werte-Seite mit den Zahlen der Sim, und das Spiel loggt Zustandswechsel und Zusammenfassungen ohne Spam je Takt; am Ende liegt eine Sitzungszusammenfassung als JSON in `reports/`.

## Beteiligte und Zielgruppen

- 🧑 Ralf: entwickelt, testet, liest Logs und Berichte.
- Claude: liest Berichte über `report_read`.

## Anforderungen

Anzeige (F10-Overlay, zweite Seite, durchschaltbar):
1. Tageszeit, Tag, Welle, Restzeit bis zur Nacht.
2. Je Spieler: Münzen, Material, Position (Units), Chunk, Stufe.
3. Objektanzahl nach Art: Truppen, Gegner, Bauten, Münzen am Boden.
4. Sim-Takt: Ticks/s, Dauer eines Takts (ms), Zeitraffer, Seed.
5. Ausgewähltes Objekt (Klick oder Pad): ID, Zustand, Ziel, Lebenspunkte, KI-Entscheidung.

Logs (slog, Emoji laut CLAUDE.md):
- 🚦 Zustandswechsel: Tageszeit, Welle, Bau fertig, Burgfall, Stufenwechsel je Spieler.
- 🏹 Kampf-Zusammenfassung je Nacht: gespawnt/getötet, eigene Verluste, Schaden je Seite.
- 🪙 Wirtschaft je Tag: Einnahmen/Ausgaben je Spieler.
- 🐢 Takt über Budget (Wert aus `data/`, Default 4 ms) mit Objektanzahl im Moment.
- 🐛 jeder Cheat-Befehl mit Spieler und Wert.
- 💾 Sitzungszusammenfassung als JSON nach `reports/` am Ende (Seed, Dauer, Tage, Ende-Ursache), lesbar über `report_read`.

Logs nur bei Wechsel oder als Zusammenfassung, nie je Takt. Logik und Zählung in Go mit Tests, Werte aus der Sim lesen, nicht im Client rechnen.

## Nicht-Ziele

- Balancing-Auswertung (Sim und Balancing gesperrt).
- Anzeige im Aufnahme-Modus.

## Regeln und Einschränkungen

Datei ≤ 400 Zeilen, Funktion ≤ 60; Log-Emojis aus einem Codepunkt. Keine Spielregel ändert sich.

Umgehängt auf Rust (B-1143, 2026-11-19): Das Spiel ist `rust/crates/k3c-game`; die Go-Vorlage steht nur im Tag `go-referenz-final` (Entscheidung 013). B-888 ist verworfen (Cheat-Fenster RU56); die Debug-Werte (F10) müssen zum Fenster aus RU56 passen, darum die Abhängigkeit.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `rust/crates/k3c-game/src/devtools/**`, `rust/crates/k3c-game/src/hud/**`, `data/*.json`
- **Abhängig von:** RU56
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

- Nacht 3 endet → eine Zeile 🏹 mit gespawnt 12, getötet 10, Verluste 2.
- Spiel verlassen → `reports/session-<zeit>.json` mit Seed, Dauer, Tagen, Ursache „verlassen“.

## Ausnahme- und Fehlerfälle

- `reports/` nicht beschreibbar → 💥-Warnung im Log, Spiel läuft weiter.
- Kein Objekt ausgewählt → Abschnitt 5 zeigt „–“.

## Akzeptanzkriterien

- **AC-01** Die Werte-Seite des F10-Overlays zeigt die Abschnitte 1–4 aus den Werten der Sim (Go-Test der Zusammenstellung).
- **AC-02** Ausgewähltes Objekt mit ID, Zustand, Ziel, Lebenspunkten (Go-Test der Auswahl).
- **AC-03** Zustandswechsel, Kampf je Nacht, Wirtschaft je Tag, Takt über Budget werden nur bei Wechsel geloggt (Go-Test: kein Log bei unverändertem Zustand).
- **AC-04** Am Ende liegt die Sitzungszusammenfassung als JSON in `reports/` (Go-Test).
- **AC-05** Sichtprüfung durch 🧑.

## Offene Fragen

Keine (Umfang von 🧑 am 2026-11-14 freigegeben: „genau so umsetzen“).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
