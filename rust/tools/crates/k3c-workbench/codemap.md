# rust/tools/crates/k3c-workbench/

## Responsibility

Backend der Entwickler-Workbench (Tauri 2): MCP-Server `k3c-workbench` und Oberfläche für Prüfläufe, Planung, Dienste, Tasks, Logs, Git, Ressourcen und Spieltest.

## Design

- `app` (Zustand, Fenster, `headless`), `rpc` (eine Aufruf-Tabelle `METHODS` für Tauri-Command `rpc` und Browser-Oberfläche), `mcp` (Katalog nur in `tools.rs`, Anleitung `instructions.md`, Prüfläufe in `check/`).
- Fachmodule: `planning`, `services`, `taskcat`, `taskrun`, `logs`, `gitops`, `gitcommit`, `github`, `playtest`, `resmon`.
- `ports.rs`: MCP 5180, UI-API 8191, Vite 5191.

## Flow

`main` → `run`: Repo-Wurzel suchen (`rust/Cargo.toml` plus `data/`), dann Fenster oder `--headless`. MCP-Aufrufe und Oberfläche gehen an dieselben Fachmodule.

## Integration

- Frontend `tools/k3c-workbench/frontend`, Dienste `tools/k3c-workbench/services.json`, Hook `tools/hooks/planGuard.ts`.
- Tasks `k3c-workbench:dev|window|build|test|lint`; EXE `bin/k3c-workbench.exe`.
