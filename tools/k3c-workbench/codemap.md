# tools/k3c-workbench/

## Responsibility

Entwicklerwerkzeug für K3C (Tauri 2): MCP-Server `k3c-workbench` (Prüfläufe, Logs, Dienste, Tasks, Planung, Spieltest) und eine Oberfläche mit den Seiten Dienste, Tasks, Planung, Git, MCP, Monitoring und Spieltest. Welten, Assets, Hub und Balancing liegen im Studio (`tools/k3c-studio`). In diesem Ordner liegen Frontend und Taskfile, das Backend in `rust/tools/crates/k3c-workbench` ([codemap](../../rust/tools/crates/k3c-workbench/codemap.md)).

## Design

- `frontend/`: React-Oberfläche ([codemap](frontend/codemap.md)); Adapter `src/api/tauri.ts` (Fenster) und `src/api/http.ts` (Browser-Dev).
- `services.json`: Dienste der Seite Dienste (Studio ohne Fenster, Spiel Debug und Release).
- `Taskfile.yml` (Namensraum `k3c-workbench`): `dev`, `dev:server`, `dev:ui`, `dev:worktree` (eigene Instanz im Worktree mit Ports aus dem Pfad, `K3C_DATA_DIR` auf die `data/` des Worktrees), `window`, `frontend`, `test`, `vitest`, `lint`, `build` (EXE `bin/k3c-workbench.exe`), `run`.
- `icon/`: App-Icon.
- Ports: MCP 5180 (Dev 5190), Oberflächen-API 8191, Vite 5191.

## Flow

1. `task k3c-workbench` = `dev:server` (`cargo run … -- --headless`) + `dev:ui` (Vite, Proxy `/api`); Rust-Änderungen brauchen einen Neustart.
2. `task k3c-workbench:window` startet das Fenster mit `bunx tauri dev`.
3. Claude Code spricht über `.mcp.json` mit dem MCP-Server auf 127.0.0.1.

## Integration

- Gemeinsame Bausteine `tools/webkit`; Hook `tools/hooks/planGuard.ts`.
- Konsumenten: Entwickler (Fenster oder Browser), MCP-Clients (`check_run`, `logs_*`, `svc_*`, `task_*`, `plan_*`, `playtest_*` …).
