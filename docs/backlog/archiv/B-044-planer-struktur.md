# B-044 · Sprints und Tickets liegen als Dateien nach Pflicht-Vorlagen

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** mittel
- **Status:** erledigt
- **Sprint:** SP00
- **Erstellt:** 2026-09-30
- **Spec:** rückwirkend
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Vorher standen alle Sprints in einer Datei `sprints.md` und das Backlog in einer Tabelle `backlog.md` (beide unter `docs/`, in SP00.3 entfernt); nichts erzwang eine einheitliche Form.

## Ziel

Sprints und Tickets liegen als Dateien nach Pflicht-Vorlagen. Nutzen: Cloud-Agenten sollen Sessions ohne Planungs-Werkzeuge abarbeiten; nur Aktives soll gelesen werden.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten, die Sessions autonom abarbeiten; Review-Session.

## Anforderungen

- Sprints in `docs/sprints/{geplant,aktiv,erledigt}/` mit eigenem Ordner, Tickets als Dateien in `docs/backlog/`, Vorlagen in `docs/vorlagen/`.
- `tests/planning.test.ts` prüft Vorlagen, Status, Index und Verweise.

## Nicht-Ziele

Spec-Inhalte (B-045).

## Regeln und Einschränkungen

Prozess nur in `docs/arbeitsweise.md`; keine neue Abhängigkeit ohne Ticket und Zustimmung im Review; Komplexitäts-Budget.

## Beispiele

Ein Cloud-Agent liest nur `sprints/aktiv/` → findet die nächste offene Session.

## Ausnahme- und Fehlerfälle

Planungs-Datei weicht von der Vorlage ab → `npm test` scheitert mit Datei und Feld.

## Akzeptanzkriterien

- **AC-01** `npm test` scheitert bei fehlenden Feldern, falschem Status oder fehlenden Index-Einträgen.

## Offene Fragen

keine

## Notizen

Umgesetzt in SP00.3.
