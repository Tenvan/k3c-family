//! Anbindung der Render-CLI (Port von `cli.go`): `bun <Skript> <befehl> …` im Checkout.

use std::ffi::OsStr;
use std::io;
use std::path::{Component, Path, PathBuf};
use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};

use super::Error;
use crate::review;

/// Die geänderten Felder, Teile und ob sich der Stil geändert hat.
pub type DiffErgebnis = review::DiffAntwort;
/// Bilddatei und Teil-Boxen `[x, y, b, h]` in Pixeln.
pub type FrameErgebnis = review::FrameAntwort;
/// Anteil abweichender Pixel, ihre Boxen und das Overlay.
pub type PixelErgebnis = review::PixelAntwort;

/// Die Render-CLI im Checkout `root`.
#[derive(Debug, Clone)]
pub struct Cli {
    pub root: PathBuf,
    pub skript: PathBuf,
    /// Begrenzt jeden Aufruf; Tests verkürzen es.
    pub zeit: Duration,
    /// Benennt Ausgabe-Ordner um; Tests lassen es scheitern.
    pub ordner_umbenennen: fn(&Path, &Path) -> io::Result<()>,
}

fn umbenennen(von: &Path, nach: &Path) -> io::Result<()> {
    std::fs::rename(von, nach)
}

/// Geschriebene Manifeste und Dateien (Pfade wie [`Cli::rel`]).
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct RenderErgebnis {
    #[serde(deserialize_with = "null_leer")]
    pub manifeste: Vec<String>,
    #[serde(deserialize_with = "null_leer")]
    pub dateien: Vec<String>,
}

fn null_leer<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<String>, D::Error> {
    Ok(Option::<Vec<String>>::deserialize(d)?.unwrap_or_default())
}

/// Wählt einen Frame; Variante leer und Stufe 0 = keine Angabe.
#[derive(Debug, Clone, Default)]
pub struct FrameArgs {
    pub animation: String,
    pub frame: i64,
    pub variante: String,
    pub stufe: i64,
}

/// Pfad als Text für die Kommandozeile.
pub(super) fn text(p: &Path) -> String {
    p.to_string_lossy().into_owned()
}

fn schraeg(p: &Path) -> String {
    p.to_string_lossy().replace('\\', "/")
}

/// Argumente von `frame` (auch als Schritt eines Sammelaufrufs).
pub(super) fn frame_args(spec: &Path, f: &FrameArgs, out: &Path) -> Vec<String> {
    let mut args = vec!["frame".to_owned(), text(spec)];
    args.extend(["--animation".to_owned(), f.animation.clone()]);
    args.extend(["--frame".to_owned(), f.frame.to_string(), "--out".to_owned(), text(out)]);
    if !f.variante.is_empty() {
        args.extend(["--variante".to_owned(), f.variante.clone()]);
    }
    if f.stufe > 0 {
        args.extend(["--stufe".to_owned(), f.stufe.to_string()]);
    }
    args
}

/// Rest von `p` unter `root` (unter Windows ohne Groß/Klein), `None` außerhalb.
fn unter(root: &Path, p: &Path) -> Option<PathBuf> {
    let gleich = |a: Component<'_>, b: Component<'_>| {
        let (a, b) = (a.as_os_str(), b.as_os_str());
        if cfg!(windows) {
            a.to_string_lossy().to_lowercase() == b.to_string_lossy().to_lowercase()
        } else {
            a == b
        }
    };
    let mut rest = p.components();
    for teil in root.components() {
        if !gleich(teil, rest.next()?) {
            return None;
        }
    }
    let rest: PathBuf = rest.collect();
    // `..` im Rest könnte wieder hinausführen.
    (!rest.components().any(|c| c == Component::ParentDir)).then_some(rest)
}

impl Cli {
    /// Das Skript wie `review::skript`: `K3C_PAPER_CLI`, sonst `tools/paper/bin/paper.ts`, sonst die Attrappe.
    pub fn new(root: impl Into<PathBuf>) -> Cli {
        let root = root.into();
        let skript = review::skript(&root);
        Cli::mit_skript(root, skript)
    }

    pub fn mit_skript(root: PathBuf, skript: PathBuf) -> Cli {
        Cli {
            root,
            skript,
            zeit: review::ZEITLIMIT,
            ordner_umbenennen: umbenennen,
        }
    }

    /// Führt einen Befehl über `review::aufruf` aus (B-706): Exit 0 → stdout, Exit 1 → abgelehnt, sonst ein Fehler.
    pub(super) fn run<S: AsRef<OsStr>>(&self, zeit: Duration, args: &[S]) -> Result<Vec<u8>, Error> {
        Ok(review::aufruf(&self.root, &self.skript, zeit, args)?)
    }

    /// Ein Pfad im Checkout relativ mit `/`; außerhalb bleibt er (mit `/`) unverändert.
    pub(super) fn rel(&self, p: &Path) -> String {
        if !p.is_absolute() {
            return schraeg(p);
        }
        match unter(&self.root, p) {
            Some(r) if r.as_os_str().is_empty() => ".".to_owned(),
            Some(r) => schraeg(&r),
            None => schraeg(p),
        }
    }

    /// Eine Pfadangabe der CLI (relativ zum Checkout oder absolut) absolut.
    pub(super) fn abs(&self, p: &str) -> PathBuf {
        let p: PathBuf = Path::new(p).components().collect();
        if p.is_absolute() { p } else { self.root.join(p) }
    }

    /// Prüft eine Spec; `Ok` = gültig, sonst abgelehnt oder ein Aufruffehler.
    pub fn validate(&self, spec: &Path) -> Result<(), Error> {
        self.run(self.zeit, &["validate".into(), text(spec)])?;
        Ok(())
    }

    /// Rendert eine Spec oder einen Gruppen-Ordner.
    pub fn render(&self, ziel: &Path) -> Result<RenderErgebnis, Error> {
        let r = self.decode(&["render".into(), text(ziel)])?;
        Ok(self.render_rel(r))
    }

    /// Macht die Pfade eines Render-Ergebnisses relativ zum Checkout.
    pub(super) fn render_rel(&self, mut r: RenderErgebnis) -> RenderErgebnis {
        for p in r.manifeste.iter_mut().chain(r.dateien.iter_mut()) {
            *p = self.rel(&self.abs(p));
        }
        r
    }

    /// Schreibt einen Frame nach `out`.
    pub fn frame(&self, spec: &Path, f: &FrameArgs, out: &Path) -> Result<FrameErgebnis, Error> {
        self.decode(&frame_args(spec, f, out))
    }

    /// Vergleicht zwei Specs.
    pub fn diff(&self, alt: &Path, neu: &Path) -> Result<DiffErgebnis, Error> {
        self.decode(&["diff".into(), text(alt), text(neu)])
    }

    /// Vergleicht zwei PNGs und schreibt das Overlay nach `out`.
    pub fn pixel_diff(&self, a: &Path, b: &Path, out: &Path) -> Result<PixelErgebnis, Error> {
        self.decode(&["pixeldiff".into(), text(a), text(b), "--out".into(), text(out)])
    }

    fn decode<T: DeserializeOwned>(&self, args: &[String]) -> Result<T, Error> {
        let raw = self.run(self.zeit, args)?;
        antwort(args.first().map_or("", String::as_str), &raw)
    }
}

/// Deutet die Antwort eines Befehls als JSON.
pub(super) fn antwort<T: DeserializeOwned>(befehl: &str, raw: &[u8]) -> Result<T, Error> {
    serde_json::from_slice(raw).map_err(|e| Error::Sonst(format!("Render-CLI {befehl}: Antwort kein JSON: {e}")))
}
