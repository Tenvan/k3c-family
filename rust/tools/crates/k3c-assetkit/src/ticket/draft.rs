//! Vorlage ausfüllen und Index-Zeile. Texte arbeiten mit `\n`; die Zeilenenden der Datei setzt erst das Schreiben.

use std::collections::{BTreeMap, HashSet};

use chrono::{DateTime, FixedOffset};

use super::Input;
use crate::error::{Error, Result};

/// Kopf eines neuen Tickets ohne Domäne, Typ und Prio (wie `plan_create`).
pub fn defaults(today: DateTime<FixedOffset>) -> BTreeMap<String, String> {
    let date = today.format("%Y-%m-%d").to_string();
    [
        ("Status", "offen"),
        ("Umgebung", "?"),
        ("Sprint", "–"),
        ("Projekt", "–"),
        ("Erstellt", date.as_str()),
        ("Spec", "Entwurf"),
        ("Revision", "1"),
        ("Freigabe", "–"),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect()
}

/// Relationsfelder (B-696) mit ihrem Standard; [`draft`] setzt sie nur, wenn die Vorlage sie kennt, damit ein Checkout
/// mit älterer Vorlage weiter Tickets anlegen kann.
pub const OPTIONAL: &[(&str, &str)] = &[("Duplikat von", "–"), ("Ersetzt", "–")];

/// Name eines Kopf-Feldes `- **Name:**` am Zeilenanfang.
fn field_name(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("- **")?;
    let end = rest.find([':', '*'])?;
    (end > 0 && rest[end..].starts_with(":**")).then(|| &rest[..end])
}

/// Ersetzt in der Vorlage die Überschrift durch `# <nr> · <title>` und setzt die Kopf-Felder; ein Feld, das die Vorlage
/// nicht kennt, ist ein Fehler. Die Werte prüft der Aufrufer.
pub fn draft(tpl: &str, nr: &str, title: &str, values: &BTreeMap<String, String>) -> Result<String> {
    let mut lines: Vec<String> = tpl.split('\n').map(str::to_string).collect();
    if !lines.first().is_some_and(|l| l.starts_with("# ")) {
        return Err(Error::Msg("vorlage ohne Überschrift `# `".to_string()));
    }
    lines[0] = format!("# {nr} · {title}");
    let mut seen = HashSet::new();
    for line in lines.iter_mut().skip(1).take_while(|l| !l.starts_with("## ")) {
        let Some(name) = field_name(line).map(str::to_string) else {
            continue;
        };
        if let Some(value) = values.get(&name) {
            *line = format!("- **{name}:** {value}");
            seen.insert(name);
        } else if let Some((_, value)) = OPTIONAL.iter().find(|(k, _)| *k == name) {
            *line = format!("- **{name}:** {value}");
        }
    }
    if let Some(missing) = values.keys().find(|k| !seen.contains(*k)) {
        return Err(Error::Msg(format!("das Feld {missing} fehlt in der Vorlage")));
    }
    Ok(lines.join("\n"))
}

/// Ersetzt den Text unter `## <name>` bis zur nächsten `## `-Überschrift.
pub(super) fn set_section(text: &str, name: &str, body: &str) -> Result<String> {
    let lines: Vec<&str> = text.split('\n').collect();
    let head = format!("## {name}");
    let start = lines
        .iter()
        .position(|l| *l == head)
        .ok_or_else(|| Error::Msg(format!("abschnitt {name} fehlt in der Vorlage")))?;
    let end = (start + 1..lines.len())
        .find(|&i| lines[i].starts_with("## "))
        .unwrap_or(lines.len());
    let mut out = lines[..start].to_vec();
    out.extend([head.as_str(), ""]);
    out.extend(body.split('\n'));
    if end < lines.len() {
        out.push("");
    }
    out.extend_from_slice(&lines[end..]);
    Ok(out.join("\n"))
}

/// Ausgangslage: Beschreibung (Zeilen mit `#` maskiert, damit keine Überschrift entsteht) und Kontextliste ohne leere
/// Einträge.
pub(super) fn situation(input: &Input) -> String {
    let mut out: Vec<String> = Vec::new();
    let desc = input.beschreibung.replace("\r\n", "\n");
    let desc = desc.trim();
    if !desc.is_empty() {
        out.extend(desc.split('\n').map(|l| {
            let mask = if l.starts_with('#') { "\\" } else { "" };
            format!("{mask}{l}")
        }));
        out.push(String::new());
    }
    let k = &input.kontext;
    for (name, value) in [
        ("App", &k.app),
        ("Seite", &k.seite),
        ("Auswahl", &k.auswahl),
        ("Version", &k.version),
        ("Zeit", &k.zeit),
    ] {
        let value = value.split_whitespace().collect::<Vec<_>>().join(" ");
        if !value.is_empty() {
            out.push(format!("- **{name}:** {value}"));
        }
    }
    if out.is_empty() {
        return "–".to_string();
    }
    out.join("\n").trim().to_string()
}

/// Zeile eines Tickets im Index `docs/backlog/README.md`; Link relativ zu `docs/backlog/`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Row {
    pub nr: String,
    pub link: String,
    pub domain: String,
    pub typ: String,
    pub prio: String,
    pub status: String,
    pub sprint: String,
    pub title: String,
}

/// Tabellenzeile im Format des Index.
pub fn index_row(r: &Row) -> String {
    let link = format!("[{}]({})", r.nr, r.link);
    let cells = [&link, &r.domain, &r.typ, &r.prio, &r.status, &r.sprint, &r.title];
    format!("| {} |", cells.map(String::as_str).join(" | "))
}

/// Hängt die Zeile eines neuen Tickets ans Ende der Tabelle unter `## Offen`.
pub fn add_to_index(index: &str, r: &Row) -> Result<String> {
    let mut lines: Vec<&str> = index.split('\n').collect();
    let start = format!("| [{}](", r.nr);
    if lines.iter().any(|l| l.starts_with(&start)) {
        return Err(Error::Msg(format!("{} steht schon im Index", r.nr)));
    }
    let at = table_end(&lines, "## Offen")
        .ok_or_else(|| Error::Msg("backlog/README.md: Tabelle unter ## Offen fehlt".to_string()))?;
    let row = index_row(r);
    lines.insert(at, &row);
    Ok(lines.join("\n"))
}

/// Die Zeile nach der ersten Tabelle unter der Überschrift (vor der nächsten `## `).
fn table_end(lines: &[&str], head: &str) -> Option<usize> {
    let mut j = lines.iter().position(|l| l.starts_with(head))? + 1;
    while j + 1 < lines.len() && !lines[j].starts_with("## ") {
        if lines[j].starts_with("| ") && lines[j + 1].starts_with("|---") {
            let mut end = j + 2;
            while end < lines.len() && lines[end].starts_with('|') {
                end += 1;
            }
            return Some(end);
        }
        j += 1;
    }
    None
}

/// Kurzname aus dem Titel: klein, Umlaute umschrieben, nur a-z, 0-9 und `-`, höchstens 50 Zeichen.
pub fn slug(title: &str) -> String {
    // je Zeichen die einfache Kleinschreibung wie Go `strings.ToLower`
    let lower: String = title.chars().filter_map(|c| c.to_lowercase().next()).collect();
    let text = lower
        .replace('ä', "ae")
        .replace('ö', "oe")
        .replace('ü', "ue")
        .replace('ß', "ss");
    let mut out = String::new();
    let mut dash = false;
    for c in text.chars() {
        if c.is_ascii_lowercase() || c.is_ascii_digit() {
            out.push(c);
            dash = false;
        } else if !dash && !out.is_empty() {
            out.push('-');
            dash = true;
        }
    }
    let mut out = out.trim_end_matches('-');
    if out.len() > 50 {
        out = &out[..50];
        if let Some(i) = out.rfind('-').filter(|&i| i > 20) {
            out = &out[..i];
        }
        out = out.trim_end_matches('-');
    }
    if out.is_empty() { "ticket" } else { out }.to_string()
}
