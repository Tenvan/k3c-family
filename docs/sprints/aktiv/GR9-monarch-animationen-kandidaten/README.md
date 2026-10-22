# GR9 · DEV, CLI · Monarch-Animationen für Wurf, Treffer, Tod und Respawn, Kandidaten-Import ohne roten Test

- **Status:** aktiv
- **Projekt:** GRA
- **Domäne:** DEV, CLI
- **Reife:** bereit
- **Tickets:** B-420, B-505
- **Start-Commit:** 013dcb34
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-22, Revision 1

## Ausgangslage

Ein erster echter Import nach `public/grafik/kandidaten/` macht `src/tools/grafikPacks.test.ts` rot, weil jede Datei zu einem Pack aus `GRAFIK_PACKS` gehören muss (B-420). Das Sheet `medieval-king` hat nur idle, run und attack; Münze geben, Treffer, Tod und Respawn zeigt das Spiel nur über Effekte bzw. Deckkraft 0,25 (B-505, Lücken 1–4 der Monarch-Seite).

## Ziel

Kandidaten lassen sich importieren, ohne `task check` zu brechen, und der Monarch hat eigene Animationen für Wurf, Treffer, Tod und Respawn. Am Ende sichtbar: Monarch-Seite ohne Lücken 1–4, Animationen im Spiel am TV, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

Spielende am TV, zwei Spieler; 🧑 wählt Sheets oder Animationen; Agent in DEV und CLI.

## Anforderungen

B-420 › Akzeptanzkriterien; B-505 › Ausgangslage (Specs noch Entwurf).

## Nicht-Ziele

Figuren für Gegner (GR8); Skill-Darstellung (SK2); Grafik-Pipeline mit Ziel-Palette (GR8, B-331).

## Regeln und Einschränkungen

Reihenfolge DEV → CLI. GR8 ist abgeschlossen (PR #434 offen); Pipeline-Ergebnisse werden hier nicht verwendet (Umschalten bleibt 🧑). Lizenzen in den CREDITS-Dateien, Atlas-Budget Xbox (GR4). Neue Animationen in `data/sprites.json`.

## Beispiele

Import einer Datei nach `public/grafik/kandidaten/` mit Credit → `task check` grün, `lizenzen.html` nennt sie. Monarch wird getroffen → Treffer-Animation statt nur Effekt.

## Ausnahme- und Fehlerfälle

Sheet ohne passende Animation → bisheriger Effekt als Rückfall. Kandidat ohne Credit-Zeile → Test bleibt rot.

## Akzeptanzkriterien

- **AC-01** Kandidaten unter `public/grafik/kandidaten/` mit Credit lassen `grafikPacks.test.ts` grün, `lizenzen.html` nennt sie (B-420/AC-01, B-420/AC-02).
- **AC-02** Der Monarch hat Animationen für Wurf, Treffer, Tod und Respawn in Sheet und `data/sprites.json`; das Spiel zeigt sie, 🧑 nimmt am TV ab (B-505).

## Offene Fragen

Entschieden autonom (Claude, 2026-10-22), 🧑 prüft im Entscheidungs-Sprint GR10: `kandidaten/` ist kein Pack, der Pack-Test nimmt den Ordner aus; Monarch-Animationen werden aus vorhandenen Frames abgeleitet (Blitz, Kippen, Einwachsen), keine neue Fremdquelle; Wurf nutzt die ersten Attack-Frames.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GR9.1 | `GR9.1-kandidaten-import.md` | Umsetzung | autonom | fertig |
| GR9.2 | `GR9.2-monarch-sheets.md` | Umsetzung | autonom | fertig |
| GR9.3 | `GR9.3-monarch-einbau.md` | Umsetzung | autonom | fertig |
| GR9.4 | `GR9.4-review.md` | Review | autonom | offen |
| GR9.5 | `GR9.5-abnahme-tv.md` | Umsetzung | Mensch | offen |

- GR9.1 Kandidaten-Import hält `task check` grün (AC-01).
- GR9.2 Sheets für give/hit/death/respawn aus vorhandenen Frames (AC-02). Der geplante Workshop zur Quellenwahl entfällt: autonom entschieden, Ableitung statt Fremdgrafik.
- GR9.3 Einbau im Client (AC-02).
- GR9.4 Review (Code-Sprint).
- GR9.5 Abnahme am TV, 🧑 (AC-02).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
