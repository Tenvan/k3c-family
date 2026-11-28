//! Ports und Dateinamen der Workbench: die festen Werte (seit RU71, vorher die der Go-Fassung).

/// MCP-Server; `.mcp.json` und der Hook `tools/hooks/planGuard.ts` erwarten ihn.
pub const MCP_PORT: u16 = 5180;
/// Oberflächen-API im Browser-Dev-Betrieb.
pub const UI_API_PORT: u16 = 8191;
/// Vite im Browser-Dev; steht auch in `tauri.conf.json`, `vite.config.ts` und im Task `k3c-workbench:dev:ui`.
pub const VITE_PORT: u16 = 5191;
/// Umgebungsvariable des MCP-Ports (`.mcp.json` wertet sie aus).
pub const ENV_MCP_PORT: &str = "K3C_DEV_PORT";
/// Umgebungsvariable des Ports der Oberflächen-API.
pub const ENV_UI_API_PORT: &str = "K3C_DEV_UI_API_PORT";
/// Eigenes Log `logs/<name>.jsonl`.
pub const LOG_NAME: &str = "k3c-workbench";
/// Nutzungsstatistik unter `<%APPDATA%>/k3c/` (Fenster).
pub const USAGE_FILE: &str = "mcp-usage.json";
/// Nutzungsstatistik ohne Fenster; die des Fensters bleibt unberührt.
pub const USAGE_FILE_HEADLESS: &str = "mcp-usage-dev.json";

/// Liest den MCP-Port aus dem Wert von [`ENV_MCP_PORT`]; leer heißt [`MCP_PORT`].
pub fn resolve_port(raw: &str) -> Result<u16, String> {
    k3c_kit::mcp::resolve_port(ENV_MCP_PORT, raw, MCP_PORT)
}

#[cfg(test)]
mod tests {
    use super::{MCP_PORT, resolve_port};

    /// Aus `TestResolvePort`.
    #[test]
    fn port_aus_der_umgebung() {
        assert_eq!(resolve_port(""), Ok(MCP_PORT));
        assert_eq!(resolve_port(" 6000 "), Ok(6000));
        for raw in ["0", "70000", "abc"] {
            assert!(resolve_port(raw).is_err(), "{raw} ohne Fehler");
        }
    }
}
