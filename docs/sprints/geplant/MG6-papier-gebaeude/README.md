# MG6 · CLI · Papierschnitt-Migration: Gebäude mit Ausbaustufen

- **Status:** geplant
- **Projekt:** PAP
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** B-699
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Welle 2, Lane `gebaeude` (Domäne PAP ab PV1.1; bis das Werkzeug PAP kennt, als CLI geführt). Läuft parallel zu den anderen MG-Sprints, sobald RD1 und MC1 erledigt sind. Ist-Stand und Stufenlogik: `tools/buildings/codemap.md`, `docs/standards/gebaeude-grafik.md`, Daten `public/grafik/gebaeude/*.json`.

## Ziel

Als Spec über MCP angelegt und gerendert: alle Gebäude als `gebaeude` mit Ausbaustufen 1–5 als Delta.

Am Ende sichtbar: alle Assets der Gruppe im Katalog von Assets, Review ohne roten Eintrag.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Entscheidung 005 › 2–4; `docs/standards/paper-spec.md`; Arbeit ausschließlich über `assets_guide`, `assets_create`, `assets_set`, `assets_preview`, `assets_review`.

## Nicht-Ziele

Spiel lädt die neuen Grafiken noch nicht (US1); `tools/buildings` bleibt bis US1.

## Regeln und Einschränkungen

Nur `public/grafik/gebaeude-paper/`. Kein `assets_style_set`: fehlende Papier-Materialien als Ticket für die Lane `stil` bzw. Default in B-701; erlaubt sind auch `index.paper.json` und `golden.json` der Gruppe. Jede Session prüft jedes Gebäude in jeder Stufe mit `assets_preview`.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Jedes Gebäude hat eine gültige Spec mit allen Stufen.
- **AC-02** `task gen:paper` erzeugt alle Stufen; Golden-Hashes eingetragen.
- **AC-03** Jedes Asset in jeder Animation mit `assets_preview` selbst geprüft, Befunde im Ergebnis; `assets_review` ohne roten Eintrag. Sichtprüfung durch 🧑 in PS1.

## Offene Fragen

keine

## Sessions

Reife Entwurf, Lane `gebaeude`; Zuschnitt beim Bereitmachen (je Session 3–6 Gebäude), dazu ein Review (Opus); keine Mensch-Session (Sichtprüfung in PS1). Umsetzung Sonnet.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
