# tools/k3c-dev/frontend/src/tasks/

## Responsibility

Reiter Tasks: Katalog der Go-Task-Ziele (`Taskfile.yml`) als Baum durchsuchen, mit Argumenten starten, stoppen, die Ausgabe verfolgen und die Freigabe-Schlösser für Agenten schalten. Stellt außerdem den wiederverwendeten Splitter bereit.

## Design

- Pure Logik: `tasks.ts` (`filterNamespaces`, `countTasks`, `findTask`, `upsertRun`, `pushHistory` mit `HISTORY_LIMIT`, `describeRun`/`describeState` -> `RunBadge`, `describeGate` Schloss -> Symbol/Text, `onlyAllowed` (🔑-Filter), `favoriteTasks`, `splitArgs`, `taskTooltip`).
- Data-Hook: `useTasks.ts` lädt Katalog (`tasks`), Läufe (`taskRuns`) und Schlösser (`taskGates`), hält per `task:state` die Lauf-Liste und den Verlauf der Sitzung aktuell; `reload`, `toggleGate` (`taskGateToggle`, bis zum Beenden) und `saveGates` (`taskGateSave`, schreibt in die Freigabedatei).
- Container: `TasksPage.tsx` – Kopf, Baum | Splitter | Detail, Fußzeile; `useTaskPrefs` merkt Auswahl, 🔑, eingeklappte Namensräume, Favoriten und Splitter (`tasks.*` über `prefs`), den Freitext nicht.
- Präsentation: `TaskTree.tsx` (★ Favoriten als Sicht auf dieselben Knoten, einklappbare Namensräume, Zeile mit Schloss, Pin, Name und Laufstatus), `TaskDetail.tsx` (Kopf mit Zustand, Argumentfeld, Start/Stopp, `ConsoleView` `task:<name>`, „Letzte Läufe“), `TaskBars.tsx` (`TasksHead`, `TasksFoot` und der gemeinsame `Splitter` samt `useSplit`, `clampSplit`, `splitShare`, `parseSplit`: Anteil 20–80 %, Doppelklick setzt zurück).
- Zustand pro Task über eine `TaskRun`-Liste, per `task:state` inkrementell aktualisiert (`upsertRun`).

## Flow

1. Beim Mount: `backend.tasks()`, `backend.taskRuns()`, `backend.taskGates()` und Abo auf `task:state`.
2. „Neu laden“ -> `backend.tasksReload()` liest den Katalog neu, danach werden die Schlösser neu geholt.
3. Start: `splitArgs(args)` -> `backend.taskStart(name, args)`; Stop: `backend.taskStop(name)`.
4. `task:state` -> `upsertRun` und `pushHistory` -> Badge via `describeRun`; Dauer über `formatDuration`.
5. Schloss-Klick -> `taskGateToggle` (Laufzeit-Schalter); „n Schalter übernehmen“ -> `taskGateSave`.
6. `ConsoleView` streamt die Ausgabe des gewählten Tasks.

## Integration

- Konsument: `App.tsx` (Tab `tasks`); `Splitter`/`useSplit` nutzen zusätzlich `planning/SprintsBacklog.tsx`, `mcp/McpPage.tsx` und `mcp/StatsView.tsx`.
- Abhängigkeiten: `api` (`TaskCatalog`, `TaskInfo`, `TaskRun`, `TaskGates`), `logs/ConsoleView`, `lib/` (`format`, `prefs`, `errors`), `ui/parts`, `@radix-ui/themes`.
- Go-Seite: `Tasks`, `TasksReload`, `TaskStart`, `TaskStop`, `TaskRuns`, `TaskGatesView`, `TaskGateToggle`, `TaskGateSave`; Event `task:state`; Mock: `api/mockTasks.ts`.
