//! Maße und Farben des Fensters, ohne Win32 (überall testbar). Alle Maße sind dip (1/96 Zoll).

/// Startgröße des Client-Bereichs in dip.
pub const START_SIZE: (i32, i32) = (520, 720);
/// Mindestgröße des Client-Bereichs in dip.
pub const MIN_SIZE: (i32, i32) = (420, 480);
/// Raster in dip.
pub const GRID: i32 = 8;
/// Titel des Fensters.
pub const TITLE: &str = "K3C Cheat-Fenster";

/// Rgb ist eine Farbe aus drei Bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    /// colorref liefert die Win32-`COLORREF`-Bytefolge `0x00BBGGRR`.
    pub const fn colorref(self) -> u32 {
        (self.0 as u32) | ((self.1 as u32) << 8) | ((self.2 as u32) << 16)
    }
}

/// Hintergrund `#1E1F22`.
pub const BG: Rgb = Rgb(0x1E, 0x1F, 0x22);
/// Text `#DCDDDE`.
pub const TEXT: Rgb = Rgb(0xDC, 0xDD, 0xDE);
/// Akzent `#4F8CFF`.
pub const ACCENT: Rgb = Rgb(0x4F, 0x8C, 0xFF);

/// scale rechnet dip bei der Auflösung dpi (96 = 100 %) in Pixel um, kaufmännisch gerundet.
pub fn scale(dip: i32, dpi: u32) -> i32 {
    let dpi = i64::from(dpi.max(1));
    let n = i64::from(dip) * dpi;
    let r = if n >= 0 { (n + 48) / 96 } else { (n - 48) / 96 };
    i32::try_from(r).unwrap_or(if dip < 0 { i32::MIN } else { i32::MAX })
}

/// unscale rechnet Pixel bei der Auflösung dpi zurück in dip (gerundet).
pub fn unscale(px: i32, dpi: u32) -> i32 {
    let dpi = i64::from(dpi.max(1));
    i32::try_from((i64::from(px) * 96 + dpi / 2) / dpi).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unscale_ist_die_umkehrung() {
        assert_eq!(unscale(780, 144), 520);
        assert_eq!(unscale(scale(333, 120), 120), 333);
        assert_eq!(unscale(5, 0), 480);
    }

    #[test]
    fn scale_rundet_und_skaliert() {
        assert_eq!(scale(520, 96), 520);
        assert_eq!(scale(520, 144), 780);
        assert_eq!(scale(520, 120), 650);
        assert_eq!(scale(1, 144), 2);
        assert_eq!(scale(-1, 144), -2);
        assert_eq!(scale(8, 0), 0);
        assert_eq!(scale(i32::MAX, u32::MAX), i32::MAX);
    }

    #[test]
    fn farben_als_colorref() {
        assert_eq!(BG.colorref(), 0x0022_1F1E);
        assert_eq!(ACCENT.colorref(), 0x00FF_8C4F);
        assert_eq!(TEXT, Rgb(220, 221, 222));
    }

    #[test]
    fn start_ist_nicht_kleiner_als_minimum() {
        assert!(START_SIZE.0 >= MIN_SIZE.0 && START_SIZE.1 >= MIN_SIZE.1);
        assert_eq!(GRID, 8);
    }
}
