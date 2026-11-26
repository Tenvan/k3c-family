// Release ohne Konsolenfenster (wie Wails); im Dev-Build bleibt die Konsole für das Log.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    spike_lib::run();
}
