# tools/k3c-studio/

## Responsibility

Wails-Desktop-Werkzeug für Game- und Level-Designer von K3C (WZ63, eigene EXE neben der Workbench): Welten, Inseln, Chunks (Welten-Editor), Gebäude-Planer (Hub) und Assets (Papierschnitt: Review, Katalog, Styleguide, Sound & Quellen). Hostet den MCP-Server `k3c-studio` (Tools `world_*`, `hub_*`, `assets_*`, `level_generate`); Entwicklung, Planung, Prüfläufe, Dienste und Balancing bleiben im MCP-Server `k3c-workbench`. Das Root-Package `main` verdrahtet die Packages unter `internal/` und stellt sie als Wails-Bindings bzw. HTTP-RPC bereit.

## Design

- Composition Root `main.go` (`run`, `windowOptions`), `app.go` (`App`: `startup`, `shutdown`, `beforeClose`, `secondInstance`, `Info`, `setMCP`); Frontend per `go:embed all:frontend/dist`.
- Bindings nach Bereich: `app_world.go` (Welten-Editor über `internal/world`: `WorldList`, `WorldDetail`, `WorldLevel`, `WorldValues`, `WorldOverrideSet`, `WorldCreate`, `WorldDelete`, `WorldNatureSet`, `WorldIslandEtappe`), `app_hub.go` (`HubPlan`, `HubCheck`, `HubRules`, `HubSave` über `internal/hubplan`), `app_assets.go` (Asset-Oberfläche über `internal/assetsui`, `AssetsStufen`, `StudioWeltSicht`), `app_studio.go` (`StudioSounds`, `StudioSoundData`, `StudioSources`, `StudioGameOpen` über `internal/studioview`).
- Client-Server-Betrieb (`headless.go`): `-headless` ohne Fenster (MCP-Port 5181, Oberflächen-API 8193), `-dev` baut sich per `engine/devrun` als Kind und startet bei Go-Änderungen neu (MCP-Port 5182); Oberfläche unter Vite 5193. `uiapi.go`: Methodenliste `uiMethods` über `tools/workkit/uikit`, `OpenInClaude`.
- `Taskfile.yml` (Namensraum `k3c-studio`): `dev`, `dev:server`, `dev:ui`, `window`, `frontend`, `typecheck`, `vitest`, `test`, `gotest`, `lint`, `build` (EXE `bin/k3c-studio`), `run`, `headless`, `headless:build`. `.golangci.yml`: depguard sperrt Importe aus `k3c-workbench` und Balancing-Pakete.
- Unterordner: `frontend/` (React-Oberfläche, siehe `frontend/src/codemap.md`), `internal/` (`mcpsrv`, `world`, `hubplan`, `assetsui`, `review`, `paperassets`, `studioview`; Balancing seit WZ63.5: `balance`, `simrun`, `datadoc`, `gamedata`, Bindings `app_balance*.go`), `build/`, `icon/`; Modul `k3c/tools/k3c-studio`.

## Flow

1. `main` → `run` → `newApp` → Wails `Run` (bzw. `runHeadless`/`runDev`); `startup` startet `mcpsrv.Server`.
2. Das Frontend ruft `App`-Methoden (Wails-Binding oder `/api/dev/rpc`); Ereignisse per Wails bzw. SSE.
3. Claude Code spricht über `.mcp.json` mit dem MCP-Server `k3c-studio` auf 127.0.0.1; `mcpsrv` nutzt dieselben Packages wie die Bindings.
4. `shutdown`: „Im Spiel ansehen“ schließen (`closeStudio`), MCP stoppen.

## Integration

- Abhängigkeiten: `tools/workkit` (`mcpkit`, `applog`, `console`, `proc`, `usage`, `uikit`, `claude`, `winicon`), `tools/assetkit`, `engine/conlog`, `engine/devrun` (gerechnet wird über die Spiel-EXE), Wails-Runtime; kein Import aus `tools/k3c-workbench`.
- Konsumenten: Game- und Level-Designer (`task k3c-studio`, `task k3c-studio:build`), MCP-Clients (`world_*`, `hub_*`, `assets_*`, `level_generate`).
