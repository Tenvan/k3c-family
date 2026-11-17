//! PNG lesen wie `paint.ReadPNG` (`native/k3c-rig/paint/assets.go`): dekodieren und mit `draw.Draw` in ein
//! vormultipliziertes `image.RGBA` umrechnen.

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
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut decoder = png::Decoder::new(std::io::BufReader::new(file));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info().map_err(|e| e.to_string())?;
    let size = reader.output_buffer_size().ok_or("PNG zu groß")?;
    let mut buf = vec![0; size];
    let info = reader.next_frame(&mut buf).map_err(|e| e.to_string())?;
    buf.truncate(info.buffer_size());
    Ok(Rgba {
        width: info.width,
        height: info.height,
        pix: premultiply(&buf, info.color_type)?,
    })
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

#[cfg(test)]
mod tests {
    use super::premul;

    #[test]
    fn vormultiplizieren_wie_go() {
        // Werte aus image/draw (NRGBA → RGBA, draw.Src)
        assert_eq!(premul(200, 128), 100);
        assert_eq!(premul(255, 255), 255);
        assert_eq!(premul(10, 0), 0);
        assert_eq!(premul(1, 255), 1);
    }
}
