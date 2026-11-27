//! EXE `k3c-studio`; alles Weitere in der Bibliothek.

// Release ohne Konsolenfenster (wie Wails); im Dev-Build bleibt die Konsole.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> std::process::ExitCode {
    k3c_studio::run()
}
