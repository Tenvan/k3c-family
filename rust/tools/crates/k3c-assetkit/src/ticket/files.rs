//! Ticket-Dateien: Nummern aus den Dateinamen, neueste Tickets, Lesen und atomares Schreiben.

use std::fs;
use std::io::{self, Write};
use std::path::Path;

use super::{Created, rename};
use crate::error::{Error, Result};
use crate::fsutil;

/// Die Nummer aus dem Dateinamen eines Tickets (`B-387-name.md`, auch mit Pfad), sonst 0.
fn number(name: &str) -> i64 {
    let base = name.trim_end_matches('/').rsplit('/').next().unwrap_or_default();
    let Some(rest) = base.strip_prefix("B-") else {
        return 0;
    };
    let digits = rest.len() - rest.trim_start_matches(|c: char| c.is_ascii_digit()).len();
    let tail = &rest[digits..];
    let is_ticket = digits > 0 && tail.starts_with('-') && tail.ends_with(".md") && !tail.contains('\n');
    if !is_ticket {
        return 0;
    }
    rest[..digits].parse().unwrap_or(i64::MAX) // wie Go `strconv.Atoi` beim Überlauf
}

/// Größte Ticket-Nummer in den Dateinamen, 0 ohne Ticket.
pub fn highest<S: AsRef<str>>(names: &[S]) -> i64 {
    names.iter().map(|n| number(n.as_ref())).max().unwrap_or(0)
}

/// Dateinamen eines Ordners, nach Namen sortiert, samt Kennzeichen „ist Ordner“.
fn list(dir: &Path) -> io::Result<Vec<(String, bool)>> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let is_dir = entry.file_type()?.is_dir();
        out.push((entry.file_name().to_string_lossy().into_owned(), is_dir));
    }
    out.sort();
    Ok(out)
}

/// Größte Nummer in `docs/backlog/` und `docs/backlog/archiv/`; ohne `docs/backlog/` ist sie nicht lesbar.
pub fn local_highest(root: &Path) -> Result<i64> {
    let backlog = root.join("docs").join("backlog");
    let names = |entries: Vec<(String, bool)>| entries.into_iter().map(|e| e.0).collect::<Vec<_>>();
    let top = highest(&names(list(&backlog)?));
    let archive = list(&backlog.join("archiv")).map_or(0, |e| highest(&names(e)));
    Ok(top.max(archive))
}

/// Die `n` Tickets mit den höchsten Nummern in `docs/backlog/` (ohne Archiv), neueste zuerst.
pub fn recent(root: &Path, n: usize) -> Result<Vec<Created>> {
    let dir = root.join("docs").join("backlog");
    let mut files: Vec<String> = list(&dir)
        .map_err(|e| Error::Msg(format!("backlog nicht lesbar: {e}")))?
        .into_iter()
        .filter(|(name, is_dir)| !is_dir && number(name) > 0)
        .map(|(name, _)| name)
        .collect();
    files.sort_by_key(|f| std::cmp::Reverse(number(f)));
    files
        .into_iter()
        .take(n)
        .map(|file| {
            let (text, _) = read_text(&dir.join(&file))?;
            let head = text.split('\n').next().unwrap_or_default();
            let title = head.split_once(" · ").map_or("", |(_, t)| t);
            Ok(Created {
                id: format!("B-{:03}", number(&file)),
                pfad: format!("docs/backlog/{file}"),
                titel: title.to_string(),
            })
        })
        .collect()
}

/// Liest eine Datei mit `\n`; das Kennzeichen merkt sich CRLF fürs Zurückschreiben. Ungültiges UTF-8 ist ein Fehler.
pub(super) fn read_text(path: &Path) -> io::Result<(String, bool)> {
    let text = fs::read_to_string(path)?;
    let crlf = text.contains("\r\n");
    Ok((text.replace("\r\n", "\n"), crlf))
}

/// Schreibt über eine Temp-Datei im selben Ordner, damit nie eine halbe Datei liegt.
pub(super) fn write_atomic(path: &Path, text: &str, crlf: bool) -> Result<()> {
    let text = if crlf {
        text.replace('\n', "\r\n")
    } else {
        text.to_string()
    };
    let dir = path.parent().unwrap_or(Path::new("."));
    let (mut file, tmp) = fsutil::create_temp(dir, ".ticket-")?;
    let written = file.write_all(text.as_bytes());
    drop(file);
    if let Err(e) = written {
        let _ = fsutil::remove(&tmp);
        return Err(Error::Msg(format!("schreiben: {e} <nil>")));
    }
    rename(&tmp, path)
}
