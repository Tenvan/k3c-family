# MG1 · CLI · Papierschnitt-Migration: Monarch und Spielerfarben

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

Welle 2, Lane `monarch` (Domäne PAP ab PV1.1; bis das Werkzeug PAP kennt, als CLI geführt). Läuft parallel zu den anderen MG-Sprints, sobald RD1 und MC1 erledigt sind. Quelle: Fixture `tools/paper/fixtures/specs/monarch.paper.json`; Spieler-Sheets aus `data/sprites.json` › `players`.

## Ziel

Als Spec über MCP angelegt und gerendert: Monarch als `figur` mit allen Animationen und Reiten; Spielerfarben als Varianten `spieler1` bis `spieler4`.

Am Ende sichtbar: alle Assets der Gruppe im Katalog von Assets, Review ohne roten Eintrag.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Entscheidung 005 › 2–4; `docs/standards/paper-spec.md`; Arbeit ausschließlich über `assets_guide`, `assets_create`, `assets_set`, `assets_preview`, `assets_review`.

## Nicht-Ziele

Spiel lädt die neuen Grafiken noch nicht (US1); Änderungen am Renderer oder neue Rig-Typen nur als Ticket an RD1.

## Regeln und Einschränkungen

Nur `public/grafik/figuren/` (Specs, gerenderte PNGs, Manifeste, `index.paper.json`, `golden.json` der Gruppe). Kein `assets_style_set`: fehlende Papier-Materialien als Ticket für die Lane `stil` bzw. Default in B-701. Jede Session prüft jedes Asset in jeder Animation mit `assets_preview`, Befunde im Ergebnis.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Jedes Asset der Gruppe hat eine gültige Spec (`assets_list` ohne `ungültig`).
- **AC-02** `task gen:paper` erzeugt alle Animationen der Gruppe; Golden-Hashes eingetragen.
- **AC-03** Jedes Asset in jeder Animation mit `assets_preview` selbst geprüft, Befunde im Ergebnis; `assets_review` ohne roten Eintrag. Sichtprüfung durch 🧑 in PS1.

## Offene Fragen

keine

## Sessions

Reife Entwurf, Lane `monarch`; **MG1.1 Integration (Sonnet):** erstes echtes Asset durchgängig mit echtem Renderer nur über `assets_guide`, `assets_create`, `assets_set`, `assets_preview` (005/AC-05), Befunde als Tickets an RD1/MC1; Zuschnitt beim Bereitmachen nach Umfang (je Session 3–6 Assets), dazu ein Review (Opus); keine Mensch-Session (Sichtprüfung in PS1). Umsetzung Sonnet.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
