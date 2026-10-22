# GR10 · CLI · Autonome Grafik-Entscheidungen prüfen (GR8, GR9)

- **Status:** geplant
- **Projekt:** GRA
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Claude hat GRA am 2026-10-22 komplett autonom umgesetzt (GR8, GR9) und dabei offene Fragen und Specs selbst entschieden. Dieser Sprint legt 🧑 die kritischen Entscheidungen vor; 🧑 ändert, was nicht gefällt.

## Ziel

Jede autonome Entscheidung ist von 🧑 bestätigt oder geändert; B-329, B-331, B-420 und B-505 sind abgeschlossen oder mit Folgeticket versehen.

## Beteiligte und Zielgruppen

🧑 entscheidet und nimmt am TV ab; Agent setzt Folgen um.

## Anforderungen

Entscheidungen (Default von Claude, Alternative):

1. **Gegner-Figuren (GR8.1, B-329):** Default: lavaSlime=fire-worm, fireSpirit=cemetery-ghost, shardling=flying-eye, crystalGuardian=skeleton (je getönt); ironBeetle und crystalSpider selbst erzeugt (`tools/spritegen`, 32×24, DB32). Alternative: Auswahl aus den GR7.3-Kandidaten oder Fremdgrafik/Bild-KI für Käfer und Spinne.
2. **Skalierung:** lavaSlime (×1,75) und crystalGuardian (×2,5 = 2×1,25) nicht ganzzahlig (B-251); Default: so lassen bis GR7.4 am TV.
3. **Stil-Bibel (GR8.2):** Entwurf mit DB32, 16 px, Outline und Licht oben links; 🧑 gibt frei oder ändert (B-331/AC-01). Die Pipeline erzwingt Outline und Licht nicht.
4. **Umschalten auf `public/grafik-db32/` (B-331/AC-04):** Default: das Spiel lädt weiter die Quellen; Umschalten und TV-Abnahme entscheidet 🧑. Dazu: `greentree` (10× hochskaliert, schrumpft auf 16×16: Lücke, neu zeichnen oder Bild-KI), `castle-tower` (512 px, keine Hochskalierung erkannt) und `woodwall` (weiche Kanten, ggf. Ausnahme `alpha-weich`). Auswahl der 57 Assets in `docs/assets/pipeline-liste.json` und Regel „Hochskalierung zählt erst ab 8 px Restkante“ bestätigen.
5. **DB32-Doppelung:** Palette steht in `tools/pipeline/palette.go` und `tools/buildings/db32.go`; ein gemeinsames Paket ist ein eigenes Ticket, falls gewünscht.
6. **`kandidaten/` (GR9.1, B-420):** Default: kein Pack, Pack-Test nimmt den Ordner aus. Alternative: eigener Eintrag in `GRAFIK_PACKS`.
7. **Monarch-Animationen (GR9.2/9.3, B-505):** Default: aus vorhandenen Frames abgeleitet (give = erste Attack-Frames, hit = Blitz + Rückstooß, death = Kippen, respawn = Einwachsen), keine Fremdquelle. Alternative: gezeichnete oder zugekaufte Sheets. Abnahme am TV: GR9.5.
9. **Offene Sessions nur für 🧑 (nicht autonom machbar):** GR7.3 (Figuren-Kandidaten wählen, Lücken unter `public/sprites/`), GR7.4 (Skalierung am TV), GR9.5 (Monarch-Animationen am TV). Sie sind Abnahmen am Gerät bzw. Auswahl durch 🧑 (`arbeitsweise.md` › SDD, Hardware entkoppelt); Claude hat sie nicht ausgeführt. Sprints GR7 und GR9 bleiben deshalb `aktiv`, GRA nicht `erledigt`.
8. **Autonom freigegebene Specs:** GR8 und GR9 jeweils Revision 1 („autonom, Claude, 2026-10-22“) bestätigen oder ändern.

## Nicht-Ziele

Neue Gegner, Sound, Atlas-Budget (GR4).

## Regeln und Einschränkungen

Manuelle Abnahmen bleiben bei 🧑 (`docs/arbeitsweise.md` › SDD). Erst nach GR8 und GR9.

## Beispiele

🧑 wählt für den Eisenkäfer ein Fremd-Sheet → neue Session ersetzt `iron-beetle`.

## Ausnahme- und Fehlerfälle

🧑 lehnt eine Entscheidung ab → Folgeticket mit der Alternative; der Default bleibt bis dahin.

## Akzeptanzkriterien

- **AC-01** Zu jeder der acht Entscheidungen steht im Ergebnis von GR10.1 „bestätigt“ oder „geändert (Ticket)“.
- **AC-02** B-329, B-331, B-420, B-505 haben Status und Folgetickets passend zu den Entscheidungen.

## Offene Fragen

Alle in Anforderungen gelistet; keine weiteren.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GR10.1 | `GR10.1-entscheidungen.md` | Workshop | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
