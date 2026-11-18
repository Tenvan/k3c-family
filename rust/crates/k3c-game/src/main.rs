//! Einstieg des nativen Spiels: `demo` (RU2.3) und `replay` (RU38.3).

use std::process::ExitCode;

use k3c_game::{demo, game, replay, still};

const USAGE: &str = "Aufruf: k3c-game [play] [Schalter des Spiels, --saves ordner --grafik ordner] | k3c-game replay <datei.jsonl.gz> [--headless | --window] [--measure N] [--out pfad] | k3c-game still <zustand> --out <png> [--grafik ordner] [--props --simview <json>] | k3c-game demo --driver <datei> [--sprites N] [--measure S] [--out datei] [--headless --steps N] [--fullscreen] [--grafik ordner] [--data ordner]";
const REPLAY_USAGE: &str =
    "Aufruf: k3c-game replay <datei.jsonl.gz> [--headless | --window] [--measure N] [--out pfad]";

/// Startet das Spiel im Fenster (RU48.1).
fn play(rest: &[String]) -> Result<(), String> {
    game::launch::run(&game::launch::parse(rest)?)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.split_first() {
        Some((cmd, rest)) if cmd == "demo" => demo::parse_args(rest).and_then(|o| demo::run(&o)),
        Some((cmd, rest)) if cmd == "still" => still::parse_args(rest).and_then(|o| still::run(&o)),
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
        Some((cmd, rest)) if cmd == "play" => play(rest),
        None => play(&[]),
        Some((cmd, _)) if !cmd.starts_with("--") => Err(USAGE.to_string()),
        Some(_) => play(&args),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Fehler: {e}");
            ExitCode::FAILURE
        }
    }
}
