//! Log-Datei rückwärts oder ab einem Byte-Cursor lesen (Port von `reader.go`).

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

use chrono::{DateTime, FixedOffset};
use regex::Regex;

use super::entry::{Entry, level_rank, parse};

/// Höchste Zahl gelesener Bytes je Anfrage.
pub const DEFAULT_BUDGET: u64 = 8 << 20;
/// Größe eines Leseblocks.
const BLOCK_SIZE: u64 = 1 << 20;

/// Filtert einen Rückwärts-Lauf. Leere Felder filtern nicht.
#[derive(Clone, Debug, Default)]
pub struct Query {
    pub min_level: String,
    pub ns: String,
    /// Auf die Meldung.
    pub pattern: Option<Regex>,
    pub since: Option<DateTime<FixedOffset>>,
    /// 0 = unbegrenzt.
    pub limit: usize,
    /// 0 = [`DEFAULT_BUDGET`].
    pub budget: u64,
}

impl Query {
    fn matches(&self, entry: &Entry) -> bool {
        if !self.min_level.is_empty() && level_rank(&entry.level) < level_rank(&self.min_level) {
            return false;
        }
        if !self.ns.is_empty() && entry.ns != self.ns {
            return false;
        }
        self.pattern.as_ref().is_none_or(|re| re.is_match(&entry.msg))
    }
}

/// Ergebnis eines Rückwärts-Laufs, neueste Einträge zuerst.
#[derive(Clone, Debug, Default)]
pub struct ScanResult {
    pub entries: Vec<Entry>,
    pub bytes_read: u64,
    /// Ältere Treffer möglich.
    pub budget_hit: bool,
    /// Unlesbare Zeilen.
    pub skipped: usize,
}

fn open(path: &Path) -> io::Result<Option<(File, u64)>> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e),
    };
    let size = file.metadata()?.len();
    Ok(Some((file, size)))
}

/// Höchstens `len` Bytes ab `from`.
fn read_at(file: &mut File, from: u64, len: u64) -> io::Result<Vec<u8>> {
    file.seek(SeekFrom::Start(from))?;
    let mut data = Vec::new();
    file.take(len).read_to_end(&mut data)?;
    Ok(data)
}

fn trim_cr(mut line: &[u8]) -> &[u8] {
    while let Some(rest) = line.strip_suffix(b"\r") {
        line = rest;
    }
    line
}

/// Liest `path` von hinten, bis Limit, Since oder Budget greift. Eine fehlende Datei ist ein leeres Ergebnis.
pub fn scan(path: &Path, query: &Query) -> io::Result<ScanResult> {
    scan_blocks(path, query, BLOCK_SIZE)
}

/// [`scan`] mit eigener Blockgröße, damit Tests Blockgrenzen erzwingen können.
pub(super) fn scan_blocks(path: &Path, query: &Query, block: u64) -> io::Result<ScanResult> {
    let mut res = ScanResult::default();
    let Some((mut file, size)) = open(path)? else {
        return Ok(res);
    };
    let budget = match query.budget {
        0 => DEFAULT_BUDGET,
        budget => budget,
    };
    let (read, hit) = reverse_lines(&mut file, (size, budget, block), |line| {
        let Some(entry) = parse(line) else {
            res.skipped += 1;
            return true;
        };
        if query.since.is_some_and(|since| entry.time < since) {
            return false;
        }
        if query.matches(&entry) {
            res.entries.push(entry);
        }
        query.limit == 0 || res.entries.len() < query.limit
    })?;
    (res.bytes_read, res.budget_hit) = (read, hit);
    Ok(res)
}

/// Gibt die Zeilen von hinten an `each`, bis es `false` liefert oder das Budget verbraucht ist. Ergebnis: gelesene
/// Bytes und ob das Budget griff.
fn reverse_lines(
    file: &mut File,
    (size, budget, block): (u64, u64, u64),
    mut each: impl FnMut(&[u8]) -> bool,
) -> io::Result<(u64, bool)> {
    let (mut pos, mut read) = (size, 0);
    let mut carry: Vec<u8> = Vec::new();
    while pos > 0 {
        if read >= budget {
            return Ok((read, true));
        }
        let n = block.min(pos);
        pos -= n;
        let mut data = read_at(file, pos, n)?;
        read += n;
        data.append(&mut carry);
        let mut lines: Vec<&[u8]> = data.split(|&b| b == b'\n').collect();
        if pos > 0 {
            carry = lines.remove(0).to_vec(); // Anfang der Zeile liegt im nächsten Block
        }
        for line in lines.into_iter().rev().map(trim_cr) {
            if !line.is_empty() && !each(line) {
                return Ok((read, false));
            }
        }
    }
    Ok((read, false))
}

/// Neue Einträge ab einem Byte-Cursor, älteste zuerst.
#[derive(Clone, Debug, Default)]
pub struct SinceResult {
    pub entries: Vec<Entry>,
    /// Für den nächsten Aufruf.
    pub cursor: i64,
    /// Datei kleiner geworden, Budget oder Limit schnitt ältere ab.
    pub truncated: bool,
    pub skipped: usize,
}

/// Liest vorwärts ab `cursor` bis zur letzten vollständigen Zeile, höchstens `budget` Bytes (0 = Standard). `limit` 0
/// ist unbegrenzt, sonst bleiben die neuesten.
pub fn read_since(path: &Path, cursor: i64, limit: usize, budget: u64) -> io::Result<SinceResult> {
    let mut res = SinceResult {
        cursor,
        ..SinceResult::default()
    };
    let Some((mut file, size)) = open(path)? else {
        return Ok(res);
    };
    let start = match u64::try_from(cursor).ok().filter(|&c| c <= size) {
        Some(start) => start,
        None => {
            (res.cursor, res.truncated) = (0, true);
            0
        }
    };
    let budget = if budget == 0 { DEFAULT_BUDGET } else { budget };
    let data = read_at(&mut file, start, (size - start).min(budget))?;
    let end = data.iter().rposition(|&b| b == b'\n').map_or(0, |at| at + 1);
    for line in data[..end].split(|&b| b == b'\n').map(trim_cr) {
        if line.is_empty() {
            continue;
        }
        match parse(line) {
            Some(entry) => res.entries.push(entry),
            None => res.skipped += 1,
        }
    }
    res.cursor += i64::try_from(end).unwrap_or(i64::MAX);
    if (data.len() as u64) < size - start {
        res.truncated = true; // Budget: der Rest kommt beim nächsten Aufruf
    }
    if limit > 0 && res.entries.len() > limit {
        res.entries.drain(..res.entries.len() - limit);
        res.truncated = true;
    }
    Ok(res)
}
