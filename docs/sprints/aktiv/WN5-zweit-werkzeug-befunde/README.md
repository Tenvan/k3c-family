# WN5 · DEV · Werkzeug-Befunde WE6 beheben

- **Status:** aktiv
- **Projekt:** WZN
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-1266, B-1291
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-26, Klasse A, Claude

## Ausgangslage

B-1266 sammelt Befunde aus WE6.5.

## Ziel

`task check:dev` ist grün und die neuen Welt-Felder sind in der MCP-Beschreibung samt Test. Am Ende sichtbar: grüner Prüflauf.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Verweise auf die Ticket-Specs (`B-009 › Anforderungen`) statt Kopien, dazu nur sprint-eigene Anforderungen.

## Nicht-Ziele

Die Go-Testfehler in `engine/**` (B-1266/AC-03): Bereich von RU41. B-1260 wartet auf die Entscheidung 013.

## Regeln und Einschränkungen

Nicht in den Bereich von RU41 greifen (`engine/**`, `cmd/**`, `testdata/**`, `Taskfile.yml`, `go.mod`).

- **Bereich:** `tools/k3c-workbench/internal/**`, `tools/k3c-studio/internal/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `check_run task:check:dev` ist grün (`B-1266/AC-01`).
- **AC-02** Ein MCP-Roundtrip-Test deckt `endBoss`, `skins`, `events` ab, die Tool-Beschreibung nennt sie (`B-1266/AC-02`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WN5.1 | `WN5.1-befunde-we6.md` | Umsetzung | autonom | fertig |
| WN5.2 | `WN5.2-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
