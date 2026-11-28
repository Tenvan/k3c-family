//! Tabellen der Projekte: Übersicht docs/projekte/README.md (Aktiv, Ruht, Erledigt) und die Sprint-Tabelle je
//! Projekt.

use std::collections::HashMap;

use super::planning::Sprint;
use super::projects::{Project, projects_in};
use super::store::{ChangeSet, quoted};
use super::tables::{
    cells, find_table, first_of, header_at, heading, join_lines, place_row, remove_row, split_lines, table_row,
};

const OVERVIEW_REL: &str = "projekte/README.md";
const SECTIONS: [(&str, &str); 3] = [("aktiv", "## Aktiv"), ("ruht", "## Ruht"), ("erledigt", "## Erledigt")];

/// Schreibt die drei Tabellen der Übersicht aus den Projekt-Dateien neu. Spalten ohne Wert aus der Datei (Ziel, Grund)
/// übernimmt sie aus der alten Zeile, sonst `–`. Der Hinweis „Noch keine Projekte …“ fällt mit dem ersten.
pub fn sync_overview(c: &mut ChangeSet) -> Result<(), String> {
    let projects = projects_in(c)?;
    let text = c.read(OVERVIEW_REL).map_err(|e| format!("{OVERVIEW_REL}: {e}"))?;
    let mut lines = split_lines(&text);
    let old = overview_rows(&lines);
    let none = HashMap::new();
    for (status, marker) in SECTIONS {
        let Some(table) = find_table(&lines, heading(marker)) else {
            return Err(format!("{OVERVIEW_REL}: Tabelle {status} fehlt"));
        };
        let rows: Vec<String> = projects
            .iter()
            .filter(|p| p.status == status)
            .map(|p| overview_row(&table.header, p, old.get(&p.file).unwrap_or(&none)))
            .collect();
        lines.splice(table.first..table.end, rows);
    }
    if !projects.is_empty() {
        lines.retain(|l| !l.starts_with("Noch keine Projekte"));
        // Doppelte Leerzeilen, die das Streichen des Hinweises hinterlässt.
        lines.dedup_by(|next, previous| next.is_empty() && previous.is_empty());
    }
    c.write(OVERVIEW_REL, join_lines(&lines));
    Ok(())
}

/// Alte Zeilen der Übersicht nach Dateiname, je Spaltenkopf.
fn overview_rows(lines: &[String]) -> HashMap<String, HashMap<String, String>> {
    let mut out = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        if !line.starts_with("| ") || line.starts_with("|---") {
            continue;
        }
        let Some(head) = header_at(lines, i) else {
            continue;
        };
        if i == index_of_header(lines, i) {
            continue;
        }
        let row: HashMap<String, String> = head.into_iter().zip(cells(line)).collect();
        let file = file_of_link(row.get("Datei").map_or("", String::as_str));
        if !file.is_empty() {
            out.insert(file, row);
        }
    }
    out
}

fn index_of_header(lines: &[String], mut i: usize) -> usize {
    while i > 0 && lines[i - 1].starts_with('|') {
        i -= 1;
    }
    i
}

/// Liest `[GRA-grafik.md](GRA-grafik.md)` oder einen nackten Dateinamen.
fn file_of_link(cell: &str) -> String {
    if let Some((_, target)) = cell.split_once("](") {
        return target.strip_suffix(')').unwrap_or(target).to_owned();
    }
    cell.trim_matches('`').to_owned()
}

fn overview_row(header: &[String], p: &Project, old: &HashMap<String, String>) -> String {
    let name = format!("{} · {}", p.id, p.title);
    let link = format!("[{}]({})", p.file, p.file);
    let row: Vec<&str> = header
        .iter()
        .map(|h| {
            let known = match h.as_str() {
                "Rang" => p.rang.as_str(),
                "Projekt" => name.as_str(),
                "Datei" => link.as_str(),
                _ => "",
            };
            first_of(&[known, old.get(h).map_or("", String::as_str), "–"])
        })
        .collect();
    table_row(&row)
}

/// Ordnet die Sprint-Tabelle des Projekts nach `order` (kommagetrennt), das genau ihre Sprints nennt.
pub fn reorder_sprints(text: &str, id: &str, order: &str) -> Result<String, String> {
    let mut lines = split_lines(text);
    let Some(table) = find_table(&lines, heading("## Sprints")) else {
        return Err(format!("{id}: Sprint-Tabelle fehlt"));
    };
    let mut rows: HashMap<String, String> = lines[table.first..table.end]
        .iter()
        .map(|l| (cells(l).swap_remove(0), l.clone()))
        .collect();
    let mut sorted = Vec::new();
    for name in order.split(',').map(str::trim) {
        let Some(row) = rows.remove(name) else {
            return Err(format!("Sprint {} steht nicht in der Tabelle von {id}", quoted(name)));
        };
        sorted.push(row);
    }
    if !rows.is_empty() {
        return Err(format!(
            "es fehlen {} Sprints von {id}; Sprints nennt alle in neuer Reihenfolge",
            rows.len()
        ));
    }
    lines.splice(table.first..table.end, sorted);
    Ok(join_lines(&lines))
}

/// Trägt einen Sprint in die Tabelle seines Projekts ein (ans Ende, bestehende Zeile an ihrem Platz) und aus allen
/// anderen aus; `–` trägt nur aus.
pub fn sync_project_sprint(c: &mut ChangeSet, sp: &Sprint) -> Result<(), String> {
    for p in projects_in(c)? {
        if p.id != sp.project && !p.sprints.contains(&sp.id) {
            continue;
        }
        let rel = format!("projekte/{}", p.file);
        let mut lines = split_lines(&c.read(&rel)?);
        let prefix = format!("| {} |", sp.id);
        if p.id == sp.project {
            let row = table_row(&[&sp.id, &sp.title, &sp.status]);
            lines = place_row(lines, &prefix, heading("## Sprints"), row).map_err(|e| format!("{}: {e}", p.id))?;
        } else {
            remove_row(&mut lines, &prefix);
        }
        c.write(&rel, join_lines(&lines));
    }
    Ok(())
}
