//! `k3c-devwin.exe`: das Cheat-Fenster als eigener Prozess (nur Windows).

use std::process::ExitCode;

#[cfg(windows)]
fn main() -> ExitCode {
    match k3c_devwin::win32::run() {
        Ok(code) => ExitCode::from(code),
        Err(e) => {
            eprintln!("❌ k3c-devwin: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(not(windows))]
fn main() -> ExitCode {
    eprintln!("🚫 k3c-devwin läuft nur unter Windows.");
    ExitCode::FAILURE
}
