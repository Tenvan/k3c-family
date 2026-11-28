//! Der GitHub-Stand der Sprints über die GitHub CLI `gh` (B-212, Port von `internal/github`): je Sprint-Branch
//! `sprint/<präfix>` der neueste PR gegen main mit CI- und Merge-Stand, dazu der letzte CI-Lauf auf main. Die Anmeldung
//! liegt bei `gh`; die Workbench speichert kein Token.

mod client;
mod prs;

#[cfg(test)]
mod github_tests;
#[cfg(test)]
mod prs_tests;

use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, FixedOffset, SecondsFormat};
use serde::{Deserialize, Deserializer, Serialize};

pub use client::{Client, RunError, RunFuture, Runner, TTL, exec_runner};
pub use prs::{MergedHead, OpenPr, PrList, parse_open};

/// Stand eines Sprints auf GitHub.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct SprintPr {
    pub number: i64,
    pub title: String,
    pub url: String,
    /// offen | Entwurf | gemergt | geschlossen
    pub state: String,
    /// grün | rot | läuft | –
    pub ci: String,
    /// konfliktfrei | Konflikt | unbekannt | – (nicht offen)
    pub merge: String,
}

/// Der letzte CI-Lauf auf main.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct MainRun {
    pub ci: String,
    pub title: String,
    pub url: String,
    pub created: String,
}

/// Stand für Oberfläche und Tool; `error` ist ein Hinweis (`gh` fehlt, nicht angemeldet, veraltet).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Data {
    /// Schlüssel: Sprint-ID in Großbuchstaben (M8).
    pub sprints: BTreeMap<String, SprintPr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<MainRun>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub error: String,
    /// RFC 3339, leer = nie geholt.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub fetched: String,
}

/// `null` gilt wie bei Go als Nullwert.
fn nullable<'de, D: Deserializer<'de>, T: Default + Deserialize<'de>>(d: D) -> Result<T, D::Error> {
    Ok(Option::<T>::deserialize(d)?.unwrap_or_default())
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Check {
    #[serde(deserialize_with = "nullable")]
    name: String,
    #[serde(deserialize_with = "nullable")]
    context: String,
    /// CheckRun: QUEUED, IN_PROGRESS, COMPLETED
    #[serde(deserialize_with = "nullable")]
    status: String,
    /// CheckRun: SUCCESS, FAILURE …
    #[serde(deserialize_with = "nullable")]
    conclusion: String,
    /// StatusContext: SUCCESS, FAILURE, PENDING, ERROR
    #[serde(deserialize_with = "nullable")]
    state: String,
    #[serde(deserialize_with = "nullable")]
    started_at: String,
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Pr {
    #[serde(deserialize_with = "nullable")]
    number: i64,
    #[serde(deserialize_with = "nullable")]
    title: String,
    #[serde(deserialize_with = "nullable")]
    state: String,
    #[serde(deserialize_with = "nullable")]
    is_draft: bool,
    #[serde(deserialize_with = "nullable")]
    head_ref_name: String,
    #[serde(deserialize_with = "nullable")]
    mergeable: String,
    #[serde(deserialize_with = "nullable")]
    url: String,
    #[serde(deserialize_with = "nullable")]
    status_check_rollup: Vec<Check>,
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct MainRunJson {
    #[serde(deserialize_with = "nullable")]
    status: String,
    #[serde(deserialize_with = "nullable")]
    conclusion: String,
    #[serde(deserialize_with = "nullable")]
    display_title: String,
    #[serde(deserialize_with = "nullable")]
    url: String,
    #[serde(deserialize_with = "nullable")]
    created_at: String,
}

/// Eine JSON-Liste von `gh`; `null` ist leer.
fn list<T: for<'de> Deserialize<'de>>(json: &[u8], what: &str) -> Result<Vec<T>, String> {
    serde_json::from_slice::<Option<Vec<T>>>(json)
        .map(Option::unwrap_or_default)
        .map_err(|e| format!("{what}: {e}"))
}

fn rfc3339(at: DateTime<FixedOffset>) -> String {
    at.to_rfc3339_opts(SecondsFormat::Secs, true)
}

/// Wertet die JSON-Ausgaben von `gh pr list` und `gh run list` aus; `runs_json` darf leer sein.
pub fn parse(prs_json: &[u8], runs_json: &[u8], now: DateTime<FixedOffset>) -> Result<Data, String> {
    let prs: Vec<Pr> = list(prs_json, "gh pr list")?;
    let mut data = Data {
        fetched: rfc3339(now),
        ..Data::default()
    };
    for pr in prs {
        let Some(id) = pr.head_ref_name.strip_prefix("sprint/").filter(|id| !id.is_empty()) else {
            continue;
        };
        let id = id.to_uppercase();
        if data.sprints.get(&id).is_some_and(|old| old.number > pr.number) {
            continue; // je Sprint der neueste PR
        }
        let sprint = SprintPr {
            number: pr.number,
            title: pr.title.clone(),
            url: pr.url.clone(),
            state: pr_state(&pr).to_owned(),
            ci: ci_state(&pr.status_check_rollup).to_owned(),
            merge: merge_state(&pr).to_owned(),
        };
        data.sprints.insert(id, sprint);
    }
    if !runs_json.is_empty() {
        let runs: Vec<MainRunJson> = list(runs_json, "gh run list")?;
        data.main = runs.into_iter().next().map(|r| MainRun {
            ci: run_ci(&r.status, &r.conclusion).to_owned(),
            title: r.display_title,
            url: r.url,
            created: r.created_at,
        });
    }
    Ok(data)
}

fn pr_state(pr: &Pr) -> &'static str {
    match pr.state.as_str() {
        "MERGED" => "gemergt",
        "CLOSED" => "geschlossen",
        _ if pr.is_draft => "Entwurf",
        _ => "offen",
    }
}

fn merge_state(pr: &Pr) -> &'static str {
    if pr.state != "OPEN" {
        return "–";
    }
    match pr.mergeable.as_str() {
        "MERGEABLE" => "konfliktfrei",
        "CONFLICTING" => "Konflikt",
        _ => "unbekannt", // GitHub rechnet noch
    }
}

/// Fasst die Checks zusammen; ein neu gestarteter Check gleichen Namens ersetzt den älteren.
fn ci_state(checks: &[Check]) -> &'static str {
    let mut latest: HashMap<String, &Check> = HashMap::new();
    for check in checks {
        let key = format!("{}{}", check.name, check.context);
        if latest.get(&key).is_none_or(|old| check.started_at >= old.started_at) {
            latest.insert(key, check);
        }
    }
    if latest.is_empty() {
        return "–";
    }
    // Die Reihenfolge der Werte ändert das Ergebnis nicht.
    let results: Vec<&str> = latest.values().map(|c| check_result(c)).collect();
    ["rot", "läuft"]
        .into_iter()
        .find(|r| results.contains(r))
        .unwrap_or("grün")
}

fn check_result(check: &Check) -> &'static str {
    match check.state.as_str() {
        "" => run_ci(&check.status, &check.conclusion), // CheckRun
        "SUCCESS" => "grün",                            // StatusContext
        "PENDING" | "EXPECTED" => "läuft",
        _ => "rot",
    }
}

fn run_ci(status: &str, conclusion: &str) -> &'static str {
    if !status.eq_ignore_ascii_case("COMPLETED") {
        return "läuft";
    }
    match conclusion.to_uppercase().as_str() {
        "SUCCESS" | "NEUTRAL" | "SKIPPED" => "grün",
        _ => "rot",
    }
}

/// Knappe Fassung für das Tool `gh_status`: eine Zeile je Sprint, dann main.
pub fn text(data: &Data) -> String {
    let mut out = Vec::new();
    if !data.error.is_empty() {
        out.push(format!("Hinweis: {}", data.error));
    }
    for (id, p) in &data.sprints {
        let mut line = format!("{id} #{} {} · CI {}", p.number, p.state, p.ci);
        if p.merge != "–" {
            line.push_str(&format!(" · {}", p.merge));
        }
        out.push(format!("{line} · {}", p.url));
    }
    if let Some(main) = &data.main {
        out.push(format!("main: CI {} · {} ({})", main.ci, main.title, main.created));
    }
    if out.is_empty() {
        return "keine Sprint-PRs".to_owned();
    }
    out.join("\n")
}
