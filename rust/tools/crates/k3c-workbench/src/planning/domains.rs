//! Domäne eines Sprints aus seinen Sessions (arbeitsweise.md › Domänen, B-356).

use std::collections::HashMap;

use super::edit::{DOMAINS, head_fields, set_field};
use super::planning::{Sprint, is_session_file, parse_sprint};
use super::store::ChangeSet;
use super::tables::{find_row, join_lines, split_lines};

/// Domänen der Sessions in Reihenfolge des ersten Auftretens, kommagetrennt.
fn session_domains(c: &mut ChangeSet, dir: &str, sprint: &Sprint) -> Result<String, String> {
    let mut list: Vec<String> = Vec::new();
    for session in sprint.sessions.iter().filter(|s| is_session_file(&s.file)) {
        let text = c.read(&format!("{dir}/{}", session.file))?;
        if let Some(domain) = head_fields(&text).remove("Domäne")
            && DOMAINS.contains(&domain.as_str())
            && !list.contains(&domain)
        {
            list.push(domain);
        }
    }
    Ok(list.join(", "))
}

/// Leitet Feld `Domäne`, Überschrift und Fahrplan-Spalte eines Sprints aus seinen Sessions ab. Ohne Session mit Domäne
/// bleibt der Sprint unverändert. Liest Session-Tabelle und Dateien inklusive geplanter Änderungen. `sync_roadmap` ist
/// die Fahrplan-Pflege (Go `syncRoadmap`, RU69.9): Sprint, Felder der README, alter und neuer Ordner.
pub fn sync_sprint_domain(
    c: &mut ChangeSet,
    dir: &str,
    state: &str,
    sync_roadmap: impl FnOnce(&mut ChangeSet, &Sprint, &HashMap<String, String>, &str, &str) -> Result<(), String>,
) -> Result<(), String> {
    let readme_path = format!("{dir}/README.md");
    let readme = c.read(&readme_path)?;
    let name = dir.rsplit('/').next().unwrap_or(dir);
    let mut sprint = parse_sprint(&readme, name, state);
    let want = session_domains(c, dir, &sprint)?;
    let field = head_fields(&readme).remove("Domäne").unwrap_or_default();
    if want.is_empty() || (want == sprint.domain && field == want) {
        return Ok(());
    }
    let mut lines = split_lines(&set_field(&readme, "Domäne", &want));
    let Some(heading) = find_row(&lines, "# ") else {
        return Err(format!("{readme_path}: Überschrift `# {} · … · …` fehlt", sprint.id));
    };
    lines[heading] = format!("# {} · {want} · {}", sprint.id, sprint.title);
    let text = join_lines(&lines);
    let head = head_fields(&text);
    c.write(&readme_path, text);
    c.notes.push(format!("{}: Domäne {want} (aus den Sessions)", sprint.id));
    sprint.domain = want;
    sync_roadmap(c, &sprint, &head, dir, dir)
}
