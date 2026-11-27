//! Entwickler-Werkzeug von K3C in Rust (RU69, Port von `tools/k3c-workbench`; Go ist das Orakel). Ein Fenster (Tauri)
//! oder `--headless` (Oberflächen-API für den Browser) hostet den MCP-Server k3c-workbench.

pub mod app;
mod capture;
pub mod gitcommit;
pub mod github;
pub mod gitops;
pub mod logs;
pub mod mcp;
pub mod planning;
pub mod platform;
pub mod playtest;
pub mod ports;
pub mod rpc;
pub mod services;
pub mod taskcat;
pub mod taskrun;

#[cfg(test)]
mod nowindow_tests;
#[cfg(test)]
pub(crate) mod testutil;

use std::process::ExitCode;

/// Hilfe zu den Schaltern.
const USAGE: &str = "Aufruf: k3c-workbench [--headless] (ohne Fenster: MCP-Server und Oberflächen-API)";

/// Einstieg der EXE: Fenster, mit `--headless` nur MCP-Server und Oberflächen-API.
pub fn run() -> ExitCode {
    match start() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("💥 k3c-workbench: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Liest die Schalter wie Go `flag` (ein oder zwei Striche). `-dev` der Go-Fassung entfällt (Entscheidung 014).
fn headless_flag(args: impl Iterator<Item = String>) -> Result<bool, String> {
    let mut headless = false;
    for arg in args {
        match arg.as_str() {
            "-headless" | "--headless" => headless = true,
            _ => return Err(format!("unbekannter Schalter {arg:?}. {USAGE}")),
        }
    }
    Ok(headless)
}

fn start() -> Result<(), String> {
    let headless = headless_flag(std::env::args().skip(1))?;
    if headless {
        platform::attach_console();
    }
    let wd = std::env::current_dir().map_err(|e| e.to_string())?;
    let root = k3c_kit::mcp::find_root(wd)?;
    let port = ports::resolve_port(&std::env::var(ports::ENV_MCP_PORT).unwrap_or_default())?;
    if headless {
        app::headless::run(root, port)
    } else {
        app::window::run(root, port)
    }
}

#[cfg(test)]
mod tests {
    use super::headless_flag;

    fn flags(args: &[&str]) -> Result<bool, String> {
        headless_flag(args.iter().map(|a| (*a).to_owned()))
    }

    #[test]
    fn schalter_wie_go() {
        assert_eq!(flags(&[]), Ok(false));
        assert_eq!(flags(&["-headless"]), Ok(true));
        assert_eq!(flags(&["--headless"]), Ok(true));
        assert!(flags(&["--dev"]).is_err());
    }
}
