//! Datei-Hilfen: Temp-Datei wie Go `os.CreateTemp`, Löschen wie `os.Remove` und Pfade wie `path.Clean`.

use std::fs::{self, File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const TEMP_TRIES: u64 = 10_000;

/// Legt in `dir` eine neue Datei `<prefix><Zahl>` an (unter Unix mit den Rechten 0600).
pub(crate) fn create_temp(dir: &Path, prefix: &str) -> io::Result<(File, PathBuf)> {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos() as u64);
    let seed = nanos ^ (u64::from(std::process::id()) << 32);
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut options, 0o600);
    for attempt in 0..TEMP_TRIES {
        let path = dir.join(format!("{prefix}{}", seed.wrapping_add(attempt)));
        match options.open(&path) {
            Ok(file) => return Ok((file, path)),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
    }
    Err(io::Error::new(io::ErrorKind::AlreadyExists, "keine freie Temp-Datei"))
}

/// Löscht eine Datei wie Go `os.Remove`: unter Windows auch eine schreibgeschützte.
pub(crate) fn remove(path: &Path) -> io::Result<()> {
    let result = fs::remove_file(path);
    #[cfg(windows)]
    if result.is_err()
        && let Ok(meta) = fs::metadata(path)
        && meta.permissions().readonly()
    {
        let mut perms = meta.permissions();
        #[allow(clippy::permissions_set_readonly_false)]
        perms.set_readonly(false);
        fs::set_permissions(path, perms)?;
        return fs::remove_file(path);
    }
    result
}

/// Go `path.Clean` für Pfade mit `/`.
pub(crate) fn path_clean(p: &str) -> String {
    let rooted = p.starts_with('/');
    let mut parts: Vec<&str> = Vec::new();
    for part in p.split('/') {
        match part {
            "" | "." => {}
            ".." => match parts.last() {
                Some(&last) if last != ".." => {
                    parts.pop();
                }
                _ if !rooted => parts.push(".."),
                _ => {}
            },
            _ => parts.push(part),
        }
    }
    let joined = parts.join("/");
    match (rooted, joined.is_empty()) {
        (true, _) => format!("/{joined}"),
        (false, true) => ".".to_string(),
        (false, false) => joined,
    }
}

/// Hängt den relativen `/`-Pfad `rel` an `base` an wie Go `filepath.Join`: ein Laufwerk in `rel` (`C:/x`) ersetzt
/// `base` nicht, wie es [`Path::join`] täte.
pub(crate) fn join_below(base: &Path, rel: &str) -> PathBuf {
    let mut out = base.as_os_str().to_os_string();
    out.push(std::path::MAIN_SEPARATOR_STR);
    out.push(rel.replace('/', std::path::MAIN_SEPARATOR_STR));
    PathBuf::from(out)
}
