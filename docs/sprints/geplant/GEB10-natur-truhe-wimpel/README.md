# GEB10 · DEV · Natur-Nachträge: hohe Bäume, Höhlen-Flora, geschlossene Truhe, Wimpel in Spielerfarbe

- **Status:** geplant
- **Projekt:** GEB
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-454, B-455, B-456, B-457
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Aus der Abnahme GEB6 (🧑, 2026-10-17): Bäume sind zu niedrig und stehen zu locker, in Höhlen passen Wald-Bäume nicht (B-454, `public/grafik/gebaeude/`). Die Truhe gibt es nur offen (`truhe.json`, B-455). Markierungsfahnen über Knoten sind neutral (`worldRenderer.ts`, B-456). Ausgang-Beschriftung, Split-Screen und vier Geschmacksfragen hat 🧑 noch nicht am laufenden Spiel geprüft (B-457).

## Ziel

Natur und Objekte wirken wie gewünscht: hohe, dichte Bäume, eigene Höhlen-Flora, offene und geschlossene Truhe, Wimpel in Spielerfarbe. Am Ende sichtbar: Vorschau im Asset-Studio und Spiel mit zwei Spielern, von 🧑 abgenommen, GEB6-Restpunkte geprüft.

## Beteiligte und Zielgruppen

Spielende am TV, zwei Spieler; Agent in DEV (Grafik-Werkstatt) und CLI (Zeichnen); 🧑 legt Maße fest und nimmt ab.

## Anforderungen

B-454, B-455, B-456, B-457 › Akzeptanzkriterien (Specs noch Entwurf).

## Nicht-Ziele

Hub-Ausbaustufen (GEB9, GEB11); Truhen-Zustand in der Simulation (falls nötig, eigenes SIM-Ticket, B-455/AC-02); neue Biome.

## Regeln und Einschränkungen

Reihenfolge DEV → CLI. Grafik nach `docs/standards/gebaeude-grafik.md` (Daten-JSON und PNG 16/32/64/128), Lizenzen in den CREDITS-Dateien. Spielerfarbe aus der vorhandenen Farbliste. 2 Spieler im Split-Screen; Atlas-Budget Xbox (GR4).

## Beispiele

Wald → Bäume etwa 3/4 der Szenenhöhe, dichter als heute. Höhle → Wurzeln an Wand und Boden statt umgefärbter Bäume. Spieler 2 markiert einen Baum → Wimpel in seiner Farbe.

## Ausnahme- und Fehlerfälle

Server kennt keinen Zustand „geschlossen“ → Truhe nur in der Vorschau, Spiel-Zuordnung begründet verschoben (Ticket SIM). Fehlende Grafik → Platzhalter-Rückfall wie heute.

## Akzeptanzkriterien

- **AC-01** Bäume höher und dichter, Höhlen-Flora als Daten-JSON und PNG, im Spiel nur in der Höhle (B-454/AC-01, B-454/AC-02).
- **AC-02** Die Truhe hat eine geschlossene Variante, die Vorschau zeigt beide; Spiel-Zuordnung oder begründet verschoben (B-455/AC-01, B-455/AC-02).
- **AC-03** Die Markierungsfahne ist ein Wimpel in Spielerfarbe, Vitest für die Farbwahl (B-456/AC-01).
- **AC-04** 🧑 hat Vorschau und Spiel abgenommen: Bäume, Flora, Truhe, Wimpel mit zwei Spielern sowie Ausgang-Beschriftung, Split-Screen und Geschmacksfragen aus GEB6 (B-454/AC-03, B-456/AC-02, B-457/AC-01, B-457/AC-02, B-457/AC-03).

## Offene Fragen

🧑: Maß und Dichte der Bäume (Regeldatei § 1); Höhlen-Flora auch in der Mine? Wann zeigt das Spiel die geschlossene Truhe (B-455)?

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- GEB10.1 (DEV) Bäume höher und dichter, Höhlen-Flora (AC-01).
- GEB10.2 (DEV) Geschlossene Truhe (AC-02).
- GEB10.3 (CLI) Wimpel in Spielerfarbe, Höhlen-Flora und Truhe im Spiel zuordnen (AC-01, AC-02, AC-03).
- GEB10.4 (CLI) Review (Code-Sprint): alle Kriterien prüfen.
- GEB10.5 (CLI, Mensch) Abnahme in Vorschau und Spiel mit zwei Spielern, GEB6-Restpunkte (AC-04).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
