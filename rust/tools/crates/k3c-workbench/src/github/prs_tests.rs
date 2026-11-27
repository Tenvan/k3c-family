//! Port von `prs_test.go`.

use std::sync::{Arc, Mutex};

use super::github_tests::fake;
use super::prs::MERGED_ARGS;
use super::*;

const VIEW: &str = r#"{"state":"OPEN","baseRefName":"main","headRefOid":"abc123"}"#;

type Calls = Arc<Mutex<Vec<Vec<String>>>>;

/// Antwortet mit `out`; `gh pr view` beantwortet `view` (Stand des PRs), außer alles scheitert.
fn fake_view(out: Result<&'static str, &'static str>, view: &'static str) -> (Client, Calls) {
    fake(move |args| match out {
        Err(e) => Err(RunError::Other(e.to_owned())),
        Ok(_) if args.get(2).is_some_and(|a| a == "view") => Ok(view.into()),
        Ok(out) => Ok(out.into()),
    })
}

fn answering(out: Result<&'static str, &'static str>) -> (Client, Calls) {
    fake_view(out, VIEW)
}

#[test]
fn offene_prs() {
    let json = r#"[{"number":452,"title":"K5","state":"OPEN","isDraft":false,"headRefName":"sprint/k5",
    "mergeable":"MERGEABLE","url":"u","author":{"login":"tenvan"},"updatedAt":"2026-10-24T08:00:00Z",
    "statusCheckRollup":[{"name":"ci","status":"COMPLETED","conclusion":"SUCCESS"}]},
    {"number":447,"title":"W11","state":"OPEN","isDraft":true,"headRefName":"sprint/w11","mergeable":"CONFLICTING",
    "url":"v","author":{"login":"x"},"updatedAt":"","statusCheckRollup":[{"context":"lint","state":"FAILURE"}]}]"#;
    let want = |number, title: &str, url: &str, head: &str, rest: [&str; 5]| OpenPr {
        number,
        title: title.to_owned(),
        url: url.to_owned(),
        head: head.to_owned(),
        author: rest[0].to_owned(),
        updated: rest[1].to_owned(),
        state: rest[2].to_owned(),
        ci: rest[3].to_owned(),
        merge: rest[4].to_owned(),
    };
    let first = ["tenvan", "2026-10-24T08:00:00Z", "offen", "grün", "konfliktfrei"];
    let second = ["x", "", "Entwurf", "rot", "Konflikt"];
    assert_eq!(
        parse_open(json.as_bytes()).unwrap(),
        vec![
            want(452, "K5", "u", "sprint/k5", first),
            want(447, "W11", "v", "sprint/w11", second),
        ]
    );
    assert!(parse_open(b"kaputt").is_err());
    let gone = br#"[{"number":1,"author":null,"updatedAt":null}]"#;
    assert_eq!(parse_open(gone).unwrap()[0].author, "");
}

#[tokio::test]
async fn gemergte_prs() {
    let (client, calls) = answering(Ok(r#"[{"number":431,"headRefName":"sprint/bd2","headRefOid":"abc"}]"#));
    let want = MergedHead {
        number: 431,
        head: "sprint/bd2".to_owned(),
        sha: "abc".to_owned(),
    };
    assert_eq!(client.merged().await.unwrap(), vec![want]);
    assert_eq!(calls.lock().unwrap()[0], MERGED_ARGS);
    assert!(
        MERGED_ARGS.join(" ").contains("--base main"),
        "nur PRs nach main zählen"
    );
    let (bad, _) = answering(Err("boom"));
    assert_eq!(bad.merged().await.unwrap_err(), "gh: boom");
}

#[tokio::test]
async fn create_argumente_und_pruefung() {
    let (client, calls) = answering(Ok("Warning: x\nhttps://github.com/tenvan/k3c-family/pull/460\n"));
    let url = client.create("feat/git-seite", "-rm -rf: Titel", "Text").await;
    assert_eq!(url.unwrap(), "https://github.com/tenvan/k3c-family/pull/460");
    // Alle Werte mit "=": nichts aus der Eingabe wird zur Option.
    let want = [
        "gh",
        "pr",
        "create",
        "--base=main",
        "--head=feat/git-seite",
        "--title=-rm -rf: Titel",
        "--body=Text",
    ];
    assert_eq!(calls.lock().unwrap()[0], want);
    let long = "x".repeat(201);
    let big = "x".repeat((32 << 10) + 1);
    for (head, title, body) in [
        ("feat/x", " ", ""),
        ("main", "T", ""),
        ("feat/x", long.as_str(), ""),
        ("feat/x", "T", big.as_str()),
    ] {
        assert!(client.create(head, title, body).await.is_err(), "{head} {title}");
    }
    assert_eq!(calls.lock().unwrap().len(), 1, "abgelehnte Aufrufe starten gh nicht");
}

fn view_args(number: &str) -> Vec<String> {
    ["gh", "pr", "view", number, "--json", "state,baseRefName,headRefOid"]
        .map(str::to_owned)
        .into()
}

#[tokio::test]
async fn merge_und_close() {
    let (client, calls) = answering(Ok(""));
    client.merge(452, true).await.unwrap();
    client.merge(452, false).await.unwrap();
    client.close(447).await.unwrap();
    let words = |line: &str| line.split(' ').map(str::to_owned).collect::<Vec<_>>();
    let want = vec![
        view_args("452"),
        words("gh pr merge 452 --squash --match-head-commit=abc123"),
        view_args("452"),
        words("gh pr merge 452 --merge --match-head-commit=abc123"),
        view_args("447"),
        words("gh pr close 447"),
    ];
    assert_eq!(*calls.lock().unwrap(), want);
}

#[tokio::test]
async fn merge_und_close_nur_bei_offenem_pr_gegen_main() {
    for (view, error) in [
        (
            r#"{"state":"OPEN","baseRefName":"sprint/x","headRefOid":"x"}"#,
            "PR #9 zielt auf sprint/x, die Seite bearbeitet nur PRs gegen main",
        ),
        (
            r#"{"state":"MERGED","baseRefName":"main","headRefOid":"x"}"#,
            "PR #9 ist nicht mehr offen",
        ),
        ("kaputt", "PR #9: Antwort von gh unlesbar"),
    ] {
        let (client, calls) = fake_view(Ok(""), view);
        assert_eq!(client.merge(9, true).await.unwrap_err(), error);
        assert_eq!(client.close(9).await.unwrap_err(), error);
        assert_eq!(calls.lock().unwrap().len(), 2, "nur view, kein merge/close");
    }
    let (client, calls) = answering(Ok(""));
    assert_eq!(client.merge(0, true).await.unwrap_err(), "ungültige PR-Nummer");
    assert_eq!(client.close(-1).await.unwrap_err(), "ungültige PR-Nummer");
    assert!(calls.lock().unwrap().is_empty());
    let (bad, _) = answering(Err("boom"));
    assert_eq!(bad.close(1).await.unwrap_err(), "gh: boom");
    let list = bad.open().await;
    assert!(!list.error.is_empty() && list.prs.is_empty(), "{list:?}");
}

/// Eine Aktion verwirft den Zwischenspeicher des Stands.
#[tokio::test]
async fn aktion_verwirft_den_zwischenspeicher() {
    let (client, calls) = fake(|args| {
        Ok(match args[2].as_str() {
            "view" => VIEW.into(),
            _ => "[]".into(),
        })
    });
    client.status(false).await;
    client.close(5).await.unwrap();
    let before = calls.lock().unwrap().len();
    client.status(false).await;
    assert_eq!(calls.lock().unwrap().len(), before + 2);
}
