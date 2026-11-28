//! Checkout-Zuordnung gegen ein echtes Git-Repo (Go prüft das über die Workbench).

use std::process::Command;

use super::*;
use crate::mcp::workspace::canon;
use crate::testutil::tmp_dir;

fn git(dir: &Path, args: &[&str]) {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["-c", "user.name=k3c", "-c", "user.email=k3c@localhost"])
        .args(["-c", "commit.gpgsign=false"])
        .args(args)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

struct Repo {
    _tmp: tempfile::TempDir,
    base: PathBuf,
    main: PathBuf,
}

impl Repo {
    fn new() -> Repo {
        let tmp = tmp_dir();
        let base = canon(tmp.path());
        let main = base.join("haupt");
        mark_root(&main);
        // Git hält keine leeren Ordner: ohne Datei fehlte data/ im Worktree.
        std::fs::write(main.join("data").join("economy.json"), "{}\n").unwrap();
        git(&main, &["init", "-q"]);
        git(&main, &["add", "."]);
        git(&main, &["commit", "-q", "-m", "start"]);
        Repo { _tmp: tmp, base, main }
    }

    fn add_worktree(&self, rel: &str, branch: &str) -> PathBuf {
        let path = join_rel(&self.base, rel);
        git(
            &self.main,
            &["worktree", "add", "-q", "-b", branch, path.to_str().unwrap()],
        );
        path
    }

    fn resolver(&self) -> Resolver {
        Resolver::new("k3c-test", &self.main)
    }
}

fn join_rel(base: &Path, rel: &str) -> PathBuf {
    clean(&base.join(rel))
}

#[tokio::test]
async fn worktree_name_und_absoluter_pfad() {
    let repo = Repo::new();
    let wt = repo.add_worktree("wt-eins", "eins");
    let r = repo.resolver();
    let listed = r.worktrees().await.unwrap();
    assert_eq!(listed.len(), 2);
    assert!(same_path(&listed[0], &repo.main));

    let by_name = r.resolve_checkout("WT-Eins").await;
    let by_name = by_name.unwrap();
    assert_eq!(by_name.name, "wt-eins");
    assert!(same_path(&by_name.root, &wt));
    let by_path = r.resolve_checkout(wt.to_str().unwrap()).await.unwrap();
    assert_eq!(by_path.name, "wt-eins");
    assert_eq!(r.resolve_checkout("wt-eins").await.unwrap().name, "wt-eins");

    let main = r.resolve_checkout(repo.main.to_str().unwrap()).await.unwrap();
    assert_eq!((main.name.as_str(), &main.root), ("", &repo.main));
}

#[tokio::test]
async fn header_ordnet_zu() {
    let repo = Repo::new();
    let wt = repo.add_worktree("wt-kopf", "kopf");
    let r = repo.resolver();
    assert_eq!(r.resolve_header(None).unwrap().root, repo.main);
    assert_eq!(r.resolve_header(Some("")).unwrap().name, "");
    let sub = wt.join("unter");
    std::fs::create_dir_all(&sub).unwrap();
    assert_eq!(r.resolve_header(sub.to_str()).unwrap().name, "wt-kopf");
    // Ein eigenes Repo mit demselben Modul ist kein Worktree dieses Repos.
    let other = Repo::new();
    let err = r.resolve_header(other.main.to_str()).unwrap_err();
    assert!(err.contains("ist kein Worktree von"), "{err}");
    // Ohne Repo-Wurzel bleibt der Header ohne Wirkung.
    let rootless = Resolver::new("k3c-test", "");
    assert_eq!(rootless.resolve_header(wt.to_str()).unwrap().name, "");
}

#[tokio::test]
async fn unbekannt_mehrdeutig_und_ohne_wurzel() {
    let repo = Repo::new();
    repo.add_worktree("a/gleich", "a");
    repo.add_worktree("b/gleich", "b");
    let r = repo.resolver();
    let err = r.resolve_checkout("fehlt").await.unwrap_err();
    assert_eq!(
        err,
        "checkout \"fehlt\": kein Worktree dieses Namens; gültige Worktrees: gleich, gleich"
    );
    let err = r.resolve_checkout("gleich").await.unwrap_err();
    assert!(err.starts_with("checkout \"gleich\" ist mehrdeutig: "), "{err}");
    let other = Repo::new();
    let err = r.resolve_checkout(other.main.to_str().unwrap()).await.unwrap_err();
    assert!(err.starts_with("checkout: "), "{err}");
    assert!(err.ends_with("; gültige Worktrees: gleich, gleich"), "{err}");

    let rootless = Resolver::new("k3c-test", "");
    assert_eq!(
        rootless.resolve_checkout("x").await.unwrap_err(),
        "checkout \"x\": kein Worktree dieses Namens; git worktree list: k3c-test kennt keine Repo-Wurzel"
    );
}

#[test]
fn worktree_namen_ohne_worktree() {
    assert_eq!(worktree_names(&[PathBuf::from("haupt")]), "keine");
}

fn object(v: Value) -> JsonObject {
    v.as_object().cloned().unwrap()
}

#[test]
fn split_checkout_mit_text_nicht_text_und_ohne_feld() {
    let (value, rest) = split_checkout(Some(object(json!({ "checkout": "wt", "a": 1 })))).unwrap();
    assert_eq!((value.as_str(), rest), ("wt", Some(object(json!({ "a": 1 })))));
    let err = split_checkout(Some(object(json!({ "checkout": 1 })))).unwrap_err();
    assert_eq!(err, "checkout muss ein Text sein (Worktree-Name oder Pfad)");
    let (value, rest) = split_checkout(Some(object(json!({ "a": 1 })))).unwrap();
    assert_eq!((value.as_str(), rest), ("", Some(object(json!({ "a": 1 })))));
    let (value, rest) = split_checkout(Some(object(json!({ "checkout": null })))).unwrap();
    assert_eq!((value.as_str(), rest), ("", Some(JsonObject::new())));
    assert_eq!(split_checkout(None).unwrap(), (String::new(), None));
}

#[test]
fn with_checkout_ergaenzt_schema_und_namen() {
    let mut schema = object(json!({ "type": "object" }));
    let mut names = vec!["id".to_owned(), "art".to_owned()];
    with_checkout(&mut schema, &mut names);
    assert_eq!(names, ["art", "checkout", "id"]);
    assert_eq!(
        schema["properties"]["checkout"],
        json!({
            "type": "string",
            "description": "Worktree-Ordnername oder absoluter Pfad des eigenen Checkouts (git rev-parse \
                --show-toplevel); gilt vor dem Header. Im Worktree immer setzen."
        })
    );
}

#[test]
fn workspace_texte() {
    let root = Workspace {
        root: PathBuf::from("r"),
        name: String::new(),
    };
    let wt = Workspace {
        root: PathBuf::from("w"),
        name: "wt".to_owned(),
    };
    assert_eq!(
        (root.source("Vite"), wt.source("Vite")),
        ("Vite".into(), "wt/Vite".into())
    );
    assert_eq!(wt.console_source("check:go"), "check:wt/go");
    assert_eq!(root.console_source("check:go"), "check:go");
    assert_eq!(wt.console_source("Vite"), "wt/Vite");
    assert_eq!(root.own_source("Vite").as_deref(), Some("Vite"));
    assert_eq!(root.own_source("wt/Vite"), None);
    assert_eq!(wt.own_source("check:wt/go").as_deref(), Some("check:go"));
    assert_eq!(wt.own_source("wt/Vite").as_deref(), Some("Vite"));
    assert_eq!(wt.own_source("Vite"), None);
    assert_eq!(
        (root.label().as_str(), wt.label().as_str()),
        ("Repo-Wurzel r", "Worktree wt (w)")
    );
    assert_eq!((root.short(), wt.short()), ("Repo-Wurzel", "wt"));
}

/// Macht `dir` zu einer Wurzel für `find_root`: Datei `rust/Cargo.toml` und Ordner `data/`.
fn mark_root(dir: &Path) {
    std::fs::create_dir_all(dir.join("rust")).unwrap();
    std::fs::create_dir_all(dir.join("data")).unwrap();
    std::fs::write(dir.join("rust").join("Cargo.toml"), "[workspace]\n").unwrap();
}

#[test]
fn find_root_sucht_aufwaerts_cargo_toml_und_data() {
    let tmp = tmp_dir();
    let base = canon(tmp.path());
    mark_root(&base);
    let deep = base.join("rust").join("tools");
    std::fs::create_dir_all(&deep).unwrap();
    assert_eq!(find_root(&base).unwrap(), base);
    assert_eq!(find_root(&deep).unwrap(), base);
    assert!(crate::mcp::same_dir(&base, base.join("rust").join("..")));
    assert!(!crate::mcp::same_dir(&base, &deep));
}

#[test]
fn find_root_nimmt_den_naechsten_worktree() {
    let tmp = tmp_dir();
    let base = canon(tmp.path());
    mark_root(&base);
    let wt = base.join(".claude").join("worktrees").join("wt1");
    mark_root(&wt);
    let sub = wt.join("rust").join("tools");
    std::fs::create_dir_all(&sub).unwrap();
    assert_eq!(find_root(&sub).unwrap(), wt);
}

/// Der Temp-Ordner liegt im Repo, darum ist die Gegenprobe: gesucht wird weiter bis zur nächsten echten Wurzel.
#[test]
fn find_root_braucht_beides() {
    let tmp = tmp_dir();
    let base = canon(tmp.path());
    mark_root(&base);
    // Spielpaket: nur data/.
    let package = base.join("dist").join("k3c-game");
    std::fs::create_dir_all(package.join("data")).unwrap();
    assert_eq!(find_root(&package).unwrap(), base);
    // Nur rust/Cargo.toml, ohne data/.
    let half = base.join("halb");
    std::fs::create_dir_all(half.join("rust")).unwrap();
    std::fs::write(half.join("rust").join("Cargo.toml"), "[workspace]\n").unwrap();
    assert_eq!(find_root(&half).unwrap(), base);
}
