//! Hilfen der Tests (Go `helpers_test.go`). Test-Repo, Fixtures und `fakeFrames` kommen aus `review`; Test-Repos
//! entstehen nur unter `.work/`.

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use serde::Serialize;
use serde_json::Value;

use super::{Dienst, Rendern, wie_go};
use crate::review::test_helpers::{FakeFrames, cli, fake_frames, lies};
pub use crate::review::test_helpers::{git_in, repo_root, schreibe, test_repo};
use crate::review::{Cli, DiffAntwort, Error, Fehler, FrameAntwort, PixelAntwort, Renderer};

/// Eine Beispielantwort aus `tools/paper/fixtures/ui/`, Zahlen wie Go vergleichbar.
pub fn fixture(methode: &str) -> Value {
    let pfad = repo_root().join(format!("tools/paper/fixtures/ui/{methode}.json"));
    wie_go(serde_json::from_slice(&lies(pfad)).expect("Fixture"))
}

/// So sieht die Oberfläche ein Ergebnis.
pub fn roundtrip(v: impl Serialize) -> Value {
    wie_go(serde_json::to_value(v).expect("JSON"))
}

/// Eine JSON-Liste von Objekten nach `id` sortiert.
pub fn nach_id(v: &Value) -> Vec<Value> {
    let mut liste = v.as_array().cloned().unwrap_or_default();
    liste.sort_by_key(|e| e["id"].as_str().unwrap_or_default().to_owned());
    liste
}

/// Dienst auf `repo` mit der Render-CLI der Repo-Wurzel.
pub fn dienst(repo: &Path) -> Dienst {
    mit(repo, Arc::new(cli()))
}

/// Dienst, dessen Frames für monarch und mauer aus den Fixtures kommen.
pub fn dienst_fake(repo: &Path) -> Dienst {
    mit(repo, Arc::new(fake_frames()))
}

pub fn mit(repo: &Path, r: Arc<dyn Rendern + Sync>) -> Dienst {
    let mut d = Dienst::new(repo);
    d.r = r;
    d
}

/// Die echte CLI mit einzelnen Befehlen, die zählen oder scheitern (gleiche Cache-Kennung).
pub struct Probe {
    cli: Cli,
    pub validiert: AtomicU32,
    validate_kaputt: bool,
    frame_verboten: bool,
}

impl Probe {
    pub fn zaehlend() -> Probe {
        Probe {
            cli: cli(),
            validiert: AtomicU32::new(0),
            validate_kaputt: false,
            frame_verboten: false,
        }
    }

    pub fn validate_kaputt() -> Probe {
        Probe {
            validate_kaputt: true,
            ..Probe::zaehlend()
        }
    }

    pub fn ohne_frame() -> Probe {
        Probe {
            frame_verboten: true,
            ..Probe::zaehlend()
        }
    }
}

impl Renderer for Probe {
    fn validate(&self, spec: &Path) -> Result<Vec<Fehler>, Error> {
        self.validiert.fetch_add(1, Ordering::SeqCst);
        if self.validate_kaputt {
            return Err(Error::Sonst("bun abgestürzt".to_owned()));
        }
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
        if self.frame_verboten {
            return Err(Error::Sonst("Frame gerendert, obwohl er im Cache liegt".to_owned()));
        }
        self.cli.frame(spec, stil, animation, frame, variante, out)
    }

    fn pixel_diff(&self, a: &Path, b: &Path, out: &Path) -> Result<PixelAntwort, Error> {
        self.cli.pixel_diff(a, b, out)
    }

    fn kennung(&self) -> String {
        self.cli.kennung()
    }
}

impl Rendern for Probe {
    fn frame_ohne(
        &self,
        spec: &Path,
        animation: &str,
        frame: i64,
        ohne: &[String],
        out: &Path,
    ) -> Result<FrameAntwort, Error> {
        self.cli.frame_ohne(spec, animation, frame, ohne, out)
    }
}

impl Rendern for FakeFrames {
    fn frame_ohne(
        &self,
        spec: &Path,
        animation: &str,
        frame: i64,
        ohne: &[String],
        out: &Path,
    ) -> Result<FrameAntwort, Error> {
        cli().frame_ohne(spec, animation, frame, ohne, out)
    }
}
