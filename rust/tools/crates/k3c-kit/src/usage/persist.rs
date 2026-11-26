//! Datei der Gesamtzeit und Zeitreihe (Port von `persist.go`). Die Sitzung wird nie gespeichert.

use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError, Weak};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::scope::ScopeAgg;
use super::{Clock, Minute, Tracker};

const FILE_VERSION: i64 = 1;
/// Bündelt Schreibvorgänge: ein fleißiger Agent ruft im Sekundentakt, die Datei muss das nicht mitmachen.
const SAVE_DELAY: Duration = Duration::from_secs(2);

type OnErr = Arc<dyn Fn(&str) + Send + Sync>;

/// Inhalt der Datei beim Lesen.
#[derive(Deserialize)]
struct FileData {
    #[serde(default)]
    version: Option<i64>,
    #[serde(rename = "allTime", default)]
    all_time: Option<ScopeAgg>,
    #[serde(default)]
    minutes: Option<Vec<Option<Minute>>>,
}

/// Inhalt der Datei beim Schreiben.
#[derive(Serialize)]
struct FileRef<'a> {
    version: i64,
    #[serde(rename = "allTime")]
    all_time: &'a ScopeAgg,
    minutes: &'a [Minute],
}

/// Pfad, Merker für das ausstehende Schreiben und der Fehler-Rückruf.
pub(super) struct Persist {
    path: PathBuf,
    on_err: OnErr,
    me: Weak<Tracker>,
    pending: AtomicBool,
    /// Schnappschuss und Schreiben am Stück, damit nie ein älterer Stand zuletzt schreibt.
    write_mu: Mutex<()>,
    /// Schreibfehler nur einmal je Programmlauf melden.
    warned: AtomicBool,
}

impl Tracker {
    /// Tracker, der Gesamtzeit und Zeitreihe in `path` speichert und beim Start lädt. Eine kaputte oder fremde Datei
    /// wird zur Sicherung (`.bak`, eine vorhandene bleibt erhalten), der Tracker beginnt leer. Lässt sich die Datei
    /// nicht lesen (z. B. gesperrt), bleibt sie unangetastet und dieser Lauf speichert nicht, damit der erste `flush`
    /// die gültige Statistik nicht überschreibt. `on_err` erfährt jeweils davon.
    pub fn open(path: impl Into<PathBuf>, now: Clock, on_err: OnErr) -> Arc<Tracker> {
        let path = path.into();
        let mut tracker = Tracker::new(now);
        match std::fs::read(&path) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => {
                on_err(&format!(
                    "statistik {} nicht lesbar ({e}); dieser Lauf zählt nur im Speicher",
                    path.display()
                ));
                return Arc::new(tracker);
            }
            Ok(data) => tracker.load(&path, &data, &on_err),
        }
        Arc::new_cyclic(|me| {
            tracker.persist = Some(Persist {
                path,
                on_err,
                me: me.clone(),
                pending: AtomicBool::new(false),
                write_mu: Mutex::new(()),
                warned: AtomicBool::new(false),
            });
            tracker
        })
    }

    fn load(&self, path: &Path, data: &[u8], on_err: &OnErr) {
        match decode(data) {
            Err(cause) => move_aside(path, &cause, &self.now, on_err),
            Ok((all_time, minutes)) => {
                let mut inner = self.lock();
                inner.all_time = all_time;
                inner.minutes = minutes;
                inner.prune((self.now)());
            }
        }
    }

    /// Plant ein Schreiben in `SAVE_DELAY`, falls noch keins aussteht.
    pub(super) fn schedule_save(&self) {
        let Some(p) = &self.persist else {
            return;
        };
        if p.pending.swap(true, Ordering::SeqCst) {
            return;
        }
        // Der Thread hält den Tracker bis zum Schreiben am Leben, wie der Go-Timer.
        let Some(me) = p.me.upgrade() else {
            return;
        };
        let spawned = std::thread::Builder::new()
            .name("usage-save".to_owned())
            .spawn(move || {
                std::thread::sleep(SAVE_DELAY);
                // Hat inzwischen ein `flush` geschrieben, ist nichts mehr zu tun.
                if me.persist.as_ref().is_some_and(|p| p.pending.load(Ordering::SeqCst)) {
                    // Den Fehler meldet `flush` selbst über `on_err`.
                    let _ = me.flush();
                }
            });
        if spawned.is_err() {
            p.pending.store(false, Ordering::SeqCst);
        }
    }

    /// Schreibt sofort (beim Beenden und aus dem Speicher-Thread). Ohne Datei tut es nichts. Reihenfolge der Sperren:
    /// `write_mu`, dann `inner`; `record` nimmt nur `inner`.
    pub fn flush(&self) -> io::Result<()> {
        let Some(p) = &self.persist else {
            return Ok(());
        };
        let writing = p.write_mu.lock().unwrap_or_else(PoisonError::into_inner);
        let data = {
            let inner = self.lock();
            p.pending.store(false, Ordering::SeqCst);
            serde_json::to_vec(&FileRef {
                version: FILE_VERSION,
                all_time: &inner.all_time,
                minutes: &inner.minutes,
            })
        };
        let result = data.map_err(io::Error::from).and_then(|data| write(&p.path, &data));
        drop(writing);
        if let Err(e) = &result
            && !p.warned.swap(true, Ordering::SeqCst)
        {
            (p.on_err)(&format!("statistik nicht gespeichert, läuft im Speicher weiter: {e}"));
        }
        result
    }
}

/// Liest die Datei und macht sie robust gegen fehlende Felder (von Hand bearbeitet oder anderes Werkzeug).
fn decode(data: &[u8]) -> Result<(ScopeAgg, Vec<Minute>), String> {
    let file: FileData = serde_json::from_slice(data).map_err(|e| e.to_string())?;
    let version = file.version.unwrap_or_default();
    let Some(all_time) = file.all_time.filter(|_| version == FILE_VERSION) else {
        return Err(format!("version {version} statt {FILE_VERSION} oder ohne Gesamtzeit"));
    };
    let mut minutes: Vec<Minute> = file
        .minutes
        .unwrap_or_default()
        .into_iter()
        .map(Option::unwrap_or_default)
        .collect();
    minutes.sort_by_key(|m| m.t);
    Ok((all_time, minutes))
}

/// `path` mit angehängter Endung (nicht ersetzt).
fn with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut name = OsString::from(path);
    name.push(suffix);
    PathBuf::from(name)
}

/// Verschiebt eine unbrauchbare Datei nach `<path>.bak`; gibt es die schon, bekommt die neue Sicherung einen
/// Zeitstempel, damit keine frühere Sicherung verloren geht.
fn move_aside(path: &Path, cause: &str, now: &Clock, on_err: &OnErr) {
    let mut bak = with_suffix(path, ".bak");
    if bak.exists() {
        bak = with_suffix(path, &format!(".{}.bak", now().format("%Y%m%d-%H%M%S")));
    }
    let shown = path.display();
    match std::fs::rename(path, &bak) {
        Err(e) => on_err(&format!(
            "statistik {shown} unbrauchbar ({cause}) und nicht verschiebbar: {e}"
        )),
        Ok(()) => on_err(&format!(
            "statistik {shown} unbrauchbar ({cause}), neu begonnen; alte Datei: {}",
            bak.display()
        )),
    }
}

/// Schreibt atomar: erst die temporäre Datei, dann umbenennen, damit ein Absturz nie eine halbe Datei hinterlässt.
/// Aufruf unter `write_mu`.
fn write(path: &Path, data: &[u8]) -> io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let tmp = with_suffix(path, ".tmp");
    std::fs::write(&tmp, data)?;
    std::fs::rename(&tmp, path)
}
