//! Offene Mensch-Sessions nach Gerät (Go `queue.go`). Stand RU69.9: der Teil, den der Fahrplan-Abschnitt „Offen am
//! Gerät“ braucht; `plan_queue` folgt in RU69.11.

use std::collections::BTreeMap;
use std::sync::LazyLock;

use regex::Regex;

use super::edit::head_end;
use super::planning::Sprint;
use super::tables::{first_of, split_lines};
use super::waves_spec::section_lines;

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
