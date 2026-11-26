//! Hilfen der Tests (Go `helpers_test.go` samt der Attrappe `fakeFrames` aus `cli_test.go`). Test-Repos entstehen nur
//! unter `.work/`, mit lokaler git-Konfiguration; die globale Konfiguration bleibt unberührt und ungelesen.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

use serde::Deserialize;

use super::{Cli, DiffAntwort, Error, Fehler, FrameAntwort, PixelAntwort, Renderer, Teile};
use crate::test_helpers::tmp_dir;

pub const MONARCH: &str = "public/grafik/figuren/monarch.paper.json";

/// Wurzel des K3C-Repos (dort liegen `tools/paper/` und die Fixtures).
pub fn repo_root() -> PathBuf {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    dir.ancestors().nth(4).expect("Repo-Wurzel").to_path_buf()
}

/// Fehlt `bun` oder `git`, überspringt der Aufrufer.
pub fn werkzeuge() -> bool {
    for name in ["bun", "git"] {
        if Command::new(name).arg("--version").output().is_err() {
            eprintln!("übersprungen: {name} fehlt (requirements.md)");
            return false;
        }
    }
    true
}

/// Geerbte Variablen, die git vom Arbeitsordner weg auf ein anderes Repo lenken (etwa aus einem Git-Hook).
const GIT_UMLENKUNG: [&str; 6] = [
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_COMMON_DIR",
    "GIT_OBJECT_DIRECTORY",
    "GIT_CONFIG_PARAMETERS",
];

/// Ohne die git-Konfiguration des Nutzers und des Rechners und nur im Repo unter `dir`.
fn abgeschirmt(cmd: &mut Command, dir: &Path) {
    for name in GIT_UMLENKUNG {
        cmd.env_remove(name);
    }
    cmd.current_dir(dir)
        .env("GIT_CONFIG_GLOBAL", repo_root().join(".work/keine-gitconfig"))
        .env("GIT_CONFIG_NOSYSTEM", "1");
}

fn ausgabe(mut cmd: Command, was: &str) -> String {
    let out = cmd.output().expect(was);
    assert!(
        out.status.success(),
        "{was}: {} {}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

/// `bun` in der Repo-Wurzel.
pub fn bun(args: &[&str]) -> String {
    let mut cmd = Command::new("bun");
    abgeschirmt(&mut cmd, &repo_root());
    cmd.args(args);
    ausgabe(cmd, &format!("bun {args:?}"))
}

/// git im Test-Repo (Testaufbau).
pub fn git_in(dir: &Path, args: &[&str]) -> String {
    assert!(dir.join(".git").is_dir(), "kein Test-Repo: {dir:?}");
    let mut cmd = Command::new("git");
    abgeschirmt(&mut cmd, dir);
    cmd.args(args);
    ausgabe(cmd, &format!("git {args:?}"))
}

pub struct TestRepo {
    _dir: tempfile::TempDir,
    pub pfad: PathBuf,
}

/// Frisches Review-Test-Repo (`tools/paper/fixtures/review-repo/erzeuge.ts`); `None` ohne bun oder git.
pub fn test_repo() -> Option<TestRepo> {
    if !werkzeuge() {
        return None;
    }
    let dir = tmp_dir();
    let pfad = dir.path().join("repo");
    bun(&[
        "tools/paper/fixtures/review-repo/erzeuge.ts",
        pfad.to_str().expect("Pfad als Text"),
    ]);
    for (name, wert) in [
        ("user.name", "Fixture"),
        ("user.email", "fixture@k3c.local"),
        ("core.autocrlf", "false"),
        ("commit.gpgsign", "false"),
    ] {
        git_in(&pfad, &["config", "--local", name, wert]);
    }
    Some(TestRepo { _dir: dir, pfad })
}

pub fn lies(pfad: impl AsRef<Path>) -> Vec<u8> {
    std::fs::read(pfad.as_ref()).unwrap_or_else(|e| panic!("{:?}: {e}", pfad.as_ref()))
}

pub fn schreibe(pfad: impl AsRef<Path>, inhalt: impl AsRef<[u8]>) {
    if let Some(ordner) = pfad.as_ref().parent() {
        std::fs::create_dir_all(ordner).expect("Ordner");
    }
    std::fs::write(pfad, inhalt).expect("schreiben");
}

pub fn fixture(name: &str) -> PathBuf {
    repo_root().join("tools/paper/fixtures").join(name)
}

pub fn cli() -> Cli {
    Cli::new(repo_root())
}

/// CLI-Attrappe, deren `frame` die handgemachten Fixture-PNGs liefert: aus dem Cache der Basis das alte, sonst das
/// neue Bild. So entstehen die `pixelAnteil`-Werte der Fixture (`beispiele.ts`).
pub struct FakeFrames {
    cli: Cli,
    pub aufrufe: AtomicU32,
}

pub fn fake_frames() -> FakeFrames {
    FakeFrames {
        cli: cli(),
        aufrufe: AtomicU32::new(0),
    }
}

fn fake_bilder() -> BTreeMap<&'static str, [&'static str; 2]> {
    BTreeMap::from([
        (
            "monarch",
            ["monarch/monarch-galopp-0.png", "monarch/monarch-galopp-0-purpur.png"],
        ),
        ("mauer", ["mauer/mauer-1.png", "mauer/mauer-1-stil.png"]),
    ])
}

#[derive(Deserialize)]
struct FrameFixture {
    teile: Teile,
}

impl Renderer for FakeFrames {
    fn validate(&self, spec: &Path) -> Result<Vec<Fehler>, Error> {
        self.cli.validate(spec)
    }

    fn diff(&self, alt: &Path, neu: &Path, stile: Option<(&Path, &Path)>) -> Result<DiffAntwort, Error> {
        self.cli.diff(alt, neu, stile)
    }

    fn frame(
        &self,
        spec: &Path,
        stil: Option<&Path>,
        animation: &str,
        frame: i64,
        variante: &str,
        out: &Path,
    ) -> Result<FrameAntwort, Error> {
        self.aufrufe.fetch_add(1, Ordering::SeqCst);
        // Die echte CLI (RD1) lehnt die Basis HEAD~1 ab (Reittier fehlt dort); die Attrappe rendert ohne CLI.
        let datei = spec.file_name().unwrap_or_default().to_string_lossy();
        let id = datei.trim_end_matches(".paper.json");
        let Some(bilder) = fake_bilder().get(id).copied() else {
            return self.cli.frame(spec, stil, animation, frame, variante, out);
        };
        let basis = spec.to_string_lossy().replace('\\', "/").contains("/basis/");
        let bild = bilder[usize::from(!basis)];
        schreibe(out, lies(fixture("render").join(bild)));
        let fx: FrameFixture = serde_json::from_slice(&lies(fixture("ui/AssetsFrame.json")))?;
        Ok(FrameAntwort {
            datei: out.to_string_lossy().into_owned(),
            teile: fx.teile,
        })
    }

    fn pixel_diff(&self, a: &Path, b: &Path, out: &Path) -> Result<PixelAntwort, Error> {
        self.cli.pixel_diff(a, b, out)
    }

    fn kennung(&self) -> String {
        self.cli.kennung()
    }
}
