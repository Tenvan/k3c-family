# tools/k3c-dev/internal/proc/

## Responsibility

Startet Kindprozesse so, dass sie samt gesamtem Prozessbaum beendet werden können (Prüfläufe, Dienste, git, gh), ohne Konsolenfenster unter Windows.

## Design

- Wrapper `Cmd` (`proc.go`): `Command(ctx, args)`, `Start`, `Wait`, `Run`, `Kill`; `Executable(name)` löst Binaries auf; `waitDelay` begrenzt das Warten auf Pipes.
- Plattform-Strategie per Build-Tags: `proc_windows.go` (Job Object via `golang.org/x/sys/windows`, `createNoWindow`, `KillTree`), `proc_other.go` (Fallback, `KillTree`). Gemeinsame interne Schnittstelle: `prepare`, `attach`, `job.terminate`, `job.close`.

## Flow

1. `Command` baut `exec.Cmd`, `prepare` setzt Flags.
2. `Start` -> `attach(pid)` hängt den Prozess an Job/Gruppe.
3. `Kill` oder Context-Abbruch -> `job.terminate` beendet den ganzen Baum; `Wait` räumt über `job.close` auf.
4. `KillTree(pid)` für fremde bzw. übernommene PIDs.

## Integration

- Konsumenten: `internal/mcpsrv/check_run.go`, `internal/services/controller.go` und `runtime.go`, `internal/taskrun/taskrun.go`, `internal/taskcat/taskcat.go`, `internal/github/client.go`, `internal/planning/worktree.go`.
- Abhängigkeiten: Standardbibliothek `os/exec`, `golang.org/x/sys/windows`.
