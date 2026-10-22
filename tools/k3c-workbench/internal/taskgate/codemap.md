# tools/k3c-workbench/internal/taskgate/

## Responsibility

Freigabe-Schloss der Tasks (Workbench-Spec § 4): entscheidet, ob ein Agent einen Task über `task_start` starten darf. Die Datei `task-freigaben.json` (neben `services.json`) hält den dauerhaften Stand, Laufzeit-Schalter gelten bis zum Beenden von k3c-workbench.

## Design

- `State`: `Open`, `Closed`, `OpenTemp` (laut Datei gesperrt, vorläufig frei), `ClosedTemp` (laut Datei frei, vorläufig gesperrt); `State.Allowed` = Open oder OpenTemp.
- `Gate` (mutex-geschützt, genutzt von Oberfläche und MCP): `Load(path)` (fehlende Datei = nichts frei), `State(name)`, `Toggle(name)` (kehrt die Wirkung um, zurück auf Dateistand entfernt den Schalter), `Pending`, `Save` (Schalter in die Datei, sortiert), `AllowedNames(all)`.

## Flow

1. `Load` beim Start; `State` je Task für `task_list`/`task_start`.
2. Die Oberfläche ruft `Toggle`; `Save` übernimmt die Schalter dauerhaft.

## Integration

- Konsumenten: `tools/k3c-workbench/app.go`, `app_tasks.go` (Wails-Host), `internal/mcpsrv/tools_tasks.go` (`TaskHost.TaskState`).
- Keine weiteren internen Abhängigkeiten.
