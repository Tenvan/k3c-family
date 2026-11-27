//! Anlegen aus den Vorlagen, Löschen und `plan_get` (Go `create.go`).

use std::collections::BTreeMap;
use std::path::Path;

use k3c_assetkit::ticket::{Row, add_to_index, defaults, draft};
use serde::Deserialize;

use super::domains::sync_sprint_domain;
use super::edit::{head_fields, plain_value, set_fields};
use super::planning::{Session, Sprint, parse_sprint, parse_ticket};
use super::projects::{check_project_ref, create_project, delete_project};
use super::projects_tables::sync_project_sprint;
use super::relations::{FIELD_DUP, check_relations, rel_ids, reverse_lines, with_reverse};
use super::similar::similar_notes;
use super::store::{ChangeSet, Ref, docs_path, lock_writes, quoted, read_text, resolve};
use super::store_resolve::{find_entry, find_sprint, is_slug, is_sprint_id, session_parts};
use super::tables::{
    find_row, find_table, heading, join_lines, remove_row, split_lines, sync_roadmap, sync_session_row,
    take_roadmap_row,
};
use super::ticketnr::next_ticket;

/// Ein neues Ticket, Sprint, Session (B-210) oder Projekt (B-357). `id` bei Sprint (M9), Session (M9.1) und Projekt
/// (GRA); ein Ticket bekommt die nächste freie Nummer.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct NewDoc {
    pub kind: String,
    pub id: String,
    pub slug: String,
    pub title: String,
    pub fields: BTreeMap<String, String>,
}

type Maker = fn(&mut ChangeSet, &NewDoc, &str) -> Result<(), String>;

/// Legt das Dokument als Kopie der Vorlage an und trägt es in Index, Fahrplan bzw. Session-Tabelle ein.
pub fn create(root: &Path, input: &NewDoc) -> Result<String, String> {
    let _writing = lock_writes();
    if !is_slug(&input.slug) {
        return Err(format!(
            "ungültiger Kurzname {}: nur a-z, 0-9 und `-`",
            quoted(&input.slug)
        ));
    }
    plain_value("Titel", &input.title)?;
    let make: Maker = match input.kind.as_str() {
        "ticket" => create_ticket,
        "sprint" => create_sprint,
        "session" => create_session,
        "projekt" => create_project,
        other => {
            return Err(format!(
                "unbekannte Art {}: ticket, sprint, session oder projekt",
                quoted(other)
            ));
        }
    };
    let mut c = ChangeSet::new(root);
    let template = c.read(&format!("vorlagen/{}.md", input.kind))?;
    make(&mut c, input, &template)?;
    c.apply()?;
    Ok(c.notes.join("\n"))
}

/// Ersetzt Überschrift und Felder (Vorgaben, dann Eingabe) einer Vorlage.
pub fn from_template(
    root: &Path,
    kind: &str,
    template: &str,
    header: &str,
    defaults: &[(&str, &str)],
    values: &BTreeMap<String, String>,
) -> Result<String, String> {
    let mut lines = split_lines(template);
    // Go bricht ohne Überschrift ab (Index -1).
    let Some(at) = find_row(&lines, "# ") else {
        return Err(format!("vorlagen/{kind}.md: Überschrift `# ` fehlt"));
    };
    lines[at] = format!("# {header}");
    let mut merged: BTreeMap<String, String> = defaults
        .iter()
        .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
        .collect();
    merged.extend(values.clone());
    set_fields(root, kind, &join_lines(&lines), &merged)
}

/// Streicht die Beispielzeilen der Tabelle unter `marker`.
pub fn without_rows(text: &str, marker: &str) -> String {
    let mut lines = split_lines(text);
    if let Some(table) = find_table(&lines, heading(marker)) {
        lines.drain(table.first..table.end);
    }
    join_lines(&lines)
}

/// Schreibt eine neue Datei mit den Zeilenenden ihrer Vorlage.
pub fn write_new(c: &mut ChangeSet, kind: &str, rel: &str, text: String) {
    c.write(rel, text);
    let crlf = c.crlf.get(&format!("vorlagen/{kind}.md")).copied().unwrap_or(false);
    c.crlf.insert(rel.to_owned(), crlf);
}

/// Vorlage füllen und Index-Zeile bilden macht der Ticket-Kern (B-595), Nummer und Feldprüfung die Planung.
fn create_ticket(c: &mut ChangeSet, input: &NewDoc, template: &str) -> Result<(), String> {
    let (nr, number_notes) = next_ticket(&c.root)?;
    let today = chrono::Local::now().fixed_offset();
    let text = draft(template, &nr, &input.title, &defaults(today)).map_err(|e| e.to_string())?;
    let text = set_fields(&c.root, "ticket", &text, &input.fields)?;
    if !rel_ids(input.fields.get(FIELD_DUP).map_or("", String::as_str)).is_empty() {
        return Err("ein neues Ticket ist kein Duplikat: das vorhandene Ticket nutzen oder danach plan_set".to_owned());
    }
    check_relations(&c.root, &nr, &head_fields(&text), &input.fields)?;
    let similar = similar_notes(c, &input.title);
    let ticket = parse_ticket(&text);
    if ticket.status != "offen" && ticket.status != "eingeplant" {
        return Err("ein neues Ticket ist offen oder eingeplant".to_owned());
    }
    check_project_ref(c, &ticket.project)?;
    let file = format!("{nr}-{}.md", input.slug);
    let rel = format!("backlog/{file}");
    let row = Row {
        nr: ticket.nr,
        link: file,
        domain: ticket.domain,
        typ: ticket.typ,
        prio: ticket.prio,
        status: ticket.status,
        sprint: ticket.sprint,
        title: ticket.title,
    };
    let index = c.read("backlog/README.md")?;
    let index = add_to_index(&index, &row).map_err(|e| format!("backlog/README.md: {e}"))?;
    write_new(c, "ticket", &rel, text);
    c.write("backlog/README.md", index);
    c.notes.push(format!("{nr} angelegt: docs/{rel} (Index ergänzt)"));
    c.notes.extend(number_notes);
    c.notes.extend(similar);
    Ok(())
}

fn create_sprint(c: &mut ChangeSet, input: &NewDoc, template: &str) -> Result<(), String> {
    let id = &input.id;
    if !is_sprint_id(id) {
        return Err(format!("Sprint-ID {}: Großbuchstaben und Zahl, z. B. M9", quoted(id)));
    }
    if find_sprint(&c.root, id).is_some() {
        return Err(format!("Sprint {id} gibt es schon"));
    }
    let domain = input.fields.get("Domäne").map_or("", String::as_str);
    let header = format!("{id} · {domain} · {}", input.title);
    let preset = [
        ("Status", "geplant"),
        ("Projekt", "–"),
        ("Reife", "Entwurf"),
        ("Tickets", "–"),
        ("Spec", "Entwurf"),
        ("Revision", "1"),
        ("Freigabe", "–"),
    ];
    let text = from_template(&c.root, "sprint", template, &header, &preset, &input.fields)?;
    if head_fields(&text).get("Status").is_none_or(|s| s != "geplant") {
        return Err("ein neuer Sprint ist geplant".to_owned());
    }
    let text = without_rows(&text, "## Sessions");
    let name = format!("{id}-{}", input.slug);
    let dir = format!("sprints/geplant/{name}");
    let sprint = parse_sprint(&text, &name, "geplant");
    let head = head_fields(&text);
    write_new(c, "sprint", &format!("{dir}/README.md"), text);
    c.notes.push(format!("{id} angelegt: docs/{dir}/ (Fahrplan ergänzt)"));
    check_project_ref(c, &sprint.project)?;
    sync_project_sprint(c, &sprint)?;
    sync_roadmap(c, &sprint, &head, "", &dir)
}

/// Vorgaben einer neuen Session; optionale Felder (PV1.1) nur, wenn die Vorlage sie hat.
fn session_preset<'a>(template: &str, branch: &'a str) -> Vec<(&'a str, &'a str)> {
    let mut preset = vec![
        ("Status", "offen"),
        ("Typ", "Umsetzung"),
        ("Agent", "autonom"),
        ("Umgebung", "?"),
        ("Branch", branch),
        ("Abhängig von", "–"),
        ("Tickets", "–"),
        ("Kriterien", "–"),
    ];
    let known = head_fields(template);
    preset.extend(
        ["Lane", "Modell"]
            .into_iter()
            .filter(|name| known.contains_key(*name))
            .map(|name| (name, "–")),
    );
    preset
}

fn create_session(c: &mut ChangeSet, input: &NewDoc, template: &str) -> Result<(), String> {
    let id = &input.id;
    let Some((prefix, number)) = session_parts(id) else {
        return Err(format!(
            "Session-ID {}: Sprint-ID, Punkt, Nummer, z. B. M9.1",
            quoted(id)
        ));
    };
    let Some(sprint) = find_sprint(&c.root, &prefix).filter(|s| s.state != "erledigt") else {
        return Err(format!("Sprint {prefix} fehlt oder ist erledigt"));
    };
    if find_entry(&docs_path(&c.root, &sprint.dir), &format!("{id}-"), false).is_some() {
        return Err(format!("Session {id} gibt es schon"));
    }
    let branch = format!("{}/{number}-{}", prefix.to_lowercase(), input.slug);
    let header = format!("{id} · {}", input.title);
    let preset = session_preset(template, &branch);
    let text = from_template(&c.root, "session", template, &header, &preset, &input.fields)?;
    let file = format!("{id}-{}.md", input.slug);
    let rel = format!("{}/{file}", sprint.dir);
    let mut head = head_fields(&text);
    write_new(c, "session", &rel, text);
    c.notes
        .push(format!("{id} angelegt: docs/{rel} (Session-Tabelle ergänzt)"));
    let row = Session {
        nr: id.clone(),
        file,
        typ: head.remove("Typ").unwrap_or_default(),
        agent: head.remove("Agent").unwrap_or_default(),
        status: head.remove("Status").unwrap_or_default(),
        ..Session::default()
    };
    sync_session_row(c, &format!("{}/README.md", sprint.dir), &row)?;
    sync_sprint_domain(c, &sprint.dir, &sprint.state, sync_roadmap)
}

/// Löscht einen Sprint-Entwurf in geplant/, eine Session darin oder ein Projekt ohne Sprints und Tickets; Tickets
/// werden verworfen, nie gelöscht.
pub fn delete(root: &Path, id: &str) -> Result<String, String> {
    let _writing = lock_writes();
    let r = resolve(root, id)?;
    if r.kind == "ticket" {
        return Err("ein Ticket wird nicht gelöscht: plan_set mit Status verworfen".to_owned());
    }
    let mut c = ChangeSet::new(root);
    if r.kind == "projekt" {
        delete_project(&mut c, &r)?;
    } else {
        delete_draft(&mut c, &r)?;
    }
    c.apply()?;
    Ok(format!("{} gelöscht: docs/{}", r.id, r.rel))
}

/// Sprint oder Session eines geplanten Sprints mit Spec Entwurf.
fn delete_draft(c: &mut ChangeSet, r: &Ref) -> Result<(), String> {
    let readme_rel = format!("{}/README.md", r.dir);
    let readme = c.read(&readme_rel)?;
    if r.state != "geplant" || head_fields(&readme).get("Spec").is_none_or(|s| s != "Entwurf") {
        return Err("löschen nur in geplanten Sprints mit Spec Entwurf; sonst Status ändern".to_owned());
    }
    if r.kind == "sprint" {
        let mut lines = split_lines(&c.read("sprints/README.md")?);
        take_roadmap_row(&mut lines, &r.id, &r.dir);
        c.write("sprints/README.md", join_lines(&lines));
        c.remove.push(r.dir.clone());
        let gone = Sprint {
            id: r.id.clone(),
            project: "–".to_owned(),
            ..Sprint::default()
        };
        return sync_project_sprint(c, &gone);
    }
    let mut lines = split_lines(&readme);
    remove_row(&mut lines, &format!("| {} |", r.id));
    c.write(&readme_rel, join_lines(&lines));
    c.remove.push(r.rel.clone());
    sync_sprint_domain(c, &r.dir, &r.state, sync_roadmap)
}

/// Pfad und Inhalt eines Dokuments; ein Ticket bekommt hinter dem Kopf die berechnete Gegenrichtung seiner Relationen
/// (B-696), die Datei bleibt unverändert.
pub fn get(root: &Path, id: &str) -> Result<String, String> {
    get_ref(root, &resolve(root, id)?)
}

/// Liest das aufgelöste Dokument wie [`get`].
pub fn get_ref(root: &Path, r: &Ref) -> Result<String, String> {
    let (mut text, _) = read_text(root, &r.rel)?;
    if r.kind == "ticket" {
        text = with_reverse(text, reverse_lines(root, &r.id));
    }
    Ok(format!("docs/{}\n\n{text}", r.rel))
}
