//! Radar mit Biome-Zeile (ITEM_RADAR, HB1.3): Go hat dafür keinen Platz, die Zeile steht seit HB1.3 in jeder Stufe
//! (`733534e7c`). Lage am gerenderten 1280×720-Bild abgelesen (je Hälfte ein Streifen); nur für `pad-tastatur`.

/// Rechtecke (x0, y0, x1, y1) in Bildpunkten, die der Standbild-Vergleich auslässt.
pub fn radar_masks(name: &str) -> Vec<(f64, f64, f64, f64)> {
    if name != "pad-tastatur" {
        return Vec::new();
    }
    vec![(830.0, 300.0, 1270.0, 350.0), (830.0, 660.0, 1270.0, 710.0)]
}
