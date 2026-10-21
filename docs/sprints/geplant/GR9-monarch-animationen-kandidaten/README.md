# GR9 · DEV · Monarch-Animationen für Wurf, Treffer, Tod und Respawn, Kandidaten-Import ohne roten Test

- **Status:** geplant
- **Projekt:** GRA
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-420, B-505
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

Reihenfolge DEV → CLI. **Abhängigkeit:** nach GR8 (Pipeline, Stil DB32/16 px). Lizenzen in den CREDITS-Dateien, Skalierung ganzzahlig (GR7.4), Atlas-Budget Xbox (GR4). Neue Animationen in `data/sprites.json`.

## Beispiele

Import einer Datei nach `public/grafik/kandidaten/` mit Credit → `task check` grün, `lizenzen.html` nennt sie. Monarch wird getroffen → Treffer-Animation statt nur Effekt.

## Ausnahme- und Fehlerfälle

Sheet ohne passende Animation → bisheriger Effekt als Rückfall. Kandidat ohne Credit-Zeile → Test bleibt rot.

## Akzeptanzkriterien

- **AC-01** Kandidaten unter `public/grafik/kandidaten/` mit Credit lassen `grafikPacks.test.ts` grün, `lizenzen.html` nennt sie (B-420/AC-01, B-420/AC-02).
- **AC-02** Der Monarch hat Animationen für Wurf, Treffer, Tod und Respawn in Sheet und `data/sprites.json`; das Spiel zeigt sie, 🧑 nimmt am TV ab (B-505).

## Offene Fragen

🧑: `kandidaten/` als eigener Eintrag in `GRAFIK_PACKS` oder vom Test ausgenommen (B-420)? Woher kommen die Monarch-Animationen (anderes Sheet, eigene Zeichnung)?

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- GR9.1 (DEV) Kandidaten-Import hält `task check` grün (AC-01).
- GR9.2 (CLI, Mensch) Workshop: Quelle der Monarch-Animationen wählen (AC-02).
- GR9.3 (CLI) Animationen in Sheet und `data/sprites.json`, Einbau im Spiel (AC-02).
- GR9.4 (CLI) Review (Code-Sprint): alle Kriterien prüfen.
- GR9.5 (CLI, Mensch) Abnahme am TV (AC-02).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
