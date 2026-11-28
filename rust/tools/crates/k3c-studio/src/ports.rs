//! Ports, Namen und Dateien des Studios: die festen Werte (seit RU71, vorher die der Go-Fassung).

/// MCP-Server; `.mcp.json` und `tools/k3c-workbench/services.json` erwarten ihn.
pub const MCP_PORT: u16 = 5181;
/// Umgebungsvariable des MCP-Ports (`.mcp.json` und die Dienste der Workbench setzen sie).
pub const MCP_PORT_ENV: &str = "K3C_STUDIO_PORT";
/// Oberflächen-API im Browser-Dev-Betrieb.
pub const UI_API_PORT: u16 = 8193;
/// Umgebungsvariable des Ports der Oberflächen-API.
pub const UI_API_PORT_ENV: &str = "K3C_STUDIO_UI_API_PORT";
/// Vite im Fenster-Dev und Browser-Dev; steht auch in `tauri.conf.json`, `vite.config.ts` und im Task `k3c-studio:dev:ui`.
pub const VITE_PORT: u16 = 5193;
/// Titel des Fensters.
pub const WINDOW_TITLE: &str = "K3C Studio";
/// Nutzungsstatistik unter `<%APPDATA%>/k3c/`.
pub const USAGE_FILE: &str = "mcp-usage-studio.json";
/// Eigenes Log `logs/<name>.jsonl`; steht auch als `log` des Dienstes in `tools/k3c-workbench/services.json`.
pub const LOG_NAME: &str = "k3c-studio";
