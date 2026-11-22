# tools/workkit/

## Responsibility

Gemeinsamer Go-Code von k3c-workbench und k3c-studio (WZ63): MCP-Gerüst, Log, Konsole, Prozesse, Nutzungszähler, Oberflächen-HTTP-Schicht, Claude-Deep-Link, Fenster-Icon. Eigenes Go-Modul `k3c/tools/workkit` (per `replace` eingebunden wie `tools/assetkit`); importiert weder die Workbench noch das Studio (depguard).

## Design

- Pakete: `mcpkit/` (MCP-Server-Gerüst), `applog/`, `console/`, `proc/`, `usage/`, `uikit/` (HTTP-RPC und Events der Wails-Oberfläche), `claude/` (`claude://`-Link), `winicon/` (Fenster-Icon, Windows). Eigene Codemaps: `applog`, `console`, `proc`, `usage`, `mcpkit`, `uikit`, `claude`, `winicon`.
- `Taskfile.yml` (Namensraum `workkit`): `gotest`, `lint`; `k3c-workbench:gotest` und `:lint` (und `k3c-studio:*`) rufen sie mit auf.
- Regel: Was beide Apps brauchen, gehört hierher; die Apps importieren einander nie.

## Flow

1. Eine App (`tools/k3c-workbench`, `tools/k3c-studio`) importiert die Pakete und bindet ihre eigenen Tools, Bindings und Pfade an.
2. Unterschiede der Apps (Tool-Katalog, Instructions, Ports) bleiben in der App; `workkit` kennt keine Tool-Namen.

## Integration

- Konsumenten: `tools/k3c-workbench`, `tools/k3c-studio`.
- Abhängigkeiten: MCP-Go-SDK, `jsonschema-go`, `golang.org/x/sys`, `engine/conlog`.
