//! Aufnahme und Ablage der Ende-zu-Ende-Bilder (RU32.3), Gegenstück zu `native/k3c-game/e2e/images_test.go`.
//! Bilder liegen unter `K3C_E2E_OUT` (Standard `.work/e2e`): `go/{a,b}` (Go-EXE, von `go test ./e2e -run TestFlowImages`),
//! `rust/{a,b}` (diese Tests), je `shot-NN.png` und `stationen.txt`; Bildpaare unter `paare/`, Kennzahlen in
//! `vergleich.json`. Nichts davon wird eingecheckt.

use k3c_det::golden;
use serde_json::Value;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Fenstergröße der Aufnahme (wie Go: `shotSize`).
pub const SHOT_SIZE: &str = "1280x720";
/// Zeitlimit je Lauf der EXE.
pub const LIMIT: Duration = Duration::from_secs(120);

/// Die Repo-Wurzel.
pub fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

/// Arbeitsordner der EXE: `K3C_ROOT`, sonst die Repo-Wurzel (Go: `exeRoot`).
pub fn exe_root() -> PathBuf {
    std::env::var("K3C_ROOT")
        .ok()
        .filter(|d| !d.is_empty())
        .map_or_else(repo, PathBuf::from)
}

/// Ordner der Bilder: `K3C_E2E_OUT`, sonst `.work/e2e`.
pub fn out_dir() -> PathBuf {
    std::env::var("K3C_E2E_OUT")
        .ok()
        .filter(|d| !d.is_empty())
        .map_or_else(|| repo().join(".work/e2e"), PathBuf::from)
}

/// Ist die gerenderte Grafik da (`task gen:paper`)? Wie Go: das Manifest der Tanne.
pub fn grafik_da() -> bool {
    exe_root().join("public/grafik/objekte/tanne/manifest.json").is_file()
}

/// Name des Bildes der Station mit Nummer `i` (ab 0).
pub fn shot_name(i: usize) -> String {
    format!("shot-{:02}.png", i + 1)
}

/// Eingabe-Folge und Takte der Stationen aus `flow.json`: `(script, [(name, tick)])`.
pub fn reference() -> (String, Vec<(String, u64)>) {
    let v: Value = golden::read(golden::testdata().join("rust/e2e/flow.json"));
    let stations = v["stations"]
        .as_array()
        .expect("stations")
        .iter()
        .map(|s| {
            (
                s["name"].as_str().expect("name").to_string(),
                s["tick"].as_u64().expect("tick"),
            )
        })
        .collect();
    (v["script"].as_str().expect("script").to_string(), stations)
}

/// Startet die EXE im Aufnahme-Modus des Ablaufs und schreibt die Bilder nach `dir`. Nach `LIMIT` wird der Prozess
/// beendet; Fehler nennen die letzten Zeilen der Ausgabe (Go: `lastLines`).
pub fn shoot(exe: &Path, dir: &Path, script: &str, at: &str) -> Result<(), String> {
    let mut child = Command::new(exe)
        .args(["--flow", "--input", script, "--shots"])
        .arg(dir)
        .args(["--at", at, "--size", SHOT_SIZE])
        .current_dir(exe_root())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("{}: {e}", exe.display()))?;
    // Ausgabe nebenher lesen, damit die Rohre nicht volllaufen
    let mut out = child.stdout.take().expect("stdout");
    let mut err = child.stderr.take().expect("stderr");
    let (o, e) = (
        std::thread::spawn(move || read_all(&mut out)),
        std::thread::spawn(move || read_all(&mut err)),
    );
    let start = Instant::now();
    let status = loop {
        if let Some(s) = child.try_wait().map_err(|e| e.to_string())? {
            break Some(s);
        }
        if start.elapsed() > LIMIT {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    let log = format!("{}{}", o.join().unwrap_or_default(), e.join().unwrap_or_default());
    match status {
        Some(s) if s.success() => Ok(()),
        Some(s) => Err(format!("{}: {s}\n{}", exe.display(), last_lines(&log, 30))),
        None => Err(format!(
            "{}: Zeitlimit {LIMIT:?}\n{}",
            exe.display(),
            last_lines(&log, 30)
        )),
    }
}

fn read_all(r: &mut impl Read) -> String {
    let mut buf = Vec::new();
    let _ = r.read_to_end(&mut buf);
    String::from_utf8_lossy(&buf).into_owned()
}

/// Die letzten `n` Zeilen von `s`.
pub fn last_lines(s: &str, n: usize) -> String {
    let lines: Vec<&str> = s.trim_end_matches('\n').lines().collect();
    lines[lines.len().saturating_sub(n)..].join("\n")
}
