//! Arbeitsordner des Pakets (RU57.3, B-1140). Das Spiel lädt Grafik, Schrift und Rig-Teile relativ zum Arbeitsordner
//! (`public/grafik`, `public/schrift`, `native/k3c-rig/assets`). Startet ein Empfänger die EXE aus einem anderen Ordner
//! (Konsole, Verknüpfung ohne „Ausführen in“), fände sie nichts. Fehlt `public/grafik` im Arbeitsordner und liegt es neben
//! der EXE, wechselt das Spiel dorthin. Nur beim Start ohne Schalter: relative Pfade in Schaltern (`--saves ordner`) gälten
//! sonst plötzlich ab dem EXE-Ordner; Entwickler starten aus der Repo-Wurzel und sind nicht betroffen.

use std::path::{Path, PathBuf};

/// Die Marke, an der ein Spielordner erkannt wird.
const MARKER: &str = "public/grafik";

/// Der Ordner, in den das Spiel wechseln soll, sonst `None`: nur ohne Schalter (oder nur `play`), wenn der Arbeitsordner
/// den Spielordner nicht enthält, der EXE-Ordner ihn aber schon.
pub fn anchor_target(
    args: &[String],
    cwd_has_game: bool,
    exe_dir: Option<&Path>,
    exe_dir_has_game: bool,
) -> Option<PathBuf> {
    let plain = args.is_empty() || (args.len() == 1 && args[0] == "play");
    if !plain || cwd_has_game || !exe_dir_has_game {
        return None;
    }
    exe_dir.map(Path::to_path_buf)
}

/// Wechselt bei Bedarf in den Ordner der EXE (siehe Modul); ein Fehlschlag lässt den Arbeitsordner stehen, das Spiel
/// meldet dann die fehlende Grafik wie bisher.
pub fn anchor_to_package(args: &[String]) {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|e| e.parent().map(Path::to_path_buf));
    let has = |d: &Path| d.join(MARKER).is_dir();
    let cwd_has = has(Path::new("."));
    let exe_has = exe_dir.as_deref().is_some_and(has);
    if let Some(dir) = anchor_target(args, cwd_has, exe_dir.as_deref(), exe_has) {
        let _ = std::env::set_current_dir(dir);
    }
}
