# GR8 · CLI · Eigene Figuren für neue Gegner und Grafik-Pipeline mit Ziel-Palette

- **Status:** geplant
- **Projekt:** GRA
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** B-329, B-331
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die sechs neuen Gegner aus K1.3 zeigen getönte Platzhalter-Sheets (B-329, `data/sprites.json`). Über 40 Packs unter `public/grafik/` haben eigene Paletten und Pixeldichten; der Stil DB32/16 px ist beschlossen, eine Pipeline fehlt (B-331). GR7 (aktiv) liefert die Figuren-Kandidaten.

## Ziel

Neue Gegner haben eigene Figuren, und alle Grafiken haben dieselbe Ziel-Palette und Pixeldichte. Am Ende sichtbar: Vorschau der Gegner und der Pipeline-Ausgabe auf `grafiken.html`, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

Spielende am TV; Agent in CLI; 🧑 wählt Figuren und nimmt die Optik ab.

## Anforderungen

B-329, B-331 › Anforderungen (Specs noch Entwurf). B-331 ist groß und kann beim Bereitmachen auf einen eigenen Sprint aufgeteilt werden.

## Nicht-Ziele

Neue Gegner oder Animationen über B-329 hinaus; Sound (SND); Asset-Verwaltung in k3c-workbench (M10).

## Regeln und Einschränkungen

Nach GR7 (aktiv). Lizenzen in den CREDITS-Dateien (Test GR6). Atlas-Budget auf der Xbox (GR4) bleibt. Skalierung ganzzahlig (GR7.4).

## Beispiele

Lavaschleim erscheint → eigene Figur statt getöntem `mushroom`. Neues Pack eingespielt → Pipeline bringt es auf Palette und 16 px.

## Ausnahme- und Fehlerfälle

Pack mit unpassender Pixeldichte → Pipeline lehnt ab und nennt das Pack. Fehlendes Sheet → Platzhalter mit Warnung 🤒.

## Akzeptanzkriterien

- **AC-01** Die sechs neuen Gegner zeigen je eine eigene Figur; die Zuordnungstabelle `docs/assets/zuordnung-objekte.md` ist aktualisiert (B-329).
- **AC-02** Eine Pipeline bringt Grafiken auf Ziel-Palette und Pixeldichte; Ergebnis und Regeln stehen in `docs/assets/` (B-331).

## Offene Fragen

🧑: Welche Figuren-Kandidaten aus GR7.3 sollen für die sechs Gegner gelten? Pipeline: Palette DB32 für alle Packs oder nur neue? Domäne der Pipeline (CLI oder DEV)?

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GR8.1 | `GR8.1-figuren-neue-gegner.md` | Umsetzung | autonom | offen |
| GR8.2 | `GR8.2-grafik-pipeline.md` | Umsetzung | autonom | offen |
| GR8.3 | `GR8.3-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
