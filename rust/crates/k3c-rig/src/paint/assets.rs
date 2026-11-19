//! Port von native/k3c-rig/paint/assets.go: lädt die Bilder der Rig-Figuren (Teil-PNGs, Bänder und Elemente aus
//! native/k3c-rig/assets) und lädt sie als Texturen über `platform::Canvas` hoch.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use k3c_platform::{Canvas, TexId};

use crate::rig::{Element, Fehler, Rig, load_elements};

/// ImageRef ist eine hochgeladene Textur samt Größe in Pixeln (Go: `*ebiten.Image`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageRef {
    pub tex: TexId,
    pub w: u32,
    pub h: u32,
}

/// Skin sind die Bilder eines Rigs in einer Variante: je Teil und je Band (None = ohne Bild).
#[derive(Debug, Clone)]
pub struct Skin {
    pub parts: Vec<Option<ImageRef>>,
    pub bands: Vec<Option<ImageRef>>,
    /// Teil, in dessen Rahmen die Stützpunkte des Bands liegen (-1 = Welt).
    pub band_part: Vec<i64>,
}

/// ElementImage ist ein optionales Element mit seinem Bild.
#[derive(Debug, Clone)]
pub struct ElementImage {
    pub element: Element,
    pub img: ImageRef,
}

/// Assets hält Rigs, Elemente und Bilder; jedes Bild wird nur einmal geladen. Bildschlüssel sind Pfade relativ zum
/// Assets-Ordner mit `/`.
#[derive(Debug)]
pub struct Assets {
    pub dir: PathBuf,
    pub images: BTreeMap<String, ImageRef>,
    pub elements: BTreeMap<String, ElementImage>,
    /// Auswahl am Platz `waffe`, "" = leer (zuletzt).
    pub weapons: Vec<String>,
    /// Auswahl am Platz `ruestung`, "" = leer (zuerst).
    pub armors: Vec<String>,
    pub white: ImageRef,
    /// Summe aller Bilder als RGBA8.
    pub bytes: i64,
    rigs: BTreeMap<String, Rc<Rig>>,
    skins: BTreeMap<String, Rc<Skin>>,
}

impl Assets {
    /// `load` liest die Elemente samt Bildern; Rigs lädt `rig` beim ersten Zugriff. Fehlt etwas, kommt eine klare
    /// Meldung.
    pub fn load(dir: &Path, canvas: &mut dyn Canvas) -> Result<Assets, Fehler> {
        let white = canvas.texture(&[255; 3 * 3 * 4], 3, 3);
        let mut a = Assets {
            dir: dir.to_path_buf(),
            images: BTreeMap::new(),
            elements: BTreeMap::new(),
            weapons: Vec::new(),
            armors: vec![String::new()],
            white: ImageRef { tex: white, w: 3, h: 3 },
            bytes: 0,
            rigs: BTreeMap::new(),
            skins: BTreeMap::new(),
        };
        for e in load_elements(&dir.join("elemente"))? {
            let img = a.image(canvas, &format!("elemente/{}", e.image.file))?;
            match e.slot.as_str() {
                "waffe" => a.weapons.push(e.id.clone()),
                "ruestung" => a.armors.push(e.id.clone()),
                _ => {}
            }
            a.elements.insert(e.id.clone(), ElementImage { element: e, img });
        }
        a.weapons.push(String::new());
        Ok(a)
    }

    /// `rig` lädt das Rig `<gruppe>/<id>` einmal (Ordner relativ zum Assets-Ordner).
    pub fn rig(&mut self, rel: &str) -> Result<Rc<Rig>, Fehler> {
        if let Some(r) = self.rigs.get(rel) {
            return Ok(Rc::clone(r));
        }
        let r = Rc::new(Rig::load(&self.dir.join(rel))?);
        self.rigs.insert(rel.to_string(), Rc::clone(&r));
        Ok(r)
    }

    /// `image` lädt ein PNG einmal (Pfad relativ zum Assets-Ordner) als vormultipliziertes RGBA.
    pub fn image(&mut self, canvas: &mut dyn Canvas, rel: &str) -> Result<ImageRef, Fehler> {
        if let Some(img) = self.images.get(rel) {
            return Ok(*img);
        }
        let path = self.dir.join(rel);
        let (pix, w, h) = read_png(&path).map_err(|e| {
            Fehler::Fehlt(format!(
                "Teil-Bild fehlt oder ist kaputt: {} (erst `task gen:rig` ausführen): {e}",
                path.display()
            ))
        })?;
        let img = ImageRef {
            tex: canvas.texture(&pix, w, h),
            w,
            h,
        };
        self.images.insert(rel.to_string(), img);
        self.bytes += pix.len() as i64;
        Ok(img)
    }

    /// `skin` stellt die Bilder eines Rigs in einer Variante zusammen (Varianten ersetzen einzelne Bilder). Schlüssel
    /// und Bildpfade gehen über den Ordner des Rigs (<gruppe>/<id>), weil dieselbe id in zwei Gruppen vorkommt.
    pub fn skin(&mut self, canvas: &mut dyn Canvas, r: &Rig, variant: &str) -> Result<Rc<Skin>, Fehler> {
        let dir = Path::new(&r.dir)
            .strip_prefix(&self.dir)
            .map_err(|e| Fehler::Rig(e.to_string()))?
            .to_string_lossy()
            .replace('\\', "/");
        let key = format!("{dir}@{variant}");
        if let Some(s) = self.skins.get(&key) {
            return Ok(Rc::clone(s));
        }
        let mut parts = Vec::with_capacity(r.parts.len());
        for p in &r.parts {
            parts.push(if p.image.file.is_empty() {
                None
            } else {
                Some(self.image(
                    canvas,
                    &format!("{dir}/{}", r.image_file(&p.id, &p.image.file, variant)),
                )?)
            });
        }
        let mut bands = Vec::with_capacity(r.bands.len());
        let mut band_part = Vec::with_capacity(r.bands.len());
        for b in &r.bands {
            let file = r.image_file(&b.id, &b.image.file, variant);
            bands.push(Some(self.image(canvas, &format!("{dir}/{file}"))?));
            band_part.push(r.index_of(&b.part));
        }
        let s = Rc::new(Skin {
            parts,
            bands,
            band_part,
        });
        self.skins.insert(key, Rc::clone(&s));
        Ok(s)
    }
}

/// `read_png` liest ein PNG als vormultipliziertes RGBA8 (das Format, das `Canvas::texture` erwartet): Pixel, Breite,
/// Höhe. Der Fehlertext ist der des Dekoders bzw. des Dateisystems.
pub fn read_png(path: &Path) -> Result<(Vec<u8>, u32, u32), String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut decoder = png::Decoder::new(std::io::BufReader::new(file));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info().map_err(|e| e.to_string())?;
    let mut buf = vec![0; reader.output_buffer_size().ok_or("PNG zu groß")?];
    let info = reader.next_frame(&mut buf).map_err(|e| e.to_string())?;
    buf.truncate(info.buffer_size());
    Ok((premultiply(&buf, info.color_type)?, info.width, info.height))
}

/// Rechnet wie `draw.Draw(…, draw.Src)` von NRGBA nach RGBA: `c * (a·0x101) / 0xff >> 8`.
fn premul(c: u8, a: u8) -> u8 {
    let sa = u32::from(a) * 0x101;
    ((u32::from(c) * sa / 0xff) >> 8) as u8
}

fn premultiply(buf: &[u8], color: png::ColorType) -> Result<Vec<u8>, String> {
    use png::ColorType::{Grayscale, GrayscaleAlpha, Indexed, Rgb, Rgba};
    Ok(match color {
        Rgba => buf
            .as_chunks::<4>()
            .0
            .iter()
            .flat_map(|p| [premul(p[0], p[3]), premul(p[1], p[3]), premul(p[2], p[3]), p[3]])
            .collect(),
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
            .flat_map(|p| {
                let g = premul(p[0], p[1]);
                [g, g, g, p[1]]
            })
            .collect(),
        Indexed => return Err("PNG indiziert".to_string()),
    })
}
