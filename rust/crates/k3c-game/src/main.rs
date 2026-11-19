//! Einstieg des nativen Spiels: Spiel (`game::run::play`, RU30.6), `demo` (RU2.3), `replay` (RU38.3) und `still`.

use std::process::ExitCode;

use k3c_game::{demo, game, menu, replay, still};

const USAGE: &str = "Aufruf: k3c-game [play] [Schalter des Spiels, --saves ordner --grafik ordner] | k3c-game replay <datei.jsonl.gz> [--headless | --window] [--measure N] [--out pfad] | k3c-game still <zustand> --out <png> [--grafik ordner] [--props --simview <json>] | k3c-game demo --driver <datei> [--sprites N] [--measure S] [--out datei] [--headless --steps N] [--fullscreen] [--grafik ordner] [--data ordner]";
const REPLAY_USAGE: &str =
    "Aufruf: k3c-game replay <datei.jsonl.gz> [--headless | --window] [--measure N] [--out pfad]";

fn main() -> ExitCode {
    // Version des Baus (K3C_VERSION aus git describe, Task rust:game:exe, B-1138); ohne Angabe zeigt das Menü „unbekannt“
    if let Some(v) = option_env!("K3C_VERSION") {
        menu::version::set_version(v);
    }
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
        // Lizenzdatei des Pakets (LIZENZEN.txt, Kopf): dieselbe Liste wie die Lizenz-Seite im Menü (RU57.1, B-1121)
        Some((cmd, _)) if cmd == "licenses" => {
            print!("{}", menu::licenses::license_text());
            Ok(())
        }
        Some((cmd, rest)) if cmd == "play" => return game::run::play(rest),
        None => return game::run::play(&[]),
        Some((cmd, _)) if !cmd.starts_with('-') => Err(USAGE.to_string()),
        Some(_) => return game::run::play(&args),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Fehler: {e}");
            ExitCode::FAILURE
        }
    }
}
