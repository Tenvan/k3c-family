# tools/k3c-dev/frontend/src/planning/

## Responsibility

Reiter Planung: Ansicht der Projektplanung (Projekte, Sprints, Sessions, Backlog-Tickets mit GitHub-PR-Status und Worktree-Änderungen) sowie der Dokumente Plan, Fragen und Glossar. Zeigt Einträge im Lesemodus, erzeugt Prompts zum Weiterarbeiten im Chat (Kopieren oder „In Claude öffnen“) und setzt Kopf-Felder (Prio, Agent, Umgebung, Projekt, Rang) per `backend.planningSet`.

## Design

- Pure Logik (ohne React, für Vitest): `planning.ts` mit `PlanFilter`/`EMPTY_FILTER`, `filterSprints`, `filterTickets`, `filterProjects`, `projectHits`, `groupTickets`, `findSession`, `linkedTickets`, `isNext`, `sortSprints`, `prioRank`, `domains`, `QUICK`/`PQUICK`-Schnellfilter, `projectState`, `parseFilter` (gemerkter Filter aus `localStorage`), `toggle`, `ghBadges`, `SPLIT_DEFAULT`.
- Container: `PlanningPage.tsx` (Umschalter Sprints & Backlog / Dokumente `PlanDoc`: `plan`, `fragen`, `glossar`), `SprintsBacklog.tsx` (Board im erp-qs-Look: `GitHubBar`, `PlanToolbar`, links Projekte, rechts Backlog über `PlanDetail`, Leiste markierter Sessions für Sammel-Prompts; Splitter `planning.split.cols`/`planning.split` über `tasks/TaskBars`; stellt `WtContext` bereit).
- Filterleiste: `filterGroups.ts` (ohne React) mit Gruppen nach Wirkung (`FilterArea` Gemeinsam, Projekte, Backlog; UND/ODER je Gruppe), `toggleValue`, `filterChips`, `isDirty`, `resetFilter`; `PlanToolbar.tsx` zeichnet Checklisten (`DropdownMenu`), Chips mit × und Trefferzahl. Der Freitext wird nicht gemerkt.
- Prompt-Texte: `prompts.ts` mit `promptProject`, `promptSprint`, `promptSession`, `promptSessions`, `openSessions`, `pickable`, `worktreeOk`, `backlogPrompt` (Tickets in `SprintTarget` einplanen, `NEW_SPRINT`). Die Prompts nennen nur IDs und Anweisung; Inhalt liest der Agent per `plan_*`.
- Projekte: `projects.ts` mit `groupProjects` (aktive nach Rang, `ABN`, „Ohne Projekt“, ruhende/erledigte), `visibleSprints`, `nextSession`, `sessionProgress`, `stepRank`/`rankedCount`/`moveRank` (Rang per `planningSet`, danach neu laden) und `byRank` (Ordnung der Sammel-Prompts). `ProjectsView.tsx` (`ProjectSections` zustandslos, testbar per `renderToStaticMarkup`) und `ProjectCard.tsx` (Karte mit Rang-Kachel, Sprint-Chips, Fortschritt, nächster Session); ohne Projekte bleibt die Sprint-Liste.
- Einklappen: `fold.tsx` (`useFold`, `unfold`, `foldAll`, `headClick`, `FoldButton`), Zustand je Karten-ID in den Prefs, ein Ereignis hält alle Karten gleich.
- Worktree-Pills: `wtPills.ts` (`indexChanges`, `marksFor`, `pillLabel`, `pillTitle`, `withWorktreeEntries` ergänzt Einträge, die nur in einem Worktree existieren) und `WorktreePills.tsx` (`WtContext`, `WtPills`, `useWorktreeDiffs` über `worktreeCheck`).
- Präsentation: `Pill.tsx` (`Pill`, `ModePill`, `RemovableChip`, Töne `sessionTone`/`stateTone`/`prioTone`/`specTone`), `SprintCard.tsx` (Sprint-Panel, `SessionRow`, `scrollToSprint`), `Backlog.tsx` (`BacklogView` je Domäne einklappbar mit Sammel-Checkbox, `useTarget`, `setField`), `Detail.tsx` (`PlanDetail` für Session, Sprint, Projekt, Ticket mit Prompt, Feldmenüs und Lesemodus), `GitHubBar.tsx` (`useGitHub`, `GitHubBar`: letzter CI-Lauf auf develop, Hinweis, Neu laden), `Reader.tsx` + `readerDoc.ts` (`parseDoc`, `slugify`; Lesemodus-Dialog: Volltext aus `planningGet`, Inhaltsverzeichnis, Suche, j/k, Kriterien-Fortschritt), `PromptParts.tsx` (`CopyPrompt`: kopieren oder „In Claude öffnen“, `DepLinks`, `FieldMenu`); Markdown über `ui/MarkdownView`.
- Live-Update-Pattern: Ein Go-Watcher meldet `planning:changed`, die Ansichten laden neu.

## Flow

1. `SprintsBacklog` lädt `backend.planningData()`, `useGitHub()` den GitHub-Stand und `useWorktreeDiffs()` den Worktree-Check; der Filter wird per `loadText` + `parseFilter` wiederhergestellt.
2. Filtereingaben laufen durch `filterSprints`/`filterTickets`/`filterProjects`; Worktree-Einträge werden mit `withWorktreeEntries` eingemischt; die Auswahl einer Session markiert verknüpfte Tickets (`linkedTickets`).
3. Ändert sich eine Planungsdatei, emittiert Go `planning:changed` -> erneutes `planningData()`, `githubStatus` und `worktreeCheck`.
4. Dokumente: `planningDocs()` liefert die vorhandenen, `planningDoc(name)` den Markdown-Text für `MarkdownView`.
5. Sessions und Tickets lassen sich markieren; `CopyPrompt` kopiert den passenden Prompt oder übergibt ihn per `backend.openInClaude` (Deep Link `claude://code/new`, schickt nicht ab).
6. Doppelklick auf den Titel im Detail öffnet den `Reader` (`planningGet(id)`).
7. `FieldMenu` und Rang-Pfeile rufen `backend.planningSet(id, Feld, Wert)`; das Ergebnis kommt über `planning:changed`.
8. PR-Links öffnen über `backend.openUrl`, nicht per Navigation.

## Integration

- Konsument: `App.tsx` (Tab `planung`).
- Abhängigkeiten: `api` (`PlanningData`, `PlanProject`, `PlanSprint`, `PlanTicket`, `GitHubData`, `WorktreeDiff`, `PlanDoc`), `lib/` (`prefs`, `errors`), `tasks/TaskBars` (`Splitter`, `useSplit`), `ui/` (`parts`, `MarkdownView`), `@radix-ui/themes`.
- Go-Seite: `PlanningData`, `PlanningDocs`, `PlanningDoc`, `PlanningGet` (wie `plan_get`), `PlanningSet`, `GitHubStatus`, `WorktreeCheck`, `OpenInClaude`; Event `planning:changed`. Im Browser liefern `api/mockPlanning.ts` und `api/mockProjects.ts` die Daten.
