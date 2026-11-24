//! Trophäe samt Fahne als gezeichnete Form (GS3.8): Andenken einer abgeschlossenen Insel auf der Seekarte und in der
//! Spielstand-Vorschau. Die Assets der Trophäe liefert GS3.2 (offen); bis dahin gilt diese Form. Die Teile stehen hier
//! einmal als Zahlen, jedes Zeichnen (HUD, Menü) setzt sie mit seinen eigenen Mitteln um.

/// Gold der Trophäe.
pub const GOLD: u32 = 0xd4a72c;
/// Rot der Fahne.
pub const RED: u32 = 0xc0392b;

/// Ein Teil der Form: gefülltes Rechteck (x, y, w, h), Scheibe oder Ring (Mitte x, y, Radius), je mit Farbe.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Part {
    Rect(f64, f64, f64, f64, u32),
    Disc(f64, f64, f64, u32),
    Ring(f64, f64, f64, u32),
}

/// parts ruft `put` für jedes Teil der Trophäe mit Mitte `cx`, Standlinie `by` und Höhe `h` auf, mit Fahne rechts daneben,
/// wenn `flag` gesetzt ist (die Fahne ragt bis `h` über die Standlinie). Ohne Allokation.
pub fn parts(cx: f64, by: f64, h: f64, flag: bool, put: &mut impl FnMut(Part)) {
    put(Part::Rect(cx - 0.25 * h, by - 0.1 * h, 0.5 * h, 0.1 * h, GOLD)); // Sockel
    put(Part::Rect(cx - 0.05 * h, by - 0.3 * h, 0.1 * h, 0.2 * h, GOLD)); // Schaft
    put(Part::Disc(cx, by - 0.45 * h, 0.22 * h, GOLD)); // Kelchboden
    put(Part::Rect(cx - 0.22 * h, by - 0.7 * h, 0.44 * h, 0.25 * h, GOLD)); // Kelch
    put(Part::Ring(cx - 0.3 * h, by - 0.55 * h, 0.1 * h, GOLD)); // Henkel links
    put(Part::Ring(cx + 0.3 * h, by - 0.55 * h, 0.1 * h, GOLD)); // Henkel rechts
    if flag {
        let px = cx + 0.45 * h;
        put(Part::Rect(px, by - h, 0.04 * h, h, GOLD)); // Fahnenstange
        for (i, w) in [0.3, 0.2, 0.1].into_iter().enumerate() {
            put(Part::Rect(
                px + 0.04 * h,
                by - h + 0.07 * h * i as f64,
                w * h,
                0.07 * h,
                RED,
            )); // Wimpel
        }
    }
}
