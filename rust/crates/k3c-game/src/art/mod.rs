//! Papierschnitt-Grafik laden: Index, Manifest, Level-Plan mit Texturbudget, Sheets, Zuordnung `data/paper.json`.
//! Planen (Pfade, Maße, Budget; ohne GPU, testbar) getrennt vom Hochladen (`Canvas::texture`).
//! Vorlage: `native/k3c-game/art/`; RU7.1 portiert manifest, plan, budget, sheet, der RU2-Lader (`Library::open`,
//! `assets`, `load`) läuft bis RU7.2 auf dem portierten Manifest weiter.

mod budget;
mod manifest;
#[cfg(test)]
mod manifest_tests;
mod paper;
mod pending;
mod plan;
#[cfg(test)]
mod plan_tests;
mod sheet;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod testutil;

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

pub use budget::Budget;
pub use manifest::{Anim, Manifest, group_assets, read_manifest};
pub use paper::paper_assets;
pub use pending::Affine;
pub use plan::{Plan, SheetSpec, plan_level, plan_sheet};
pub use sheet::{Sheet, SheetFrame, upload};

/// Fehler beim Laden; die Meldung nennt immer den Pfad. `Missing` entspricht Gos `ErrMissing`.
#[derive(Debug)]
pub enum ArtError {
    Missing(String),
    Broken(String),
    /// Mehrere Fehler wie `errors.Join` (Meldungen durch Zeilenumbruch getrennt).
    Joined(Vec<ArtError>),
}

impl ArtError {
    /// Fasst Fehler zusammen wie `errors.Join`: keiner → `None`.
    pub fn join(errs: Vec<ArtError>) -> Option<ArtError> {
        if errs.is_empty() {
            None
        } else {
            Some(Self::Joined(errs))
        }
    }

    /// Wie `errors.Is(err, ErrMissing)`: wahr, wenn dieser oder ein enthaltener Fehler fehlende Grafik meldet.
    pub fn is_missing(&self) -> bool {
        match self {
            Self::Missing(_) => true,
            Self::Broken(_) => false,
            Self::Joined(errs) => errs.iter().any(Self::is_missing),
        }
    }
}

impl fmt::Display for ArtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(m) => write!(f, "fehlende Grafik: {m}"),
            Self::Broken(m) => write!(f, "{m}"),
            Self::Joined(errs) => {
                for (i, e) in errs.iter().enumerate() {
                    if i > 0 {
                        writeln!(f)?;
                    }
                    write!(f, "{e}")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for ArtError {}

/// RGBA8-Bild, Zeilen oben nach unten.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

/// Ein Bild einer Animation: Sheet, Rechteck (x, y, Breite, Höhe in Pixeln), Fußpunkt (Anteil von Breite und Höhe).
#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub sheet: usize,
    pub rect: [u32; 4],
    pub anker: [f64; 2],
}

/// Lage einer Animation in `Asset::frames`.
#[derive(Debug, Clone, PartialEq)]
pub struct AnimRange {
    pub start: usize,
    pub frames: usize,
    pub fps: f64,
    /// Rahmen in Einheiten (x, y, Breite, Höhe).
    pub rahmen: [f64; 4],
}

/// Geladenes Asset: je Animation ein Sheet, alle Frames mit Rechteck und Fußpunkt.
#[derive(Debug, Clone)]
pub struct Asset {
    pub sheets: Vec<Image>,
    pub frames: Vec<Frame>,
    pub animationen: BTreeMap<String, AnimRange>,
}

/// Grafik-Ordner mit Zuordnung; öffnet Assets per Name `gruppe/id`.
pub struct Library {
    dir: PathBuf,
    paper: serde_json::Value,
}

impl Library {
    /// Öffnet den Grafik-Ordner (`public/grafik`) und die Zuordnung `data/paper.json`.
    pub fn open(grafik_dir: &Path, paper_json: &Path) -> Result<Self, ArtError> {
        if !grafik_dir.is_dir() {
            return Err(ArtError::Missing(format!(
                "Grafik-Ordner {}; erst `task gen:paper` ausführen",
                grafik_dir.display()
            )));
        }
        let text = std::fs::read_to_string(paper_json)
            .map_err(|e| ArtError::Missing(format!("{} ({e})", paper_json.display())))?;
        let paper = serde_json::from_str(&text)
            .map_err(|e| ArtError::Broken(format!("{}: kaputt ({e})", paper_json.display())))?;
        Ok(Self {
            dir: grafik_dir.to_path_buf(),
            paper,
        })
    }

    /// Alle Assets, die `data/paper.json` nennt (`gruppe/id`, sortiert, ohne Doppelte).
    pub fn assets(&self) -> Result<Vec<String>, ArtError> {
        paper_assets(&self.dir, &self.paper)
    }

    /// Lädt Asset `gruppe/id` (ein Zusatz `@variante` wird ignoriert).
    pub fn load(&self, asset: &str) -> Result<Asset, ArtError> {
        let name = asset.split('@').next().unwrap_or(asset);
        let m = read_manifest(&self.dir, name)?;
        build_asset(&self.dir.join(name), &m)
    }
}

fn build_asset(folder: &Path, m: &Manifest) -> Result<Asset, ArtError> {
    let mut asset = Asset {
        sheets: Vec::new(),
        frames: Vec::new(),
        animationen: BTreeMap::new(),
    };
    for (name, a) in &m.anims {
        let (fw, fh, rahmen) = m.resolve(a);
        let img = read_png(&folder.join(&a.file))?;
        let (Ok(fw), Ok(fh), Ok(frames)) = (u32::try_from(fw), u32::try_from(fh), u32::try_from(a.frames)) else {
            return Err(ArtError::Broken(format!(
                "{}: negative Maße",
                folder.join(&a.file).display()
            )));
        };
        if fw == 0 || fh == 0 || img.width < fw * frames || img.height < fh {
            return Err(ArtError::Broken(format!(
                "{}: Sheet {}x{} zu klein für {} Frames à {fw}x{fh}",
                folder.join(&a.file).display(),
                img.width,
                img.height,
                a.frames
            )));
        }
        let sheet = asset.sheets.len();
        let anker = [-rahmen[0] / rahmen[2], -rahmen[1] / rahmen[3]];
        asset.animationen.insert(
            name.clone(),
            AnimRange {
                start: asset.frames.len(),
                frames: frames as usize,
                fps: a.fps,
                rahmen,
            },
        );
        asset.frames.extend((0..frames).map(|i| Frame {
            sheet,
            rect: [i * fw, 0, fw, fh],
            anker,
        }));
        asset.sheets.push(img);
    }
    Ok(asset)
}

fn read_png(path: &Path) -> Result<Image, ArtError> {
    let file = std::fs::File::open(path).map_err(|e| {
        ArtError::Missing(format!(
            "Sheet {} ({e}); erst `task gen:paper` ausführen",
            path.display()
        ))
    })?;
    let broken = |e: png::DecodingError| ArtError::Broken(format!("{}: PNG kaputt ({e})", path.display()));
    let mut decoder = png::Decoder::new(std::io::BufReader::new(file));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info().map_err(broken)?;
    let mut buf = vec![
        0;
        reader
            .output_buffer_size()
            .ok_or_else(|| ArtError::Broken(format!("{}: PNG zu groß", path.display())))?
    ];
    let info = reader.next_frame(&mut buf).map_err(broken)?;
    buf.truncate(info.buffer_size());
    let rgba = to_rgba(&buf, info.color_type, path)?;
    Ok(Image {
        width: info.width,
        height: info.height,
        rgba,
    })
}

fn to_rgba(buf: &[u8], color: png::ColorType, path: &Path) -> Result<Vec<u8>, ArtError> {
    use png::ColorType::{Grayscale, GrayscaleAlpha, Rgb, Rgba};
    Ok(match color {
        Rgba => buf.to_vec(),
        Rgb => buf
            .as_chunks::<3>()
            .0
            .iter()
            .flat_map(|p| [p[0], p[1], p[2], 255])
            .collect(),
        Grayscale => buf.iter().flat_map(|&g| [g, g, g, 255]).collect(),
        GrayscaleAlpha => buf
            .as_chunks::<2>()
            .0
            .iter()
            .flat_map(|p| [p[0], p[0], p[0], p[1]])
            .collect(),
        png::ColorType::Indexed => return Err(ArtError::Broken(format!("{}: PNG indiziert", path.display()))),
    })
}
