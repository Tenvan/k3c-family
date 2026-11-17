//! Texturbudget der Sheets; Vorlage `native/k3c-game/art/budget.go`.

use std::fmt;

/// Sheets liegen als RGBA8 (vormultipliziert) auf der GPU.
const BYTES_PER_PIXEL: i64 = 4;

/// Budget ist das Texturbudget: Summe aus Breite × Höhe × 4 Byte je Bild.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Budget {
    pub images: i64,
    pub bytes: i64,
}

impl Budget {
    /// Zählt ein Bild der Größe w × h dazu.
    pub fn add(&mut self, w: i64, h: i64) {
        self.images += 1;
        self.bytes += w * h * BYTES_PER_PIXEL;
    }

    /// Zählt bereits berechnete Bytes dazu (z. B. die Teil-Bilder der Rigs).
    pub fn add_bytes(&mut self, n: i64) {
        self.bytes += n;
    }

    /// Liefert die Summe in Mebibyte.
    pub fn mib(&self) -> f64 {
        self.bytes as f64 / f64::from(1u32 << 20)
    }
}

impl fmt::Display for Budget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Bilder, {:.1} MiB RGBA", self.images, self.mib())
    }
}
