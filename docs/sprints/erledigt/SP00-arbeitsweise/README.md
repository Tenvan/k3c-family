# SP00 · INF · Arbeitsweise einführen

- **Status:** erledigt
- **Domäne:** INF
- **Prio:** hoch
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
| SP00.5 | `SP00.5-review.md` | Review | autonom | fertig |

## Abnahme

**2026-09-30, SP00.5 (autonomer Agent).** Geprüft: 85 Dateien aus `git diff --stat df6e1de..origin/main -- docs
tests/planning.test.ts CLAUDE.md README.md .github` (86 Einträge; `docs/game-design.md` nicht, dort nur Reittiere aus
PR #15; `docs/sessions.md` als Löschung geprüft). Jede Datei vollständig gelesen, migrierte Tickets und Sprints gegen
ihren Stand vor SP00.4 (`55d15cb`) verglichen.

- **AC-01 geprüft:** `docs/arbeitsweise.md` beschreibt Ticket → Sprint → Session, Review-Session mit Checkliste und
  Komplexitäts-Budget; `docs/backlog/README.md` und `.github/pull_request_template.md` liegen vor.
- **AC-02 geprüft, mit Einschränkung:** Entscheidung 001 hat `Status: beschlossen` (Go einzige Engine, Wails optional),
  B-003/AC-01 erfüllt. B-001/AC-01 nur teilweise: „Standardbibliothek (`net/http`)“ steht nicht in 001, nur im alten
  Backlog (`4c2e4d7`) → B-048 (Frage an 🧑).
- **AC-03 geprüft:** `npm test` grün; `tests/planning.test.ts` nimmt Felder und Überschriften aus `docs/vorlagen/`,
  prüft Status ↔ Ordner, Index und Verweise (B-044/AC-01).
- **AC-04 geprüft:** Vorlagen haben Spec-Abschnitte und -Felder (B-045/AC-01); `tests/planning.test.ts` prüft
  Abschnitte, `Spec`-Werte, AC-Folge, `Freigabe` ↔ `freigegeben`, `B-NNN/AC-NN`- und Session-Verweise, Abdeckung und
  aktive Sprints (B-045/AC-02, durch Lesen des Tests, keine Probe); alle Tickets und Sprints migriert, `npm test` grün
  (B-045/AC-03); Abschnitt SDD in `docs/arbeitsweise.md` (B-045/AC-04).

**Behobene Befunde:** B-045 stand nach Umsetzung noch auf `eingeplant`/`Entwurf` → `erledigt`/`rückwirkend` (auch Index).
SP01.2: „Fertig, wenn“-grep war unerfüllbar (Treffer in `docs/sprints/`, `docs/roadmap.md`, `public/sprites/CREDITS.md`)
→ Ausnahme `docs/sprints/`, beide Dateien in Doku-Liste und Erlaubte Dateien; „9 Dateien“ im Root von `data/` → 8 JSON-Dateien.
SP03: AC-02 verlangt `/api/health` „wie heute“, das es nicht gibt → Offene Frage (Kriterium unverändert); Regel verwies
für `net/http`/`log/slog` auf 001 → B-001. SP10.2 ohne „Log folgen“ (B-002/AC-03) → ergänzt. B-008 erfand
„🧑 leitet den Abend“ → entfernt. ALT zählte Reittier-Sprites zum Stand `df6e1de` → korrigiert. SP00.4-Ergebnis
„14 Sprints“ → 15. Entscheidung 001 „Sprint SP2“ → SP02. Roadmap „Schritt 0 (als Nächstes!)“ widersprach dem Fahrplan
→ „einschiebbar, Sprint X1“. B-044: alte Dateinamen nur noch als Text, ohne `docs/`-Pfad.

**Neue Tickets:** B-048 (Standardbibliothek in 001 oder B-001 ändern), B-049 (SP09 und SP03 überschreiten ihre Domäne).
Belassen: B-004/B-013 nennen in der Ausgangslage schon `data/` (erledigt sich mit SP01.2); B-037 › Nicht-Ziele
beschreibt den SP08-Umfang statt einer Ticket-Grenze.
