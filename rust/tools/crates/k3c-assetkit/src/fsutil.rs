//! Datei-Hilfen: Temp-Datei wie Go `os.CreateTemp`, Löschen wie `os.Remove` und Pfade wie `path.Clean`.

use std::fs::{self, File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const TEMP_TRIES: u64 = 10_000;

/// Legt in `dir` eine neue Datei `<prefix><Zahl>` an (unter Unix mit den Rechten 0600).
pub(crate) fn create_temp(dir: &Path, prefix: &str) -> io::Result<(File, PathBuf)> {
    create_temp_with(dir, prefix, "")
}

/// Wie [`create_temp`] mit Endung: `<prefix><Zahl><suffix>`. Wie Go ohne Pfadtrenner im Muster.
pub(crate) fn create_temp_with(dir: &Path, prefix: &str, suffix: &str) -> io::Result<(File, PathBuf)> {
    if [prefix, suffix].iter().any(|p| p.contains(['/', '\\'])) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "pattern contains path separator",
        ));
    }
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos() as u64);
    let seed = nanos ^ (u64::from(std::process::id()) << 32);
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut options, 0o600);
    for attempt in 0..TEMP_TRIES {
        let path = dir.join(format!("{prefix}{}{suffix}", seed.wrapping_add(attempt)));
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

/// Temp-Datei, die beim Fallenlassen verschwindet (Go `defer os.Remove`), auch wenn ein Download abgebrochen wird.
/// Nach Umbenennen oder Verschieben bleibt das Löschen ohne Wirkung.
pub(crate) struct TempFile(PathBuf);

impl TempFile {
    pub(crate) fn new(path: PathBuf) -> TempFile {
        TempFile(path)
    }

    pub(crate) fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = remove(&self.0);
    }
}

/// Go `path.Ext`: ab dem letzten Punkt des letzten Elements, sonst leer.
pub(crate) fn path_ext(p: &str) -> &str {
    let name = &p[p.rfind('/').map_or(0, |i| i + 1)..];
    name.rfind('.').map_or("", |i| &name[i..])
}

/// Go `path.Base`: letztes Element ohne Schrägstriche am Ende.
pub(crate) fn path_base(p: &str) -> &str {
    if p.is_empty() {
        return ".";
    }
    let trimmed = p.trim_end_matches('/');
    match trimmed.rsplit('/').next() {
        Some(name) if !name.is_empty() => name,
        _ => "/",
    }
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

/// Rechte 0644 wie Go nach `os.CreateTemp` (das 0600 anlegt); unter Windows ohne Wirkung.
pub(crate) fn make_readable(path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o644));
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
}
