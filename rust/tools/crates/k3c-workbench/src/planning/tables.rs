//! Zeilen und Markdown-Tabellen: Index, Fahrplan und Session-Tabelle (Go `tables.go`).

use std::collections::HashMap;

use k3c_assetkit::ticket::{Row, index_row};

use super::index::{ARCHIVE_HEAD, ARCHIVE_INDEX, INDEX_FILES, MAIN_INDEX, TABLE_HEAD};
use super::planning::{Session, Sprint, Ticket};
use super::roadmap_order::roadmap_place;
use super::store::{ChangeSet, exists};

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

pub fn table_row<S: AsRef<str>>(cells: &[S]) -> String {
    let cells: Vec<&str> = cells.iter().map(AsRef::as_ref).collect();
    format!("| {} |", cells.join(" | "))
}

pub fn find_row<S: AsRef<str>>(lines: &[S], prefix: &str) -> Option<usize> {
    lines.iter().position(|l| l.as_ref().starts_with(prefix))
}

/// Setzt `row` in die Tabelle unter `marker`: an die Stelle der alten Zeile (`prefix`), wenn sie schon in dieser
/// Tabelle steht, sonst alte Zeile weg und neue ans Ende.
pub fn place_row(
    mut lines: Vec<String>,
    prefix: &str,
    marker: impl Fn(&str) -> bool,
    row: String,
) -> Result<Vec<String>, String> {
    let Some(table) = find_table(&lines, &marker) else {
        return Err("keine Tabelle gefunden".to_owned());
    };
    if let Some(i) = find_row(&lines, prefix).filter(|i| (table.first..table.end).contains(i)) {
        lines[i] = row;
        return Ok(lines);
    }
    remove_row(&mut lines, prefix);
    let end = find_table(&lines, &marker).map_or(0, |t| t.end);
    lines.insert(end, row);
    Ok(lines)
}

/// Entfernt die erste Zeile mit dem Präfix.
pub fn remove_row(lines: &mut Vec<String>, prefix: &str) {
    if let Some(i) = find_row(lines, prefix) {
        lines.remove(i);
    }
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

/// Trägt das Ticket in seinen Index ein (B-696 Teil B): offen und eingeplant in docs/backlog/README.md unter „Offen“,
/// erledigt und verworfen in docs/backlog/archiv/README.md; aus dem anderen Index verschwindet die Zeile. Altes Format
/// (Archiv-Tabelle im Haupt-Index, kein Archiv-Index) bleibt bis zum Neuaufbau; fehlen beide, entsteht der Archiv-Index.
pub fn sync_index(c: &mut ChangeSet, t: &Ticket, rel: &str) -> Result<(), String> {
    let main = c.read(MAIN_INDEX)?; // liest auch die Zeilenenden für einen neuen Archiv-Index
    let file = rel.rsplit('/').next().unwrap_or(rel);
    let (mut target, mut link, mut section) = (MAIN_INDEX, file.to_owned(), "## Offen");
    if rel.starts_with("backlog/archiv/") {
        (target, section) = (ARCHIVE_INDEX, "## Archiv");
        let legacy = find_table(&split_lines(&main), heading("## Archiv")).is_some();
        if legacy && !exists(&c.root, ARCHIVE_INDEX) {
            (target, link) = (MAIN_INDEX, format!("archiv/{file}"));
        }
    }
    for other in INDEX_FILES.into_iter().filter(|f| *f != target) {
        drop_rows(c, other, &t.nr, file)?;
    }
    let text = match c.read(target) {
        Ok(text) => text,
        Err(e) if target != ARCHIVE_INDEX || exists(&c.root, ARCHIVE_INDEX) => {
            return Err(format!("{target}: {e}"));
        }
        Err(_) => {
            let crlf = c.crlf.get(MAIN_INDEX).copied().unwrap_or(false);
            c.crlf.insert(ARCHIVE_INDEX.to_owned(), crlf);
            format!("{ARCHIVE_HEAD}\n## Archiv\n\n{TABLE_HEAD}")
        }
    };
    let prefix = format!("| [{}]({link})", t.nr);
    let row = index_row(&Row {
        nr: t.nr.clone(),
        link: link.clone(),
        domain: t.domain.clone(),
        typ: t.typ.clone(),
        prio: t.prio.clone(),
        status: t.status.clone(),
        sprint: t.sprint.clone(),
        title: t.title.clone(),
    });
    let rest = rows_without(&split_lines(&text), &t.nr, file, &link);
    let lines = place_row(rest, &prefix, heading(section), row).map_err(|e| format!("{target}: {e}"))?;
    c.write(target, join_lines(&lines));
    c.first.push(target.to_owned()); // erst eintragen, dann aus dem anderen Index entfernen (B-862)
    Ok(())
}

/// Ohne die Zeilen des Tickets (Link `file` oder `archiv/file`), außer der mit dem Link `keep`.
pub fn rows_without(lines: &[String], nr: &str, file: &str, keep: &str) -> Vec<String> {
    let starts: Vec<String> = [file.to_owned(), format!("archiv/{file}")]
        .iter()
        .filter(|link| *link != keep)
        .map(|link| format!("| [{nr}]({link})"))
        .collect();
    lines
        .iter()
        .filter(|l| !starts.iter().any(|s| l.starts_with(s)))
        .cloned()
        .collect()
}

/// Entfernt die Zeilen des Tickets aus einer Index-Datei; ein fehlender Archiv-Index ist kein Fehler.
fn drop_rows(c: &mut ChangeSet, rel: &str, nr: &str, file: &str) -> Result<(), String> {
    let text = match c.read(rel) {
        Ok(text) => text,
        Err(_) if rel == ARCHIVE_INDEX && !exists(&c.root, rel) => return Ok(()),
        Err(e) => return Err(format!("{rel}: {e}")),
    };
    let lines = split_lines(&text);
    let kept = rows_without(&lines, nr, file, "");
    if kept.len() != lines.len() {
        c.write(rel, join_lines(&kept));
    }
    Ok(())
}

/// Fahrplan-Tabelle für einen Sprint-Ordner (docs/sprints/README.md).
fn roadmap_marker(state: &str) -> &'static str {
    match state {
        "aktiv" => "## Aktiv",
        "erledigt" => "## Erledigt",
        _ => "## Geplant",
    }
}

/// Alte Werte einer Fahrplan-Zeile nach Spaltenkopf und ihr Platz.
pub type OldRow = (HashMap<String, String>, Option<usize>);

/// Verschiebt bzw. schreibt die Fahrplan-Zeile des Sprints; Spalten ohne bekannten Wert übernimmt sie aus der alten
/// Zeile (z. B. „Am Ende sichtbar“), sonst `–`.
pub fn sync_roadmap(
    c: &mut ChangeSet,
    sp: &Sprint,
    _fields: &HashMap<String, String>,
    old_dir: &str,
    new_dir: &str,
) -> Result<(), String> {
    const REL: &str = "sprints/README.md";
    let mut lines = split_lines(&c.read(REL)?);
    let (old, at) = take_roadmap_row(&mut lines, &sp.id, old_dir);
    let Some(table) = find_table(&lines, heading(roadmap_marker(&sp.status))) else {
        return Err(format!("{REL}: Tabelle für {} fehlt", sp.status));
    };
    let folder = format!("`{}/`", new_dir.strip_prefix("sprints/").unwrap_or(new_dir));
    let mut known = HashMap::from([
        ("Sprint", sp.id.as_str()),
        ("Domäne", sp.domain.as_str()),
        ("Reife", sp.reife.as_str()),
        ("Projekt", sp.project.as_str()),
        ("Ordner", folder.as_str()),
    ]);
    if c.retitle {
        known.insert("Thema", sp.title.as_str());
    }
    let row: Vec<&str> = table
        .header
        .iter()
        .map(|h| {
            let known = known.get(h.as_str()).copied().unwrap_or_default();
            let before = old.get(h).map_or("", String::as_str);
            // Thema der alten Zeile bleibt.
            let title = if h == "Thema" { sp.title.as_str() } else { "" };
            first_of(&[known, before, title, "–"])
        })
        .collect();
    // Gleiche Tabelle und gleiches Projekt: Platz behalten (Reihenfolge zählt); sonst nach Projekt-Rang (B-366).
    let moved = old
        .get("Projekt")
        .is_some_and(|p| project_cell(p) != project_cell(&sp.project));
    let kept = at.filter(|at| (table.first..=table.end).contains(at) && !moved);
    let at = match kept {
        Some(at) => at,
        None => roadmap_place(c, &lines, &table, sp)?,
    };
    lines.insert(at, table_row(&row));
    c.write(REL, join_lines(&lines));
    Ok(())
}

/// Nimmt die Fahrplan-Zeile des Sprints (erste Spalte = ID, Ordner-Spalte = `dir`) heraus und liefert ihre Werte nach
/// Spaltenkopf und ihren Platz. Zeilen anderer Tabellen mit demselben Ordner (z. B. „Offen am Gerät“) bleiben.
pub fn take_roadmap_row(lines: &mut Vec<String>, id: &str, dir: &str) -> OldRow {
    let mut old = HashMap::new();
    if dir.is_empty() {
        return (old, None);
    }
    let start = format!("| {id} |");
    let key = format!("`{}/`", dir.strip_prefix("sprints/").unwrap_or(dir));
    let Some(i) = lines.iter().position(|l| l.starts_with(&start) && l.contains(&key)) else {
        return (old, None);
    };
    // Eine Zeile mit falscher Spaltenzahl (KB6, B-842) liefert keine Werte: die Zellen wären verschoben.
    let values = cells(&lines[i]);
    if let Some(header) = header_at(lines, i)
        && header.len() == values.len()
    {
        old.extend(header.into_iter().zip(values));
    }
    lines.remove(i);
    (old, Some(i))
}

/// Spaltenkopf der Tabelle, zu der Zeile `i` gehört.
pub fn header_at<S: AsRef<str>>(lines: &[S], i: usize) -> Option<Vec<String>> {
    let mut head = i;
    while head > 0 && lines[head - 1].as_ref().starts_with('|') {
        head -= 1;
    }
    let ruled = lines.get(head + 1)?.as_ref().starts_with("|---");
    ruled.then(|| cells(lines[head].as_ref()))
}

/// Ein leeres Projekt steht im Fahrplan als „–“; so zählt ein leeres Feld nicht als Projektwechsel (B-403/N4).
fn project_cell(project: &str) -> &str {
    if project.is_empty() { "–" } else { project }
}

/// Schreibt die Zeile der Session in die Session-Tabelle der Sprint-README; fehlt die Tabelle (Sprint im Entwurf mit
/// Stichpunkten), entsteht sie direkt unter `## Sessions`.
pub fn sync_session_row(c: &mut ChangeSet, readme: &str, s: &Session) -> Result<(), String> {
    let mut lines = split_lines(&c.read(readme)?);
    if find_table(&lines, heading("## Sessions")).is_none() {
        let Some(i) = find_row(&lines, "## Sessions") else {
            return Err(format!("{readme}: Abschnitt Sessions fehlt"));
        };
        let head = ["", "| Nr. | Datei | Typ | Agent | Status |", "|---|---|---|---|---|"];
        lines.splice(i + 1..i + 1, head.map(str::to_owned));
    }
    let file = format!("`{}`", s.file);
    let row = table_row(&[&s.nr, &file, &s.typ, &s.agent, &s.status]);
    let lines = place_row(lines, &format!("| {} |", s.nr), heading("## Sessions"), row)?;
    c.write(readme, join_lines(&lines));
    Ok(())
}
