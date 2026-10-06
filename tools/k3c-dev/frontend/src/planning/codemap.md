# tools/k3c-dev/frontend/src/planning/

## Responsibility

Reiter Planung: Ansicht der Projektplanung (Sprints, Sessions, Backlog-Tickets mit GitHub-PR-Status) sowie der Dokumente Plan, Fragen und Glossar. Erzeugt Prompts zum Weiterarbeiten im Chat (Kopieren in die Zwischenablage) und setzt Kopf-Felder (Prio, Agent, Umgebung) per `backend.planningSet`.

## Design

- Pure Logik: `planning.ts` (ohne React, für Vitest) mit `PlanFilter`/`EMPTY_FILTER`, `filterSprints`, `filterTickets`, `groupTickets`, `findSession`, `linkedTickets`, `isNext`, `sortSprints`, `prioRank`, `domains`, `QUICK`-Schnellfilter, `parseFilter` (gemerkter Filter aus `localStorage`), `toggle`, `ghBadges`.
- Container: `PlanningPage.tsx` (Umschalter Sprints & Backlog / Dokumente `PlanDoc`: `plan`, `fragen`, `glossar`), `SprintsBacklog.tsx` (Filterleiste, `GitHubBar`, markierte Sessions für Sammel-Prompts, Planungsdaten + GitHub-Status).
- Prompt-Texte: `prompts.ts` (ohne React) mit `promptSprint`, `promptSession`, `promptSessions`, `promptWorktree` (autonom im Worktree, nur Sessions mit `worktreeOk`: offen/in Arbeit, `autonom`, `offline`), `worktreeBlocked`, `worktreeVariant` (`PromptVariant`), `openSessions`, `pickable`, `backlogPrompt` (Tickets in `SprintTarget` einplanen, `NEW_SPRINT`). Die Prompts nennen nur IDs und Anweisung; Inhalt liest der Agent per `plan_*`.
- Präsentation: `SprintCard.tsx` (`SprintCard`, `ModeBadge`, `tone`, `prioTone`, `scrollToSprint`), `Backlog.tsx` (`BacklogList` mit Mehrfachauswahl und gemerktem Sprint-Ziel, `SessionDetail`), `PromptParts.tsx` (`CopyPrompt` mit Variantenmenü, `DepLinks`, `FieldMenu` zum direkten Ändern); Markdown über `ui/MarkdownView`.
- Live-Update-Pattern: Ein Go-Watcher meldet `planning:changed`, die Ansichten laden neu.

## Flow

1. `SprintsBacklog` lädt `backend.planningData()` und `backend.githubStatus(false)`; der Filter wird per `loadText` + `parseFilter` wiederhergestellt.
2. Filtereingaben laufen durch `filterSprints`/`filterTickets`; die Auswahl einer Session markiert verknüpfte Tickets (`linkedTickets`).
3. Ändert sich eine Planungsdatei, emittiert Go `planning:changed` -> erneutes `planningData()`.
4. Dokumente: `planningDocs()` liefert die vorhandenen, `planningDoc(name)` den Markdown-Text für `MarkdownView`.
5. Sessions (`SprintCard`) und Tickets (`BacklogList`) lassen sich markieren; `CopyPrompt` kopiert den passenden Prompt, das Menü ▾ bietet „Autonom im Worktree abarbeiten“ (gesperrt mit Grund aus `worktreeBlocked`).
6. `FieldMenu` ruft `backend.planningSet(id, Feld, Wert)`; das Ergebnis kommt über `planning:changed`.
7. PR-Links öffnen über `backend.openUrl`, nicht per Navigation.

## Integration

- Konsument: `App.tsx` (Tab `planung`).
- Abhängigkeiten: `api` (`PlanningData`, `PlanSprint`, `PlanTicket`, `GitHubData`, `PlanDoc`), `lib/` (`prefs`, `errors`), `ui/` (`parts`, `MarkdownView`), `@radix-ui/themes`.
- Go-Seite: `PlanningData`, `PlanningDocs`, `PlanningDoc`, `GitHubStatus`, `PlanningSet`; Event `planning:changed`. Im Browser liefert `api/mockPlanning.ts` die Daten.
