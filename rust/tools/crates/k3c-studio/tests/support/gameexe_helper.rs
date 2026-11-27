//! Spielt in `tests/gameexe_run.rs` die Spiel-EXE (Gegenstück zu Go `TestHelperProcess`): `gameexe-helper <Modus> …`.
//! Kein Teil des Studios.

use std::io::{BufRead, Read, Write};
use std::net::TcpListener;
use std::process::ExitCode;
use std::time::Duration;

/// Wartezeit, nach der ein vergessener Helfer von selbst endet.
const LIFETIME: Duration = Duration::from_secs(60);

fn arg_after(args: &[String], flag: &str) -> Option<String> {
    let at = args.iter().position(|a| a == flag)?;
    args.get(at + 1).cloned()
}

/// Eine Zeile auf stdout, sofort geschrieben.
fn say(line: &str) {
    let mut out = std::io::stdout();
    // Ist die Pipe zu, gibt es niemanden mehr zu unterrichten.
    let _ = writeln!(out, "{line}").and_then(|()| out.flush());
}

fn quoted(text: &str) -> String {
    let escaped = text.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{escaped}\"")
}

/// Eingabedatei lesen und als result zurückgeben.
fn lines(args: &[String]) -> ExitCode {
    let Some(input) = arg_after(args, "--input").and_then(|p| std::fs::read_to_string(p).ok()) else {
        say(r#"{"event":"error","message":"keine Eingabe"}"#);
        return ExitCode::from(2);
    };
    say("kein JSON");
    say(r#"{"event":"progress","done":1,"total":2}"#);
    say(&format!(r#"{{"event":"result","text":{}}}"#, quoted(&input)));
    eprintln!("❌ Meldung");
    ExitCode::SUCCESS
}

/// Wartet auf das Ende von stdin wie `--stop-stdin`.
fn wait() -> ExitCode {
    say(r#"{"event":"progress","done":1,"total":9}"#);
    let mut rest = Vec::new();
    let _ = std::io::stdin().lock().read_to_end(&mut rest);
    say(r#"{"event":"aborted"}"#);
    ExitCode::from(130)
}

/// Ignoriert stdin und startet einen Enkel mit offenem Port.
fn hang(port_file: &str) -> ExitCode {
    // Der Starter ordnet diesen Prozess erst nach dem Start dem Job zu; der Enkel soll nicht davor entstehen.
    std::thread::sleep(Duration::from_millis(200));
    let Ok(mut child) =
        std::env::current_exe().and_then(|exe| std::process::Command::new(exe).args(["grandchild", port_file]).spawn())
    else {
        return ExitCode::from(2);
    };
    // Erst melden, wenn der Enkel seinen Port geschrieben hat.
    for _ in 0..100 {
        if std::fs::metadata(port_file).is_ok_and(|m| m.len() > 0) {
            break;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    say(r#"{"event":"progress","done":1,"total":9}"#);
    std::thread::sleep(LIFETIME);
    let _ = child.wait();
    ExitCode::SUCCESS
}

fn grandchild(port_file: &str) -> ExitCode {
    let Ok(listener) = TcpListener::bind("127.0.0.1:0") else {
        return ExitCode::from(2);
    };
    let Ok(address) = listener.local_addr() else {
        return ExitCode::from(2);
    };
    if std::fs::write(port_file, address.to_string()).is_err() {
        return ExitCode::from(2);
    }
    std::thread::sleep(LIFETIME);
    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let port_file = args.get(1).map(String::as_str).unwrap_or_default();
    match args.first().map(String::as_str) {
        Some("lines") => lines(&args),
        Some("fail") => {
            say(r#"{"event":"error","message":"kaputt"}"#);
            ExitCode::FAILURE
        }
        Some("exit") => ExitCode::from(3),
        Some("long") => {
            // Eine Zeile über der Grenze von 16 MB, danach zählt nichts mehr.
            say(&"x".repeat((16 << 20) + 2));
            say(r#"{"event":"result","text":"zu spät"}"#);
            ExitCode::SUCCESS
        }
        Some("wait") => wait(),
        Some("hang") => hang(port_file),
        Some("grandchild") => grandchild(port_file),
        _ => {
            let mut line = String::new();
            let _ = std::io::stdin().lock().read_line(&mut line);
            ExitCode::from(2)
        }
    }
}
