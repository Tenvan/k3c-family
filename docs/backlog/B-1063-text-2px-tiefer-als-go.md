# B-1063 · Rust setzt Text im Software-Rasterer rund 2 px tiefer als Go

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU16
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Im Bildvergleich RU16.6 (Stand Baustelle) liegt Text (Namen, Preise, Wartegründe) in Rust rund 2 px tiefer als in Go (Platzierungs-Code props/paint identisch; vermutet: Schrift-Layout in k3c-platform ttf/frame_canvas, Ascent und Zeilenabstand gegen Ebitengine text/v2). Die Textfelder sind darum im Pixelvergleich maskiert (tests/still_props.rs).

## Ziel

Text steht in Rust an derselben Stelle wie in Go; die Maske im Bildvergleich entfällt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Der Bildvergleich Stand Baustelle besteht ohne Textmaske in den Schwellen von RU13.6.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Befund (Claude, 2026-11-18, gemessen mit einem Go-Test auf `text.GoTextFace` und einem Rust-Test auf `ttf::Atlas`):

- Ascent und Zeilenhöhe sind gleich: Go `HAscent` 7,559 / 9,448 / 11,338 / 15,117 / 22,676 / 30,234 (Größen 8, 10, 12, 16, 24, 32), Rust `Fonts::ascent` dieselben Werte. Ursache liegt nicht in Metrik oder Zeilenabstand.
- Ursache der Tiefe: Ebitengine rundet die Grundlinie ab (`text/v2/gotext.go`, `goTextGlyphImageInfo`: `origin.Y &^= (1<<6)-1`), `ttf::Atlas::layout` rundete `y + ascent + dy` zum nächsten Pixel. Bei Nachkommaanteil ab 0,5 (Größe 8: 7,56; Größe 24: 22,68) steht Rust dadurch genau 1 px tiefer. Gemessene Glyph-Oberkanten Go gegen Rust (H): Größe 8: 1 / 2, Größe 10: 1 / 1, Größe 12: 2 / 2, Größe 16: 3 / 3, Größe 24: 4 / 5.
- Fix umgesetzt: senkrecht `floor` statt `round` (`k3c-platform/src/ttf.rs`), Test `ttf_grundlinie_wird_abgerundet_wie_in_go`.
- Ungeprüft: Die Spec nennt rund 2 px; belegt sind höchstens 1 px aus der Rundung. Der Rest kann aus dem Rasterer kommen (Go rastert die Umrisse mit Subpixel-Versatz und legt ein Zeilen-Margin von 1 px an, fontdue rastert auf ganzer Grundlinie). Der Bildvergleich ohne Maske (AC-01) braucht die gerenderte Grafik (`gen:paper`, hier gesperrt) und steht aus; die Maske `text_masks` in `tests/still_props.rs` bleibt, bis er mit dem Fix gelaufen ist (Abnahme B-1059).
- Waagrecht rundet Go auf `adjustGranularity`, Rust auf ganze Pixel: nicht untersucht.
