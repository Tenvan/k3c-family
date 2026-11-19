//! Tests von `mirror_fragment` (RU55.3): Lage, Helligkeit, Lichtquellen, Wasserfarbe, Packung.

use super::*;

const WATER: [f32; 3] = [0.2, 0.4, 0.6];

/// Streifen 100 … 200 über der Gehlinie 200, Spiegel ab Achse 210 (Ufer 200 … 210), Rechteck 0 … 400 × 210 … 400.
fn uni() -> MirrorUniforms {
    MirrorUniforms {
        src_top: 100.0,
        ground: 200.0,
        axis: 210.0,
        squash: 1.0,
        brightness: 0.65,
        water: WATER,
        glint: [1.0; 3],
        ..MirrorUniforms::default()
    }
}

const RECT: [f32; 4] = [0.0, 210.0, 400.0, 190.0];

fn red(_x: f32, _y: f32) -> [f32; 3] {
    [1.0, 0.0, 0.0]
}

#[test]
fn objekt_ueber_der_gehlinie_erscheint_dunkler_unter_der_achse() {
    // Ein Pixel unter der Achse zeigt das Bild direkt über der Gehlinie, mit 65 % Helligkeit.
    let c = mirror_fragment(&uni(), RECT, [50.0, 211.0], &red);
    assert!((c[0] - 0.65).abs() < 1e-4 && c[1] < 1e-4 && c[2] < 1e-4, "{c:?}");
    assert_eq!(c[3], 1.0);
}

#[test]
fn spiegel_liest_gespiegelt_und_gestaucht() {
    // Die Quelle trägt ihre Höhe in der Farbe: gespiegelt ist die Farbe 20 px unter der Achse die von 20 px/squash über der Gehlinie.
    let by_y = |_x: f32, y: f32| [y / 400.0, 0.0, 0.0];
    let mut u = uni();
    u.brightness = 1.0;
    let c = mirror_fragment(&u, RECT, [50.0, 230.0], &by_y);
    assert!((c[0] - (200.0 - 20.0 - 0.0) / 400.0).abs() < 1e-3, "{c:?}");
    u.squash = 0.5;
    let c = mirror_fragment(&u, RECT, [50.0, 230.0], &by_y);
    assert!((c[0] - (200.0 - 40.0) / 400.0).abs() < 1e-3, "{c:?}");
}

#[test]
fn ueber_dem_streifen_ist_wasserfarbe() {
    // 150 px unter der Achse zeigt bei squash 1 die Höhe 50 über der Gehlinie = über src_top (100): Wasser.
    let c = mirror_fragment(&uni(), RECT, [50.0, 360.0], &red);
    assert!(c[2] > 0.5 && c[0] < 0.4, "{c:?}");
}

#[test]
fn lichtquelle_bleibt_hell_nachts_nicht_am_tag() {
    let lamp = |_: f32, _: f32| [1.0, 0.9, 0.6];
    let mut u = uni();
    let day = mirror_fragment(&u, RECT, [50.0, 211.0], &lamp);
    u.emissive = 1.0;
    let night = mirror_fragment(&u, RECT, [50.0, 211.0], &lamp);
    assert!(day[0] < 0.7, "am Tag dunkler: {day:?}");
    assert!(night[0] > 0.95 && night[1] > 0.85, "nachts hell: {night:?}");
    // Dunkles Bild bleibt auch nachts dunkel (Licht ist nur, was hell ist).
    let dark = mirror_fragment(&u, RECT, [50.0, 211.0], &|_, _| [0.2, 0.2, 0.2]);
    assert!(dark[0] < 0.2, "{dark:?}");
}

#[test]
fn glanzstriche_sind_hell_und_waagerecht() {
    let mut u = uni();
    u.glint_mix = 1.0;
    u.phase_glint = std::f32::consts::FRAC_PI_2;
    let mut hits = 0;
    for y in 0..190 {
        for x in 0..400 {
            let c = mirror_fragment(&u, RECT, [x as f32 + 0.5, 210.0 + y as f32 + 0.5], &red);
            hits += usize::from(c == [1.0, 1.0, 1.0, 1.0]);
        }
    }
    assert!(hits > 100 && hits < 190 * 400 / 4, "Glanz-Pixel: {hits}");
    u.glint_mix = 0.0;
    assert_ne!(mirror_fragment(&u, RECT, [10.5, 215.5], &red), [1.0; 4]);
}

#[test]
fn packung_hat_die_feste_groesse_und_reihenfolge() {
    let mut u = uni();
    u.emissive = 0.5;
    let call = crate::MirrorCall {
        dst: crate::Rect::new(1.0, 210.0, 400.0, 190.0),
        u,
    };
    let mut out = [0u8; MIRROR_BYTES];
    pack_mirror(&call, &mut out);
    let f = |i: usize| f32::from_ne_bytes(out[4 * i..4 * i + 4].try_into().unwrap_or_default());
    assert_eq!([f(0), f(1), f(2), f(3)], [1.0, 210.0, 400.0, 190.0]);
    assert_eq!([f(8), f(9), f(10), f(11)], [210.0, 200.0, 1.0, 0.65]);
    assert_eq!(f(15), 0.5);
    assert_eq!([f(20), f(21), f(22)], WATER);
    assert_eq!(MIRROR_BYTES, 28 * 4);
}
