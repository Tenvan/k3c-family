//! Port von `native/k3c-game/game/overlay_test.go`.

use std::cell::Cell;
use std::path::Path;
use std::rc::Rc;

use k3c_platform::NullCanvas;

use super::*;
use crate::art::load_font;
use crate::view::cell::stats_text_size_lines;
use crate::view::settings::Settings;

/// AC-07 (B-816): Mit Leistungsanzeige stehen Bildrate und Rechenzeit (Update, Draw) im Text; im Aufnahme-Modus feste
/// Striche statt Zeiten, damit das Bild vom Takt abhängt.
#[test]
fn stats_text_zeilen() {
    let (l1, l2) = stats_text(false, true, 59.6, 7, 0.42, 3.25);
    assert!(
        l1.contains("60 FPS") && l2.starts_with("U 0.4 D 3."),
        "Fenster: {l1:?} {l2:?}"
    );
    let (l1, l2) = stats_text(false, false, 60.0, 7, 1.0, 2.0);
    assert!(l2.is_empty() && l1.contains("60 FPS"), "aus: {l1:?} {l2:?}");
    let (r1, r2) = stats_text(true, true, 12.0, 345, 1.0, 2.0);
    assert!(
        r1.contains("345") && !r1.contains("FPS") && r2 == "U --- D ---",
        "Aufnahme: {r1:?} {r2:?}"
    );
    assert_eq!(
        stats_text(true, true, 99.0, 345, 5.0, 5.0),
        (r1.clone(), r2.clone()),
        "Aufnahme hängt von Zeiten ab"
    );
    let (w, _, _) = stats_text_size_lines(2);
    for s in [l1.as_str(), r1.as_str(), r2.as_str(), "U12.3 D45.6"] {
        assert!(s.len() as i64 * 6 + 4 <= w, "{s:?} passt nicht in {w} px");
    }
}

#[test]
fn avg_mittel() {
    let mut a = Avg::default();
    assert_eq!(a.take(), 0.0, "leer");
    a.add(0.002);
    a.add(0.004);
    let got = a.take();
    assert!((2.999..=3.001).contains(&got), "Mittel {got}");
    assert!(a.n == 0 && a.take() == 0.0, "nicht zurückgesetzt");
}

fn frame(stats: bool, recording: bool) -> Frame {
    Frame {
        settings: Settings {
            stats,
            ..Settings::default()
        },
        recording,
        ..Frame::default()
    }
}

/// AC-06 (B-816): Ohne Leistungsanzeige (und im Aufnahme-Modus) fragt die Schleife die Wanduhr nie ab; mit Anzeige misst
/// das Update zweimal je Takt. Go prüft das über `Game.advanceTimed` (RU26.4); hier über `time_update`, das diese Schleife
/// aufruft.
#[test]
fn no_clock_when_stats_off() {
    let calls = Rc::new(Cell::new(0));
    let mut o = StatsOverlay::new(true, None);
    let c = Rc::clone(&calls);
    o.now = Box::new(move || {
        c.set(c.get() + 1);
        f64::from(c.get()) * 0.001
    });
    let mut ran = 0;
    o.time_update(&frame(false, false), || ran += 1);
    assert!(
        calls.get() == 0 && o.upd.n == 0,
        "Anzeige aus: {} Zeitnahmen",
        calls.get()
    );
    o.time_update(&frame(true, true), || ran += 1);
    assert_eq!(calls.get(), 0, "Aufnahme-Modus");
    o.measuring = true;
    o.time_update(&frame(true, false), || ran += 1);
    assert_eq!(calls.get(), 0, "Messmodus");
    o.measuring = false;
    o.time_update(&frame(true, false), || ran += 1);
    assert!(
        calls.get() == 2 && o.upd.n == 1,
        "Anzeige an: {} Zeitnahmen, {} Werte",
        calls.get(),
        o.upd.n
    );
    assert_eq!(ran, 4, "das Update läuft immer");
}

/// Takt- und Leistungsanzeige mit der Schrift des Spiels (NG13): eine und zwei Zeilen (F2) liegen samt Fläche ganz in
/// `stats_rect`, die das HUD frei hält; der Text steht rechtsbündig.
#[test]
fn stats_box_in_stats_rect() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../public/schrift/k3c.ttf");
    let Ok(font) = load_font(&path, true) else {
        eprintln!("Schrift fehlt: {}", path.display());
        return;
    };
    let texts = [["T 9999999", "U --- D ---"], ["9999 FPS", "U99.9 D99.9"]];
    for (w, h) in [(1280, 720), (1920, 1080), (800, 600)] {
        let r = RectF::of(stats_rect(rect(0, 0, w, h)));
        let px = stats_px(r);
        for tx in texts {
            let (w0, th) = font.measure(tx[0], px);
            let tw = [w0, font.measure(tx[1], px).0];
            for lines in 1..=2 {
                let b = stats_box(r, tw, th, lines, px);
                assert!(
                    b.x >= r.x && b.y >= r.y && b.x + b.w <= r.x + r.w + 1e-9 && b.y + b.h <= r.y + r.h + 1e-9,
                    "{w}x{h} {tx:?}, {lines} Zeilen: Fläche {b:?} nicht in StatsRect {r:?}"
                );
                assert!((b.x + b.w - (r.x + r.w)).abs() < 1e-9, "rechtsbündig");
            }
        }
    }
}

/// Zeichnen ohne Schrift: Fläche, dann eine bzw. zwei Textzeilen; der Text entsteht höchstens viermal je Sekunde
/// Spielzeit neu (FPS) bzw. je Takt (Aufnahme-Modus).
#[test]
fn draw_pixel_und_intervall() {
    let mut o = StatsOverlay::new(true, None);
    let mut c = NullCanvas::default();
    let mut f = frame(false, false);
    o.draw(&mut c, &f, 60.0, None);
    assert_eq!((c.rects, c.texts), (1, 1));
    assert!(o.text1[0].contains("60 FPS"));
    f.clock = 0.1;
    o.draw(&mut c, &f, 30.0, None);
    assert!(
        o.text1[0].contains("60 FPS"),
        "vor Ablauf des Intervalls bleibt der Text: {:?}",
        o.text1
    );
    f.clock = 0.25;
    o.draw(&mut c, &f, 30.0, None);
    assert!(o.text1[0].contains("30 FPS"), "{:?}", o.text1);
    f.settings.stats = true;
    let mut c = NullCanvas::default();
    o.draw(&mut c, &f, 30.0, None);
    assert_eq!((c.rects, c.texts), (1, 2), "zwei Zeilen, Text sofort neu beim Wechsel");
    assert!(o.text1[1].starts_with('U'));

    let mut rec = StatsOverlay::new(false, None);
    let mut f = frame(false, true);
    f.tick = 5;
    rec.draw(&mut c, &f, 60.0, None);
    f.tick = 6;
    rec.draw(&mut c, &f, 60.0, None);
    assert!(
        rec.text1[0].contains('6') && !rec.text1[0].contains("FPS"),
        "{:?}",
        rec.text1
    );
}

/// Mit der Schrift des Spiels: Fläche, dann Text je Zeile in dieser Reihenfolge.
#[test]
fn draw_font_zeichnet_flaeche_vor_text() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../public/schrift/k3c.ttf");
    let Ok(mut font) = load_font(&path, true) else {
        eprintln!("Schrift fehlt: {}", path.display());
        return;
    };
    let mut o = StatsOverlay::new(true, Some(&font));
    let mut c = NullCanvas::default();
    let f = frame(true, false);
    o.draw(&mut c, &f, 60.0, Some(&mut font));
    assert_eq!(c.rects, 1, "eine Fläche");
    assert_eq!(c.texts, 2, "zwei Zeilen");
}

/// B-1180: Die Fläche der Pixel-Schrift passt zur längsten Zeile und bleibt kleiner als vorher (Maßstab 2, nicht 5).
#[test]
fn pixel_flaeche_passt_zur_laengsten_zeile() {
    let k = PIXEL_SCALE;
    assert_eq!(k, 2, "kleine Anzeige");
    assert!((pixel_width(8) - 92.0).abs() < 1e-3, "{{fps:4.0}} FPS");
    assert!(pixel_width(12) > pixel_width(8));
    assert!(pixel_width(12) * k as f32 <= 300.0, "bleibt klein");
}

/// B-1180: Die FPS werden gemessen, sobald die Anzeige sie zeigt, auch ohne die zweite Zeile.
#[test]
fn fps_werden_ohne_zweite_zeile_gemessen() {
    let mut s = StatsOverlay::new(true, None);
    assert!(s.shows_fps());
    s.measuring = true;
    assert!(!s.shows_fps(), "beim Messen keine eigene Zeitnahme");
    assert!(
        !StatsOverlay::new(false, None).shows_fps(),
        "Aufnahme-Modus zeigt den Takt"
    );
}
