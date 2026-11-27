//! Port von `github_test.go`; Testdaten byte-gleich unter `tests/data/github/`. Nie das echte `gh`: jeder Client hier
//! hat einen gestellten Aufruf.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use chrono::{DateTime, FixedOffset};
use tokio::time::Instant;

use super::*;

const PRS: &[u8] = include_bytes!("../../tests/data/github/prs.json");
const RUNS: &[u8] = include_bytes!("../../tests/data/github/runs.json");

/// Gestellter Aufruf: merkt sich die Argumente und fragt `answer`.
pub(super) fn fake(
    answer: impl Fn(&[String]) -> Result<Vec<u8>, RunError> + Send + Sync + 'static,
) -> (Client, Arc<Mutex<Vec<Vec<String>>>>) {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let seen = Arc::clone(&calls);
    let run: Runner = Arc::new(move |args: Vec<String>| -> RunFuture {
        let result = answer(&args);
        seen.lock().unwrap().push(args);
        Box::pin(async move { result })
    });
    (Client::with_runner(run), calls)
}

fn noon() -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339("2026-10-04T12:00:00Z").unwrap()
}

#[test]
fn parse_der_testdaten() {
    let d = parse(PRS, RUNS, noon()).unwrap();
    let want = [
        ("M8", 110, "offen", "grün", "konfliktfrei"), // neuer PR gewinnt, neuerer Check-Lauf gewinnt
        ("M7", 108, "Entwurf", "läuft", "Konflikt"),  // CheckRun läuft, StatusContext grün
        ("LT1", 107, "offen", "rot", "unbekannt"),    // StatusContext ERROR
        ("S5", 95, "gemergt", "–", "–"),              // ohne Checks, nicht offen
    ];
    assert_eq!(d.sprints.len(), want.len(), "feature/x zählt nicht: {:?}", d.sprints);
    for (id, number, state, ci, merge) in want {
        let g = &d.sprints[id];
        assert_eq!((g.number, g.state.as_str()), (number, state), "{id}: {g:?}");
        assert_eq!((g.ci.as_str(), g.merge.as_str()), (ci, merge), "{id}: {g:?}");
    }
    let main = d.main.as_ref().unwrap();
    assert_eq!((main.ci.as_str(), main.title.as_str()), ("rot", "M7 · Seiten (#102)"));
    assert_eq!(d.fetched, "2026-10-04T12:00:00Z");
    let text = text(&d);
    for line in [
        "M8 #110 offen · CI grün · konfliktfrei · https://",
        "S5 #95 gemergt · CI – · https://",
        "main: CI rot",
    ] {
        assert!(text.contains(line), "Text ohne {line:?}:\n{text}");
    }
}

#[test]
fn parse_leer_und_kaputt() {
    let d = parse(b"[]", b"", noon()).unwrap();
    assert!(d.sprints.is_empty() && d.main.is_none(), "{d:?}");
    assert_eq!(text(&d), "keine Sprint-PRs");
    assert!(
        parse(b"kein json", b"", noon())
            .unwrap_err()
            .starts_with("gh pr list: ")
    );
    assert!(
        parse(b"[]", b"kaputt", noon())
            .unwrap_err()
            .starts_with("gh run list: ")
    );
    // Go nimmt `null` als Nullwert.
    let nulls = br#"[{"number":1,"headRefName":"sprint/x","statusCheckRollup":null,"mergeable":null}]"#;
    assert_eq!(parse(nulls, b"null", noon()).unwrap().sprints["X"].ci, "–");
}

/// JSON wie `typesGit.ts` und `types.ts`: leere Felder fehlen, `sprints` ist immer ein Objekt.
#[test]
fn json_wie_go() {
    let empty = serde_json::to_value(Data::default()).unwrap();
    assert_eq!(empty, serde_json::json!({"sprints": {}}));
    let list = serde_json::to_value(PrList::default()).unwrap();
    assert_eq!(list, serde_json::json!({"prs": []}));
}

#[tokio::test]
async fn client_hinweis_und_zwischenspeicher() {
    let step = Arc::new(Mutex::new(0));
    let now = Arc::clone(&step);
    let (client, calls) = fake(move |args| match *now.lock().unwrap() {
        0 => Err(RunError::NotFound),
        // Der main-Lauf fehlt, die PRs reichen.
        1 if args[1] == "run" => Err(RunError::Other("kein Netz".to_owned())),
        1 => Ok(PRS.to_vec()),
        _ => Err(RunError::Timeout),
    });
    let d = client.status(false).await;
    assert!(d.error.contains("gh nicht gefunden") && d.sprints.is_empty(), "{d:?}");

    *step.lock().unwrap() = 1;
    let d = client.status(false).await;
    assert!(d.error.is_empty() && d.main.is_none(), "{d:?}");
    assert_eq!(d.sprints["M8"].number, 110);

    let n = calls.lock().unwrap().len();
    client.status(false).await;
    assert_eq!(calls.lock().unwrap().len(), n, "Zwischenspeicher umgangen");

    *step.lock().unwrap() = 2;
    let d = client.status(true).await;
    assert_eq!(d.sprints["M8"].number, 110, "{d:?}");
    assert!(d.error.starts_with("gh antwortet nicht (Netz?) (Stand von "), "{d:?}");
    assert!(text(&d).starts_with("Hinweis: gh antwortet nicht"));
}

#[tokio::test]
async fn feste_argumente_und_ablauf_der_frist() {
    let (client, calls) = fake(|_| Ok(b"[]".to_vec()));
    client.status(false).await;
    let seen = calls.lock().unwrap().clone();
    assert_eq!(
        seen[0].join(" "),
        "gh pr list --state all --base main --limit 50 --json \
         number,title,state,isDraft,headRefName,mergeable,mergeStateStatus,statusCheckRollup,url"
    );
    assert_eq!(
        seen[1].join(" "),
        "gh run list --branch main --limit 1 --json status,conclusion,displayTitle,url,createdAt"
    );
}

/// Nach [`TTL`] wird neu geholt.
#[tokio::test]
async fn zwischenspeicher_laeuft_ab() {
    let (client, calls) = fake(|_| Ok(b"[]".to_vec()));
    client.status(false).await;
    for (age, want) in [(TTL - Duration::from_secs(5), 2), (TTL, 4)] {
        client.cache.lock().await.at = Instant::now().checked_sub(age);
        client.status(false).await;
        assert_eq!(calls.lock().unwrap().len(), want, "Alter {age:?}");
    }
}

#[tokio::test]
async fn haengender_aufruf_endet_an_der_frist() {
    let run: Runner = Arc::new(|_| -> RunFuture { Box::pin(std::future::pending()) });
    let end = Client::with_runner(run).run(Instant::now(), &["gh"]).await;
    assert_eq!(end, Err(RunError::Timeout));
}

#[test]
fn hinweise_zu_fehlern() {
    let login = RunError::Exit("To get started with GitHub CLI, please run:  gh auth login\n".to_owned());
    assert_eq!(client::hint(&login), "gh nicht angemeldet: `gh auth login` ausführen");
    assert_eq!(
        client::hint(&RunError::Exit("HTTP 502\nmehr\n".to_owned())),
        "gh: HTTP 502"
    );
    assert_eq!(client::hint(&RunError::Other("boom".to_owned())), "gh: boom");
}
