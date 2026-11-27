//! Die Render-CLI (paper-spec.md › CLI-Vertrag) hinter dem Trait [`Renderer`]; Tests ersetzen einzelne Befehle.

use std::collections::BTreeMap;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use super::aufruf::{ZEITLIMIT, aufruf, skript};
use super::cache::SpecUnlesbar;
use super::hash::RENDERER_VERSION;
use super::types::{Fehler, FeldAenderung, go_zahl, null_leer};

/// Fehler des Review-Pakets.
#[derive(Debug)]
pub enum Error {
    /// Exit 1 der CLI: fachlich abgelehnt mit Fehlern je Regel.
    Abgelehnt(Vec<Fehler>),
    /// Eine Spec ist kein lesbares JSON.
    Unlesbar(SpecUnlesbar),
    Sonst(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Abgelehnt(fehler) => {
                let texte: Vec<&str> = fehler.iter().map(|e| e.text.as_str()).collect();
                f.write_str(&texte.join("; "))
            }
            Error::Unlesbar(u) => u.fmt(f),
            Error::Sonst(text) => f.write_str(text),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Sonst(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Sonst(e.to_string())
    }
}

/// Teil-Boxen `[x, y, b, h]` in Pixeln des Frames je Teil.
pub type Teile = BTreeMap<String, Vec<i64>>;

/// Antwort von `diff`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DiffAntwort {
    #[serde(deserialize_with = "null_leer")]
    pub felder: Vec<FeldAenderung>,
    #[serde(deserialize_with = "null_leer")]
    pub teile: Vec<String>,
    pub stil: bool,
}

/// Antwort von `frame`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FrameAntwort {
    pub datei: String,
    #[serde(deserialize_with = "null_leer")]
    pub teile: Teile,
}

/// Antwort von `pixeldiff`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PixelAntwort {
    #[serde(serialize_with = "go_zahl")]
    pub anteil: f64,
    #[serde(deserialize_with = "null_leer")]
    pub boxen: Vec<Vec<i64>>,
    pub datei: String,
}

/// Die Render-CLI.
pub trait Renderer: Sync {
    /// Regelverstöße der Spec; leer = gültig.
    fn validate(&self, spec: &Path) -> Result<Vec<Fehler>, Error>;
    /// Vergleicht zwei Specs; mit beiden Stil-Dateien meldet `stil`, ob ihr stilHash verschieden ist.
    fn diff(&self, alt: &Path, neu: &Path, stile: Option<(&Path, &Path)>) -> Result<DiffAntwort, Error>;
    /// Rendert einen Frame nach `out` mit der Stil-Datei `stil` (leer: die des Checkouts; B-705).
    fn frame(
        &self,
        spec: &Path,
        stil: Option<&Path>,
        animation: &str,
        frame: i64,
        variante: &str,
        out: &Path,
    ) -> Result<FrameAntwort, Error>;
    /// Vergleicht zwei PNGs und schreibt das Overlay nach `out`.
    fn pixel_diff(&self, a: &Path, b: &Path, out: &Path) -> Result<PixelAntwort, Error>;
    /// Renderer-Version, unter der der Cache die Ergebnisse ablegt.
    fn kennung(&self) -> String {
        RENDERER_VERSION.to_owned()
    }
}

/// Ruft die Render-CLI (siehe [`skript`]) im Ordner `root` auf.
#[derive(Debug, Clone)]
pub struct Cli {
    pub root: PathBuf,
}

fn arg(a: impl AsRef<OsStr>) -> OsString {
    a.as_ref().to_owned()
}

impl Cli {
    pub fn new(root: impl Into<PathBuf>) -> Cli {
        Cli { root: root.into() }
    }

    /// Führt einen Befehl aus und dekodiert die Antwort.
    fn run<T: DeserializeOwned>(&self, args: &[OsString]) -> Result<T, Error> {
        let out = aufruf(&self.root, &skript(&self.root), ZEITLIMIT, args)?;
        Ok(serde_json::from_slice(&out)?)
    }

    /// Rendert einen Frame nach `out`, die Teile in `ohne` ohne ihre Lagen (B-734: Teil in der Vorschau ausblenden).
    pub fn frame_ohne(
        &self,
        spec: &Path,
        animation: &str,
        frame: i64,
        ohne: &[String],
        out: &Path,
    ) -> Result<FrameAntwort, Error> {
        let mut args = vec![arg("frame"), arg(spec), arg("--animation"), arg(animation)];
        args.extend([arg("--frame"), arg(frame.to_string()), arg("--out"), arg(out)]);
        if !ohne.is_empty() {
            args.extend([arg("--ohne"), arg(ohne.join(","))]);
        }
        self.run(&args)
    }
}

impl Renderer for Cli {
    fn validate(&self, spec: &Path) -> Result<Vec<Fehler>, Error> {
        let antwort: Result<serde_json::Map<String, serde_json::Value>, Error> =
            self.run(&[arg("validate"), arg(spec)]);
        match antwort {
            Ok(_) => Ok(Vec::new()),
            Err(Error::Abgelehnt(fehler)) => Ok(fehler),
            Err(e) => Err(e),
        }
    }

    fn diff(&self, alt: &Path, neu: &Path, stile: Option<(&Path, &Path)>) -> Result<DiffAntwort, Error> {
        let mut args = vec![arg("diff"), arg(alt), arg(neu)];
        if let Some((stil_alt, stil_neu)) = stile {
            args.extend([arg("--stil-alt"), arg(stil_alt)]);
            args.extend([arg("--stil-neu"), arg(stil_neu)]);
        }
        self.run(&args)
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
        let mut args = vec![arg("frame"), arg(spec), arg("--animation"), arg(animation)];
        args.extend([arg("--frame"), arg(frame.to_string()), arg("--out"), arg(out)]);
        if !variante.is_empty() {
            args.extend([arg("--variante"), arg(variante)]);
        }
        if let Some(stil) = stil {
            args.extend([arg("--stil"), arg(stil)]);
        }
        self.run(&args)
    }

    fn pixel_diff(&self, a: &Path, b: &Path, out: &Path) -> Result<PixelAntwort, Error> {
        self.run(&[arg("pixeldiff"), arg(a), arg(b), arg("--out"), arg(out)])
    }

    /// Die Attrappe bekommt einen Zusatz, damit ihre Bilder nach dem Wechsel auf die echte CLI nicht weiter aus dem
    /// Cache kommen (RD1).
    fn kennung(&self) -> String {
        let script = skript(&self.root);
        if script.to_string_lossy().ends_with("cli-stub.ts") {
            return format!("{RENDERER_VERSION}+attrappe");
        }
        RENDERER_VERSION.to_owned()
    }
}

#[cfg(test)]
#[path = "cli_tests.rs"]
mod tests;
