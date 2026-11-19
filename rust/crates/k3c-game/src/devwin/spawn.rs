//! Start des Cheat-Fensters als eigener Prozess (RU56.11, B-1135): Pfad neben der Spiel-EXE, Pipe-Name, Argumente.
//! Ohne `unsafe` (`std::process`); weder Netz noch Firewall-Regel, die Verbindung ist eine lokale Named Pipe.

use std::path::{Path, PathBuf};
use std::process::{Child, Command};

/// EXE ist der Dateiname des Fensters neben `k3c-game.exe`.
pub const EXE: &str = "k3c-devwin.exe";

/// exe_path liefert den Pfad der Fenster-EXE neben der Spiel-EXE.
pub fn exe_path(game_exe: &Path) -> PathBuf {
    game_exe.parent().unwrap_or_else(|| Path::new(".")).join(EXE)
}

/// pipe_name baut den Pipe-Namen `k3c-devwin-<pid>-<nonce>` (die Zufallskomponente erschwert Vorwegnehmen des Namens).
pub fn pipe_name(pid: u32, nonce: u64) -> String {
    format!("k3c-devwin-{pid}-{nonce:x}")
}

/// nonce liefert eine Zahl, die sich von Start zu Start ändert (Uhrzeit, ohne Zufallsbibliothek).
pub fn nonce() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos() as u64)
}

/// args sind die Argumente der Fenster-EXE: Pipe und Spielprozess (das Fenster schließt sich mit dem Spiel).
pub fn args(pipe: &str, pid: u32) -> Vec<String> {
    vec![
        "--pipe".to_string(),
        pipe.to_string(),
        "--exit-with-game".to_string(),
        pid.to_string(),
    ]
}

/// pipe_path ist der volle Pfad der Pipe für den Client.
pub fn pipe_path(pipe: &str) -> String {
    format!(r"\\.\pipe\{pipe}")
}

/// spawn startet das Fenster ohne Konsolenfenster; eine fehlende EXE ergibt einen Fehlertext, keinen Panic.
pub fn spawn(exe: &Path, pipe: &str, pid: u32) -> Result<Child, String> {
    if !exe.is_file() {
        return Err(format!("{} fehlt (liegt neben k3c-game.exe)", exe.display()));
    }
    let mut cmd = Command::new(exe);
    cmd.args(args(pipe, pid));
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    cmd.spawn()
        .map_err(|e| format!("{} nicht gestartet: {e}", exe.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pfad_pipe_und_argumente() {
        let exe = exe_path(Path::new("C:/spiel/k3c-game.exe"));
        assert_eq!(exe, Path::new("C:/spiel").join("k3c-devwin.exe"));
        let pipe = pipe_name(42, 0xAB);
        assert_eq!(pipe, "k3c-devwin-42-ab");
        assert_eq!(
            args(&pipe, 42),
            ["--pipe", "k3c-devwin-42-ab", "--exit-with-game", "42"]
        );
        assert_eq!(pipe_path(&pipe), r"\\.\pipe\k3c-devwin-42-ab");
        assert_ne!(pipe_name(1, nonce()), pipe_name(1, nonce().wrapping_add(1)));
    }

    #[test]
    fn fehlende_exe_ist_ein_fehler() {
        let e = spawn(Path::new("gibt-es-nicht/k3c-devwin.exe"), "x", 1).unwrap_err();
        assert!(e.contains("fehlt"), "{e}");
    }
}
