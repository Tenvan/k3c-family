//! Sucht ffmpeg (Port von `ffmpeg_find.go`, `ffmpeg_find_windows.go` und `ffmpeg_find_other.go`). Die Quellen sind
//! austauschbar, damit Tests sie ohne Rechner-Zustand prüfen.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::Error;

/// Nennt die `ffmpeg.exe` ausdrücklich (Vorrang vor allem anderen, B-1230).
pub const FFMPEG_ENV: &str = "K3C_FFMPEG";

/// Sucht ein Programm nach Namen.
pub type LookPath = Arc<dyn Fn(&str) -> Result<String, Error> + Send + Sync>;

/// Die Quellen der Suche.
pub(super) struct Finder<'a> {
    pub env: &'a dyn Fn(&str) -> String,
    pub look_path: &'a dyn Fn(&str) -> Option<String>,
    /// User- und Machine-PATH aus der Registry (nur Windows).
    pub reg_paths: Option<&'a dyn Fn() -> Vec<String>>,
    pub glob: &'a dyn Fn(&str) -> Vec<String>,
    pub is_file: &'a dyn Fn(&str) -> bool,
}

pub(super) fn exe_name() -> &'static str {
    if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" }
}

fn join(base: &str, parts: &[&str]) -> String {
    let mut path = PathBuf::from(base);
    path.extend(parts);
    path.to_string_lossy().into_owned()
}

impl Finder<'_> {
    /// `K3C_FFMPEG`, dann der PATH des Prozesses, dann der aktuelle PATH aus der Registry (ein vor der Installation
    /// gestarteter Prozess kennt ihn nicht), dann der WinGet-Ordner. Der Fehler nennt die geprüften Orte; andere
    /// Namen laufen nur über den PATH.
    pub(super) fn find(&self, name: &str) -> Result<String, Error> {
        if name != "ffmpeg" {
            return (self.look_path)(name).ok_or_else(|| Error::Other(format!("{name} nicht im PATH")));
        }
        let mut tried = Vec::new();
        let value = (self.env)(FFMPEG_ENV).trim().to_owned();
        if !value.is_empty() {
            if (self.is_file)(&value) {
                return Ok(value);
            }
            tried.push(format!("{FFMPEG_ENV}={value} (Datei fehlt)"));
        }
        if let Some(found) = (self.look_path)("ffmpeg") {
            return Ok(found);
        }
        tried.push("PATH des Prozesses".to_owned());
        if cfg!(windows) || self.reg_paths.is_some() {
            let dirs = self.reg_paths.map_or_else(Vec::new, |paths| paths());
            if let Some(found) = dirs.iter().map(|d| join(d, &[exe_name()])).find(|p| (self.is_file)(p)) {
                return Ok(found);
            }
            tried.push("PATH aus der Registry".to_owned());
            // Neueste Version zuletzt.
            if let Some(found) = self.winget_pattern().and_then(|pattern| (self.glob)(&pattern).pop()) {
                return Ok(found);
            }
            tried.push("WinGet-Ordner".to_owned());
        }
        Err(Error::NoFfmpeg(tried))
    }

    fn winget_pattern(&self) -> Option<String> {
        let base = (self.env)("LOCALAPPDATA");
        let parts = [
            "Microsoft",
            "WinGet",
            "Packages",
            "Gyan.FFmpeg*",
            "ffmpeg-*",
            "bin",
            exe_name(),
        ];
        (!base.is_empty()).then(|| join(&base, &parts))
    }
}

/// `*` für beliebig viele Zeichen innerhalb eines Namens.
fn matches(pattern: &str, name: &str) -> bool {
    let Some((head, rest)) = pattern.split_once('*') else {
        return pattern == name;
    };
    let Some(name) = name.strip_prefix(head) else {
        return false;
    };
    (0..=name.len())
        .filter(|i| name.is_char_boundary(*i))
        .any(|i| matches(rest, &name[i..]))
}

/// Wie Go `filepath.Glob` für Muster mit `*`: je Ordnerebene nach Namen sortiert.
fn glob(pattern: &str) -> Vec<String> {
    let mut found = vec![PathBuf::new()];
    for component in Path::new(pattern).components() {
        let part = component.as_os_str().to_string_lossy();
        if !part.contains('*') {
            for path in &mut found {
                path.push(component);
            }
            continue;
        }
        found = found.iter().flat_map(|dir| children(dir, &part)).collect();
    }
    found
        .into_iter()
        .filter(|p| p.symlink_metadata().is_ok())
        .map(|p| p.to_string_lossy().into_owned())
        .collect()
}

fn children(dir: &Path, pattern: &str) -> Vec<PathBuf> {
    let Ok(read) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut names: Vec<String> = read
        .flatten()
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|name| matches(pattern, name))
        .collect();
    names.sort();
    names.into_iter().map(|name| dir.join(name)).collect()
}

/// Wie Go `exec.LookPath`: `PATH` mit den Endungen aus `PATHEXT`, relative Einträge zählen nicht.
fn path_lookup(name: &str) -> Option<String> {
    let found = crate::services::executable(name, Path::new(""));
    (found.is_absolute() && found.is_file()).then(|| found.to_string_lossy().into_owned())
}

/// Der Pfad zur ffmpeg (oder zu `name`) auf diesem Rechner.
pub fn find_ffmpeg(name: &str) -> Result<String, Error> {
    let env = |key: &str| std::env::var(key).unwrap_or_default();
    let is_file = |path: &str| Path::new(path).is_file();
    #[cfg(windows)]
    let registry: Option<&dyn Fn() -> Vec<String>> = Some(&registry::path_dirs);
    #[cfg(not(windows))]
    let registry: Option<&dyn Fn() -> Vec<String>> = None;
    let finder = Finder {
        env: &env,
        look_path: &path_lookup,
        reg_paths: registry,
        glob: &glob,
        is_file: &is_file,
    };
    finder.find(name)
}

/// Macht aus `%NAME%` den Wert der Variable (leer, wenn es sie nicht gibt); ein einzelnes `%` und `$` bleiben.
#[cfg(windows)]
fn expand_percent(text: &str, lookup: &dyn Fn(&str) -> String) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some((head, tail)) = rest.split_once('%') {
        let Some((name, after)) = tail.split_once('%') else {
            break;
        };
        out.push_str(head);
        if !name.is_empty() {
            out.push_str(&lookup(name));
        }
        rest = after;
    }
    out.push_str(rest);
    out
}

/// Der aktuelle PATH des Benutzers und des Systems aus der Registry.
#[cfg(windows)]
mod registry {
    #![allow(unsafe_code)]

    use windows::Win32::Foundation::ERROR_SUCCESS;
    use windows::Win32::System::Registry::{
        HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, RRF_NOEXPAND, RRF_RT_REG_EXPAND_SZ, RRF_RT_REG_SZ, RegGetValueW,
    };
    use windows::core::{HSTRING, w};

    /// Der Wert `Path` unter `root\key`, unaufgelöst.
    fn path_value(root: HKEY, key: &str) -> Option<String> {
        let key = HSTRING::from(key);
        let flags = RRF_RT_REG_SZ | RRF_RT_REG_EXPAND_SZ | RRF_NOEXPAND;
        let mut size = 0u32;
        // SAFETY: ohne Datenzeiger fragt der Aufruf nur die Größe ab; `key` und `size` überleben ihn.
        let status = unsafe { RegGetValueW(root, &key, w!("Path"), flags, None, None, Some(&raw mut size)) };
        if status != ERROR_SUCCESS {
            return None;
        }
        let mut buf = vec![0u16; (size as usize).div_ceil(2)];
        let data = buf.as_mut_ptr().cast();
        // SAFETY: `buf` fasst mindestens `size` Bytes; Windows schreibt höchstens so viele.
        let status = unsafe { RegGetValueW(root, &key, w!("Path"), flags, None, Some(data), Some(&raw mut size)) };
        if status != ERROR_SUCCESS {
            return None;
        }
        buf.truncate(size as usize / 2);
        let end = buf.iter().position(|c| *c == 0).unwrap_or(buf.len());
        Some(String::from_utf16_lossy(&buf[..end]))
    }

    pub(super) fn path_dirs() -> Vec<String> {
        let sources = [
            (HKEY_CURRENT_USER, "Environment"),
            (
                HKEY_LOCAL_MACHINE,
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment",
            ),
        ];
        let lookup = |name: &str| std::env::var(name).unwrap_or_default();
        sources
            .into_iter()
            .filter_map(|(root, key)| path_value(root, key))
            // Erst trennen, dann auflösen: ein einzelnes % wandert nicht über ein ; hinweg.
            .flat_map(|value| value.split(';').map(str::to_owned).collect::<Vec<_>>())
            .map(|dir| super::expand_percent(dir.trim(), &lookup).trim().to_owned())
            .filter(|dir| !dir.is_empty())
            .collect()
    }
}

#[cfg(test)]
#[path = "ffmpeg_find_tests.rs"]
mod tests;
