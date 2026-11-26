//! Checkout je Aufruf über den laufenden Server: Kopf `X-K3C-Root`, Argument `checkout`, Zeile schreibender Tools.

use std::fs;
use std::path::PathBuf;

use serde_json::json;

use super::tests::{Client, checkout_dir, options, result, started};
use super::*;
use crate::mcp::workspace::clean;
use crate::testutil::tmp_dir;

/// Repo-Wurzel mit einem Worktree `wt` (wie `git worktree add` ihn anlegt) und ein fremdes Repo.
fn repo() -> (tempfile::TempDir, PathBuf, PathBuf, PathBuf) {
    let tmp = tmp_dir();
    // Bereinigt wie der Server die Pfade aus Header und Argument (`K3C_TMP` nutzt `/`, ohne die Variable steht `..` darin).
    let base = clean(tmp.path());
    let (root, worktree, foreign) = (base.join("main"), base.join("wt"), base.join("fremd"));
    for dir in [&root, &worktree, &foreign] {
        checkout_dir(dir);
    }
    let gitdir = root.join(".git").join("worktrees").join("wt");
    fs::create_dir_all(&gitdir).unwrap();
    fs::write(worktree.join(".git"), format!("gitdir: {}\n", gitdir.display())).unwrap();
    (tmp, root, worktree, foreign)
}

#[tokio::test(flavor = "multi_thread")]
async fn schreibende_tools_nennen_den_checkout() {
    let (_tmp, root, worktree, foreign) = repo();
    let server = started(options(&root)).await;
    let mut client = Client::new(server.url());
    client.initialize().await;

    let (texts, is_error) = result(&client.call("schreib", json!({"text": "a"})).await);
    let line = format!("Checkout: Repo-Wurzel {}", root.display());
    assert_eq!(
        (texts, is_error),
        (vec!["a in Repo-Wurzel".to_owned(), line.clone()], false)
    );
    // Auch die Ablehnung eines schreibenden Tools nennt ihn, nach dem Parameter-Hinweis.
    let (texts, _) = result(&client.call("schreib", json!({"text": "a", "x": 1})).await);
    assert_eq!(texts[1..], ["gültige Parameter: checkout, text".to_owned(), line]);

    client.root = Some(worktree.display().to_string());
    let (texts, _) = result(&client.call("schreib", json!({"text": "b"})).await);
    let line = format!("Checkout: Worktree wt ({})", worktree.display());
    assert_eq!(texts, ["b in wt".to_owned(), line.clone()]);
    // Lesende Tools bekommen keine Zeile.
    let (texts, _) = result(&client.call("echo", json!({"text": "c"})).await);
    assert_eq!(texts, ["c"]);

    client.root = Some(foreign.display().to_string());
    let (texts, is_error) = result(&client.call("echo", json!({"text": "d"})).await);
    assert!(is_error && texts.len() == 1, "{texts:?}");
    assert!(texts[0].contains("ist kein Worktree von"), "{texts:?}");

    // Das Argument gilt vor dem Header; der Handler sieht es nicht.
    let arguments = json!({"text": "e", "checkout": worktree.display().to_string()});
    let (texts, is_error) = result(&client.call("schreib", arguments).await);
    assert_eq!((texts, is_error), (vec!["e in wt".to_owned(), line], false));
    let calls = server.calls();
    assert_eq!(calls[0].args, r#"{"text":"e"}"#);

    client.root = None;
    let (texts, is_error) = result(&client.call("schreib", json!({"text": "f", "checkout": 3})).await);
    assert_eq!(
        (texts, is_error),
        (
            vec!["checkout muss ein Text sein (Worktree-Name oder Pfad)".to_owned()],
            true
        )
    );
    assert_eq!(server.calls()[0].args, r#"{"checkout":3,"text":"f"}"#);
    // Ein Tool ohne das Argument lehnt es als unbekannt ab.
    let (texts, _) = result(&client.call("echo", json!({"text": "g", "checkout": "wt"})).await);
    assert_eq!(texts[1], "gültige Parameter: text");
    server.stop().await.unwrap();
}

#[test]
fn eigener_checkout_und_zuordnung() {
    let (_tmp, root, worktree, _) = repo();
    let server = Server::new(options(&root), super::tests::App, Default::default(), None);
    let mut extensions = rmcp::model::Extensions::new();
    let own = server.workspace(&extensions);
    assert_eq!((own.root.clone(), own.name.as_str()), (root.clone(), ""));
    assert!(server.own_checkout(&own));
    let other = Workspace {
        root: worktree,
        name: "wt".to_owned(),
    };
    extensions.insert(other.clone());
    assert_eq!(server.workspace(&extensions), other);
    assert!(!server.own_checkout(&other));
}
