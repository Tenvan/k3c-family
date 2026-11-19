//! Bildvergleich Go gegen Rust je Station (RU32.4, AC-02). Eingabe: die Bilder aus RU32.3
//! (`K3C_E2E_OUT/go/a/shot-NN.png` und `K3C_E2E_OUT/rust/a/shot-NN.png`, Standard `.work/e2e`; fehlen sie, überspringt der
//! Test). Je Station: mittlere absolute Abweichung je Kanal (RGB, 0 … 255) und Anteil der Pixel, bei denen ein Kanal um mehr
//! als `PIXEL_TOL` abweicht, auf voller Auflösung. Formel und Schwellen wie der Standbild-Vergleich `tests/still.rs`
//! (RU13.6), damit beide gleich bewertet werden; die Schwellen werden nie angehoben, um einen Stand grün zu machen.
//!
//! Der Test schlägt bei einer Station über der Schwelle **nicht** fehl (Ticket je Station); er schreibt
//! `K3C_E2E_OUT/vergleich.json` und je Station das Bildpaar `paare/NN-<station>.png` (links Go, Mitte Rust, rechts
//! Differenz: Pixel über der Toleranz rot auf abgedunkeltem Go-Bild). Eine abweichende Bildgröße ist ein Fehler.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

mod common;

use common::images::{out_dir, reference, shot_name};
use k3c_game::still::Image;
use k3c_game::still::pngio::{read_png, write_png};
use serde_json::json;

/// Schwellen je Station wie in `tests/still.rs`.
const MAX_MEAN: f64 = 0.5;
const MAX_SHARE: f64 = 0.005;
const PIXEL_TOL: i32 = 24;

/// (mittlere Abweichung je Kanal, Anteil abweichender Pixel); Bildgröße muss gleich sein.
fn diff(a: &Image, b: &Image) -> (f64, f64) {
    assert_eq!((a.width, a.height), (b.width, b.height), "Bildgröße");
    let (mut sum, mut bad) = (0u64, 0u64);
    for (p, q) in a.pix.as_chunks::<4>().0.iter().zip(b.pix.as_chunks::<4>().0) {
        let d: Vec<i32> = (0..3).map(|k| (i32::from(p[k]) - i32::from(q[k])).abs()).collect();
        sum += d.iter().map(|&v| u64::try_from(v).unwrap_or(0)).sum::<u64>();
        bad += u64::from(d.iter().any(|&v| v > PIXEL_TOL));
    }
    let n = u64::from(a.width) * u64::from(a.height);
    (sum as f64 / (3 * n) as f64, bad as f64 / n as f64)
}

/// Differenzbild: Go abgedunkelt, Pixel über der Toleranz rot.
fn diff_image(go: &Image, rust: &Image) -> Image {
    let mut pix = Vec::with_capacity(go.pix.len());
    for (p, q) in go.pix.as_chunks::<4>().0.iter().zip(rust.pix.as_chunks::<4>().0) {
        let bad = (0..3).any(|k| (i32::from(p[k]) - i32::from(q[k])).abs() > PIXEL_TOL);
        if bad {
            pix.extend_from_slice(&[255, 0, 0, 255]);
        } else {
            pix.extend_from_slice(&[p[0] / 3, p[1] / 3, p[2] / 3, 255]);
        }
    }
    Image {
        width: go.width,
        height: go.height,
        pix,
    }
}

/// Drei Bilder nebeneinander.
fn side_by_side(parts: [&Image; 3]) -> Image {
    let (w, h) = (parts[0].width, parts[0].height);
    let mut pix = Vec::with_capacity(parts.iter().map(|p| p.pix.len()).sum());
    for y in 0..h as usize {
        for p in parts {
            let row = y * w as usize * 4;
            pix.extend_from_slice(&p.pix[row..row + w as usize * 4]);
        }
    }
    Image {
        width: w * 3,
        height: h,
        pix,
    }
}

fn solid(w: u32, h: u32, c: [u8; 4]) -> Image {
    Image {
        width: w,
        height: h,
        pix: c.repeat((w * h) as usize),
    }
}

/// Die Formel selbst (läuft immer): gleiche Bilder ergeben 0, ein Pixel über der Toleranz zählt als abweichend.
#[test]
fn kennzahlen_der_formel() {
    let a = solid(4, 2, [10, 20, 30, 255]);
    assert_eq!(diff(&a, &a), (0.0, 0.0));
    let mut b = a.clone();
    b.pix[0] += u8::try_from(PIXEL_TOL + 1).unwrap(); // ein Pixel von acht, ein Kanal über der Toleranz
    let (mean, share) = diff(&a, &b);
    assert!((share - 0.125).abs() < 1e-12, "Anteil {share}");
    assert!((mean - 25.0 / 24.0).abs() < 1e-12, "Mittel {mean}");
    let d = diff_image(&a, &b);
    assert_eq!(&d.pix[0..4], &[255, 0, 0, 255], "abweichendes Pixel rot");
    assert_eq!(&d.pix[4..8], &[3, 6, 10, 255], "übrige Pixel abgedunkelt");
    let s = side_by_side([&a, &b, &d]);
    assert_eq!((s.width, s.height), (12, 2));
}

#[test]
fn bildvergleich_je_station() {
    let out = out_dir();
    let (_, stations) = reference();
    let go = |i| out.join("go/a").join(shot_name(i));
    let rs = |i| out.join("rust/a").join(shot_name(i));
    if !(0..stations.len()).all(|i| go(i).is_file() && rs(i).is_file()) {
        eprintln!(
            "übersprungen: Bilder fehlen unter {} (go/a, rust/a; RU32.3)",
            out.display()
        );
        return;
    }
    let mut rows = Vec::new();
    for (i, (name, tick)) in stations.iter().enumerate() {
        let (g, r) = (read_png(&go(i)).unwrap(), read_png(&rs(i)).unwrap());
        assert_eq!(
            (g.width, g.height),
            (r.width, r.height),
            "{name}: Bildgröße Go {}x{}, Rust {}x{}",
            g.width,
            g.height,
            r.width,
            r.height
        );
        let (mean, share) = diff(&g, &r);
        let over = mean > MAX_MEAN || share > MAX_SHARE;
        let pair = out.join("paare").join(format!("{:02}-{name}.png", i + 1));
        write_png(&pair, &side_by_side([&g, &r, &diff_image(&g, &r)])).unwrap();
        eprintln!(
            "{:02} {name:<12} Takt {tick:<6} mittel {mean:.3} abweichend {share:.4} {}",
            i + 1,
            if over { "ÜBER DER SCHWELLE" } else { "ok" }
        );
        rows.push(json!({"nr": i + 1, "station": name, "tick": tick, "mean": mean, "share": share, "over": over, "paar": pair.to_string_lossy()}));
    }
    let doc = json!({"maxMean": MAX_MEAN, "maxShare": MAX_SHARE, "pixelTol": PIXEL_TOL, "stationen": rows});
    std::fs::write(out.join("vergleich.json"), serde_json::to_string_pretty(&doc).unwrap()).unwrap();
}
