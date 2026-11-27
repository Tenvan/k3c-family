//! Reihenfolge der Sprints: Abhängigkeiten folgen aus dem Feld „Abhängig von“ ihrer Sessions (SP12.3 → Sprint SP12).
//! Sprints eines Projekts ordnet der Rang des Projekts und ihr Platz in seiner Sprint-Tabelle; danach kommen Sprints
//! ohne Projekt in Fahrplan-Reihenfolge. Eine Voraussetzung erbt dabei den Platz ihrer Abnehmer.

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use regex::Regex;

use super::planning::{Data, Sprint, fields, take};
use super::projects::Project;

pub(super) static SESSION_REF: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"(?-u:\b)[A-Z]+[0-9]+\.[0-9]+(?-u:\b)").ok());

/// Umgebung, Domäne und die Session-IDs aus „Abhängig von“ einer Session-Datei.
fn session_meta(text: &str) -> (String, String, Vec<String>) {
    let text = text.replace("\r\n", "\n");
    let f = fields(&text.split('\n').collect::<Vec<_>>());
    let wanted = take(&f, "Abhängig von");
    let deps = SESSION_REF
        .as_ref()
        .map(|re| re.find_iter(&wanted).map(|m| m.as_str().to_owned()).collect())
        .unwrap_or_default();
    (take(&f, "Umgebung"), take(&f, "Domäne"), deps)
}

/// `GR5.2` → `GR5`.
fn sprint_of(nr: &str) -> &str {
    nr.split('.').next().unwrap_or_default()
}

fn read_sessions(sprint: &mut Sprint) {
    for session in &mut sprint.sessions {
        (session.env, session.domain, session.deps) = session_meta(&session.text);
        for dep in &session.deps {
            let id = sprint_of(dep);
            if id != sprint.id && !sprint.deps.iter().any(|d| d == id) {
                sprint.deps.push(id.to_owned());
            }
        }
    }
}

/// Setzt die Abhängigkeiten aller Sprints und ordnet die offenen nach Abhängigkeit und Projektplatz; erledigte folgen
/// in der bisherigen Reihenfolge.
pub fn rank(data: &mut Data) {
    let (mut open, mut done) = (Vec::new(), Vec::new());
    for mut sprint in std::mem::take(&mut data.sprints) {
        read_sessions(&mut sprint);
        if sprint.status == "erledigt" {
            done.push(sprint);
        } else {
            open.push(sprint);
        }
    }
    let key = sprint_key(&data.projects);
    data.sprints = order(open, |s| s.id.clone(), |s| s.deps.clone(), |a, b| key(a) < key(b));
    data.sprints.append(&mut done);
}

/// Projekte ohne Rang (ABN, ruhend, erledigt) stehen hinter denen mit Rang.
pub(super) const NO_RANK: i64 = 1 << 20;

/// Sortierschlüssel eines Sprints: zuerst die aus einer Projekt-Tabelle nach (Rang, Tabellenplatz), dann die ohne
/// Projekt (untereinander gleich, sie behalten ihre Reihenfolge).
pub fn sprint_key(projects: &[Project]) -> impl Fn(&Sprint) -> [i64; 3] {
    let mut at: HashMap<String, (i64, i64)> = HashMap::new();
    for project in projects {
        let rank = project.rang.parse().unwrap_or(NO_RANK);
        for (place, id) in (0_i64..).zip(&project.sprints) {
            at.insert(id.clone(), (rank, place));
        }
    }
    move |sprint| match at.get(&sprint.id) {
        Some(&(rank, place)) => [0, rank, place],
        None => [1, 0, 0],
    }
}

/// Sortiert topologisch nach Abhängigkeiten; unter den jeweils freien Einträgen entscheidet `less`, bei Gleichstand die
/// bisherige Reihenfolge. Eine Voraussetzung erbt den wichtigsten ihrer (auch indirekten) Abnehmer. Abhängigkeiten
/// außerhalb der Liste gelten als erfüllt, ein Zyklus wird am kleinsten Eintrag aufgebrochen. O(n²).
pub fn order<T>(
    items: Vec<T>,
    id: impl Fn(&T) -> String,
    deps: impl Fn(&T) -> Vec<String>,
    less: impl Fn(&T, &T) -> bool,
) -> Vec<T> {
    let ids: Vec<String> = items.iter().map(&id).collect();
    let wanted: Vec<Vec<String>> = items.iter().map(&deps).collect();
    let eff = effective(&items, &ids, &wanted, &less);
    let before = |a: usize, b: usize| {
        for (x, y) in [(eff[a], eff[b]), (a, b)] {
            let (first, second) = (less(&items[x], &items[y]), less(&items[y], &items[x]));
            if first || second {
                return first;
            }
        }
        a < b
    };
    let mut pending: HashSet<&str> = ids.iter().map(String::as_str).collect();
    let mut rest: Vec<usize> = (0..items.len()).collect();
    let mut picked = Vec::with_capacity(items.len());
    while !rest.is_empty() {
        let free = |i: usize| wanted[i].iter().all(|d| *d == ids[i] || !pending.contains(d.as_str()));
        let i = rest.remove(pick_next(&rest, free, before));
        pending.remove(ids[i].as_str());
        picked.push(i);
    }
    let mut slots: Vec<Option<T>> = items.into_iter().map(Some).collect();
    picked.into_iter().filter_map(|i| slots[i].take()).collect()
}

/// Der vorderste freie Eintrag; ist keiner frei (Zyklus), der vorderste überhaupt.
fn pick_next(rest: &[usize], free: impl Fn(usize) -> bool, before: impl Fn(usize, usize) -> bool) -> usize {
    let mut pick: Option<usize> = None;
    for (k, &i) in rest.iter().enumerate() {
        if free(i) && pick.is_none_or(|p| before(i, rest[p])) {
            pick = Some(k);
        }
    }
    pick.unwrap_or_else(|| (0..rest.len()).fold(0, |best, k| if before(rest[k], rest[best]) { k } else { best }))
}

/// Je Eintrag der wichtigste unter ihm und allen, die (indirekt) auf ihn warten.
fn effective<T>(items: &[T], ids: &[String], deps: &[Vec<String>], less: &impl Fn(&T, &T) -> bool) -> Vec<usize> {
    let index: HashMap<&str, usize> = ids.iter().enumerate().map(|(i, id)| (id.as_str(), i)).collect();
    // users[i]: Einträge, die direkt von i abhängen
    let mut users: Vec<Vec<usize>> = vec![Vec::new(); items.len()];
    for (i, wanted) in deps.iter().enumerate() {
        for dep in wanted {
            if let Some(&j) = index.get(dep.as_str())
                && j != i
            {
                users[j].push(i);
            }
        }
    }
    (0..items.len())
        .map(|i| {
            waiting(&users, i)
                .into_iter()
                .fold(i, |best, u| if less(&items[u], &items[best]) { u } else { best })
        })
        .collect()
}

/// Alle Einträge, die direkt oder indirekt auf `i` warten.
fn waiting(users: &[Vec<usize>], i: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut seen = HashSet::from([i]);
    let mut stack = vec![i];
    while let Some(n) = stack.pop() {
        for &u in &users[n] {
            if seen.insert(u) {
                stack.push(u);
                out.push(u);
            }
        }
    }
    out
}
