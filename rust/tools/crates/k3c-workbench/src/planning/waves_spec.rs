//! Planungsangaben für Arbeitswellen (B-783, Go `waves_spec.go`): Ticket und Sprint-README tragen im Abschnitt
//! „Regeln und Einschränkungen“ die Zeilen Bereich, Abhängig von, Braucht 🧑, Größe und Sperre; eine Session liefert
//! ihre Dateimenge aus „Erlaubte Dateien“.

use std::collections::HashSet;
use std::sync::LazyLock;

use regex::Regex;

use super::tables::first_of;

/// Planungsangaben eines Tickets oder einer Sprint-README.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlanSpec {
    /// Pfad-Globs aus „Bereich“ ohne Sammeldateien.
    pub area: Vec<String>,
    /// IDs aus „Abhängig von“.
    pub deps: Vec<String>,
    /// Einträge aus „Abhängig von“, die keine ID sind.
    pub bad_deps: Vec<String>,
    /// „Braucht 🧑“; leer, wenn die Zeile fehlt.
    pub needs: String,
    /// „Größe“: S, M, L
    pub size: String,
    /// „Sperre“ ohne `–`
    pub locks: Vec<String>,
    /// Mindestens eine der Zeilen steht da.
    pub has_lines: bool,
    /// Namen der Zeilen, die dastehen.
    pub seen: HashSet<String>,
}

/// Die Planungsangaben in der Reihenfolge der Vorlage.
pub const PLAN_LINES: [&str; 5] = ["Bereich", "Abhängig von", "Braucht 🧑", "Größe", "Sperre"];

const SECTION_RULES: &str = "## Regeln und Einschränkungen";

/// Sammeldateien berührt fast jede Session; sie zählen nicht zum Bereich (arbeitsweise.md › Sammeldateien).
const COLLECT_FILES: [&str; 5] = [
    "docs/backlog/README.md",
    "docs/backlog/archiv/README.md",
    "docs/sprints/README.md",
    "docs/projekte/README.md",
    "docs/glossar.md",
];

// Muster wie Go: `\s` und `\d` meinen dort nur ASCII.
static PLAN_LINE: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"^- \*\*(Bereich|Abhängig von|Braucht 🧑|Größe|Sperre):\*\*[\t\n\f\r ]*(.*)$").ok());
static PLAN_ID: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"^(B-[0-9]+|[A-Z]+[0-9]+(?:\.[0-9]+[a-z]?)?)$").ok());
static BACKTICK: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new("`([^`]+)`").ok());

/// Liest die Planungsangaben aus dem Abschnitt „Regeln und Einschränkungen“.
pub fn parse_plan_spec(text: &str) -> PlanSpec {
    let mut spec = PlanSpec::default();
    for line in section_lines(text, SECTION_RULES) {
        let Some(found) = PLAN_LINE
            .as_ref()
            .and_then(|re| re.captures(line.trim_end_matches(' ')))
        else {
            continue;
        };
        let name = found.get(1).map_or("", |m| m.as_str());
        let value = found.get(2).map_or("", |m| m.as_str()).trim();
        spec.has_lines = true;
        spec.seen.insert(name.to_owned());
        match name {
            "Bereich" => spec.area = parse_globs(value),
            "Abhängig von" => (spec.deps, spec.bad_deps) = parse_ids(value),
            "Braucht 🧑" => first_of(&[value, "?"]).clone_into(&mut spec.needs),
            "Größe" => value.clone_into(&mut spec.size),
            _ => spec.locks = parse_locks(value),
        }
    }
    spec
}

/// Zeilen unter der Überschrift `head` bis zur nächsten `## `.
pub fn section_lines(text: &str, head: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut inside = false;
    for line in text.replace("\r\n", "\n").split('\n') {
        if line.starts_with("## ") {
            inside = line.trim() == head;
        } else if inside {
            out.push(line.to_owned());
        }
    }
    out
}

fn backticked(value: &str) -> Vec<&str> {
    BACKTICK.as_ref().map_or_else(Vec::new, |re| {
        re.captures_iter(value)
            .filter_map(|c| c.get(1).map(|m| m.as_str()))
            .collect()
    })
}

/// Dateimenge einer Session: je Stichpunkt unter „Erlaubte Dateien“ alle Pfade in Backticks (`a/**`, `b.go`);
/// Backticks ohne `/` und `.` (z. B. „nur über `plan_*`“) sind kein Pfad.
pub fn allowed_files(text: &str) -> Vec<String> {
    section_lines(text, "## Erlaubte Dateien")
        .iter()
        .filter(|line| line.trim().starts_with("- "))
        .flat_map(|line| backticked(line))
        .filter(|path| path.contains(['/', '.']) && !path.contains(' '))
        .flat_map(parse_globs)
        .collect()
}

/// Zerlegt „`a/**`, `b.go`“ oder „a/**, b.go“ in Globs ohne Sammeldateien; `–` ist leer.
pub fn parse_globs(value: &str) -> Vec<String> {
    let mut raw = backticked(value);
    if raw.is_empty() {
        raw = value.split(',').collect();
    }
    raw.into_iter()
        .filter_map(|glob| {
            let glob = glob.trim().replace('\\', "/");
            let glob = glob.strip_prefix("./").unwrap_or(&glob);
            let none = glob.is_empty() || glob.starts_with('–') || glob == "-";
            (!none && !COLLECT_FILES.contains(&glob)).then(|| glob.to_owned())
        })
        .collect()
}

/// Zerlegt „B-782, WZ44.1“; `–` ist leer, Freitext landet im zweiten Wert.
pub fn parse_ids(value: &str) -> (Vec<String>, Vec<String>) {
    let (mut ids, mut bad) = (Vec::new(), Vec::new());
    for part in value.replace('`', "").split(',') {
        let part = part.trim();
        if part.is_empty() || part.starts_with('–') || part == "-" {
        } else if PLAN_ID.as_ref().is_some_and(|re| re.is_match(part)) {
            ids.push(part.to_owned());
        } else {
            bad.push(part.to_owned());
        }
    }
    (ids, bad)
}

/// Zerlegt „exe, taskfile“ ohne Backticks; `–` ist leer.
pub fn parse_locks(value: &str) -> Vec<String> {
    value
        .replace('`', "")
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty() && !s.starts_with('–') && *s != "-")
        .map(str::to_lowercase)
        .collect()
}

/// Der feste Teil eines Globs bis zum ersten Platzhalter als Ordner-Präfix. Ein fester Pfad gilt zugleich als Ordner
/// (B-839): `tools/paper` überlappt mit `tools/paper/cli/**`, aber nicht mit `tools/paper2/**`.
fn folder_prefix(glob: &str) -> String {
    match glob.find(['*', '?', '[', '{']) {
        Some(at) => glob[..at].to_owned(),
        // Ein Pfad mit `/` am Ende ist schon ein Ordner (wie `ordner/**`).
        None if glob.ends_with('/') => glob.to_owned(),
        None => format!("{glob}/"),
    }
}

/// Zwei Globs überlappen, wenn ein fester Teil Präfix des anderen ist.
pub fn globs_overlap(a: &str, b: &str) -> bool {
    if a == b {
        return true;
    }
    let (a, b) = (folder_prefix(a), folder_prefix(b));
    a.starts_with(&b) || b.starts_with(&a)
}

/// Das erste überlappende Paar zweier Bereiche, für die Meldung formatiert.
pub fn area_overlap(a: &[String], b: &[String]) -> Option<String> {
    a.iter().find_map(|x| {
        let y = b.iter().find(|y| globs_overlap(x, y))?;
        Some(if x == y {
            format!("`{x}`")
        } else {
            format!("`{x}` / `{y}`")
        })
    })
}

/// Die erste gemeinsame Sperre.
pub fn shared_lock<'a>(a: &'a [String], b: &[String]) -> Option<&'a str> {
    a.iter().find(|x| b.contains(x)).map(String::as_str)
}
