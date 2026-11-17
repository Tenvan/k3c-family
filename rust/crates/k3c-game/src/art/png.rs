//! PNG lesen für die Bibliothek: dünne Hülle um `k3c_rig::paint::read_png` (dieselbe Stelle für Sheets und
//! Rig-Teile, vorher doppelt).

use std::path::Path;

/// Vormultipliziertes RGBA8-Bild, Zeilen oben nach unten.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rgba {
    pub width: u32,
    pub height: u32,
    pub pix: Vec<u8>,
}

/// Liest das PNG unter path; der Fehlertext ist der des Dekoders bzw. des Dateisystems.
pub fn read_png(path: &Path) -> Result<Rgba, String> {
    let (pix, width, height) = k3c_rig::paint::read_png(path)?;
    Ok(Rgba { width, height, pix })
}

#[cfg(test)]
mod tests {
    use super::read_png;
    use crate::art::testutil::temp_dir;

    #[test]
    fn vormultiplizieren_wie_go() {
        // Werte aus image/draw (NRGBA → RGBA, draw.Src): (200,128) → 100, (255,255) → 255, (10,0) → 0
        let path = temp_dir("png").join("a.png");
        let file = std::fs::File::create(&path).unwrap();
        let mut enc = png::Encoder::new(file, 3, 1);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        let mut w = enc.write_header().unwrap();
        w.write_image_data(&[200, 200, 200, 128, 255, 255, 255, 255, 10, 10, 10, 0])
            .unwrap();
        w.finish().unwrap();
        let img = read_png(&path).unwrap();
        assert_eq!((img.width, img.height), (3, 1));
        assert_eq!(img.pix, [100, 100, 100, 128, 255, 255, 255, 255, 0, 0, 0, 0]);
    }
}
