//! Port von `paperassets_test.go` (die Hilfen liegen in `test_helpers.rs`): die Anbindung der Render-CLI.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use super::test_helpers::{MONARCH_PFAD, repo_root, schreibe, test_repo, unter};
use super::*;
use crate::test_helpers::tmp_dir;

#[test]
fn cli_validate() {
    let Some((w, cli)) = test_repo() else { return };
    cli.validate(&unter(&w.pfad, MONARCH_PFAD)).unwrap();
    let fremd = repo_root().join("tools/paper/fixtures/specs/ungueltig/fremdes-material.paper.json");
    let err = cli.validate(&fremd).unwrap_err();
    let fehler = err.abgelehnt().unwrap_or_else(|| panic!("{err}"));
    assert!(fehler.len() == 1 && fehler[0].regel == "material-unbekannt", "{err}");
    let err = cli.run(cli.zeit, &["gibtsnicht"]).unwrap_err().to_string();
    assert!(err.contains("unbekannter Befehl"), "{err}");
}

#[test]
fn cli_pfade_relativ() {
    let dir = tmp_dir();
    let root = dir.path().join("repo");
    let cli = Cli::mit_skript(root.clone(), PathBuf::new());
    assert_eq!(cli.rel(&unter(&root, "public/grafik/x.png")), "public/grafik/x.png");
    let relativ = "tools/paper/fixtures/render/monarch/manifest.json";
    assert_eq!(cli.rel(Path::new(relativ)), relativ);
    let fremd = dir.path().join("anders").join("y.png");
    assert_eq!(cli.rel(&fremd), fremd.to_string_lossy().replace('\\', "/"));
}

/// Variablen dieses Prozesses bleiben unberührt (`set_var` ist `unsafe`): Der Test läuft als eigener Prozess mit
/// geänderter Umgebung. `true` im Kind.
fn im_kindprozess(test: &str, name: &str, wert: &Path) -> bool {
    const MARKE: &str = "K3C_TEST_KIND";
    if std::env::var_os(MARKE).is_some() {
        return true;
    }
    let mut cmd = Command::new(std::env::current_exe().unwrap());
    cmd.args([&format!("paperassets::tests::{test}"), "--exact", "--nocapture"])
        .env(MARKE, "1")
        .env(name, wert);
    let out = cmd.output().unwrap();
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success() && text.contains("1 passed"), "{text}");
    false
}

#[test]
fn cli_pfad_aus_umgebung() {
    if !im_kindprozess("cli_pfad_aus_umgebung", "K3C_PAPER_CLI", Path::new("x/y.ts")) {
        return;
    }
    let c = Cli::new("R");
    assert!(c.skript == Path::new("x/y.ts") && c.root == Path::new("R"), "{c:?}");
}

/// B-706: gemeinsame Anbindung mit `review`, ohne `paper.ts` die Attrappe.
#[test]
fn cli_pfad_standard_und_attrappe() {
    if std::env::var_os("K3C_PAPER_CLI").is_some_and(|v| !v.is_empty()) {
        eprintln!("übersprungen: K3C_PAPER_CLI ist gesetzt");
        return;
    }
    let dir = tmp_dir();
    let root = dir.path();
    assert_eq!(Cli::new(root).skript, unter(root, "tools/paper/fixtures/cli-stub.ts"));
    schreibe(root, "tools/paper/bin/paper.ts", "");
    assert_eq!(Cli::new(root).skript, unter(root, "tools/paper/bin/paper.ts"));
}

/// B-706: Fehlt bun im PATH, nennt der Fehler requirements.md.
#[test]
fn cli_ohne_bun() {
    let dir = tmp_dir();
    if !im_kindprozess("cli_ohne_bun", "PATH", dir.path()) {
        return;
    }
    let cli = Cli::mit_skript(dir.path().to_path_buf(), "x.ts".into());
    let err = cli.validate(Path::new("x.paper.json")).unwrap_err().to_string();
    assert!(err.contains("requirements.md"), "{err}");
}

/// Review MC1.5: Eine hängende CLI endet nach dem Zeitlimit (sonst hält sie die Asset-Sperre für immer).
#[test]
fn cli_zeitlimit() {
    let Some((w, mut cli)) = test_repo() else { return };
    let skript = w.pfad.parent().unwrap().join("haengt.ts");
    fs::write(&skript, "await Bun.sleep(30000);\n").unwrap();
    cli.skript = skript;
    cli.zeit = Duration::from_millis(300);
    let start = Instant::now();
    let err = cli.validate(&unter(&w.pfad, MONARCH_PFAD)).unwrap_err().to_string();
    assert!(err.contains("Zeitlimit"), "{err}");
    assert!(start.elapsed() < Duration::from_secs(15), "{:?}", start.elapsed());
}
