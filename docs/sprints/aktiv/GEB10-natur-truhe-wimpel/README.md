# GEB10 · DEV, CLI · Natur-Nachträge: hohe Bäume, Höhlen-Flora, geschlossene Truhe, Wimpel in Spielerfarbe

- **Status:** aktiv
- **Projekt:** GEB
- **Domäne:** DEV, CLI
- **Reife:** bereit
- **Tickets:** B-454, B-455, B-456, B-457
- **Start-Commit:** 013dcb34
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-22 (Auftrag von Ralf: GEB komplett autonom)

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

Autonom entschieden (Auftrag 🧑 2026-10-22: GEB komplett autonom), Meldung im Nachgang; 🧑 ändert, was nicht passt:

1. **Baummaß:** Szenenhöhe = sichtbare Welt über der Bodenlinie: `GROUND_Y` 900 − `WORLD_TOP` 420 = 480 px = **15 Kacheln** (so hoch wie die Hintergrund-Schichten, § 10). ¾ davon: Bäume **11,25 Kacheln** hoch (360 px), 2–4 Kacheln breit je Form (Stamm unten sichtbar, Krone oben). Begründung: 1 Kachel Breite trägt bei 11 Kacheln Höhe keine Krone.
2. **Dichte:** Vorschau-Reihe mit Schritt **1 Kachel** (bei 2–4 Kacheln Breite stark überlappend, heute ½ Kachel bei 1 Kachel Breite). Im Spiel wirkt der Wald dichter durch die breiteren Kronen; mehr Bäume je Abschnitt wären ein Wert in `data/biomes/forest.json` (REG, eigenes Ticket, nicht in diesem Sprint).
3. **Höhlen-Flora:** eigene JSONs statt Umfärbung: `wurzel`, `wurzel-2`, `wurzel-3` (Wurzeln hängen von der Decke über die Wand bis in den Boden, 15 Kacheln hoch = bis zur Oberkante des Bilds) und `hoehlenbusch` (1 × 1). Die Bäume verlieren ihren Höhle-Look (nur noch Wald). Im Spiel: Baum-Knoten außerhalb des Walds zeigen Wurzeln, Büsche außerhalb des Walds den Höhlenbusch; die Mine zeigt wie bisher den Höhle-Look (dort stehen heute keine Bäume).
4. **Geschlossene Truhe:** eigene JSON `truhe-geschlossen` (kein Stufen-Farbtausch, weil der Goldhaufen wegfällt). Der Server kennt keinen Zustand „offen“, braucht ihn aber nicht: Eine liegende Truhe ist ungeöffnet → **geschlossen**; verschwindet sie (aufgehoben), zeigt der Client kurz die offene Truhe, die dann ausblendet. Keine Änderung an `engine/`, `data/` oder Protokoll.
5. **Wimpel-Farbe:** aus `PLAYER_COLORS` (`src/core/constants.ts`). Der Snapshot kennt nur `marked`, nicht wer markiert hat; der Client merkt sich den Spieler aus dem Ereignis `coinGive` (`to: mark`) am nächsten Knoten. Unbekannt (Plantage, nach Neuladen) → neutrale helle Farbe. Ein Feld `markedBy` im Snapshot wäre genauer (SIM + Protokoll, eigenes Ticket, Entscheidung 🧑).
6. **Plantage-Pappel:** wird wie alle Bäume hoch und steht deshalb hinter der Farm statt davor (Setzling bleibt davor). **Setzling** bleibt 1 × 1 (junge Pflanze).
7. **B-457** (Ausgang-Beschriftung, Split-Screen, Geschmack) setzt kein Agent um: Prüfliste in GEB10.5.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB10.1 | `GEB10.1-hohe-baeume-hoehlenflora.md` | Umsetzung | autonom | fertig |
| GEB10.2 | `GEB10.2-truhe-geschlossen.md` | Umsetzung | autonom | fertig |
| GEB10.3 | `GEB10.3-wimpel-flora-truhe-im-spiel.md` | Umsetzung | autonom | fertig |
| GEB10.4 | `GEB10.4-review.md` | Review | autonom | fertig |
| GEB10.5 | `GEB10.5-abnahme.md` | Workshop | Mensch | blockiert |

- GEB10.1 (DEV) Bäume höher und dichter, Höhlen-Flora als JSON und PNG, Vorschau (AC-01).
- GEB10.2 (DEV) Geschlossene Truhe, Vorschau zeigt beide (AC-02).
- GEB10.3 (CLI) Wimpel in Spielerfarbe, Höhlen-Flora und Truhe im Spiel zuordnen, alte Höhle-Look-PNGs aufräumen (AC-01, AC-02, AC-03).
- GEB10.4 (CLI) Review (Code-Sprint): alle Kriterien prüfen.
- GEB10.5 (CLI, Mensch) Abnahme in Vorschau und Spiel mit zwei Spielern, GEB6-Restpunkte (AC-04).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
