//! Gleichartige Meldungen zu Gruppen verdichten (Port von `digest.go`).

use std::cmp::Reverse;
use std::collections::HashMap;
use std::sync::LazyLock;

use chrono::{DateTime, FixedOffset};
use regex::Regex;
use serde::Serialize;

use super::entry::{Entry, go_time, level_rank};

/// Begrenzt den Schlüssel einer Gruppe.
pub(super) const FINGERPRINT_MAX: usize = 200;

/// Ersetzen veränderliche Teile einer Meldung, in dieser Reihenfolge: Die Zahlen-Regel zerlegte sonst UUIDs und Pfade,
/// bevor diese als Ganzes erkannt sind. Die Klassen sind ausgeschrieben, weil Go `\w` und `\d` nur ASCII meinen.
static MASKS: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
    [
        (
            r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}",
            "<id>",
        ),
        (
            r"(?:[A-Za-z]:)?[\\/]?(?:[0-9A-Za-z_.\-]+[\\/])+[0-9A-Za-z_.\-]+",
            "<path>",
        ),
        (r#""[^"]*"|'[^']*'|„[^“]*“"#, "<s>"),
        (r"[0-9]+(?:[.,][0-9]+)?", "<n>"),
    ]
    .into_iter()
    // Feste Muster; dass alle vier übersetzen, prüft `fingerprint_reihenfolge`.
    .filter_map(|(pattern, with)| Regex::new(pattern).ok().map(|re| (re, with)))
    .collect()
});

/// Macht gleichartige Meldungen gleich.
pub fn fingerprint(msg: &str) -> String {
    let mut out = msg.to_owned();
    for (re, with) in MASKS.iter() {
        out = re.replace_all(&out, regex::NoExpand(with)).into_owned();
    }
    if let Some((end, _)) = out.char_indices().nth(FINGERPRINT_MAX) {
        out.truncate(end);
    }
    out
}

/// Gleichartige Einträge: gleicher Namespace und gleicher Fingerabdruck (`LogGroup` in `types.ts`).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Group {
    pub ns: String,
    pub fingerprint: String,
    /// Höchstes Level der Gruppe.
    pub level: String,
    pub count: usize,
    #[serde(serialize_with = "go_time")]
    pub first: DateTime<FixedOffset>,
    #[serde(serialize_with = "go_time")]
    pub last: DateTime<FixedOffset>,
    /// Neueste Meldung.
    pub example: String,
}

impl Group {
    fn new(entry: &Entry, fingerprint: String) -> Group {
        Group {
            ns: entry.ns.clone(),
            fingerprint,
            level: entry.level.clone(),
            count: 0,
            first: entry.time,
            last: entry.time,
            example: entry.msg.clone(),
        }
    }

    fn add(&mut self, entry: &Entry) {
        self.count += 1;
        if level_rank(&entry.level) > level_rank(&self.level) {
            self.level.clone_from(&entry.level);
        }
        self.first = self.first.min(entry.time);
        if entry.time > self.last {
            self.last = entry.time;
            self.example.clone_from(&entry.msg);
        }
    }
}

/// Fasst Einträge zu Gruppen zusammen, häufigste zuerst, bei Gleichstand die jüngste.
pub fn digest(entries: &[Entry]) -> Vec<Group> {
    let mut by_key: HashMap<(&str, String), usize> = HashMap::new();
    let mut out: Vec<Group> = Vec::new();
    for entry in entries {
        let print = fingerprint(&entry.msg);
        let at = *by_key.entry((&entry.ns, print.clone())).or_insert_with(|| {
            out.push(Group::new(entry, print));
            out.len() - 1
        });
        out[at].add(entry);
    }
    // Stabil wie Go `SliceStable`.
    out.sort_by_key(|g| (Reverse(g.count), Reverse(g.last)));
    out
}
