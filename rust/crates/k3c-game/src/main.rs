//! Einstieg des nativen Spiels: `demo` (RU2.3) und `replay` (RU38.3).

use std::process::ExitCode;

use k3c_game::{demo, replay};

const USAGE: &str = "Aufruf: k3c-game demo --driver <datei> [--sprites N] [--measure S] [--out datei] [--headless --steps N] [--fullscreen] [--grafik ordner] [--data ordner]";
const REPLAY_USAGE: &str =
    "Aufruf: k3c-game replay <datei.jsonl.gz> [--headless | --window] [--measure N] [--out pfad]";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.split_first() {
        Some((cmd, rest)) if cmd == "demo" => demo::parse_args(rest).and_then(|o| demo::run(&o)),
        Some((cmd, rest)) if cmd == "replay" => match replay::cli::parse_args(rest) {
            Ok(o) => replay::run::run(&o),
            Err(e) => {
                eprintln!(
                    "Fehler: {e}
{REPLAY_USAGE}"
                );
                return ExitCode::from(2);
            }
        },
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
