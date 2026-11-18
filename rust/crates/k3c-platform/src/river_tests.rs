//! Tests von `river_fragment`. Die festen Werte sind von Hand aus `river.kage` gerechnet (Phase 0, Reihe 0, `p.x` nahe 0,
//! sodass die Sinus-Terme verschwinden oder ihr Zweig eindeutig ist); dazu ein Vergleich mit einer unabhängig
//! geschriebenen `f64`-Rechnung über alle Reihen.

use super::*;

const BASE: [f32; 3] = [0.0, 0.0, 1.0];
const WAVE: [f32; 3] = [1.0, 0.0, 0.0];
const SHADE: [f32; 3] = [0.0, 1.0, 0.0];
const GLINT: [f32; 3] = [1.0, 1.0, 1.0];

/// Sechs Reihen ab y = 100, je 64 hoch, Kachelbreite 1, erste Kante bei x = 0 (u = p.x, v = p.y - Reihenanfang).
fn uni(phase: [f32; 3]) -> RiverUniforms {
    let mut u = RiverUniforms {
        phase,
        base: BASE,
        wave: WAVE,
        shade: SHADE,
        glint: GLINT,
        ..RiverUniforms::default()
    };
    for i in 0..6 {
        u.rows[4 * i..4 * i + 4].copy_from_slice(&[100.0 + 64.0 * i as f32, 64.0, 1.0, 0.0]);
    }
    u
}

fn near(a: [f32; 4], b: [f32; 4]) {
    for k in 0..4 {
        assert!((a[k] - b[k]).abs() < 1e-3, "{a:?} != {b:?}");
    }
}

fn rgba(c: [f32; 3]) -> [f32; 4] {
    [c[0], c[1], c[2], 1.0]
}

#[test]
fn oberhalb_der_ersten_reihe_ist_transparent() {
    assert_eq!(river_fragment(&uni([0.0; 3]), [0.0, 99.9]), [0.0; 4]);
}

#[test]
fn grund_band_und_schatten_der_ersten_reihe() {
    let u = uni([0.0; 3]);
    // v = 0: Band bei 32 (sin 0), Abstand 32 -> Grund.
    near(river_fragment(&u, [0.0, 100.0]), rgba(BASE));
    // v = 32: sin(pi) = 0, line = 32 -> Wellenfarbe voll.
    near(river_fragment(&u, [0.0, 132.0]), rgba(WAVE));
    // v = 36: Verzerrung u = -0.0095, line = 31.17, v liegt in (line + 3, line + 6), Abstand 4.8 -> Schatten.
    near(river_fragment(&u, [0.0, 136.0]), rgba(SHADE));
}

#[test]
fn glanz_an_und_aus() {
    // v = 51 (Glanzlinie bei 51.2), u = 0.0262: s = 0.079 < 0.2, Band weit weg -> Grund; Glanz mischt 0.6.
    let on = river_fragment(&uni([0.0, 0.0, std::f32::consts::FRAC_PI_2]), [0.05, 151.0]);
    near(on, [0.6, 0.6, 1.0, 1.0]);
    // sin(3 pi / 2) = -1: on = 0 -> kein Glanz.
    let off = river_fragment(&uni([0.0, 0.0, 3.0 * std::f32::consts::FRAC_PI_2]), [0.05, 151.0]);
    near(off, rgba(BASE));
}

#[test]
fn letzte_reihe_und_klemmung_unten() {
    let u = uni([0.0; 3]);
    // Reihe 5 ab 420; weit unter ihrem Ende ist v auf 64 geklemmt, Band bei ca. 31 -> Grund.
    near(river_fragment(&u, [0.0, 900.0]), rgba(BASE));
    near(river_fragment(&u, [0.0, 420.0]), rgba(BASE));
}

#[test]
fn reihenhoehe_null_gibt_kein_nan() {
    let mut u = uni([0.0; 3]);
    u.rows[1] = 0.0;
    let c = river_fragment(&u, [0.0, 100.0]);
    assert!(c.iter().all(|v| v.is_finite()), "{c:?}");
    near(c, rgba(BASE));
}

/// Unabhängige `f64`-Fassung von `Fragment`, Zeile für Zeile aus `river.kage`.
fn reference(u: &RiverUniforms, px: f64, py: f64) -> [f64; 3] {
    let tau = std::f64::consts::TAU;
    let r = |i: usize, k: usize| f64::from(u.rows[4 * i + k]);
    let mut i0 = 0;
    for i in 1..6 {
        if py >= r(i, 0) {
            i0 = i;
        }
    }
    let n = i0 as f64;
    let v = ((py - r(i0, 0)) / r(i0, 1)).clamp(0.0, 1.0) * 64.0;
    let mut x = (px - r(i0, 3)) / r(i0, 2);
    let ph = |k: usize| f64::from(u.phase[k]);
    x += 0.025 * (tau * v / 64.0 + ph(0) + n * 1.3).sin();
    let line = 32.0 + 7.0 * (tau * 2.0 * x).sin() + 1.5 * (ph(1) + n * 0.9).sin();
    let col = |a: [f32; 3]| a.map(f64::from);
    let mut c = col(u.base);
    if v > line + 3.0 && v < line + 6.0 {
        c = col(u.shade);
    }
    let t = (1.0 - (v - line).abs() / 4.0).clamp(0.0, 1.0);
    c = [0, 1, 2].map(|k| c[k] * (1.0 - t) + col(u.wave)[k] * t);
    let s = x * 3.0 + n * 0.37;
    let on = 0.5 + 0.5 * (ph(2) + s.floor() * 2.3 + n * 1.7).sin();
    let g = if s - s.floor() < 0.2 && (v - 51.2).abs() < 1.5 && on > 0.6 {
        0.6
    } else {
        0.0
    };
    [0, 1, 2].map(|k| c[k] * (1.0 - g) + col(u.glint)[k] * g)
}

#[test]
fn stimmt_mit_f64_referenz_ueber_alle_reihen() {
    let mut u = uni([1.0, 2.0, 0.5]);
    u.rows[2] = 37.0;
    u.rows[23] = -5.0;
    let (mut off, mut total) = (0, 0);
    for y in (100..500).step_by(7) {
        for x in (0..400).step_by(3) {
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
            let got = river_fragment(&u, [px, py]);
            let want = reference(&u, f64::from(px), f64::from(py));
            // Zweigkanten (Schatten, Glanz) können in f32 und f64 verschieden fallen: wenige Ausreißer sind erlaubt.
            let close = (0..3).all(|k| (f64::from(got[k]) - want[k]).abs() < 2e-3);
            off += usize::from(!close);
            total += 1;
            assert_eq!(got[3], 1.0);
        }
    }
    assert!(off * 100 < total, "{off} Abweichungen von {total} Punkten");
}
