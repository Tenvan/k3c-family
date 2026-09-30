# B-044 · Sprints und Tickets liegen als Dateien nach Pflicht-Vorlagen

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** mittel
- **Status:** erledigt
- **Sprint:** SP00
- **Erstellt:** 2026-09-30

## Beschreibung

Sprints in `docs/sprints/{geplant,aktiv,erledigt}/` mit eigenem Ordner, Tickets als Dateien in `docs/backlog/`, Vorlagen in `docs/vorlagen/`, geprüft durch `tests/planning.test.ts`.

## Warum

Cloud-Agenten sollen Sessions ohne Planungs-Werkzeuge abarbeiten; nur Aktives soll gelesen werden.

## Akzeptanz

`npm test` scheitert bei fehlenden Feldern, falschem Status oder fehlenden Index-Einträgen.

## Notizen

Umgesetzt in SP00.3.
