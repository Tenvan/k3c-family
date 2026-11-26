//! Pixel-Vergleich eines Frames zwischen Basis und Arbeitsstand.

use std::path::Path;

use super::cache::{Quelle, bild, temp_name, uebernehme};
use super::cli::{Error, PixelAntwort, Renderer};
use super::hash::render_key;
use super::stil::erste_animation;

/// Vergleicht einen Frame von Basis (`qa`) und Arbeitsstand (`qn`) mit `pixeldiff`; das Overlay liegt im Cache des
/// Arbeitsstands (`datei` absolut).
pub fn overlay(
    r: &dyn Renderer,
    root: &Path,
    qa: &Quelle,
    qn: &Quelle,
    animation: &str,
    frame: i64,
) -> Result<PixelAntwort, Error> {
    let alt = bild(r, root, qa, animation, frame, "")?;
    let neu = bild(r, root, qn, animation, frame, "")?;
    let key = render_key(&qa.spec_hash, &qa.stil_hash);
    let kurz = key.get(..16).unwrap_or(&key);
    let name = format!("pixeldiff-{kurz}-{animation}-{frame}.png");
    let ordner = qn.ordner(root, r);
    let ziel = ordner.join(&name);
    let tmp = temp_name(&ordner, &name)?;
    let ergebnis = r.pixel_diff(&alt.datei, &neu.datei, &tmp).and_then(|p| {
        uebernehme(&tmp, &ziel)?;
        Ok(p)
    });
    match ergebnis {
        Ok(mut p) => {
            p.datei = ziel.to_string_lossy().into_owned();
            Ok(p)
        }
        Err(e) => {
            let _ = std::fs::remove_file(&tmp);
            Err(e)
        }
    }
}

/// Der `pixeldiff`-Anteil von Frame 0 der ersten Animation (Vertrag); ohne Animation 0. Lässt sich die Basis nicht
/// mehr rendern (fachlich abgelehnt), gilt das ganze Bild als geändert (1).
pub fn pixel_anteil(r: &dyn Renderer, root: &Path, qa: &Quelle, qn: &Quelle) -> Result<f64, Error> {
    let animation = erste_animation(&qn.spec)?;
    if animation.is_empty() {
        return Ok(0.0);
    }
    match overlay(r, root, qa, qn, &animation, 0) {
        Ok(p) => Ok(p.anteil),
        Err(Error::Abgelehnt(_)) => Ok(1.0),
        Err(e) => Err(e),
    }
}
