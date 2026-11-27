//! Port von `gitcommit_test.go`. Die Repos sind Wegwerf-Repos unter `.work/` mit abgeschottetem git (`Fixture`).

use super::*;
use crate::gitops::fixture_tests::Fixture;

fn message(kind: &str, scope: &str, subject: &str, body: &str) -> Message {
    Message {
        kind: kind.to_owned(),
        scope: scope.to_owned(),
        subject: subject.to_owned(),
        body: body.to_owned(),
    }
}

fn names(paths: &[&str]) -> Vec<String> {
    paths.iter().map(|p| (*p).to_owned()).collect()
}

#[test]
fn nachricht() {
    let m = message(" feat ", "srv", " Git-Seite ", "Rumpf").normalize().unwrap();
    assert_eq!(m.format(), "feat(srv): Git-Seite\n\nRumpf\n");
    let long = "ä".repeat(MAX_SUBJECT + 1);
    for (field, bad) in [
        ("type", message("wip", "", "x", "")),
        ("scope", message("fix", "Srv Neu", "x", "")),
        ("scope", message("fix", "-x", "x", "")),
        ("subject", message("fix", "", "", "")),
        ("subject", message("fix", "", "a\nb", "")),
        ("subject", message("fix", "", &long, "")),
    ] {
        let error = bad.normalize().unwrap_err();
        assert_eq!(error.field, field, "{error}");
        assert!(error.to_string().starts_with(&format!("{field}: ")));
    }
    assert!(message("fix", "", &"ä".repeat(MAX_SUBJECT), "").normalize().is_ok());
    assert_eq!(message("docs", "", "kurz", "").format(), "docs: kurz\n");
}

#[test]
fn status_lesen() {
    let out = b"## main...origin/main\0M  a.go\0 M b.go\0MM c.go\0R  neu.go\0alt.go\0?? d.go\0UU e.go\0";
    let state = parse_status(out).unwrap();
    assert_eq!(state.branch, "main");
    let staged: Vec<(&str, &str)> = state
        .staged
        .iter()
        .map(|f| (f.path.as_str(), f.status.as_str()))
        .collect();
    assert_eq!(staged, [("a.go", "M"), ("c.go", "M"), ("neu.go", "R")]);
    let unstaged: Vec<&str> = state.unstaged.iter().map(|f| f.status.as_str()).collect();
    assert_eq!(unstaged, ["M", "M", "?", "U"]);
    assert_eq!(parse_status(b"## No commits yet on neu\0").unwrap().branch, "neu");
    assert!(
        parse_status(b"kaputt\0")
            .unwrap_err()
            .starts_with("git status: unerwarteter Eintrag")
    );
}

#[test]
fn pfade() {
    for bad in [
        "",
        "../x",
        "a/../../x",
        "..",
        "/abs",
        "\\abs",
        "\\\\server\\share",
        "C:/x",
        "C:x",
        "a/../C:/x",
        "a\nb",
        "a\0b",
    ] {
        assert!(check_paths(&names(&[bad])).is_err(), "{bad:?} muss abgelehnt werden");
    }
    assert_eq!(
        check_paths(&names(&["a/b.go", "dir\\c.go", "a/../b", "-x", "--all"])),
        Ok(())
    );
    assert_eq!(check_paths(&[]).unwrap_err(), "keine Dateien angegeben");
}

/// Ein leeres Repo unter `.work/` mit abgeschottetem git; die Sicherung prüft, dass git wirklich dieses Repo trifft.
async fn new_repo(f: &Fixture) -> Git {
    let root = f.tmp.path().join("repo");
    std::fs::create_dir(&root).unwrap();
    f.sh(&root, &["init", "-q", "-b", "sprint/test"]).await;
    let top = f.sh(&root, &["rev-parse", "--show-toplevel"]).await;
    assert!(top.ends_with("/repo"), "{top}");
    std::fs::write(root.join("a.txt"), "eins\n").unwrap();
    Git::with_env(root, f.env.clone())
}

/// Der ganze Weg in einem echten Repo: untracked → stagen → committen → Index leer.
#[tokio::test]
async fn repo() {
    let f = Fixture::empty();
    let git = new_repo(&f).await;
    let error = git.commit(&message("chore", "", "leer", "")).await.unwrap_err();
    assert_eq!(error, "nichts gestaged");
    let state = git.read().await.unwrap();
    assert_eq!((state.unstaged.len(), state.unstaged[0].status.as_str()), (1, "?"));

    let a = names(&["a.txt"]);
    git.stage(&a).await.unwrap();
    let state = git.read().await.unwrap();
    assert_eq!((state.staged.len(), state.staged[0].status.as_str()), (1, "A"));
    git.unstage(&a).await.unwrap(); // vor dem ersten Commit: rm --cached
    assert!(git.read().await.unwrap().staged.is_empty());

    git.stage(&a).await.unwrap();
    let hash = git.commit(&message("feat", "srv", "erster", "Rumpf")).await.unwrap();
    assert!(!hash.is_empty());
    let state = git.read().await.unwrap();
    assert!(state.staged.is_empty() && state.unstaged.is_empty(), "{state:?}");
    assert_eq!(state.recent.len(), 1);
    assert!(state.recent[0].contains("feat(srv): erster"), "{state:?}");

    // Mit HEAD nimmt `restore --staged` die Datei aus dem Index, der Arbeitsbaum bleibt.
    std::fs::write(f.tmp.path().join("repo/a.txt"), "zwei\n").unwrap();
    git.stage(&a).await.unwrap();
    git.unstage(&a).await.unwrap();
    assert!(git.read().await.unwrap().staged.is_empty());
    let kept = std::fs::read_to_string(f.tmp.path().join("repo/a.txt")).unwrap();
    assert_eq!(kept, "zwei\n");
}

/// Pfade sind wörtlich (kein Glob, keine Option) und verlassen den Checkout nicht.
#[tokio::test]
async fn pfade_sind_woertlich() {
    let f = Fixture::empty();
    let git = new_repo(&f).await;
    for path in ["*.txt", "--all", "-A", ":/"] {
        let error = git.stage(&names(&[path])).await.unwrap_err();
        assert!(error.starts_with("git --literal-pathspecs: "), "{path}: {error}");
    }
    std::fs::write(f.tmp.path().join("aussen.txt"), "x").unwrap();
    assert!(git.stage(&names(&["../aussen.txt"])).await.is_err());
    assert!(git.read().await.unwrap().staged.is_empty());
}

/// Hooks laufen, auch mit der Ausgabe im Fehler; ein Commit entsteht dann nicht.
#[tokio::test]
async fn hooks_laufen() {
    let f = Fixture::empty();
    let git = new_repo(&f).await;
    let hooks = f.tmp.path().join("repo/.git/hooks");
    std::fs::create_dir_all(&hooks).unwrap();
    let hook = hooks.join("pre-commit");
    std::fs::write(&hook, "#!/bin/sh\necho Hook sagt nein\nexit 1\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    git.stage(&names(&["a.txt"])).await.unwrap();
    let error = git.commit(&message("feat", "", "x", "")).await.unwrap_err();
    assert!(
        error.starts_with("git commit: ") && error.contains("Hook sagt nein"),
        "{error}"
    );
    assert!(git.read().await.unwrap().recent.is_empty());
}
