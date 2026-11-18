//! Tests des Aufnahme-Modus (Go: `shots_test.go`) und je Station ein Lauf gegen den Software-Rasterer (AC-02): die Bilder
//! entstehen im Ordner `.work/shots-test/` des Repos (ignoriert) und werden danach entfernt.

use std::path::{Path, PathBuf};

use super::*;
use crate::game::Libs;
use crate::game::launch::build_game;
use crate::game::scene::{MENUS, parse_args};
use crate::game::shots_canvas::ShotCanvas;
use crate::game::shots_image::{shrink, strip};
use crate::game::shots_run::{open_recording, record, remove_temp, shot_saves};
use crate::still::pngio::read_png;

const BACK: [u8; 4] = [255, 0, 255, 255];

fn filled(w: u32, h: u32, c: [u8; 4]) -> Image {
    let pix = c.iter().copied().cycle().take(w as usize * h as usize * 4).collect();
    Image {
        width: w,
        height: h,
        pix,
    }
}

fn px(m: &Image, x: usize, y: usize) -> [u8; 4] {
    let i = (y * m.width as usize + x) * 4;
    [m.pix[i], m.pix[i + 1], m.pix[i + 2], m.pix[i + 3]]
}

#[test]
fn shrink_averages() {
    let mut src = filled(4, 2, [0, 0, 0, 255]);
    src.pix[..4].copy_from_slice(&[200, 100, 40, 255]);
    let got = shrink(&src, 2);
    assert_eq!((got.width, got.height), (2, 1), "Größe");
    assert_eq!(px(&got, 0, 0), [50, 25, 10, 255], "Mittel");
    assert_eq!(px(&got, 1, 0), [0, 0, 0, 255], "rechter Block");
    let one = shrink(&src, 1);
    assert_eq!(px(&one, 0, 0), px(&src, 0, 0), "Faktor 1 kopiert");
    assert_eq!(one, src, "Faktor 1 ergibt dasselbe Bild");
}

#[test]
fn shrink_rest_am_rand_faellt_weg() {
    let src = filled(5, 3, [9, 9, 9, 255]);
    let got = shrink(&src, 2);
    assert_eq!((got.width, got.height), (2, 1));
    assert_eq!(got.pix.len(), 2 * 4);
}

#[test]
fn strip_side_by_side() {
    let (a, b) = (filled(3, 2, [255, 0, 0, 255]), filled(2, 3, [0, 0, 255, 255]));
    let s = strip(&[a.clone(), b.clone()], 1);
    assert_eq!((s.width, s.height), (6, 3), "Größe");
    let checks = [
        (0, 0, px(&a, 0, 0)),
        (2, 1, px(&a, 0, 0)),
        (3, 0, BACK),
        (0, 2, BACK),
        (4, 0, px(&b, 0, 0)),
        (5, 2, px(&b, 0, 0)),
    ];
    for (x, y, want) in checks {
        assert_eq!(px(&s, x, y), want, "({x},{y})");
    }
}

#[test]
fn strip_ohne_bilder_ist_ein_punkt() {
    let s = strip(&[], 4);
    assert_eq!((s.width, s.height), (1, 1));
    assert_eq!(px(&s, 0, 0), BACK);
}

fn work(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../.work/shots-test")
        .join(name);
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

/// Ein Lauf des Aufnahme-Modus mit `extra` als Schalter; liefert die Dateien (sortiert) und die Größe des Streifens.
fn station(name: &str, extra: &[&str]) -> (PathBuf, Vec<String>) {
    let dir = work(name);
    let mut v: Vec<String> = ["--shots", "--saves", "--size", "320x180"]
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    v.insert(1, dir.join("out").display().to_string());
    v.insert(3, dir.join("saves").display().to_string());
    v.extend(extra.iter().map(|s| (*s).to_string()));
    let mut a = parse_args(&v, "s").unwrap_or_else(|e| panic!("{name}: {e}"));
    let tmp = shot_saves(&a).unwrap_or_else(|e| panic!("{name}: {e}"));
    a.saves = tmp.display().to_string();
    let mut canvas = ShotCanvas::new(320, 180);
    let mut g = build_game(&a, open_recording(&a), &Libs::default()).unwrap_or_else(|e| panic!("{name}: {e}"));
    let res = record(&mut g, &mut canvas);
    remove_temp(&tmp);
    res.unwrap_or_else(|e| panic!("{name}: {e}"));
    let mut files: Vec<String> = std::fs::read_dir(dir.join("out"))
        .unwrap_or_else(|e| panic!("{name}: {e}"))
        .filter_map(|e| Some(e.ok()?.file_name().to_string_lossy().into_owned()))
        .collect();
    files.sort();
    (dir, files)
}

fn names(n: usize) -> Vec<String> {
    let mut v: Vec<String> = (1..=n).map(|i| format!("shot-{i:02}.png")).collect();
    v.push("strip.png".to_string());
    v
}

/// Streifen: so breit wie die Vorschauen samt Lücken (320 px breite Bilder bleiben ungekürzt, Faktor 1).
fn check_strip(dir: &Path, n: usize) {
    let s = read_png(&dir.join("out/strip.png")).unwrap_or_else(|e| unreachable!("{e}"));
    assert_eq!((s.width as usize, s.height), (n * 320 + (n - 1) * 4, 180), "Streifen");
    let first = read_png(&dir.join("out/shot-01.png")).unwrap_or_else(|e| unreachable!("{e}"));
    assert_eq!((first.width, first.height), (320, 180), "Bild");
    let _ = std::fs::remove_dir_all(dir);
}

/// `--seed` mit Vorlauf `--tick`: `--count` Bilder im Abstand `--every`, dazu der Streifen.
#[test]
fn station_seed_mit_vorlauf() {
    let (dir, files) = station(
        "seed",
        &["--seed", "k3c", "--tick", "6", "--count", "3", "--every", "2"],
    );
    assert_eq!(files, names(3));
    check_strip(&dir, 3);
}

/// `--menu` mit allen sieben Seiten.
#[test]
fn station_menu_alle_seiten() {
    for m in MENUS {
        let (dir, files) = station(&format!("menu-{m}"), &["--menu", m, "--count", "1"]);
        assert_eq!(files, names(1), "--menu {m}");
        let shot = read_png(&dir.join("out/shot-01.png")).unwrap_or_else(|e| unreachable!("{e}"));
        let lit = shot
            .pix
            .as_chunks::<4>()
            .0
            .iter()
            .filter(|p| p[..3] != [0, 0, 0])
            .count();
        assert!(
            lit > 100,
            "--menu {m}: das Bild ist (fast) schwarz, {lit} helle Bildpunkte"
        );
        check_strip(&dir, 1);
    }
}

/// `--flow`: Ablauf ab dem Hauptmenü mit der Eingabe-Folge, auch ohne laufendes Spiel.
#[test]
fn station_flow_ohne_spiel() {
    let (dir, files) = station(
        "flow",
        &[
            "--flow",
            "--count",
            "3",
            "--every",
            "2",
            "--input",
            "0@1=a:down,0@3=a:up",
        ],
    );
    assert_eq!(files, names(3));
    check_strip(&dir, 3);
}

/// `--at`: Bilder genau nach diesen Takten; der Vorlauf ist der erste Takt.
#[test]
fn station_at_nach_takten() {
    let (dir, files) = station("at", &["--seed", "k3c", "--at", "2,5,11"]);
    assert_eq!(files, names(3));
    check_strip(&dir, 3);
}

/// `--input` bewegt die Spieler; `--stall` zeigt (mit Grafik) die Ladeanzeige und läuft hier ohne Grafik durch.
#[test]
fn station_input_und_stall() {
    let (dir, files) = station(
        "input",
        &["--seed", "k3c", "--input", "0@0=R,1@0=LS,0@4=a:menu", "--count", "2"],
    );
    assert_eq!(files, names(2));
    check_strip(&dir, 2);
    let (dir, files) = station("stall", &["--seed", "k3c", "--stall", "--count", "1"]);
    assert_eq!(files, names(1));
    check_strip(&dir, 1);
}

/// Gleiche Schalter ergeben gleiche Bilder (Bytes) und denselben Streifen.
#[test]
fn gleiche_schalter_gleiche_bytes() {
    let extra = ["--seed", "k3c", "--tick", "4", "--count", "2", "--every", "3"];
    let (d1, f1) = station("det-a", &extra);
    let (d2, f2) = station("det-b", &extra);
    assert_eq!(f1, f2);
    for f in &f1 {
        let (a, b) = (
            std::fs::read(d1.join("out").join(f)),
            std::fs::read(d2.join("out").join(f)),
        );
        assert!(
            a.is_ok() && a.as_ref().ok() == b.as_ref().ok(),
            "{f} unterscheidet sich"
        );
    }
    let _ = (std::fs::remove_dir_all(d1), std::fs::remove_dir_all(d2));
}

/// Ein früherer Lauf bleibt nicht liegen: alte `shot-*.png` und der Streifen verschwinden beim Start.
#[test]
fn start_raeumt_alte_bilder_weg() {
    let dir = work("alt");
    std::fs::create_dir_all(&dir).unwrap_or_else(|e| unreachable!("{e}"));
    for n in ["shot-07.png", "strip.png", "bleibt.txt"] {
        std::fs::write(dir.join(n), b"x").unwrap_or_else(|e| unreachable!("{e}"));
    }
    let a =
        parse_args(&["--shots".to_string(), dir.display().to_string()], "s").unwrap_or_else(|e| unreachable!("{e}"));
    Shooter::new(&a).unwrap_or_else(|e| unreachable!("{e}"));
    assert!(!dir.join("shot-07.png").exists() && !dir.join("strip.png").exists());
    assert!(dir.join("bleibt.txt").exists());
    let _ = std::fs::remove_dir_all(dir);
}

/// `--load` kopiert den Spielstand in den Wegwerf-Ordner; ohne Datei ist es ein Fehler.
#[test]
fn shot_saves_kopiert_den_spielstand() {
    let dir = work("saves");
    std::fs::create_dir_all(&dir).unwrap_or_else(|e| unreachable!("{e}"));
    std::fs::write(dir.join("alt.json"), b"{}").unwrap_or_else(|e| unreachable!("{e}"));
    let v = |load: &str| {
        [
            "--saves".to_string(),
            dir.display().to_string(),
            "--load".to_string(),
            load.to_string(),
        ]
    };
    let a = parse_args(&v("alt"), "s").unwrap_or_else(|e| unreachable!("{e}"));
    let tmp = shot_saves(&a).unwrap_or_else(|e| unreachable!("{e}"));
    assert_eq!(std::fs::read(tmp.join("alt.json")).ok().as_deref(), Some(&b"{}"[..]));
    assert!(tmp.starts_with(&dir), "Wegwerf-Ordner neben den Spielständen");
    remove_temp(&tmp);
    assert!(!tmp.exists());
    let b = parse_args(&v("fehlt"), "s").unwrap_or_else(|e| unreachable!("{e}"));
    let err = shot_saves(&b).err().unwrap_or_default();
    assert!(err.starts_with("fehlt: Spielstand nicht kopiert"), "{err}");
    let _ = std::fs::remove_dir_all(dir);
}

/// Texturen überleben die Bilder: Das Spiel lädt einmal hoch und zeichnet viele Bilder; eine freigegebene Textur bleibt
/// danach weg, ohne die anderen Handles zu verschieben.
#[test]
fn shot_canvas_haelt_texturen_ueber_bilder() {
    use k3c_platform::{Canvas, Rect};
    let tex = |c: u8| [c, 0, 0, 255].repeat(4);
    let mut c = ShotCanvas::new(4, 4);
    let (a, b) = (c.texture(&tex(255), 2, 2), c.texture(&tex(100), 2, 2));
    let all = Rect::new(0.0, 0.0, 2.0, 2.0);
    let draw = |c: &mut ShotCanvas, t, x: f32| c.sprite(t, all, Rect::new(x, 0.0, 2.0, 2.0), [1.0; 4]);
    for _ in 0..2 {
        draw(&mut c, a, 0.0);
        draw(&mut c, b, 2.0);
        let img = c.next_image();
        assert_eq!((px(&img, 0, 0), px(&img, 3, 0)), ([255, 0, 0, 255], [100, 0, 0, 255]));
    }
    c.free_texture(a);
    let late = c.texture(&tex(50), 2, 2); // neues Handle nach dem Freigeben und nach zwei Bildern
    draw(&mut c, a, 0.0);
    draw(&mut c, b, 2.0);
    draw(&mut c, late, 0.0);
    let img = c.next_image();
    assert_eq!(
        (px(&img, 0, 0), px(&img, 3, 0)),
        ([50, 0, 0, 255], [100, 0, 0, 255]),
        "frei: nichts, spät: sichtbar"
    );
}
