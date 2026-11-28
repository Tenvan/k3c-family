//! Offene PRs und PR-Aktionen für die Seite Git (B-580, Port von `prs.go`). Ziel ist immer main; Branches löscht die
//! Seite selbst (`gitops`), deshalb nie `--delete-branch`: `gh` würde dabei auch den Checkout wechseln.

use chrono::{Local, SecondsFormat};
use serde::{Deserialize, Serialize};
use tokio::time::Instant;

use super::client::{Client, TIMEOUT, hint};
use super::{Pr, ci_state, list, merge_state, nullable, pr_state};

/// Ein offener PR gegen main.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct OpenPr {
    pub number: i64,
    pub title: String,
    pub url: String,
    pub head: String,
    pub author: String,
    /// RFC 3339
    pub updated: String,
    /// offen | Entwurf
    pub state: String,
    /// grün | rot | läuft | –
    pub ci: String,
    /// konfliktfrei | Konflikt | unbekannt
    pub merge: String,
}

/// Die offenen PRs; `error` ist ein Hinweis (`gh` fehlt, nicht angemeldet).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct PrList {
    pub prs: Vec<OpenPr>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub error: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub fetched: String,
}

pub(super) const OPEN_ARGS: [&str; 11] = [
    "gh",
    "pr",
    "list",
    "--state",
    "open",
    "--base",
    "main",
    "--limit",
    "50",
    "--json",
    "number,title,state,isDraft,headRefName,mergeable,statusCheckRollup,url,author,updatedAt",
];

const MAX_TITLE: usize = 200;
const MAX_BODY: usize = 32 << 10;

/// Ein gemergter PR mit seinem letzten Commit; die Seite Git erkennt daran Squash-Merges.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct MergedHead {
    #[serde(deserialize_with = "nullable")]
    pub number: i64,
    #[serde(rename = "headRefName", deserialize_with = "nullable")]
    pub head: String,
    #[serde(rename = "headRefOid", deserialize_with = "nullable")]
    pub sha: String,
}

/// Nur PRs nach main: ein PR in einen Sprint- oder Feature-Branch bringt nichts nach main.
pub(super) const MERGED_ARGS: [&str; 11] = [
    "gh",
    "pr",
    "list",
    "--state",
    "merged",
    "--base",
    "main",
    "--limit",
    "500",
    "--json",
    "number,headRefName,headRefOid",
];

/// Wertet `gh pr list --json` mit Autor und Zeit aus, in der Reihenfolge von `gh` (neueste Nummer zuerst).
pub fn parse_open(json: &[u8]) -> Result<Vec<OpenPr>, String> {
    #[derive(Default, Deserialize)]
    #[serde(default)]
    struct Author {
        #[serde(deserialize_with = "nullable")]
        login: String,
    }
    #[derive(Default, Deserialize)]
    #[serde(default, rename_all = "camelCase")]
    struct Open {
        #[serde(flatten)]
        pr: Pr,
        #[serde(deserialize_with = "nullable")]
        author: Author,
        #[serde(deserialize_with = "nullable")]
        updated_at: String,
    }
    let open: Vec<Open> = list(json, "gh pr list")?;
    Ok(open
        .into_iter()
        .map(|o| OpenPr {
            state: pr_state(&o.pr).to_owned(),
            ci: ci_state(&o.pr.status_check_rollup).to_owned(),
            merge: merge_state(&o.pr).to_owned(),
            number: o.pr.number,
            title: o.pr.title,
            url: o.pr.url,
            head: o.pr.head_ref_name,
            author: o.author.login,
            updated: o.updated_at,
        })
        .collect())
}

impl Client {
    /// Die offenen PRs ohne Zwischenspeicher (die Seite fragt nur auf Knopfdruck).
    pub async fn open(&self) -> PrList {
        let out = self.run(Instant::now() + TIMEOUT, &OPEN_ARGS).await;
        match out.map_err(|e| hint(&e)).and_then(|out| parse_open(&out)) {
            Ok(prs) => PrList {
                prs,
                error: String::new(),
                fetched: Local::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            },
            Err(error) => PrList {
                error,
                ..PrList::default()
            },
        }
    }

    /// Die letzten 500 nach main gemergten PRs (eine Abfrage für alle Branches).
    pub async fn merged(&self) -> Result<Vec<MergedHead>, String> {
        let out = self.run(Instant::now() + 2 * TIMEOUT, &MERGED_ARGS).await;
        list(&out.map_err(|e| hint(&e))?, "gh pr list")
    }

    /// Legt einen PR von `head` nach main an und liefert seine Adresse.
    pub async fn create(&self, head: &str, title: &str, body: &str) -> Result<String, String> {
        let title = title.trim();
        if title.is_empty() {
            return Err("kein Titel angegeben".to_owned());
        }
        if title.chars().count() > MAX_TITLE || body.len() > MAX_BODY {
            return Err("zu langer Titel oder zu lange Beschreibung".to_owned());
        }
        if head == "main" {
            return Err(format!("von {head} gibt es keinen PR"));
        }
        // Werte mit "=": ein Titel, der mit "-" beginnt, bleibt so ein Wert und wird keine Option.
        let args = [
            "gh".to_owned(),
            "pr".to_owned(),
            "create".to_owned(),
            "--base=main".to_owned(),
            format!("--head={head}"),
            format!("--title={title}"),
            format!("--body={body}"),
        ];
        let out = self.act(&args).await?;
        Ok(out.trim().rsplit('\n').next().unwrap_or_default().trim().to_owned())
    }

    /// Mergt PR `number`, mit `squash` als Squash-Merge. Nur offene PRs gegen main; `--match-head-commit` lässt GitHub
    /// ablehnen, wenn seit dem Lesen jemand nachgepusht hat.
    pub async fn merge(&self, number: i64, squash: bool) -> Result<(), String> {
        let head = self.check_open(number).await?;
        let how = if squash { "--squash" } else { "--merge" };
        let number = number.to_string();
        let pinned = format!("--match-head-commit={head}");
        self.act(&["gh", "pr", "merge", &number, how, &pinned]).await?;
        Ok(())
    }

    /// Schließt PR `number` (nur offen und gegen main) ohne Merge.
    pub async fn close(&self, number: i64) -> Result<(), String> {
        self.check_open(number).await?;
        self.act(&["gh", "pr", "close", &number.to_string()]).await?;
        Ok(())
    }

    /// Liest den PR frisch und liefert seinen Head-Commit; die Liste der Oberfläche kann veraltet sein.
    async fn check_open(&self, number: i64) -> Result<String, String> {
        #[derive(Default, Deserialize)]
        #[serde(default, rename_all = "camelCase")]
        struct View {
            #[serde(deserialize_with = "nullable")]
            state: String,
            #[serde(deserialize_with = "nullable")]
            base_ref_name: String,
            #[serde(deserialize_with = "nullable")]
            head_ref_oid: String,
        }
        if number <= 0 {
            return Err("ungültige PR-Nummer".to_owned());
        }
        let number_arg = number.to_string();
        let args = [
            "gh",
            "pr",
            "view",
            &number_arg,
            "--json",
            "state,baseRefName,headRefOid",
        ];
        let out = self.run(Instant::now() + TIMEOUT, &args).await.map_err(|e| hint(&e))?;
        let view: View = serde_json::from_slice(&out).unwrap_or_default();
        if view.head_ref_oid.is_empty() {
            return Err(format!("PR #{number}: Antwort von gh unlesbar"));
        }
        if view.state != "OPEN" {
            return Err(format!("PR #{number} ist nicht mehr offen"));
        }
        if view.base_ref_name != "main" {
            let base = view.base_ref_name;
            return Err(format!(
                "PR #{number} zielt auf {base}, die Seite bearbeitet nur PRs gegen main"
            ));
        }
        Ok(view.head_ref_oid)
    }

    /// Ruft `gh` für eine Aktion auf und verwirft den Zwischenspeicher, damit der nächste Stand frisch ist.
    async fn act<S: AsRef<str>>(&self, args: &[S]) -> Result<String, String> {
        let out = self.run(Instant::now() + 2 * TIMEOUT, args).await;
        self.cache.lock().await.at = None;
        out.map(|o| String::from_utf8_lossy(&o).into_owned())
            .map_err(|e| hint(&e))
    }
}
