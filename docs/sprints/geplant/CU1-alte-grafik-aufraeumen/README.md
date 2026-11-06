# CU1 · CLI · Cleanup zum Schluss: alte Pixel-Grafik, Register und Gebäude-Renderer entfernen

- **Status:** geplant
- **Projekt:** PAP
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** B-700
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Letzter Sprint des Projekts PAP, nach der Abnahme in PS1. Umfang: B-700.

## Ziel

Im Repo gibt es keine alte Pixel-Grafik und keinen Code mehr, der sie erzeugt oder lädt.

Am Ende sichtbar: `public/grafik-db32/`, `public/atlas/`, `tools/buildings/` fehlen; `task check:all` grün.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

B-700 › Anforderungen.

## Nicht-Ziele

Neue Grafik; Sound.

## Regeln und Einschränkungen

Gelöscht wird nur, was per Suche nachweislich ungenutzt ist. Sessions nacheinander nach Domäne.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** B-700/AC-01 bis AC-03 erfüllt.
- **AC-02** `task check:all` und `task buildall` grün.

## Offene Fragen

keine

## Sessions

Reife Entwurf:

- CU1.1 CLI · Sonnet: Packs, Atlas, `data/assets.json`-Register, `data/sprites.json`, Zuordnungs-Doku, CREDITS (AC-01)
- CU1.2 INF · Sonnet: `tools/buildings`, Tasks `gen:gebaeude`/`gen:atlas`, CI, Codemaps (AC-01, AC-02)
- CU1.3 Review · Opus

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
