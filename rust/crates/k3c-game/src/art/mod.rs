//! Papierschnitt-Grafik laden (RU2.2): Index, Manifest, PNG-Sheets, Zuordnung `data/paper.json`.
//! Headless, ohne Grafik-Kontext; das Hochladen auf die GPU übernimmt RU2.3 über `Canvas::texture`.
//! Vorlage: `native/k3c-game/art/`.

mod manifest;
mod paper;
#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

pub use manifest::{Anim, Manifest, group_assets, read_manifest};
pub use paper::paper_assets;

/// Fehler beim Laden; die Meldung nennt immer den Pfad.
#[derive(Debug)]
pub enum ArtError {
    Missing(String),
    Broken(String),
}

impl fmt::Display for ArtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(m) => write!(f, "fehlende Grafik: {m}"),
            Self::Broken(m) => write!(f, "{m}"),
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
    for (name, a) in &m.animationen {
        let (fw, fh, rahmen) = m.resolve(a);
        let img = read_png(&folder.join(&a.datei))?;
        if fw == 0 || fh == 0 || img.width < fw * a.frames || img.height < fh {
            return Err(ArtError::Broken(format!(
                "{}: Sheet {}x{} zu klein für {} Frames à {fw}x{fh}",
                folder.join(&a.datei).display(),
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
                frames: a.frames as usize,
                fps: a.fps,
                rahmen,
            },
        );
        asset.frames.extend((0..a.frames).map(|i| Frame {
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
