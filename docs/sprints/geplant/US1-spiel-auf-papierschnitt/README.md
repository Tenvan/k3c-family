# US1 · CLI · Spiel lädt Papierschnitt-Grafik, alte Pixel-Assets und Gebäude-Renderer entfernt

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

Welle 3 nach RD1, MC1, WB1, RV1 und MG1–MG8. Das Spiel lädt heute Pixel-Sheets über `data/sprites.json`, `public/atlas/` und `public/grafik/` (`src/scenes/render/`).

## Ziel

Das Spiel lädt die Papierschnitt-PNGs über die Gruppen-Indizes `public/grafik/<gruppe>/index.paper.json`; Server-Watch und Server-Check kennen die neuen Dateien; Doku ist nachgezogen.

Am Ende sichtbar: Spiel im PC-Browser im Papierschnitt-Look (Abnahme in PS1).

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Entscheidung 005 › Umsetzung (Welle 3); B-701 D-09, D-11.

## Nicht-Ziele

Skalierung Xbox/Mobile (Folgeticket); Entfernen alter Grafik (CU1); Abnahme (PS1).

## Regeln und Einschränkungen

Sessions nacheinander nach Domäne: CLI → SRV → INF → Doku. Engine-Schatten nach B-701 D-09 (Default Phaser-Filter, sonst Sprite-Kopie).

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Das Spiel lädt jede Figur, jedes Gebäude und jeden Hintergrund aus den Gruppen-Indizes (Test in `tests/projectRules.test.ts`: kein Lade-Pfad auf `atlas/` oder `grafik-db32/`).
- **AC-02** `k3c-server -check-data` meldet fehlende Dateien aus einem Gruppen-Index, der alte Stand bleibt aktiv (Go-Test); der Watch ignoriert `*.paper.json` (Go-Test).
- **AC-03** `task check:all` grün.
- **AC-04** CLAUDE.md, `docs/arbeitsweise.md` und Codemaps beschreiben Papierschnitt, Workbench-Bereiche und den Wegfall des Studios.

## Offene Fragen

keine (Defaults in B-701)

## Sessions

Reife Entwurf:

- US1.1 CLI · Sonnet: Loader, Renderer, Engine-Schatten (AC-01)
- US1.2 SRV · Sonnet: Server-Check und Watch (AC-02)
- US1.3 INF · Sonnet: Tasks, CI (AC-03)
- US1.4 INF · Haiku: Doku und Codemaps (AC-04)
- US1.5 Review · Opus

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
