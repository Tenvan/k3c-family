//! Hilfen der Tests (Go `paperassets_test.go`, Hilfen). Test-Repos entstehen nur unter `.work/`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::Cli;
use crate::kitx::sha256_hex;
pub(crate) use crate::review::test_helpers::{repo_root, werkzeuge};
use crate::test_helpers::tmp_dir;

pub const MONARCH_PFAD: &str = "public/grafik/figuren/monarch.paper.json";

/// Die CLI-Attrappe (PV1.4).
pub fn stub_skript() -> PathBuf {
    repo_root().join("tools/paper/fixtures/cli-stub.ts")
}

/// Pfad unter `root` aus einem Pfad mit `/`.
pub fn unter(root: &Path, rel: &str) -> PathBuf {
    let mut p = root.to_path_buf();
    p.extend(rel.split('/'));
    p
}

pub fn schreibe(root: &Path, rel: &str, text: impl AsRef<[u8]>) {
    let p = unter(root, rel);
    fs::create_dir_all(p.parent().expect("Ordner")).expect("Ordner anlegen");
    fs::write(p, text).expect("schreiben");
}

pub fn lies(root: &Path, rel: &str) -> String {
    fs::read_to_string(unter(root, rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

/// Legt `src` (relativ zur Repo-Wurzel) unter `root/dst` ab.
pub fn kopiere(root: &Path, src: &str, dst: &str) {
    let b = fs::read(unter(&repo_root(), src)).unwrap_or_else(|e| panic!("{src}: {e}"));
    schreibe(root, dst, b);
}

/// git nur im Test-Repo: ohne umlenkende Variablen und ohne die Konfiguration des Nutzers.
fn git(root: &Path, args: &[&str]) {
    let mut cmd = Command::new("git");
    for name in [
        "GIT_DIR",
        "GIT_WORK_TREE",
        "GIT_INDEX_FILE",
        "GIT_COMMON_DIR",
        "GIT_OBJECT_DIRECTORY",
        "GIT_CONFIG_PARAMETERS",
    ] {
        cmd.env_remove(name);
    }
    cmd.current_dir(root)
        .env("GIT_CONFIG_GLOBAL", repo_root().join(".work/keine-gitconfig"))
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_AUTHOR_NAME", "test")
        .env("GIT_AUTHOR_EMAIL", "t@example.invalid")
        .env("GIT_COMMITTER_NAME", "test")
        .env("GIT_COMMITTER_EMAIL", "t@example.invalid")
        .args(args);
    let out = cmd.output().expect("git");
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Ordner unter `.work/`; `pfad` verschwindet mit dem Wert.
pub struct Wurzel {
    _dir: tempfile::TempDir,
    pub pfad: PathBuf,
}

pub fn leere_wurzel() -> Wurzel {
    let dir = tmp_dir();
    // Ohne `..`: Der Ordner aus `tmp_dir` enthält sie, Pfade der CLI kommen aber normalisiert zurück.
    let pfad = std::path::absolute(dir.path().join("repo")).expect("absolut");
    fs::create_dir_all(&pfad).expect("Wurzel");
    Wurzel { _dir: dir, pfad }
}

/// Monarch (Reiter), Pferd (Reittier) und Mauer ohne git und ohne bun.
pub fn spec_root() -> Wurzel {
    let w = leere_wurzel();
    for (name, gruppe) in [("monarch", "figuren"), ("pferd", "reittiere"), ("mauer", "gebaeude")] {
        let src = format!("tools/paper/fixtures/specs/{name}.paper.json");
        kopiere(&w.pfad, &src, &format!("public/grafik/{gruppe}/{name}.paper.json"));
    }
    w
}

/// Git-Repo mit Stil-Datei und den Fixture-Specs (ein Commit); `None` ohne bun oder git.
pub fn test_repo() -> Option<(Wurzel, Cli)> {
    if !werkzeuge() {
        return None;
    }
    let w = spec_root();
    let root = &w.pfad;
    for pfad in ["public/grafik/stil.paper.json", "tools/paper/schema/stil.schema.json"] {
        kopiere(root, pfad, pfad);
    }
    schreibe(root, "public/grafik/figuren/monarch/idle.png", "alt");
    schreibe(root, ".gitignore", ".cache/\n");
    git(root, &["init", "-q", "-b", "main"]);
    git(root, &["config", "--local", "core.autocrlf", "false"]);
    git(root, &["add", "-A"]);
    git(root, &["commit", "-q", "--no-gpg-sign", "-m", "Basis"]);
    let cli = Cli::mit_skript(root.clone(), stub_skript());
    Some((w, cli))
}

fn dateien(p: &Path, out: &mut Vec<PathBuf>) {
    if p.is_file() {
        out.push(p.to_path_buf());
        return;
    }
    for e in fs::read_dir(p).into_iter().flatten().flatten() {
        dateien(&e.path(), out);
    }
}

/// Ein Hash über alle Dateien unter den Pfaden (relativ zu `root`).
pub fn hash_baum(root: &Path, rels: &[&str]) -> String {
    let mut alle = Vec::new();
    for rel in rels {
        dateien(&unter(root, rel), &mut alle);
    }
    alle.sort();
    let mut h = Vec::new();
    for p in alle {
        h.extend(p.to_string_lossy().bytes());
        h.extend(fs::read(&p).unwrap_or_default());
    }
    sha256_hex(&h)
}

/// Eine Spec, die validate ablehnt (Material `scharlach`).
pub const BANNER_UNGUELTIG: &str = concat!(
    r#"{"id":"banner","rig":"requisite","version":1,"beschreibung":"Banner","teile":[{"id":"tuch","pivot":[0,0],"#,
    r#""lagen":[{"art":"flaeche","form":"M0 0L10 0L10 10Z","material":"scharlach"}]}],"#,
    r#""ausgabe":{"hoehe_px":64,"rahmen":[0,0,10,10]}}"#
);

/// Ein Pfad als JSON-Text mit `/` (für TypeScript-Quellen).
pub fn ts_text(p: &Path) -> String {
    serde_json::Value::String(p.to_string_lossy().replace('\\', "/")).to_string()
}

/// Ersetzt die CLI durch ein Skript über der Attrappe: `render` liefert, was der JS-Rumpf `render` (a = Argumente,
/// Ergebnis {code, stdout, stderr} oder undefined = Attrappe) zurückgibt, auch als Schritt von `batch`. Jeder
/// bun-Start schreibt eine Zeile in die gelieferte Zähl-Datei (B-856). Das Skript liegt neben dem Test-Repo.
pub fn stub_mit(cli: &mut Cli, render: &str) -> PathBuf {
    let dir = cli.root.parent().expect("Temp-Ordner").to_path_buf();
    let zaehler = dir.join("starts.txt");
    let batch_ts = repo_root().join("tools/paper/cli/batch.ts");
    let kopf = format!(
        "import {{ run }} from {};\nimport {{ batch, schritteAus }} from {};\nconst zaehler = {};\n",
        ts_text(&cli.skript),
        ts_text(&batch_ts),
        ts_text(&zaehler)
    );
    let rumpf = r"import { appendFileSync, mkdirSync, writeFileSync } from 'node:fs';
import { basename, dirname, join } from 'node:path';
appendFileSync(zaehler, process.argv[2] + '\n');
type E = { code: 0 | 1 | 2; stdout: string; stderr: string };
const render = (a: string[]): E | undefined => { RENDER };
const lauf = (a: string[]): E => (a[0] === 'render' ? render(a) : undefined) ?? run(a);
const a = process.argv.slice(2);
const r: E = a[0] === 'batch' ? { code: 0, stdout: JSON.stringify({ ok: true, schritte: batch(schritteAus(a[1]), lauf) }), stderr: '' } : lauf(a);
if (r.stdout) process.stdout.write(r.stdout + '\n');
if (r.stderr) process.stderr.write(r.stderr + '\n');
process.exit(r.code);
";
    let skript = dir.join("stub.ts");
    fs::write(&skript, kopf + &rumpf.replace("RENDER", render)).expect("Stub schreiben");
    cli.skript = skript;
    zaehler
}

/// Der Ordner fehlt oder ist leer.
pub fn leer(dir: &Path) {
    let namen: Vec<_> = fs::read_dir(dir).into_iter().flatten().flatten().collect();
    assert!(namen.is_empty(), "{dir:?} nicht leer: {namen:?}");
}
