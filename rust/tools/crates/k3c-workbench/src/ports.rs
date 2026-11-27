//! Ports und eigene Dateinamen, mit denen die Rust-Fassung neben der Go-Fassung läuft. RU71 stellt nur diese Datei auf
//! die Go-Werte um.

/// MCP-Server (Go: 5180).
pub const MCP_PORT: u16 = 5280;
/// Oberflächen-API im Browser-Dev-Betrieb (Go: 8191).
pub const UI_API_PORT: u16 = 8290;
/// Vite im Browser-Dev; steht auch in `tauri.conf.json` und im Task `tools-rust:workbench:dev`.
pub const VITE_PORT: u16 = 5291;
/// Umgebungsvariable des MCP-Ports. Eigener Name, weil `.mcp.json` die der Go-Fassung auswertet.
pub const ENV_MCP_PORT: &str = "K3C_RS_DEV_PORT";
/// Umgebungsvariable des Ports der Oberflächen-API.
pub const ENV_UI_API_PORT: &str = "K3C_RS_DEV_UI_API_PORT";
/// Eigenes Log `logs/<name>.jsonl`; die Go-Workbench schreibt eine andere Datei.
pub const LOG_NAME: &str = "k3c-workbench-rs";
/// Nutzungsstatistik unter `<%APPDATA%>/k3c/` (Fenster).
pub const USAGE_FILE: &str = "mcp-usage-rs.json";
/// Nutzungsstatistik ohne Fenster; die des Fensters bleibt unberührt.
pub const USAGE_FILE_HEADLESS: &str = "mcp-usage-rs-dev.json";

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
