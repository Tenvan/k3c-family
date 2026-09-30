# SP00 · INF · Arbeitsweise einführen

- **Status:** aktiv
- **Domäne:** INF
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-001, B-003, B-044, B-045
- **Start-Commit:** df6e1de
- **Spec:** rückwirkend
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Vorher gab es nur `docs/sessions.md` mit Phasen; nichts erzwang eine einheitliche Form, die Server-Frage war offen, und Kriterien hatten weder IDs noch Freigabe.

## Ziel

Das Projekt hat eine feste Arbeitsweise (Tickets → Sprints → Sessions, Review-Pflicht, Komplexitäts-Budget), eine
Architektur-Entscheidung für die Go-Engine und einen Planer, den ein Cloud-Agent ohne Planungs-Werkzeuge abarbeiten kann.
Am Ende sichtbar: `docs/arbeitsweise.md`, `docs/sprints/`, `docs/backlog/`, `tests/planning.test.ts` grün.

## Beteiligte und Zielgruppen

🧑 entscheidet (Workshop) und gibt Specs frei; Entwickler und Cloud-Agenten arbeiten danach; die Review-Session prüft.

## Anforderungen

B-001 und B-003 › Anforderungen (Server-Entscheidung), B-044 › Anforderungen (Planer), B-045 › Anforderungen (SDD).

## Nicht-Ziele

Lint, Go-Gerüst und Regel-Tests für Code (SP01, B-009, B-033). Kein Spiel-Code.

## Regeln und Einschränkungen

Prozess nur in `docs/arbeitsweise.md`; Vorlagen sind Pflicht, `tests/planning.test.ts` prüft sie. SP00 hat ausnahmsweise 5 Sessions: die SDD-Migration (SP00.4) kam nach der Planung dazu und soll mit ins Review.

## Beispiele

Ein Cloud-Agent liest nur `sprints/aktiv/` → findet die nächste offene Session und arbeitet sie ohne Rückfragen gegen ihre Kriterien ab.

## Ausnahme- und Fehlerfälle

Eine Planungs-Datei weicht von der Vorlage ab oder ein Kriterium hat keine Session → `npm test` scheitert mit Datei und Feld.

## Akzeptanzkriterien

- **AC-01** `docs/arbeitsweise.md` beschreibt Tickets → Sprints → Sessions, Review-Pflicht und Komplexitäts-Budget; Backlog und PR-Vorlage liegen vor.
- **AC-02** Entscheidung 001 (Go-Engine) ist beschlossen (B-001/AC-01, B-003/AC-01).
- **AC-03** Sprints und Tickets liegen als Dateien nach Pflicht-Vorlagen, `tests/planning.test.ts` prüft sie (B-044/AC-01).
- **AC-04** Tickets und Sprints sind Specs nach SDD, `tests/planning.test.ts` prüft Spec-Felder, Kriterien und Abdeckung (B-045/AC-01 bis B-045/AC-04).

## Offene Fragen

keine (Spec von SP01 am 2026-09-30 freigegeben)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SP00.1 | `SP00.1-plan-backlog.md` | Umsetzung | Mensch | fertig |
| SP00.2 | `SP00.2-server-entscheidung.md` | Workshop | Mensch | fertig |
| SP00.3 | `SP00.3-planer-struktur.md` | Umsetzung | Mensch | fertig |
| SP00.4 | `SP00.4-sdd.md` | Umsetzung | Mensch | fertig |
| SP00.5 | `SP00.5-review.md` | Review | autonom | offen |

## Abnahme

–
