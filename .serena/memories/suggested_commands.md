# Suggested Commands

Einziger Einstieg: `task` (Taskfile.yml, `task --list`). Neue Aufgaben ins Taskfile des jeweiligen Projekts, nicht in Root oder `package.json`.

- k3c-workbench MCP zuerst: Prüfungen `check_run`, Logs `logs_errors`/`logs_query`, Testläufe nur `sim_test` (Simulationen derzeit gesperrt). Shell-Befehle nur, wenn k3c-workbench nicht läuft. Im Worktree immer `checkout` setzen.
- `task install` Abhängigkeiten (bun, Wurzel und Workbench-Frontend)
- `task build` natives Spiel nach `dist/k3c-game/` · `task native:game` Spiel starten
- `task test` Vitest (Filter: `task test -- planning`)
- `task check:diff` vor jedem Commit · `check` · `check:go` · `check:dev` · `check:all` (inkl. Build)
- `task go:golden` Golden-Files neu schreiben, nur bei gewollter Regeländerung (Begründung im Commit)
- `task k3c-workbench` Workbench im Dev-Modus; `:window` Fenster, `:build` EXE; Pattern `build|run|dev` je EXE-Projekt
- `task gen:rules|gen:paper`, `task ops:aufraeumen|ops:backup`

Windows: Shell ist PowerShell/Git Bash. EXEs mit festem Pfad starten (Firewall-Abfrage vermeiden), kein `go run`.
