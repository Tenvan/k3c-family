//! Einstieg des nativen Spiels: bisher nur `demo` (RU2.3).

use std::process::ExitCode;

use k3c_game::demo;

const USAGE: &str = "Aufruf: k3c-game demo --driver <datei> [--sprites N] [--measure S] [--out datei] [--headless --steps N] [--fullscreen] [--grafik ordner] [--data ordner]";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.split_first() {
        Some((cmd, rest)) if cmd == "demo" => demo::parse_args(rest).and_then(|o| demo::run(&o)),
        _ => Err(USAGE.to_string()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Fehler: {e}");
            ExitCode::FAILURE
        }
    }
}
