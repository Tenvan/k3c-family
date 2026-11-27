//! Zeilen und Markdown-Tabellen. Stand RU69.8: nur die Lesehilfen; Index, Fahrplan und Tabellenzeilen folgen in
//! RU69.9.

/// Eine Tabelle in einer Zeilenliste: Kopf, erste Datenzeile, Ende (exklusiv).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub header: Vec<String>,
    pub first: usize,
    pub end: usize,
}

/// Sucht die erste Tabelle nach der Zeile, für die `marker` gilt, und vor der nächsten `## `-Überschrift.
pub fn find_table<S: AsRef<str>>(lines: &[S], marker: impl Fn(&str) -> bool) -> Option<Table> {
    let start = lines.iter().position(|l| marker(l.as_ref()))?;
    let line = |i: usize| lines[i].as_ref();
    let mut j = start + 1;
    while j + 1 < lines.len() && !line(j).starts_with("## ") {
        if line(j).starts_with("| ") && line(j + 1).starts_with("|---") {
            let mut end = j + 2;
            while end < lines.len() && line(end).starts_with('|') {
                end += 1;
            }
            return Some(Table {
                header: cells(line(j)),
                first: j + 2,
                end,
            });
        }
        j += 1;
    }
    None
}

/// Marker für [`find_table`]: Zeile beginnt mit `name`.
pub fn heading(name: &str) -> impl Fn(&str) -> bool + '_ {
    move |line| line.starts_with(name)
}

/// Zellen einer Tabellenzeile; immer mindestens eine.
pub fn cells(row: &str) -> Vec<String> {
    row.trim()
        .trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_owned())
        .collect()
}

pub fn find_row<S: AsRef<str>>(lines: &[S], prefix: &str) -> Option<usize> {
    lines.iter().position(|l| l.as_ref().starts_with(prefix))
}

pub fn split_lines(text: &str) -> Vec<String> {
    text.split('\n').map(str::to_owned).collect()
}

pub fn join_lines<S: AsRef<str>>(lines: &[S]) -> String {
    lines.iter().map(AsRef::as_ref).collect::<Vec<_>>().join("\n")
}

/// Der erste nicht leere Wert, sonst leer.
pub fn first_of<'a>(values: &[&'a str]) -> &'a str {
    values.iter().find(|v| !v.is_empty()).copied().unwrap_or_default()
}
