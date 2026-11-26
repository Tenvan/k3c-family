//! Größe und Position eines App-Fensters zwischen den Starts (Workbench, Studio). Kennt das Fenster-Framework nicht:
//! Aufrufer reichen Lesen und Setzen der Position als Funktionen herein.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Vorgabe der Fensterbreite (B-064 › Fenster).
pub const DEFAULT_WIDTH: i64 = 1280;
/// Vorgabe der Fensterhöhe.
pub const DEFAULT_HEIGHT: i64 = 800;
/// Kleinste Fensterbreite.
pub const MIN_WIDTH: i64 = 900;
/// Kleinste Fensterhöhe.
pub const MIN_HEIGHT: i64 = 600;

/// Gemerkte Größe und Position des Fensters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct State {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
}

const DEFAULT: State = State {
    x: 0,
    y: 0,
    width: DEFAULT_WIDTH,
    height: DEFAULT_HEIGHT,
};

/// Fehlende Felder und `null` behalten wie beim Go-Dekodieren die Vorgabe.
#[derive(Default, Deserialize)]
#[serde(default)]
struct Stored {
    x: Option<i64>,
    y: Option<i64>,
    width: Option<i64>,
    height: Option<i64>,
}

fn read(path: &Path) -> Option<State> {
    let value: serde_json::Value = serde_json::from_slice(&fs::read(path).ok()?).ok()?;
    let stored = match value {
        serde_json::Value::Null => Stored::default(),
        // serde läse auch eine Liste als Struktur, Go nicht.
        serde_json::Value::Object(_) => serde_json::from_value(value).ok()?,
        _ => return None,
    };
    Some(State {
        x: stored.x.unwrap_or(DEFAULT.x),
        y: stored.y.unwrap_or(DEFAULT.y),
        width: stored.width.unwrap_or(DEFAULT.width).max(MIN_WIDTH),
        height: stored.height.unwrap_or(DEFAULT.height).max(MIN_HEIGHT),
    })
}

/// Liest den gemerkten Zustand. Das zweite Feld ist `false`, wenn die Datei fehlt oder kaputt ist; dann gilt die Vorgabe
/// und die Position bleibt dem Fenster überlassen. Zu kleine Maße werden aufs Minimum gehoben. Negative Positionen
/// bleiben (Bildschirm links oder oberhalb des Hauptbildschirms); ob der Bildschirm noch da ist, prüft [`on_screen`].
pub fn load(path: &Path) -> (State, bool) {
    match read(path) {
        Some(state) => (state, true),
        None => (DEFAULT, false),
    }
}

/// Setzt die absolute Position (`x`, `y`). Das Fenster liest sie absolut, setzt sie aber relativ zum Arbeitsbereich des
/// Bildschirms, auf dem es gerade steht; der Zug auf (0, 0) misst diesen Versatz.
pub fn place(mut get: impl FnMut() -> (i64, i64), mut set: impl FnMut(i64, i64), x: i64, y: i64) {
    set(0, 0);
    let (ox, oy) = get();
    set(x.wrapping_sub(ox), y.wrapping_sub(oy));
}

/// Schreibt den Zustand; der Ordner wird bei Bedarf angelegt.
pub fn save(path: &Path, state: State) -> io::Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let data = serde_json::to_vec(&state).map_err(io::Error::other)?;
    // Ziel liegt außerhalb des Repos: bewusst ohne `write_atomic`.
    fs::write(path, data)
}

/// Go `os.UserConfigDir`: unter Windows `%AppData%`, sonst `$XDG_CONFIG_HOME` bzw. `$HOME/.config`.
fn user_config_dir() -> Option<PathBuf> {
    let var = |name: &str| std::env::var_os(name).filter(|v| !v.is_empty()).map(PathBuf::from);
    if cfg!(windows) {
        return var("AppData");
    }
    match var("XDG_CONFIG_HOME") {
        Some(dir) => dir.is_absolute().then_some(dir),
        None => var("HOME").map(|home| home.join(".config")),
    }
}

/// Datei unter `<Konfigurationsordner>/k3c`, nicht im Repo.
pub fn config_path(name: &str) -> PathBuf {
    user_config_dir()
        .unwrap_or_else(|| Path::new(".work").join("config"))
        .join("k3c")
        .join(name)
}

/// Liegt ein Punkt der Titelleiste (100 px vom linken Rand) auf einem angeschlossenen Bildschirm, das Fenster also
/// greifbar? Fehlt der Bildschirm (abgesteckt, Anordnung geändert), bleibt die Position dem Fenster überlassen. Die
/// Breite taugt nicht als Maß: `x` ist physisch, `width` in DIP gespeichert.
#[cfg(windows)]
#[allow(unsafe_code)]
pub fn on_screen(state: State) -> bool {
    use windows::Win32::Foundation::RECT;
    use windows::Win32::Graphics::Gdi::{MONITOR_DEFAULTTONULL, MonitorFromRect};

    // Abschneiden auf 32 Bit wie Go `int32()`.
    let px = state.x.wrapping_add(100) as i32;
    let py = state.y.wrapping_add(15) as i32;
    let rect = RECT {
        left: px,
        top: py,
        right: px.wrapping_add(1),
        bottom: py.wrapping_add(1),
    };
    // SAFETY: `rect` lebt während des Aufrufs; MonitorFromRect liest es nur und gibt ein Handle ohne Besitz zurück.
    let monitor = unsafe { MonitorFromRect(&rect, MONITOR_DEFAULTTONULL) };
    !monitor.is_invalid()
}

/// Geprüft wird nur unter Windows; anderswo gilt die gemerkte Position.
#[cfg(not(windows))]
pub fn on_screen(_state: State) -> bool {
    true
}

#[cfg(test)]
#[path = "windowstate_tests.rs"]
mod tests;
