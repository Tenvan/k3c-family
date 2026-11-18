//! Atomares Schreiben: erst eine temporäre Datei im selben Ordner, dann umbenennen. Ein unterbrochenes Schreiben lässt
//! den alten Stand heil. Port von `engine/store/atomic.go`.
use std::io::{self, Write};
use std::path::Path;
use std::time::Duration;

/// Wiederholungen beim Ersetzen einer kurz gesperrten Datei (B-187): bis zu `RENAME_TRIES` Versuche.
pub const RENAME_TRIES: u32 = 6;
/// Pause zwischen zwei Versuchen (Go `renamePause`).
pub const RENAME_PAUSE: Duration = Duration::from_millis(20);

type RenameFn = Box<dyn FnMut(&Path, &Path) -> io::Result<()>>;
type ReadFn = Box<dyn FnMut(&Path) -> io::Result<Vec<u8>>>;

/// Dateizugriff mit den Nähten aus Go (`renameFile`, `readFile`), der Wartezeit und dem Log. Log-Zeilen sammelt der
/// Puffer `log` (jede beginnt mit einem Emoji, Felder wie in Go); der Aufrufer leert ihn mit `take_log` und gibt sie weiter.
pub struct Fs {
    pub rename: RenameFn,
    pub read: ReadFn,
    pub pause: Duration,
    pub tries: u32,
    counter: u64,
    log: Vec<String>,
}

impl Default for Fs {
    fn default() -> Self {
        Fs {
            rename: Box::new(|a, b| std::fs::rename(a, b)),
            read: Box::new(|p| std::fs::read(p)),
            pause: RENAME_PAUSE,
            tries: RENAME_TRIES,
            counter: 0,
            log: Vec::new(),
        }
    }
}

/// Windows: ERROR_ACCESS_DENIED (5) und ERROR_SHARING_VIOLATION (32), wenn ein Fremdprozess (Virenscanner, Indexer)
/// die Datei kurz hält. Auf anderen Systemen wird nie wiederholt.
#[cfg(windows)]
pub fn retryable(err: &io::Error) -> bool {
    matches!(err.raw_os_error(), Some(5 | 32))
}

#[cfg(not(windows))]
pub fn retryable(_err: &io::Error) -> bool {
    false
}

fn base(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

impl Fs {
    /// Holt die gesammelten Log-Zeilen ab und leert den Puffer.
    pub fn take_log(&mut self) -> Vec<String> {
        std::mem::take(&mut self.log)
    }

    /// Wartet zwischen zwei Versuchen. Einzige Ausnahme vom Verbot von Threads und Uhr im Crate: Unter Windows hält
    /// ein Fremdprozess die Datei kurz, die Pause ist ein Feld (Tests setzen sie auf 0).
    fn wait(&self) {
        if !self.pause.is_zero() {
            std::thread::sleep(self.pause);
        }
    }

    /// Benennt `old` nach `new` um und ersetzt eine vorhandene Datei. Unter Windows wird ein kurz verweigertes Ersetzen
    /// begrenzt wiederholt (⏳ im Log, nach dem letzten Versuch ❌); andere Fehler melden sofort. Scheitert es, bleibt
    /// `new` unverändert.
    pub fn rename_file(&mut self, old: &Path, new: &Path) -> io::Result<()> {
        let mut res = (self.rename)(old, new);
        let mut i = 1;
        while i < self.tries {
            let Err(e) = &res else { break };
            if !retryable(e) {
                break;
            }
            self.log.push(format!(
                "⏳ Datei gesperrt, neuer Versuch ns=store datei={} versuch={} err={e}",
                base(new),
                i + 1
            ));
            self.wait();
            res = (self.rename)(old, new);
            i += 1;
        }
        if let (true, Err(e)) = (i > 1, &res) {
            self.log.push(format!(
                "❌ Datei bleibt gesperrt, nicht ersetzt ns=store datei={} versuche={i} err={e}",
                base(new)
            ));
        }
        res
    }

    /// Liest `path` wie `std::fs::read`. Unter Windows hält ein Virenscanner eine gerade umbenannte Datei kurz offen
    /// (B-274); das Lesen wird dann wie `rename_file` begrenzt wiederholt.
    pub fn read_retry(&mut self, path: &Path) -> io::Result<Vec<u8>> {
        let mut res = (self.read)(path);
        let mut i = 1;
        while i < self.tries && res.as_ref().err().is_some_and(retryable) {
            self.wait();
            res = (self.read)(path);
            i += 1;
        }
        res
    }

    /// Schreibt `data` nach `path` über eine temporäre Datei `.tmp-<pid>-<zähler>` im selben Ordner.
    pub fn write_atomic(&mut self, path: &Path, data: &[u8]) -> io::Result<()> {
        let dir = path.parent().unwrap_or_else(|| Path::new("."));
        std::fs::create_dir_all(dir)?;
        let (tmp_path, mut tmp) = self.create_temp(dir)?;
        let mut res = tmp.write_all(data).and_then(|()| tmp.flush());
        drop(tmp);
        if res.is_ok() {
            res = self.rename_file(&tmp_path, path); // ersetzt unter Windows und Linux eine vorhandene Datei
        }
        if res.is_err() {
            let _ = std::fs::remove_file(&tmp_path);
        }
        res
    }

    /// Legt eine neue Datei `.tmp-<pid>-<zähler>` an (`create_new`, bei Kollision weiterzählen; kein Zufall).
    fn create_temp(&mut self, dir: &Path) -> io::Result<(std::path::PathBuf, std::fs::File)> {
        loop {
            self.counter += 1;
            let path = dir.join(format!(".tmp-{}-{}", std::process::id(), self.counter));
            match std::fs::OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(f) => return Ok((path, f)),
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
                Err(e) => return Err(e),
            }
        }
    }
}
