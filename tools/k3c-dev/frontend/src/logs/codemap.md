# tools/k3c-dev/frontend/src/logs/

## Responsibility

Logs-Bereich der Dienste-Seite: Quellenleiste sowie je Quelle Konsole (Live-Puffer), Log-Datei-Abfrage und verdichtete Fehleransicht. Die Konsolenansicht wird auch von der Tasks-Seite wiederverwendet.

## Design

- Pure Logik (testbar, ohne React): `lines.ts` (`mergeLines` mit Lücken-Erkennung über `seq`, `visibleLines`, `markOf`, `dotTone`, `pickSource`, `MAX_LINES`), `sources.ts` (`Group`, `logFor`, `otherGroups`, `describe`, `tagsOf`), `logview.ts` (`TABS`, `LIMITS`, `LEVEL_FILTERS`, `tabEnabled`, `pickTab`), `ansi.ts` (`parseAnsi` SGR → `Span[]`, `stripAnsi`; Text bleibt Text, nie HTML).
- Data-Hooks: `useSources.ts` (Quellenliste), `useConsole.ts` (Tail laden, `console:line` anhängen, bei Lücke oder Zustandswechsel gebündelt nachladen, 300 ms), `useLogQuery.ts` (`logsQuery`).
- Präsentation: `SourceBar.tsx`, `SourcePanel.tsx` (Radix `Tabs` `konsole`/`log`/`fehler`, Reiter via `prefs`), `ConsoleView.tsx`, `LogTab.tsx` (Filter, debounced), `ErrorsTab.tsx` (`logsErrors`), `RoleTags.tsx` (Farbe je Einordnung).

## Flow

1. `useSources()` lädt `backend.sources()`; `ServicesPage` wählt eine Quelle, `SourcePanel` erhält `source` und Log-Name (`logFor`).
2. `ConsoleView` -> `useConsole(name)`: `backend.consoleTail`, danach `backend.on('console:line')`; Zeilen der Ladezeit werden gepuffert und per `mergeLines` über `seq` dedupliziert.
3. Erkennt `mergeLines` eine Lücke oder wechselt `source:state`, lädt der Hook den Puffer erneut.
4. `LogTab` -> `useLogQuery` -> `backend.logsQuery(source, LogQuery)`; `ErrorsTab` -> `backend.logsErrors`.
5. Konsolenzeilen werden mit `parseAnsi` in farbige Spans zerlegt und als Fehler/Warnung markiert.

## Integration

- Konsumenten: `services/ServicesPage.tsx` (Quellenleiste, Panel), `services/ServiceCard.tsx` (`RoleTags`), `tasks/TasksPage.tsx` (`ConsoleView name="task:<name>"`).
- Abhängigkeiten: `api` (`backend`, Typen), `lib/` (`format`, `prefs`, `errors`, `useDebounced`), `ui/parts` (`NoticeCard`, `StatusBadge`, `Tone`), `@radix-ui/themes`.
- Go-Seite: Wails-Methoden `Sources`, `ConsoleTail`, `LogsQuery`, `LogsErrors`; Events `console:line`, `source:state`.
