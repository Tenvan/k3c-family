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

Geprüft 2026-11-18 (Claude), nicht umgesetzt, Ursache nicht belegt. Befund: Oberlänge der Schrift ist in Rust und Go gleich (`k3c.ttf`, 24 px: 22,676; fontdue gegen Go `HAscent`), Go setzt die Grundlinie bei `y + HAscent` (`text/v2` layout.go, oben ausgerichtet), Rust bei `y + ascent` (`ttf.rs`): die Formel in der Plattform stimmt. Vermutung (ungeprüft): Die 2 px kommen aus der zu niedrigen Zeilenhöhe in `art/font.rs` (B-1065, jetzt behoben): unten verankerte Elemente stehen dadurch 2,5 bis 2,9 px tiefer. Für Welttexte (`props/paint.rs`, y unabhängig von der Zeilenhöhe) erklärt das die Abweichung nicht. Prüfen und Maske entfernen geht nur mit erzeugter Grafik (`gen:paper`, in diesem Lauf gesperrt): `tests/still_props.rs` und `tests/still_hud.rs` nach dem Merge von B-1065 ohne Maske laufen lassen; bleibt eine Abweichung, Glyph-Raster (Hinting, `dy` aus `ymin`) gegen Go vergleichen.
