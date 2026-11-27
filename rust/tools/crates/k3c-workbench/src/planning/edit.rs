//! Felder und Abschnitte schreiben (Go `edit.go`): `set` ändert Kopf-Felder und zieht Index, Session-Tabelle,
//! Fahrplan, Ordner und Projekte nach; `section` und `sections` ersetzen Abschnitte. `Create` und `Delete` folgen in
//! RU69.10.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use super::domains::sync_sprint_domain;
use super::next_lanes::{MODELS, check_lane, insert_field};
use super::planning::{Session, field_line, fields, parse_sprint, parse_ticket};
use super::projects::{check_project_ref, set_project};
use super::projects_tables::sync_project_sprint;
use super::relations::{FIELD_DUP, FIELD_REPLACE, check_rel_format, ticket_relations};
use super::store::{ChangeSet, Ref, STATES, lock_writes, quoted, read_text, resolve};
use super::tables::{join_lines, split_lines, sync_index, sync_roadmap, sync_session_row};

pub const DOMAINS: [&str; 8] = ["REG", "SIM", "SRV", "CLI", "PLAT", "INF", "DEV", "PAP"];
const SPECS: [&str; 3] = ["Entwurf", "freigegeben", "rückwirkend"];
const PRIOS: [&str; 4] = ["hoch", "mittel", "niedrig", "?"];
/// offline = ohne laufende Dienste prüfbar; live = braucht Server, Browser oder Gerät; ? = nicht eingeordnet.
pub const ENVS: [&str; 3] = ["offline", "live", "?"];
/// Pseudo-Feld für den Titel in der Überschrift `# ID · DOMÄNE · Titel` (Sprints und Tickets).
const FIELD_TITLE: &str = "Titel";

/// Auswahlfelder je Art, wie in tests/planning.test.ts.
fn allowed(kind: &str, name: &str) -> Option<&'static [&'static str]> {
    Some(match (kind, name) {
        ("ticket" | "sprint" | "session", "Domäne") => &DOMAINS,
        ("ticket" | "session", "Umgebung") => &ENVS,
        ("ticket" | "sprint", "Spec") => &SPECS,
        ("ticket", "Typ") => &["Idee", "Problem", "Schuld", "Frage"],
        ("ticket", "Prio") => &PRIOS,
        ("ticket", "Status") => &["offen", "eingeplant", "erledigt", "verworfen"],
        ("sprint", "Status") => &STATES,
        ("sprint", "Reife") => &["Entwurf", "bereit"],
        ("session", "Status") => &["offen", "in Arbeit", "fertig", "blockiert", "verworfen"],
        ("session", "Typ") => &["Umsetzung", "Review", "Workshop"],
        ("session", "Agent") => &["autonom", "Mensch"],
        ("session", "Modell") => &MODELS,
        ("projekt", "Status") => &["aktiv", "ruht", "erledigt"],
        _ => return None,
    })
}

/// Der Kopf ist der Teil vor der ersten `## `-Überschrift (Kopfzeile und Felder); Rückgabe ist seine Zeilenzahl.
pub fn head_end<S: AsRef<str>>(lines: &[S]) -> usize {
    lines
        .iter()
        .position(|l| l.as_ref().starts_with("## "))
        .unwrap_or(lines.len())
}

pub fn head_fields(text: &str) -> HashMap<String, String> {
    let lines = split_lines(text);
    fields(&lines[..head_end(&lines)])
}

/// Feldnamen der Vorlage docs/vorlagen/<kind>.md mit ihren Werten.
fn template_fields(root: &Path, kind: &str) -> Result<HashMap<String, String>, String> {
    let (text, _) = read_text(root, &format!("vorlagen/{kind}.md")).map_err(|e| format!("ohne Vorlage {kind}: {e}"))?;
    Ok(head_fields(&text))
}

fn check_value(kind: &str, name: &str, value: &str, known: &HashMap<String, String>) -> Result<(), String> {
    if !known.contains_key(name) {
        let mut names: Vec<&str> = known.keys().map(String::as_str).collect();
        names.sort_unstable();
        return Err(format!("unbekanntes Feld {} ({})", quoted(name), names.join(", ")));
    }
    plain_value(name, value)?;
    if let Some(values) = allowed(kind, name).filter(|values| !values.contains(&value)) {
        return Err(format!("{name} = {} ungültig ({})", quoted(value), values.join(", ")));
    }
    if kind == "session" && name == "Lane" {
        check_lane(value)?;
    }
    if name == "Revision" && (value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit())) {
        return Err("die Revision muss eine Zahl sein".to_owned());
    }
    if kind == "ticket" && (name == FIELD_DUP || name == FIELD_REPLACE) {
        return check_rel_format(name, value);
    }
    Ok(())
}

/// Einzeilig, ohne `|` (Werte landen in Tabellen), nicht leer.
fn plain_value(name: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.contains(['|', '\r', '\n']) {
        return Err(format!("{name}: Wert leer, mehrzeilig oder mit `|`"));
    }
    Ok(())
}

pub fn set_field(text: &str, name: &str, value: &str) -> String {
    let mut lines = split_lines(text);
    let end = head_end(&lines);
    for line in &mut lines[..end] {
        if field_line(line).is_some_and(|(found, _)| found == name) {
            *line = format!("- **{name}:** {value}");
        }
    }
    join_lines(&lines)
}

/// Prüft alle Werte, bevor es einen setzt; danach müssen Spec und Freigabe zusammenpassen.
pub fn set_fields(root: &Path, kind: &str, text: &str, values: &BTreeMap<String, String>) -> Result<String, String> {
    let known = template_fields(root, kind)?;
    let mut text = text.to_owned();
    for (name, value) in values {
        check_value(kind, name, value, &known)?;
        if head_fields(&text).contains_key(name) {
            text = set_field(&text, name, value);
            continue;
        }
        text = insert_field(kind, &text, name, value).ok_or_else(|| {
            let template = known.get(name).map_or("", String::as_str);
            format!("das Feld {name} fehlt in der Datei (Vorlage: {template}), nichts geändert")
        })?;
    }
    check_head(&head_fields(&text))?;
    Ok(text)
}

fn check_head(f: &HashMap<String, String>) -> Result<(), String> {
    let release = f.get("Freigabe").map_or("", String::as_str);
    if let Some(spec) = f.get("Spec")
        && (spec == "freigegeben") == release.starts_with('–')
    {
        return Err(format!(
            "die Spec {} passt nicht zu Freigabe {}: freigegeben braucht Datum und Quelle, sonst `–`",
            quoted(spec),
            quoted(release)
        ));
    }
    // Go nennt bei mehreren offenen Feldern ein zufälliges; hier das erste nach Name.
    if let Some((name, value)) = f.iter().filter(|(_, v)| v.contains(" | ")).min() {
        return Err(format!("das Feld {name} fehlt noch (Vorlage: {value})"));
    }
    Ok(())
}

/// Ändert Kopf-Felder eines Dokuments und zieht Index, Session-Tabelle, Fahrplan, Ordner und Projekte nach.
pub fn set(root: &Path, id: &str, values: &BTreeMap<String, String>) -> Result<String, String> {
    let _writing = lock_writes();
    let r = resolve(root, id)?;
    let title = values.get(FIELD_TITLE);
    let mut values = check_set(r.kind, values)?;
    let mut c = ChangeSet::new(root);
    c.retitle = title.is_some();
    if r.kind == "projekt" {
        set_project(&mut c, &r, &values)?;
    } else {
        if let Some(project) = values.get("Projekt") {
            check_project_ref(&mut c, project)?;
        }
        let mut text = c.read(&r.rel)?;
        if let Some(title) = title {
            text = set_title(&text, r.kind, title);
        }
        if r.kind == "ticket" {
            values = ticket_relations(&mut c, &r, &text, &values)?;
        }
        text = set_fields(root, r.kind, &text, &values).map_err(|e| format!("{}: {e}", r.rel))?;
        follow(&mut c, &r, text)?;
    }
    c.apply()?;
    Ok(c.notes.join("\n"))
}

/// Prüft Domäne (Sprint) und Titel (nur Sprint und Ticket) und liefert die Werte ohne das Pseudo-Feld Titel.
fn check_set(kind: &str, values: &BTreeMap<String, String>) -> Result<BTreeMap<String, String>, String> {
    if kind == "sprint" && values.get("Domäne").is_some_and(|d| !d.is_empty()) {
        return Err(
            "die Domäne eines Sprints ist aus seinen Sessions abgeleitet; Domäne der Sessions setzen".to_owned(),
        );
    }
    let mut out = values.clone();
    let Some(title) = out.remove(FIELD_TITLE) else {
        return Ok(out);
    };
    if kind != "sprint" && kind != "ticket" {
        return Err(format!("{FIELD_TITLE} lässt sich nur bei Sprints und Tickets setzen"));
    }
    plain_value(FIELD_TITLE, &title)?;
    Ok(out)
}

/// Ersetzt den Titel in der ersten Überschrift, ID und Domäne bleiben.
fn set_title(text: &str, kind: &str, title: &str) -> String {
    // Sprint: ID · DOMÄNE · Titel; Ticket: ID · Titel
    let parts = if kind == "ticket" { 2 } else { 3 };
    let mut lines = split_lines(text);
    if let Some(line) = lines.iter_mut().find(|l| l.starts_with("# ")) {
        let found: Vec<&str> = line.splitn(parts, " · ").collect();
        if found.len() == parts {
            *line = format!("{} · {title}", found[..parts - 1].join(" · "));
        }
    }
    join_lines(&lines)
}

/// Schreibt das Dokument an seinen Platz (Ordner folgt dem Status) und zieht die Verweise nach.
fn follow(c: &mut ChangeSet, r: &Ref, text: String) -> Result<(), String> {
    match r.kind {
        "ticket" => follow_ticket(c, r, text),
        "sprint" => follow_sprint(c, r, text),
        _ => follow_session(c, r, text),
    }
}

fn follow_session(c: &mut ChangeSet, r: &Ref, text: String) -> Result<(), String> {
    let mut f = head_fields(&text);
    c.write(&r.rel, text);
    c.notes.push(format!("{} geändert: docs/{}", r.id, r.rel));
    let session = Session {
        nr: r.id.clone(),
        file: base(&r.rel).to_owned(),
        typ: f.remove("Typ").unwrap_or_default(),
        agent: f.remove("Agent").unwrap_or_default(),
        status: f.remove("Status").unwrap_or_default(),
        ..Session::default()
    };
    sync_session_row(c, &format!("{}/README.md", r.dir), &session)?;
    sync_sprint_domain(c, &r.dir, &r.state, sync_roadmap)
}

fn base(rel: &str) -> &str {
    rel.rsplit('/').next().unwrap_or(rel)
}

fn follow_ticket(c: &mut ChangeSet, r: &Ref, text: String) -> Result<(), String> {
    let t = parse_ticket(&text);
    let folder = if t.status == "erledigt" || t.status == "verworfen" {
        "archiv/"
    } else {
        ""
    };
    let rel = format!("backlog/{folder}{}", base(&r.rel));
    if rel != r.rel {
        c.moves.push((r.rel.clone(), rel.clone()));
        let crlf = c.crlf.get(&r.rel).copied().unwrap_or(false);
        c.crlf.insert(rel.clone(), crlf);
    }
    c.write(&rel, text);
    c.notes
        .push(format!("{} geändert: docs/{rel} (Index nachgezogen)", r.id));
    sync_index(c, &t, &rel)
}

fn follow_sprint(c: &mut ChangeSet, r: &Ref, text: String) -> Result<(), String> {
    let f = head_fields(&text);
    let status = f.get("Status").map_or("", String::as_str);
    let sp = parse_sprint(&text, base(&r.dir), status);
    if sp.status == "aktiv" && (sp.reife != "bereit" || sp.spec == "Entwurf") {
        return Err("aktiv nur mit Reife bereit und freigegebener Spec".to_owned());
    }
    let dir = format!("sprints/{}/{}", sp.status, base(&r.dir));
    let readme = format!("{dir}/README.md");
    if dir != r.dir {
        c.moves.push((r.dir.clone(), dir.clone()));
        let crlf = c.crlf.get(&r.rel).copied().unwrap_or(false);
        c.crlf.insert(readme.clone(), crlf);
    }
    c.write(&readme, text);
    c.notes
        .push(format!("{} geändert: docs/{dir}/ (Fahrplan nachgezogen)", r.id));
    sync_project_sprint(c, &sp)?;
    sync_roadmap(c, &sp, &f, &r.dir, &dir)
}

/// Ersetzt den Inhalt des Abschnitts `## <name>`; der Abschnitt muss in der Datei stehen.
pub fn section(root: &Path, id: &str, name: &str, body: &str) -> Result<String, String> {
    sections(root, id, &BTreeMap::from([(name.to_owned(), body.to_owned())]))
}

/// Ersetzt mehrere Abschnitte in einem Schreibvorgang (B-518). Erst werden alle Texte und Namen geprüft; ein
/// unbekannter Abschnitt lässt die Datei unverändert.
pub fn sections(root: &Path, id: &str, bodies: &BTreeMap<String, String>) -> Result<String, String> {
    if bodies.is_empty() {
        return Err("keine Abschnitte angegeben".to_owned());
    }
    let _writing = lock_writes();
    let r = resolve(root, id)?;
    let is_heading = |l: &str| l.starts_with("# ") || l.starts_with("## ");
    if bodies.values().any(|body| body.split('\n').any(is_heading)) {
        return Err("der Text darf keine Überschrift `#`/`##` enthalten (Unterüberschriften ab `###`)".to_owned());
    }
    let mut c = ChangeSet::new(root);
    let mut lines = split_lines(&c.read(&r.rel)?);
    let names = section_names(&lines, &r.id, bodies)?;
    for name in &names {
        replace_section(&mut lines, name, bodies.get(*name).map_or("", String::as_str));
    }
    c.write(&r.rel, join_lines(&lines).trim_end_matches('\n').to_owned() + "\n");
    c.apply()?;
    Ok(format!("{} · {} ersetzt: docs/{}", r.id, names.join(", "), r.rel))
}

/// Prüft, dass es jeden Abschnitt gibt, und liefert die Namen in der Reihenfolge der Datei.
fn section_names<'a>(lines: &[String], id: &str, bodies: &'a BTreeMap<String, String>) -> Result<Vec<&'a str>, String> {
    let (mut found, mut missing) = (Vec::new(), Vec::new());
    for name in bodies.keys() {
        let wanted = format!("## {name}");
        match lines.iter().position(|l| *l == wanted) {
            Some(at) => found.push((at, name.as_str())),
            None => missing.push(quoted(name)),
        }
    }
    if !missing.is_empty() {
        missing.sort();
        let have: Vec<&str> = lines.iter().filter_map(|l| l.strip_prefix("## ")).collect();
        return Err(format!(
            "unbekannter Abschnitt {} in {id}, nichts geändert (vorhanden: {})",
            missing.join(", "),
            have.join(", ")
        ));
    }
    found.sort_unstable();
    Ok(found.into_iter().map(|(_, name)| name).collect())
}

/// Ersetzt den Inhalt unter `## <name>` bis zur nächsten `## `-Überschrift.
fn replace_section(lines: &mut Vec<String>, name: &str, body: &str) {
    let title = format!("## {name}");
    let Some(start) = lines.iter().position(|l| *l == title) else {
        return;
    };
    let end = (start + 1..lines.len())
        .find(|&i| lines[i].starts_with("## "))
        .unwrap_or(lines.len());
    let mut replacement = vec![title, String::new()];
    replacement.extend(split_lines(body.trim_end_matches('\n')));
    if end < lines.len() {
        replacement.push(String::new());
    }
    lines.splice(start..end, replacement);
}
