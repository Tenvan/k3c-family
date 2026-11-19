//! Bild-Hilfen des Aufnahme-Modus (Go: `Shrink`, `average`, `Strip` aus `shots.go`) auf `still::Image` (RGBA8, Zeilen
//! oben nach unten). Abweichung von der Session-Datei: Statt einer eigenen Struktur `Rgba` dient das vorhandene
//! `still::Image` (gleiche Felder), damit das Bild des Rasterers ohne Kopie in den Streifen geht.

use crate::still::Image;

/// Lücken im Streifen, auffällig (Go: `stripBack`).
const STRIP_BACK: [u8; 4] = [255, 0, 255, 255];

/// Verkleinert `src` um den ganzzahligen Faktor `k` (Mittel über k×k Bildpunkte; ein Rest am Rand fällt weg).
pub fn shrink(src: &Image, k: usize) -> Image {
    if k <= 1 {
        return src.clone();
    }
    let (w, h) = (src.width as usize / k, src.height as usize / k);
    let mut pix = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            pix.extend_from_slice(&average(src, x * k, y * k, k));
        }
    }
    Image {
        width: w as u32,
        height: h as u32,
        pix,
    }
}

/// Das Mittel des k×k-Blocks ab (x0, y0) (ganzzahlig, abgerundet wie in Go).
fn average(src: &Image, x0: usize, y0: usize, k: usize) -> [u8; 4] {
    let mut sum = [0_usize; 4];
    let stride = src.width as usize * 4;
    for y in y0..y0 + k {
        let row = &src.pix[y * stride + x0 * 4..y * stride + (x0 + k) * 4];
        for px in row.as_chunks::<4>().0 {
            for (s, v) in sum.iter_mut().zip(px) {
                *s += usize::from(*v);
            }
        }
    }
    let n = k * k;
    [0, 1, 2, 3].map(|i| u8::try_from(sum[i] / n).unwrap_or(u8::MAX))
}

/// Legt die Bilder nebeneinander, oben bündig, mit `gap` px Abstand; die Lücken sind Magenta.
pub fn strip(imgs: &[Image], gap: usize) -> Image {
    let mut w = 0;
    let mut h = 0;
    for (i, m) in imgs.iter().enumerate() {
        if i > 0 {
            w += gap;
        }
        w += m.width as usize;
        h = h.max(m.height as usize);
    }
    let (w, h) = (w.max(1), h.max(1));
    let mut pix: Vec<u8> = STRIP_BACK.iter().copied().cycle().take(w * h * 4).collect();
    let mut x = 0;
    for m in imgs {
        let mw = m.width as usize;
        for y in 0..m.height as usize {
            let dst = (y * w + x) * 4;
            pix[dst..dst + mw * 4].copy_from_slice(&m.pix[y * mw * 4..(y + 1) * mw * 4]);
        }
        x += mw + gap;
    }
    Image {
        width: w as u32,
        height: h as u32,
        pix,
    }
}
