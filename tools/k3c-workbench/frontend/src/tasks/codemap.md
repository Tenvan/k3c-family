# tools/k3c-workbench/frontend/src/tasks/

## Responsibility

Reiter Tasks: Katalog der Go-Task-Ziele (`Taskfile.yml`) als Baum durchsuchen, mit Argumenten starten, stoppen, die Ausgabe verfolgen.

## Design

- Pure Logik: `tasks.ts` (`filterNamespaces` nach derselben Regel wie `taskcat.Filter` in Go, `countTasks`, `findTask`, `upsertRun`, `pushHistory` mit `HISTORY_LIMIT`, `describeRun`/`describeState` -> `RunBadge`, `favoriteTasks`, `splitArgs`, `taskTooltip`).
- Data-Hook: `useTasks.ts` lädt Katalog (`tasks`) und Läufe (`taskRuns`), hält per `task:state` die Lauf-Liste und den Verlauf der Sitzung aktuell; `reload`.
- Container: `TasksPage.tsx` (`active`, meldet Seite und Auswahl per `useTicketPage` aus `@webkit/lib/ticketContext`) – Kopf, Baum | Splitter | Detail, Fußzeile; `useTaskPrefs` merkt Auswahl, eingeklappte Namensräume, Favoriten und Splitter (`tasks.*` über `prefs`), den Freitext nicht.
- Präsentation: `TaskTree.tsx` (★ Favoriten als Sicht auf dieselben Knoten, einklappbare Namensräume, Zeile mit Pin, Name und Laufstatus; keine Freigabe-Schlösser mehr), `TaskDetail.tsx` (Kopf mit Zustand, Argumentfeld, Start/Stopp, `ConsoleView` `task:<name>`, „Letzte Läufe“), `TaskBars.tsx` (`TasksHead` mit Filter und „Neu laden“, `TasksFoot`). Der Splitter kommt aus `@webkit/atoms` (`Splitter`) und `@webkit/lib/split` (`useSplit`, Anteil `tasks.split`); Namensräume und Zeilen sind Radix-Knöpfe (`Button`/`IconButton` ghost).
- Zustand pro Task über eine `TaskRun`-Liste, per `task:state` inkrementell aktualisiert (`upsertRun`).

## Flow

1. Beim Mount: `backend.tasks()`, `backend.taskRuns()` und Abo auf `task:state`.
2. „Neu laden“ -> `backend.tasksReload()` liest den Katalog neu.
3. Start: `splitArgs(args)` -> `backend.taskStart(name, args)`; Stop: `backend.taskStop(name)`.
4. `task:state` -> `upsertRun` und `pushHistory` -> Badge via `describeRun`; Dauer über `formatDuration`.
6. `ConsoleView` streamt die Ausgabe des gewählten Tasks.

## Integration

- Konsument: `App.tsx` (Tab `tasks`).
- Abhängigkeiten: `api` (`TaskCatalog`, `TaskInfo`, `TaskRun`), `logs/ConsoleView`, `lib/` (`format`, `prefs`), `@webkit/lib/errors`, `@webkit/lib/split`, `@webkit/lib/ticketContext`, `@webkit/atoms`, `@webkit/molecules`.
- Go-Seite: `Tasks`, `TasksReload`, `TaskStart`, `TaskStop`, `TaskRuns`; Event `task:state`; Mock: `api/mockTasks.ts`.
