//! Ports, Namen und Dateien, mit denen die Rust-Fassung neben der Go-Fassung läuft (V2). RU71 ändert nur diese Datei.

/// MCP-Server (Go: 5181).
pub const MCP_PORT: u16 = 5281;
/// Umgebungsvariable des MCP-Ports.
pub const MCP_PORT_ENV: &str = "K3C_STUDIO_RS_PORT";
/// Oberflächen-API im Browser-Dev-Betrieb.
pub const UI_API_PORT: u16 = 8293;
/// Umgebungsvariable des Ports der Oberflächen-API.
pub const UI_API_PORT_ENV: &str = "K3C_STUDIO_RS_UI_API_PORT";
/// Vite im Fenster-Dev und Browser-Dev; steht auch in `tauri.conf.json` und im Task `tools-rust:studio:dev`.
pub const VITE_PORT: u16 = 5294;
/// Titel des Fensters.
pub const WINDOW_TITLE: &str = "K3C Studio (Rust)";
/// Nutzungsstatistik unter `<%APPDATA%>/k3c/`; eigene Datei, damit die laufende Go-Fassung ihre behält.
pub const USAGE_FILE: &str = "mcp-usage-studio-rs.json";
/// Eigenes Log `logs/<name>.jsonl`; in dieselbe Datei wie Go zu schreiben, störte beim Rotieren (Review RU70.1).
pub const LOG_NAME: &str = "k3c-studio-rs";
