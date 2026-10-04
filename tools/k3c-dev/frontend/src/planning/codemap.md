# tools/k3c-dev/frontend/src/planning/

## Responsibility

Reiter Planung: Read-only-Ansicht der Projektplanung (Sprints, Sessions, Backlog-Tickets mit GitHub-PR-Status) sowie der Dokumente Plan, Fragen und Glossar.

## Design

- Pure Logik: `planning.ts` (ohne React, für Vitest) mit `PlanFilter`/`EMPTY_FILTER`, `filterSprints`, `filterTickets`, `groupTickets`, `findSession`, `linkedTickets`, `isNext`, `QUICK`-Schnellfilter, `parseFilter` (gemerkter Filter aus `localStorage`), `toggle`, `ghBadges`.
- Container: `PlanningPage.tsx` (Umschalter Sprints & Backlog / Dokumente `PlanDoc`: `plan`, `fragen`, `glossar`), `SprintsBacklog.tsx` (Filterleiste, Planungsdaten + GitHub-Status).
- Präsentation: `SprintCard.tsx` (`SprintCard`, `BacklogList`, `SessionDetail`); Markdown über `ui/MarkdownView`.
- Live-Update-Pattern: Ein Go-Watcher meldet `planning:changed`, die Ansichten laden neu.

## Flow

1. `SprintsBacklog` lädt `backend.planningData()` und `backend.githubStatus(false)`; der Filter wird per `loadText` + `parseFilter` wiederhergestellt.
2. Filtereingaben laufen durch `filterSprints`/`filterTickets`; die Auswahl einer Session markiert verknüpfte Tickets (`linkedTickets`).
3. Ändert sich eine Planungsdatei, emittiert Go `planning:changed` -> erneutes `planningData()`.
4. Dokumente: `planningDocs()` liefert die vorhandenen, `planningDoc(name)` den Markdown-Text für `MarkdownView`.
5. PR-Links öffnen über `backend.openUrl`, nicht per Navigation.

## Integration

- Konsument: `App.tsx` (Tab `planung`).
- Abhängigkeiten: `api` (`PlanningData`, `PlanSprint`, `PlanTicket`, `GitHubData`, `PlanDoc`), `lib/` (`prefs`, `errors`), `ui/` (`parts`, `MarkdownView`), `@radix-ui/themes`.
- Go-Seite: `PlanningData`, `PlanningDocs`, `PlanningDoc`, `GitHubStatus`; Event `planning:changed`. Im Browser liefert `api/mockPlanning.ts` die Daten.
