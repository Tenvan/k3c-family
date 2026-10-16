# tools/k3c-dev/frontend/src/planning/

## Responsibility

Reiter Planung: Ansicht der Projektplanung (Sprints, Sessions, Backlog-Tickets mit GitHub-PR-Status) sowie der Dokumente Plan, Fragen und Glossar. Erzeugt Prompts zum Weiterarbeiten im Chat (Kopieren in die Zwischenablage) und setzt Kopf-Felder (Prio, Agent, Umgebung) per `backend.planningSet`.

## Design

- Pure Logik: `planning.ts` (ohne React, für Vitest) mit `PlanFilter`/`EMPTY_FILTER`, `filterSprints`, `filterTickets`, `groupTickets`, `findSession`, `linkedTickets`, `isNext`, `sortSprints`, `prioRank`, `domains`, `QUICK`-Schnellfilter, `parseFilter` (gemerkter Filter aus `localStorage`), `toggle`, `ghBadges`.
- Container: `PlanningPage.tsx` (Umschalter Sprints & Backlog / Dokumente `PlanDoc`: `plan`, `fragen`, `glossar`), `SprintsBacklog.tsx` (Board im erp-qs-Look, B-439: `GitHubBar`, `PlanToolbar`, Projekte | Splitter | Backlog über Detail, markierte Sessions für Sammel-Prompts, gemerkte Splitter `planning.split.cols` und `planning.split`).
- Filterleiste (B-439): `filterGroups.ts` (ohne React) mit Gruppen nach Wirkung (Gemeinsam, Projekte, Backlog), `toggleValue`, `filterChips`, `isDirty`, `resetFilter`; `PlanToolbar.tsx` zeichnet Checklisten (`DropdownMenu`), Chips mit × und Trefferzahl. Der Freitext wird nicht gemerkt.
- Prompt-Texte: `prompts.ts` (ohne React) mit `promptSprint`, `promptSession`, `promptSessions`, `openSessions`, `pickable`, `backlogPrompt` (Tickets in `SprintTarget` einplanen, `NEW_SPRINT`). Die Prompts nennen nur IDs und Anweisung; Inhalt liest der Agent per `plan_*`.
- Präsentation: `Pill.tsx` (`Pill`, `ModePill`, Töne, `RemovableChip`), `SprintCard.tsx` (Sprint-Panel, `SessionRow`, `scrollToSprint`), `Backlog.tsx` (`BacklogView` je Domäne einklappbar mit Sammel-Checkbox, `useTarget`), `Detail.tsx` (`PlanDetail` für Session, Sprint, Projekt, Ticket), `Reader.tsx` + `readerDoc.ts` (Lesemodus: Volltext aus `planningGet`, Inhaltsverzeichnis, Suche, j/k), `PromptParts.tsx` (`CopyPrompt`: kopieren oder „In Claude öffnen“, `DepLinks`, `FieldMenu` zum direkten Ändern); Markdown über `ui/MarkdownView`.
- Projekte (B-358): `projects.ts` (ohne React) mit `groupProjects` (aktive nach Rang, `ABN`, „Ohne Projekt“, ruhende/erledigte), `nextSession`, `sessionProgress`, `stepRank`/`rankedCount`/`moveRank` (Rang per `planningSet`, danach neu laden) und `byRank` (Ordnung der Sammel-Prompts). `ProjectsView.tsx` (`ProjectSections` zustandslos, testbar per `renderToStaticMarkup`) und `ProjectCard.tsx`; ohne Projekte bleibt die Sprint-Liste.
- Live-Update-Pattern: Ein Go-Watcher meldet `planning:changed`, die Ansichten laden neu.

## Flow

1. `SprintsBacklog` lädt `backend.planningData()` und `backend.githubStatus(false)`; der Filter wird per `loadText` + `parseFilter` wiederhergestellt.
2. Filtereingaben laufen durch `filterSprints`/`filterTickets`; die Auswahl einer Session markiert verknüpfte Tickets (`linkedTickets`).
3. Ändert sich eine Planungsdatei, emittiert Go `planning:changed` -> erneutes `planningData()`.
4. Dokumente: `planningDocs()` liefert die vorhandenen, `planningDoc(name)` den Markdown-Text für `MarkdownView`.
5. Sessions (`SprintCard`) und Tickets (`BacklogList`) lassen sich markieren; `CopyPrompt` kopiert den passenden Prompt oder öffnet ihn per Claude-Funke als neue Code-Session in Claude Desktop (`openInClaude`, Deep Link `claude://code/new`, schickt nicht ab).
6. `FieldMenu` ruft `backend.planningSet(id, Feld, Wert)`; das Ergebnis kommt über `planning:changed`.
7. PR-Links öffnen über `backend.openUrl`, nicht per Navigation.

## Integration

- Konsument: `App.tsx` (Tab `planung`).
- Abhängigkeiten: `api` (`PlanningData`, `PlanSprint`, `PlanTicket`, `GitHubData`, `PlanDoc`), `lib/` (`prefs`, `errors`), `ui/` (`parts`, `MarkdownView`), `@radix-ui/themes`.
- Go-Seite: `PlanningData`, `PlanningDocs`, `PlanningDoc`, `PlanningGet` (wie `plan_get`), `GitHubStatus`, `PlanningSet`; Event `planning:changed`. Im Browser liefert `api/mockPlanning.ts` die Daten.
