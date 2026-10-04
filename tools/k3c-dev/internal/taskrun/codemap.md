# tools/k3c-dev/internal/taskrun/

## Responsibility
Führt Taskfile-Tasks aus k3c-dev aus: Start, Stopp, Zustand und Exit-Code, höchstens ein Lauf je Task. Kein Supervisor: ein Task endet mit Exit-Code und wird nie neu gestartet. Dazu die zentrale Argument-Validierung für `task <name> -- args`.

## Design
- Runner: `Runner` (`taskrun.go`) mit `Options`, `New`, `Start(name, args)`, `Stop(ctx, name)`, `StopAll(ctx)`, `All()`; interner `entry` je Lauf.
- Zustandsmodell: `State` und `Run` mit Exit-Code und Reason; Startfehler erscheinen als `Run` im Zustand Failed, damit die Oberfläche ihn zeigt.
- Fehler: `ErrAlreadyRunning`, `ErrNotRunning`.
- Prozessbaum über `internal/proc` (Stopp beendet den ganzen Baum); Ausgabe zeilenweise über `console.LineWriter` (`line`), Zustandsänderungen über `emit`.
- `args.go`: `ValidateArg`/`ValidateArgs`, `ForbiddenArgChars` (Batch-/Shell-Metazeichen auf der Kette task.exe → .cmd → mvdan/sh), `MaxArgLen`=200; lehnt mit Begründung ab, bereinigt nicht.

## Flow
1. `Start`: `ValidateArgs` → Doppellauf prüfen → `task <name> [-- args]` per `proc` starten → Zustand Running, `emit`.
2. `waitFor` (Goroutine): wartet auf Prozessende, leert den `LineWriter`, setzt Exit-Code/State, `emit`.
3. `Stop`: Prozessbaum beenden, auf Abrechnung warten (bis `ctx` endet).
4. `StopAll` beim Beenden der App.

## Integration
- Konsumenten: `tools/k3c-dev/app.go` (Lebenszyklus, `StopAll`), `app_tasks.go` (Start/Stopp aus der Task-Ansicht).
- Abhängigkeiten: `internal/proc`, `internal/console`.
