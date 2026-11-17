//! Level-Plan: welche Sheets ein Level braucht, mit Pfaden, Maßen und Texturbudget. Liest nur Manifeste und PNG-Köpfe
//! (wie `image.DecodeConfig`), keine Pixel und keine GPU; alle fehlenden Dateien werden gesammelt gemeldet.
//! Vorlage: `native/k3c-game/art/plan.go`.

use std::path::{Path, PathBuf};

use super::{ArtError, Budget, Manifest, read_manifest};

/// SheetSpec ist ein geplantes Sheet: eine Animation eines Assets als Streifen.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SheetSpec {
    pub asset: String,
    pub anim: String,
    pub path: PathBuf,
    pub frames: i64,
    pub fps: f64,
    /// Größe eines Frames.
    pub fw: i64,
    pub fh: i64,
    /// Größe des PNG.
    pub w: i64,
    pub h: i64,
    /// Fußpunkt als Anteil des Frames (von links oben); Rahmen-Einheiten je Bild-Pixel (für den Maßstab).
    pub origin_x: f64,
    pub origin_y: f64,
    pub units_per_px: f64,
}

/// Plan ist die Liste aller Sheets eines Levels samt Budget.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Plan {
    pub level: String,
    pub sheets: Vec<SheetSpec>,
    pub budget: Budget,
}

/// Plant alle Animationen der Assets (`gruppe/id`) unter dem Grafik-Ordner dir. Fehlt etwas, kommt ein Fehler
/// (`ArtError::Missing`) mit allen fehlenden Pfaden; es entsteht nie ein halber Plan.
pub fn plan_level(dir: &Path, level: &str, assets: &[String]) -> Result<Plan, ArtError> {
    let mut p = Plan {
        level: level.to_string(),
        ..Plan::default()
    };
    let mut errs = Vec::new();
    for asset in assets {
        let m = match read_manifest(dir, asset) {
            Ok(m) => m,
            Err(e) => {
                errs.push(e);
                continue;
            }
        };
        for name in m.anim_names() {
            match plan_sheet(dir, asset, &name, &m) {
                Ok(spec) => {
                    p.budget.add(spec.w, spec.h);
                    p.sheets.push(spec);
                }
                Err(e) => errs.push(e),
            }
        }
    }
    match ArtError::join(errs) {
        Some(e) => Err(e),
        None => Ok(p),
    }
}

/// Prüft eine Animation: Maße im Manifest, PNG vorhanden und groß genug.
pub fn plan_sheet(dir: &Path, asset: &str, anim: &str, m: &Manifest) -> Result<SheetSpec, ArtError> {
    let a = m.anims.get(anim).cloned().unwrap_or_default();
    let (fw, fh, r) = m.resolve(&a);
    let path = dir.join(asset).join(&a.file);
    if a.frames < 1 || fw < 1 || fh < 1 || r[2] == 0.0 || r[3] == 0.0 {
        return Err(ArtError::Broken(format!(
            "{}: Animation {anim} in {asset} hat ungültige Maße",
            path.display()
        )));
    }
    let (w, h) = png_size(&path)?;
    if w < a.frames * fw || h < fh {
        return Err(ArtError::Broken(format!(
            "{}: Bild {w}×{h} zu klein für {} Frames à {fw}×{fh}",
            path.display(),
            a.frames
        )));
    }
    Ok(SheetSpec {
        asset: asset.to_string(),
        anim: anim.to_string(),
        path,
        frames: a.frames,
        fps: a.fps,
        fw,
        fh,
        w,
        h,
        origin_x: -r[0] / r[2],
        origin_y: -r[1] / r[3],
        units_per_px: r[3] / fh as f64,
    })
}

/// Liest nur den PNG-Kopf (Breite, Höhe) wie `image.DecodeConfig`.
fn png_size(path: &Path) -> Result<(i64, i64), ArtError> {
    let file = std::fs::File::open(path).map_err(|e| {
        ArtError::Missing(format!(
            "Sheet {} ({e}); erst `task gen:paper` ausführen",
            path.display()
        ))
    })?;
    let reader = png::Decoder::new(std::io::BufReader::new(file))
        .read_info()
        .map_err(|e| ArtError::Broken(format!("{}: Sheet kaputt ({e})", path.display())))?;
    let info = reader.info();
    Ok((i64::from(info.width), i64::from(info.height)))
}
