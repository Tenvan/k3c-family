# GR8 · CLI, DEV · Eigene Figuren für neue Gegner und Grafik-Pipeline mit Ziel-Palette

- **Status:** erledigt
- **Projekt:** GRA
- **Domäne:** CLI, DEV
- **Reife:** bereit
- **Tickets:** B-329, B-331
- **Start-Commit:** 013dcb34
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-22, Revision 1

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

Entschieden autonom (Claude, 2026-10-22), 🧑 prüft im Entscheidungs-Sprint GR10:
- Figuren für die sechs Gegner: GR7.3 (Auswahl durch 🧑) ist offen; Default = ungenutzte Sheets aus `public/sprites/` je Gegner (Feuergeist, Lavaschleim, Splitterwicht, Kristallwächter) und zwei selbst erzeugte Sheets (Eisenkäfer, Kristallspinne, eigene Arbeit, CC0).
- Pipeline: DB32 für alle Packs, Ergebnis getrennt von der Quelle in einem eigenen Ordner; das Spiel lädt weiter die Quellen, das Umschalten (B-331/AC-04, TV-Abnahme) entscheidet 🧑.
- Domäne der Pipeline: DEV (Go-CLI unter `tools/`).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GR8.1 | `GR8.1-figuren-neue-gegner.md` | Umsetzung | autonom | fertig |
| GR8.2 | `GR8.2-grafik-pipeline.md` | Umsetzung | autonom | fertig |
| GR8.3 | `GR8.3-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

- 2026-10-22, Review GR8.3 (Reviewer-Agent, Diff `origin/develop...sprint/gr8`): keine schweren Befunde; AC-01 (GR8.1) und AC-02 (GR8.2) mit Nachweis in den Session-Ergebnissen. `task check` und `task check:go` grün; `check:race` nicht gefahren (CI).
- Offen für 🧑 (Sprint GR10): Figuren-Auswahl und Optik am TV, Stil-Bibel-Freigabe, Umschalten des Spiels auf `public/grafik-db32/` (B-331/AC-04). B-329 und B-331 bleiben deshalb `eingeplant`.
- Version: v0.18.0 vorgeschlagen (Minor: neue Figuren, Werkzeug), gesetzt erst nach Bestätigung durch 🧑.
