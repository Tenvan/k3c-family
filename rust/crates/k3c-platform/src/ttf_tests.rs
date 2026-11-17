use std::path::Path;

use super::*;

/// Die Spielschrift aus dem Repo (`public/schrift/k3c.ttf`).
pub fn k3c_ttf() -> Vec<u8> {
    let p = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../public/schrift/k3c.ttf");
    std::fs::read(&p).unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

fn loaded() -> (Fonts, FontId) {
    let mut f = Fonts::default();
    let id = f.load(&k3c_ttf()).unwrap();
    (f, id)
}

#[test]
fn ttf_umlaute_haben_eigene_glyphen() {
    let (f, id) = loaded();
    let font = f.get(id).unwrap();
    for ch in "ÄÖÜäöüß".chars() {
        assert_ne!(font.lookup_glyph_index(ch), 0, "{ch} fällt auf die Ersatz-Glyphe");
    }
}

#[test]
fn ttf_breite_waechst_mit_der_groesse() {
    let (f, id) = loaded();
    let (a, b) = (f.width(id, "Grüße", 16.0), f.width(id, "Grüße", 32.0));
    assert!(a > 0.0 && b > 1.8 * a, "{a} → {b}");
    assert_eq!(f.width(id, "", 32.0), 0.0);
    assert_eq!(f.width(FontId(9), "x", 32.0), 0.0, "unbekannte Schrift");
    assert_eq!(
        f.width(id, "Ab", 20.4),
        f.width(id, "Ab", 20.0),
        "auf ganze Pixel gerundet"
    );
}

#[test]
fn ttf_zweites_zeichnen_legt_keine_glyphe_an() {
    let (f, id) = loaded();
    let mut a = Atlas::default();
    let mut up = Vec::new();
    a.layout(&f, (id, 24.0), "Äpfel über Öl", (10.0, 20.0), &mut up);
    let (glyphs, quads) = (a.glyphs(), a.quads.len());
    assert!(glyphs >= 9 && !up.is_empty(), "{glyphs} Glyphen");
    assert_eq!(quads, 11, "Leerzeichen ohne Quad");
    up.clear();
    a.layout(&f, (id, 24.0), "Äpfel über Öl", (10.0, 20.0), &mut up);
    assert_eq!(a.glyphs(), glyphs);
    assert!(up.is_empty());
    a.layout(&f, (id, 25.0), "Ä", (0.0, 0.0), &mut up);
    assert_eq!(a.glyphs(), glyphs + 1, "andere Größe, neue Glyphe");
}

#[test]
fn ttf_glyphen_stehen_auf_der_grundlinie_und_laufen_nach_rechts() {
    let (f, id) = loaded();
    let mut a = Atlas::default();
    let mut up = Vec::new();
    a.layout(&f, (id, 40.0), "HH", (100.0, 50.0), &mut up);
    let base = 50.0 + f.ascent(id, 40.0);
    let (h1, h2) = (a.quads[0].1, a.quads[1].1);
    assert!(
        (h1.y + h1.h - base).abs() <= 1.0,
        "H endet auf der Grundlinie: {h1:?}, {base}"
    );
    assert!(h1.y > 50.0 - 1.0, "nicht über der Zeile");
    assert!(
        (h2.x - h1.x - f.width(id, "H", 40.0)).abs() <= 1.0,
        "Abstand = Breite, auf ganze Pixel gerundet"
    );
    let u = &up[0];
    assert_eq!(u.rgba.len() as u32, u.w * u.h * 4);
}

#[test]
fn ttf_voller_atlas_beginnt_von_vorn() {
    let mut a = Atlas::default();
    assert_eq!(a.place(ATLAS - 1, 10), Some((0, 0)));
    assert_eq!(a.place(10, 10), Some((0, 11)), "neue Zeile");
    assert_eq!(a.place(10, ATLAS - 5), None, "kein Platz mehr in diesem Bild");
    assert_eq!(a.place(1, 1), None, "bis zum Bildanfang voll");
    a.begin_frame();
    assert_eq!(a.place(10, ATLAS - 5), Some((0, 0)), "am Bildanfang von vorn");
    assert_eq!(a.place(ATLAS, 1), None, "zu groß");
}

#[test]
fn ttf_kaputte_datei_nennt_fehler() {
    assert!(Fonts::default().load(b"keine schrift").is_err());
}

#[test]
fn ttf_frame_und_nullcanvas_messen_gleich() {
    use crate::{Canvas, Frame, NullCanvas};
    let (mut f, mut n) = (Frame::new(), NullCanvas::default());
    let (a, b) = (f.font(&k3c_ttf()).unwrap(), n.font(&k3c_ttf()).unwrap());
    for px in [12.0, 24.5, 48.0] {
        assert_eq!(f.text_width(a, "Größe Ü", px), n.text_width(b, "Größe Ü", px));
    }
    f.begin();
    f.text_font(a, "Öl", 0.0, 0.0, 30.0, [1.0; 4]);
    f.finish();
    assert_eq!(f.instance_count(), 2);
    let (_, ups) = f.take_atlas_updates().unwrap();
    assert_eq!(ups.count(), 2, "zwei neue Glyphen");
    assert_eq!(f.take_pending().count(), 1, "Atlas-Textur");
    f.begin();
    f.text_font(a, "lÖ", 0.0, 0.0, 30.0, [1.0; 4]);
    f.finish();
    assert_eq!(f.take_atlas_updates().unwrap().1.count(), 0, "beide schon im Atlas");
}

#[test]
fn ttf_zu_grosse_glyphe_behaelt_vorschub() {
    let (f, id) = loaded();
    let mut a = Atlas::default();
    let mut up = Vec::new();
    a.layout(&f, (id, 3000.0), "HH", (0.0, 0.0), &mut up);
    assert!(a.quads.is_empty() && up.is_empty(), "passt nicht in den Atlas");
    a.layout(&f, (id, 3000.0), "HH", (0.0, 0.0), &mut up);
    assert_eq!(a.glyphs(), 1, "gemerkt, nicht neu gerastert");
    assert!(f.width(id, "HH", 3000.0) > 1000.0);
}
