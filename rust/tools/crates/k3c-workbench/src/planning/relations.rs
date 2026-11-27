//! Relationen zwischen Tickets (B-696 Teil A): „Duplikat von“ (genau eine ID) und „Ersetzt“ (Liste) stehen nur im
//! neueren Ticket. Die Gegenrichtung („Ersetzt durch“, „Duplikate“) berechnet `get` beim Lesen; die Datei des
//! Gegenstücks bleibt unverändert. Fehlt ein Feld (Altdatei), gilt es als `–`.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::Read as _;
use std::path::Path;

use super::edit::{head_end, head_fields};
use super::planning::is_ticket_file;
use super::store::{ChangeSet, Ref, docs_path, entries, is_ticket_id, quoted};
use super::tables::{join_lines, split_lines};
use super::ticketid::{ticket_cmp, ticket_file_id};

pub const FIELD_DUP: &str = "Duplikat von";
pub const FIELD_REPLACE: &str = "Ersetzt";
/// Der Kopf eines Tickets steht in den ersten 8 KB.
const HEAD_LIMIT: u64 = 8 << 10;

/// Der Teil eines Ticket-Kopfs, den die Relationen brauchen.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RelInfo {
    pub status: String,
    pub dup: String,
    pub replaces: Vec<String>,
}

impl RelInfo {
    /// Verweise eines Tickets (Duplikat und Ersatz).
    fn edges(&self) -> impl Iterator<Item = &String> {
        Some(&self.dup)
            .filter(|d| !d.is_empty())
            .into_iter()
            .chain(&self.replaces)
    }
}

/// Ticket-ID → Relationen.
pub type Relations = HashMap<String, RelInfo>;

/// IDs eines Relationsfelds; leer oder `–` heißt keine.
pub fn rel_ids(value: &str) -> Vec<String> {
    let value = value.trim();
    if value.is_empty() || value.starts_with('–') {
        return Vec::new();
    }
    value.split(',').map(|p| p.trim().to_owned()).collect()
}

/// `–`, bei „Duplikat von“ genau eine ID, bei „Ersetzt“ eine Liste `B-001, B-002` ohne Dopplung.
pub fn check_rel_format(name: &str, value: &str) -> Result<(), String> {
    let ids = rel_ids(value);
    if value != "–" && ids.is_empty() {
        return Err(format!("{name}: `–` oder Ticket-IDs wie B-123"));
    }
    if name == FIELD_DUP && ids.len() > 1 {
        return Err(format!("{name}: genau eine ID, nicht {}", ids.len()));
    }
    if value != "–" && value != ids.join(", ") {
        return Err(format!("{name}: IDs mit „, “ trennen, z. B. B-001, B-002"));
    }
    for (i, id) in ids.iter().enumerate() {
        if !is_ticket_id(id) {
            return Err(format!("{name}: {} ist keine Ticket-ID (B-123)", quoted(id)));
        }
        if ids[..i].contains(id) {
            return Err(format!("{name}: {id} doppelt"));
        }
    }
    Ok(())
}

/// Kopf einer Datei (bis zur ersten `## `-Überschrift, höchstens [`HEAD_LIMIT`] Bytes).
fn read_head(path: &Path) -> String {
    let mut bytes = Vec::new();
    let Ok(file) = File::open(path) else {
        return String::new();
    };
    // Wie Go: ein Lesefehler lässt das schon Gelesene gelten.
    let _ = file.take(HEAD_LIMIT).read_to_end(&mut bytes);
    let mut text = String::from_utf8_lossy(&bytes).replace("\r\n", "\n");
    if let Some(end) = text.find("\n## ") {
        text.truncate(end);
    }
    text
}

/// Status und Relationen aller Tickets in Backlog und Archiv.
pub fn relations(root: &Path) -> Relations {
    let mut out = Relations::new();
    for folder in ["backlog", "backlog/archiv"] {
        let dir = docs_path(root, folder);
        for entry in entries(&dir).iter().filter(|e| !e.is_dir && is_ticket_file(&e.name)) {
            let f = head_fields(&read_head(&dir.join(&entry.name)));
            let get = |name: &str| f.get(name).map_or("", String::as_str);
            // Doppelt vergebene Nummer (B-199, B-696): das Ticket im Backlog gilt, es wird zuerst gelesen.
            out.entry(ticket_file_id(&entry.name)).or_insert_with(|| RelInfo {
                status: get("Status").to_owned(),
                dup: rel_ids(get(FIELD_DUP)).into_iter().next().unwrap_or_default(),
                replaces: rel_ids(get(FIELD_REPLACE)),
            });
        }
    }
    out
}

/// Der neue Wert, sonst der alte aus dem Kopf.
fn pick<'a>(values: &'a BTreeMap<String, String>, head: &'a HashMap<String, String>, name: &str) -> &'a str {
    values.get(name).or_else(|| head.get(name)).map_or("", String::as_str)
}

/// Prüft gesetzte Relationen gegen alle Tickets: jede ID gibt es, kein Selbstverweis, kein Zyklus, ein Duplikat ist
/// nicht erledigt. Ohne Relationsfeld in `values` passiert nichts. Die Notizen sind Hinweise (Wurzel der Kette).
pub fn check_relations(
    root: &Path,
    id: &str,
    head: &HashMap<String, String>,
    values: &BTreeMap<String, String>,
) -> Result<Vec<String>, String> {
    if !values.contains_key(FIELD_DUP) && !values.contains_key(FIELD_REPLACE) {
        return Ok(Vec::new());
    }
    for name in [FIELD_DUP, FIELD_REPLACE] {
        if let Some(value) = values.get(name) {
            check_rel_format(name, value)?;
        }
    }
    let mut all = relations(root);
    let dup = rel_ids(pick(values, head, FIELD_DUP));
    let replaces = rel_ids(pick(values, head, FIELD_REPLACE));
    check_targets(&all, id, dup.iter().chain(&replaces))?;
    let done = |status: Option<&String>| status.is_some_and(|s| s == "erledigt");
    if !dup.is_empty() && (done(head.get("Status")) || done(values.get("Status"))) {
        return Err(format!(
            "{id} ist erledigt und damit kein Duplikat (🧑 entscheidet), nichts geändert"
        ));
    }
    let mine = all.entry(id.to_owned()).or_default();
    mine.dup = dup.first().cloned().unwrap_or_default();
    mine.replaces = replaces;
    if let Some(path) = cycle_path(&all, id) {
        return Err(format!(
            "{id} ergäbe einen Zyklus {}, nichts geändert",
            path.join(" → ")
        ));
    }
    let Some(first) = dup.first() else {
        return Ok(Vec::new());
    };
    let top = chain_root(&all, first);
    if &top == first {
        return Ok(Vec::new());
    }
    Ok(vec![format!(
        "Hinweis: {first} ist selbst verworfen oder ersetzt; Wurzel der Kette ist {top}"
    )])
}

/// Jedes Ziel gibt es und keins ist das Ticket selbst.
fn check_targets<'a>(all: &Relations, id: &str, targets: impl Iterator<Item = &'a String>) -> Result<(), String> {
    for target in targets {
        if target == id {
            return Err(format!("{id} verweist auf sich selbst, nichts geändert"));
        }
        if !all.contains_key(target) {
            let near = near_ids(all, target);
            return Err(format!(
                "{target} gibt es nicht (Backlog und Archiv){near}, nichts geändert"
            ));
        }
    }
    Ok(())
}

/// Prüft die Relationen für `Set` und setzt bei „Duplikat von“ den Status verworfen; die Rückgabe ist eine Kopie.
pub fn ticket_relations(
    c: &mut ChangeSet,
    r: &Ref,
    text: &str,
    values: &BTreeMap<String, String>,
) -> Result<BTreeMap<String, String>, String> {
    let notes = check_relations(&c.root, &r.id, &head_fields(text), values)?;
    c.notes.extend(notes);
    let mut out = values.clone();
    let dup = rel_ids(values.get(FIELD_DUP).map_or("", String::as_str));
    if let Some(first) = dup.first()
        && out.get("Status").is_none_or(|s| s != "verworfen")
    {
        out.insert("Status".to_owned(), "verworfen".to_owned());
        c.notes
            .push(format!("{} ist Duplikat von {first}: Status verworfen", r.id));
    }
    Ok(out)
}

/// Sucht einen Weg von `id` über Verweise zurück zu `id`.
fn cycle_path(all: &Relations, id: &str) -> Option<Vec<String>> {
    fn walk(all: &Relations, id: &str, path: &mut Vec<String>, seen: &mut HashSet<String>) -> bool {
        let Some(info) = path.last().and_then(|current| all.get(current)) else {
            return false;
        };
        for next in info.edges() {
            if next == id {
                path.push(next.clone());
                return true;
            }
            if seen.insert(next.clone()) {
                path.push(next.clone());
                if walk(all, id, path, seen) {
                    return true;
                }
                path.pop();
            }
        }
        false
    }
    let mut path = vec![id.to_owned()];
    walk(all, id, &mut path, &mut HashSet::new()).then_some(path)
}

fn sorted(mut ids: Vec<String>) -> Vec<String> {
    ids.sort_by(|a, b| ticket_cmp(a, b));
    ids
}

/// Alle Tickets, deren Eintrag `wanted` erfüllt, nach Nummer.
fn tickets_where(all: &Relations, wanted: impl Fn(&RelInfo) -> bool) -> Vec<String> {
    sorted(all.iter().filter(|(_, r)| wanted(r)).map(|(k, _)| k.clone()).collect())
}

/// Folgt Duplikat und Ersatz bis zu einem Ticket, das weder Duplikat noch ersetzt ist. „Ersetzt durch“ ist das erste
/// Ticket (nach Nummer), das das aktuelle ersetzt.
fn chain_root(all: &Relations, id: &str) -> String {
    let mut current = id.to_owned();
    let mut seen = HashSet::from([current.clone()]);
    loop {
        let dup = all.get(&current).map(|r| r.dup.clone()).filter(|d| !d.is_empty());
        let next = dup.or_else(|| tickets_where(all, |r| r.replaces.contains(&current)).into_iter().next());
        match next {
            Some(next) if seen.insert(next.clone()) => current = next,
            _ => return current,
        }
    }
}

/// Vorhandene IDs, die sich in einer Ziffer unterscheiden (Tippfehler), höchstens fünf.
fn near_ids(all: &Relations, id: &str) -> String {
    let one_off = |k: &str| k.len() == id.len() && k.bytes().zip(id.bytes()).filter(|(a, b)| a != b).count() == 1;
    let mut hits = sorted(all.keys().filter(|k| one_off(k)).cloned().collect());
    if hits.is_empty() {
        return String::new();
    }
    hits.truncate(5);
    format!("; ähnlich: {}", hits.join(", "))
}

/// Berechnete Gegenrichtung für `get`: wer `id` ersetzt und wessen Duplikat `id` ist.
pub fn reverse_lines(root: &Path, id: &str) -> Vec<String> {
    let all = relations(root);
    let replaced = tickets_where(&all, |r| r.replaces.iter().any(|x| x == id));
    let dups = tickets_where(&all, |r| r.dup == id);
    [("Ersetzt durch", replaced), ("Duplikate", dups)]
        .into_iter()
        .filter(|(_, ids)| !ids.is_empty())
        .map(|(label, ids)| format!("> **{label}:** {} (berechnet)", ids.join(", ")))
        .collect()
}

/// Setzt die Gegenrichtung als Zitatzeilen hinter den Kopf eines Tickets.
pub fn with_reverse(text: String, reverse: Vec<String>) -> String {
    if reverse.is_empty() {
        return text;
    }
    let mut lines = split_lines(&text);
    let at = head_end(&lines);
    lines.splice(at..at, reverse.into_iter().chain([String::new()]));
    join_lines(&lines)
}
