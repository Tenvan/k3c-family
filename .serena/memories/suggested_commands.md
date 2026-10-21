# Suggested Commands

Einziger Einstieg: `task` (Taskfile.yml, `task --list`). Neue Aufgaben ins Taskfile des jeweiligen Projekts, nicht in Root oder `package.json`.

- k3c-dev MCP zuerst: Prüfungen `check_run`, Dienste `svc_start`/`svc_stop`, Logs `logs_errors`/`logs_query`, Testläufe nur `sim_test`. Shell-Befehle nur, wenn k3c-dev nicht läuft. Im Worktree immer `checkout` setzen.
- `task install` Abhängigkeiten (4× npm ci)
- `task dev` Vite (5183, LAN) · `task start` Go-Server ohne Web-Build · `task serve` Build + Server (8080)
- `task test` Vitest (Filter: `task test -- planning`)
- `task check` Lint + Typecheck + Tests · `check:go` · `check:dev` · `check:studio` · `check:all` (inkl. Build)
- `task go:golden` Golden-Files neu schreiben, nur bei gewollter Regeländerung (Begründung im Commit)
- `task k3c-dev` / `task k3c-studio` Wails-Fenster im Watch-Modus; `:build` für EXE; Pattern `build|run|dev` je EXE-Projekt
- `task gen:atlas|gen:gebaeude|gen:rules`, `task ops:aufraeumen|ops:backup`

Windows: Shell ist PowerShell/Git Bash; blockiert die Execution Policy `npm.ps1`, `npm.cmd` nutzen. EXEs mit festem Pfad starten (Firewall-Abfrage vermeiden), kein `go run`.
