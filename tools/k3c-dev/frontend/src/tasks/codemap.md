# tools/k3c-dev/frontend/src/tasks/

## Responsibility

Reiter Tasks: Katalog der Go-Task-Ziele (`Taskfile.yml`) durchsuchen, mit Argumenten starten, stoppen und die Konsolenausgabe des Laufs verfolgen.

## Design

- Pure Logik: `tasks.ts` (`filterNamespaces`, `countTasks`, `findTask`, `upsertRun`, `describeRun`/`describeState` -> `RunBadge`, `splitArgs`, `taskTooltip`).
- Container/Präsentation: `TasksPage.tsx` – Katalog nach `TaskNamespace` gruppiert, Suchfeld, Argumentzeile, Start/Stop (`ActionButton`), Laufstatus-Badges, gemerkte Auswahl via `prefs`.
- Wiederverwendung: Die Konsole kommt aus `logs/ConsoleView` (Quelle `task:<name>`).
- Zustand pro Task über eine `TaskRun`-Liste, per `task:state` inkrementell aktualisiert (`upsertRun`).

## Flow

1. Beim Mount: `backend.tasks()` (Katalog), `backend.taskRuns()` (bisherige Läufe), Abo auf `task:state`.
2. „Neu laden“ -> `backend.tasksReload()` liest den Katalog neu.
3. Start: `splitArgs(args)` -> `backend.taskStart(name, args)`; Stop: `backend.taskStop(name)`.
4. `task:state` -> `upsertRun` -> Badge via `describeRun`; Dauer über `formatDuration`.
5. `ConsoleView` streamt die Ausgabe des gewählten Tasks.

## Integration

- Konsument: `App.tsx` (Tab `tasks`).
- Abhängigkeiten: `api` (`TaskCatalog`, `TaskInfo`, `TaskRun`), `logs/ConsoleView`, `lib/` (`format`, `prefs`, `errors`), `ui/parts`, `@radix-ui/themes`.
- Go-Seite: `Tasks`, `TasksReload`, `TaskStart`, `TaskStop`, `TaskRuns`; Event `task:state`; Mock: `api/mockTasks.ts`.
