//! Die Ralf-Queue (B-783, Go `queue.go`, docs/arbeitsweise.md › Ralf-Queue): Freigaben, Entscheidungen und Abnahmen
//! je Gerät. Das Gerät steht in einer Zeile `Gerät:` im Abschnitt Kontext, sonst gilt „unbekannt“. Der Fahrplan nutzt
//! die offenen Mensch-Sessions für „Offen am Gerät“.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use serde::Deserialize;

use super::edit::head_end;
use super::planning::Sprint;
use super::tables::{first_of, split_lines};
use super::waves::first_lines;
use super::waves_nodes::{WaveData, load_wave_data, needs_human};
use super::waves_spec::section_lines;

/// Eingabe von [`queue`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct QueueFilter {
    /// nur Abnahmen an diesem Gerät: PC oder unbekannt
    pub device: String,
}

#[derive(Default)]
struct Collected {
    release: Vec<String>,
    decide: Vec<String>,
    accept: Vec<QueueItem>,
}

/// Die Ralf-Queue aus den lokalen Dateien.
pub fn queue(root: &Path, f: &QueueFilter) -> Result<String, String> {
    let waves = load_wave_data(root)?;
    let mut found = queue_docs(&waves);
    for sprint in waves.data.sprints.iter().filter(|s| s.status == "aktiv") {
        found.accept.extend(human_items(sprint));
    }
    let only_accept = !f.device.is_empty();
    if only_accept {
        let want = norm_device(&f.device).to_lowercase();
        found.accept.retain(|q| q.device.to_lowercase() == want);
        found.release.clear();
        found.decide.clear();
    }
    Ok(queue_text(found, only_accept))
}

/// Eine Zeile aus Ticket oder Sprint mit `Braucht 🧑` ≠ `–`.
struct Doc<'a> {
    id: &'a str,
    kind: &'a str,
    title: &'a str,
    domain: &'a str,
    spec: &'a str,
    status: &'a str,
}

fn add_doc(out: &mut Collected, waves: &WaveData, doc: &Doc) {
    let needs = waves.specs.get(doc.id).map_or("", |s| s.needs.as_str());
    if !needs_human(needs) || doc.status == "erledigt" || doc.status == "verworfen" {
        return;
    }
    let line = format!(
        "- {} · {} ({}, {}, Spec {}, Braucht 🧑: {needs})",
        doc.id,
        doc.title,
        doc.kind,
        first_of(&[doc.domain, "?"]),
        first_of(&[doc.spec, "?"])
    );
    let low = needs.to_lowercase();
    if low.contains("freigabe") && doc.spec != "freigegeben" {
        out.release.push(line.clone());
    }
    if low.contains("entscheidung") {
        out.decide.push(line.clone());
    }
    if low.contains("abnahme") || low.contains("gerät") {
        out.accept.push(QueueItem {
            device: device_in(needs),
            line,
        });
    }
}

/// Sammelt Tickets und Sprints mit `Braucht 🧑` ≠ `–`.
fn queue_docs(waves: &WaveData) -> Collected {
    let mut out = Collected::default();
    for t in &waves.data.tickets {
        let doc = Doc {
            id: &t.nr,
            kind: "Ticket",
            title: &t.title,
            domain: &t.domain,
            spec: &t.spec,
            status: &t.status,
        };
        add_doc(&mut out, waves, &doc);
    }
    for s in &waves.data.sprints {
        let doc = Doc {
            id: &s.id,
            kind: "Sprint",
            title: &s.title,
            domain: &s.domain,
            spec: &s.spec,
            status: &s.status,
        };
        add_doc(&mut out, waves, &doc);
    }
    out
}

/// Findet in einem Wert wie „Abnahme (PC)“ das Gerät; sonst „unbekannt“.
fn device_in(value: &str) -> String {
    let device = norm_device(value);
    if DEVICES.contains(&device.as_str()) {
        device
    } else {
        UNKNOWN_DEVICE.to_owned()
    }
}

/// Kopf mit Zahlen, Freigaben, Entscheidungen, Abnahmen gruppiert nach Gerät.
fn queue_text(found: Collected, only_accept: bool) -> String {
    let accepted = found.accept.len();
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for q in found.accept {
        groups.entry(q.device).or_default().push(q.line);
    }
    let order = device_order(&groups);
    let mut out = vec![format!(
        "Ralf-Queue: {} Freigaben · {} Entscheidungen · {accepted} Abnahmen an {} Geräten",
        found.release.len(),
        found.decide.len(),
        order.len()
    )];
    if !only_accept {
        out.push("── Freigaben ──".to_owned());
        out.extend(first_lines(found.release, "keine"));
        out.push("── Entscheidungen ──".to_owned());
        out.extend(first_lines(found.decide, "keine"));
    }
    out.push("── Abnahmen je Gerät ──".to_owned());
    if order.is_empty() {
        out.push("keine".to_owned());
    }
    for device in order {
        let lines = groups.remove(&device).unwrap_or_default();
        out.push(format!("{device} ({}):", lines.len()));
        out.extend(lines);
    }
    out.join("\n")
}

/// Reihenfolge der Geräte; andere folgen nach Name, „unbekannt“ zuletzt.
const DEVICES: [&str; 1] = ["PC"];
const UNKNOWN_DEVICE: &str = "unbekannt";

// Muster wie Go: `\s` und `\b` meinen dort nur ASCII.
static DEVICE: LazyLock<Option<Regex>> = LazyLock::new(|| {
    let s = r"[\t\n\f\r ]*";
    Regex::new(&format!(r"^{s}(?:- )?(?:\*\*)?Gerät:(?:\*\*)?{s}(.+?){s}$")).ok()
});
static PC: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"(?i)(?-u:\b)pc(?-u:\b)").ok());

/// Ein Eintrag der Ralf-Queue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueItem {
    pub device: String,
    pub line: String,
}

/// Offene oder laufende Sessions eines Sprints, die nicht autonom sind, mit ihrem Gerät (B-842).
pub fn human_items(sprint: &Sprint) -> Vec<QueueItem> {
    sprint
        .sessions
        .iter()
        .filter(|x| (x.status == "offen" || x.status == "in Arbeit") && x.agent != "autonom")
        .map(|x| QueueItem {
            device: device_of(&x.text),
            line: format!(
                "- {} · {} (Sprint {}, {}, Agent {})",
                x.nr,
                x.titel,
                sprint.id,
                x.status,
                first_of(&[&x.agent, "?"])
            ),
        })
        .collect()
}

/// Gerät aus einer Zeile `Gerät:` im Abschnitt Kontext (oder im Kopf); sonst „unbekannt“.
pub fn device_of(text: &str) -> String {
    let all = split_lines(&text.replace("\r\n", "\n"));
    let head = &all[..head_end(&all)];
    section_lines(text, "## Kontext")
        .iter()
        .chain(head)
        .find_map(|l| Some(DEVICE.as_ref()?.captures(l)?.get(1)?.as_str().to_owned()))
        .map_or_else(|| UNKNOWN_DEVICE.to_owned(), |found| norm_device(&found))
}

/// Ordnet Schreibweisen den Geräten zu; Unbekanntes bleibt wie geschrieben.
pub fn norm_device(value: &str) -> String {
    let v = value.trim_matches(['`', '*']).trim();
    if v.is_empty() {
        String::new()
    } else if PC.as_ref().is_some_and(|re| re.is_match(v)) {
        "PC".to_owned()
    } else if v.to_lowercase() == UNKNOWN_DEVICE {
        UNKNOWN_DEVICE.to_owned()
    } else {
        v.to_owned()
    }
}

/// Bekannte Geräte in fester Reihenfolge, andere nach Name, „unbekannt“ zuletzt.
pub fn device_order(groups: &BTreeMap<String, Vec<String>>) -> Vec<String> {
    let filled = |d: &str| groups.get(d).is_some_and(|lines| !lines.is_empty());
    let mut out: Vec<String> = DEVICES.iter().filter(|d| filled(d)).map(|d| (*d).to_owned()).collect();
    out.extend(
        groups
            .keys()
            .filter(|d| !DEVICES.contains(&d.as_str()) && *d != UNKNOWN_DEVICE)
            .cloned(),
    );
    if filled(UNKNOWN_DEVICE) {
        out.push(UNKNOWN_DEVICE.to_owned());
    }
    out
}
