//! Studio für Game- und Level-Designer von K3C in Rust (RU70, Port von `tools/k3c-studio`; Go ist das Orakel). Ein
//! Fenster (Tauri) oder `--headless` (Oberflächen-API für den Browser) hostet den MCP-Server k3c-studio.

pub mod app;
pub mod gamedata;
pub mod mcpsrv;
pub mod platform;
pub mod ports;
#[cfg(test)]
pub(crate) mod test_helpers;

use std::process::ExitCode;

/// Hilfe zu den Schaltern.
const USAGE: &str = "Aufruf: k3c-studio [--headless] (ohne Fenster: MCP-Server und Oberflächen-API)";

/// Einstieg der EXE: Fenster, mit `--headless` nur MCP-Server und Oberflächen-API.
pub fn run() -> ExitCode {
    match start() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("k3c-studio: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Liest die Schalter wie Go `flag` (ein oder zwei Striche).
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
    let raw = std::env::var(ports::MCP_PORT_ENV).unwrap_or_default();
    let port = mcpsrv::resolve_port(&raw)?;
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
