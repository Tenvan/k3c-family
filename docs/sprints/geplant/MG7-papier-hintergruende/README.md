# MG7 · CLI · Papierschnitt-Migration: Hintergründe je Biom

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

Welle 2, Lane `hintergruende` (Domäne PAP ab PV1.1; bis das Werkzeug PAP kennt, als CLI geführt). Läuft parallel zu den anderen MG-Sprints, sobald RD1 und MC1 erledigt sind. Biome aus `data/biomes/*.json`; Varianten-Schema heute `hg-<biom>-<fern|mitte|nah>-v<n>` (`tools/buildings/hires.go`).

## Ziel

Als Spec über MCP angelegt und gerendert: Hintergründe je Biom als `hintergrund` mit Ebenen fern, mitte, nah und ihren Varianten.

Am Ende sichtbar: alle Assets der Gruppe im Katalog von Assets, Review ohne roten Eintrag.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Entscheidung 005 › 2–4; `docs/standards/paper-spec.md`; Arbeit ausschließlich über `assets_guide`, `assets_create`, `assets_set`, `assets_preview`, `assets_review`.

## Nicht-Ziele

Spiel lädt die neuen Grafiken noch nicht (US1); Parallax-Logik bleibt im Client.

## Regeln und Einschränkungen

Nur `public/grafik/hintergruende/`. Kein `assets_style_set`: fehlende Papier-Materialien als Ticket für die Lane `stil` bzw. Default in B-701; erlaubt sind auch `index.paper.json` und `golden.json` der Gruppe. Jede Session prüft jede Ebene und Variante mit `assets_preview`.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Jede Ebene und Variante hat eine gültige Spec.
- **AC-02** `task gen:paper` erzeugt alle Ebenen; Golden-Hashes eingetragen.
- **AC-03** Jedes Asset in jeder Animation mit `assets_preview` selbst geprüft, Befunde im Ergebnis; `assets_review` ohne roten Eintrag. Sichtprüfung durch 🧑 in PS1.

## Offene Fragen

keine

## Sessions

Reife Entwurf, Lane `hintergruende`; Zuschnitt je Biom, dazu ein Review (Opus); keine Mensch-Session (Sichtprüfung in PS1). Umsetzung Sonnet.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
