# tools/k3c-studio/

## Responsibility

Werkzeug für Game- und Level-Designer von K3C (WZ63, Tauri 2, eigene EXE neben der Workbench): Balancing, Welten, Inseln, Chunks (Welten-Editor), Gebäude-Planer (Hub) und Assets (Papierschnitt: Review, Katalog, Styleguide, Sound & Quellen). MCP-Server `k3c-studio` (Tools `world_*`, `hub_*`, `assets_*`, `level_generate`, seit WZ63.5 `sim_*`, Berichte und Spielstände); Entwicklung, Planung, Prüfläufe, Dienste und Logs bleiben im MCP-Server `k3c-workbench`. In diesem Ordner liegen Frontend und Taskfile, das Backend in `rust/tools/crates/k3c-studio` ([codemap](../../rust/tools/crates/k3c-studio/codemap.md)).

## Design

- `frontend/`: React-Oberfläche ([codemap](frontend/src/codemap.md)); Adapter `src/api/tauri.ts` (Fenster) und `src/api/http.ts` (Browser-Dev).
- `Taskfile.yml` (Namensraum `k3c-studio`): `dev`, `dev:server`, `dev:ui`, `window`, `frontend`, `typecheck`, `vitest`, `test`, `lint`, `build` (EXE `bin/k3c-studio.exe`), `build:headless`, `run`, `headless` (Dienst ohne Fenster aus `bin/k3c-studio-headless.exe`).
- `icon/`: App-Icon.
- Ports: MCP 5181 (Dev 5182), Oberflächen-API 8193, Vite 5193.

## Flow

1. `task k3c-studio` = `dev:server` (`cargo run … -- --headless`) + `dev:ui` (Vite, Proxy `/api`); Rust-Änderungen brauchen einen Neustart.
2. `task k3c-studio:window` startet das Fenster mit `bunx tauri dev`.
3. Nach geänderten Quellen vor dem Dienststart `task k3c-studio:build`.
4. Claude Code spricht über `.mcp.json` mit dem MCP-Server `k3c-studio` auf 127.0.0.1.

## Integration

- Gemeinsame Bausteine `tools/webkit`; gerechnet wird über die Spiel-EXE.
- Konsumenten: Game- und Level-Designer, MCP-Clients.
