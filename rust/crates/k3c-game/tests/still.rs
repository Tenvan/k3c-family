//! Bildvergleich Rust gegen Go an fünf Ständen (RU13.6, B-1014) und Test der Positionen aus dem Zustand.
//!
//! Die Referenz schreibt `native/k3c-game/world/rustref_test.go` (`-write`): Zustand `<name>.state.json.gz` und das
//! Go-Bild `<name>.go.png`, dazu `<name>.go-nolight.png` (ohne Lichtkarte, nur noch Referenz). Seit RU55 zeichnet Rust die
//! Lichtkarte (B-1056); der Test vergleicht mit dem vollständigen Go-Bild.
//! Der Fluss steht in beiden auf Kacheln; der Shader-Fluss hat kein Go-Golden (Go-Lauf gesperrt, B-1059) und wird
//! gegen die Software-Referenz `river_fragment` geprüft (`fluss_shader_im_standbild`, B-1073).
//!
//! Die gerenderte Grafik (`public/grafik`, `task gen:paper`) ist nicht eingecheckt: Ohne sie überspringen die Tests
//! mit Hinweis; `K3C_GRAFIK_DIR` zeigt auf einen anderen Ordner. Mit `K3C_STILL_OUT=<ordner>` schreiben sie die
//! Rust-Bilder (zum Ansehen) und die Abweichungsbilder.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

use std::path::{Path, PathBuf};

use k3c_game::still::{self, Image, StillState, pngio};
use k3c_sim::state::{Enemy, Troop};

const STAENDE: [&str; 5] = ["morgen", "mittag", "abend", "nacht", "nacht-gegner"];

/// Schwellen je Stand (Ergebnis RU13.6): mittlere Abweichung je Kanal (0 … 255) und Anteil der Pixel, bei denen ein
/// Kanal um mehr als `PIXEL_TOL` abweicht.
const MAX_MEAN: f64 = 0.5;
const MAX_SHARE: f64 = 0.005;
const PIXEL_TOL: i32 = 24;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn bild(name: &str, ext: &str) -> PathBuf {
    root().join("testdata/rust/bild").join(format!("{name}.{ext}"))
}

fn grafik_da() -> bool {
    let dir = still::config(&root(), None).grafik_dir;
    let probe = std::fs::read_dir(dir.join("hintergruende"))
        .is_ok_and(|d| d.flatten().any(|e| e.path().join("manifest.json").is_file()));
    if !probe {
        // K3C_STILL_REQUIRE=1 (Abnahme): fehlende Grafik ist ein Fehler statt eines Überspringens
        assert!(
            std::env::var("K3C_STILL_REQUIRE").is_err(),
            "gerenderte Grafik fehlt (task gen:paper)"
        );
        eprintln!("übersprungen: gerenderte Grafik fehlt (task gen:paper oder K3C_GRAFIK_DIR)");
    }
    probe
}

fn state(name: &str) -> StillState {
    still::load_state(&bild(name, "state.json.gz")).unwrap()
}

fn draw(s: &StillState) -> Image {
    still::render(s, &still::config(&root(), None)).unwrap()
}

/// Go-Bild, mit dem verglichen wird: das vollständige mit Lichtkarte (seit RU55 zeichnet Rust sie, B-1056).
fn go_bild(name: &str) -> Image {
    pngio::read_png(&bild(name, "go.png")).unwrap()
}

/// (mittlere Abweichung je Kanal, Anteil abweichender Pixel)
fn diff(a: &Image, b: &Image) -> (f64, f64) {
    assert_eq!((a.width, a.height), (b.width, b.height), "Bildgröße");
    let (mut sum, mut bad) = (0u64, 0u64);
    for (p, q) in a.pix.as_chunks::<4>().0.iter().zip(b.pix.as_chunks::<4>().0) {
        let d: Vec<i32> = (0..3).map(|k| (i32::from(p[k]) - i32::from(q[k])).abs()).collect();
        sum += d.iter().map(|&v| v as u64).sum::<u64>();
        bad += u64::from(d.iter().any(|&v| v > PIXEL_TOL));
    }
    let n = u64::from(a.width) * u64::from(a.height);
    (sum as f64 / (3 * n) as f64, bad as f64 / n as f64)
}

fn dump(name: &str, img: &Image) {
    if let Some(dir) = std::env::var("K3C_STILL_OUT").ok().filter(|d| !d.is_empty()) {
        pngio::write_png(&Path::new(&dir).join(format!("{name}.png")), img).unwrap();
    }
}

/// Ohne Grafik: Die fünf Zustände sind lesbar (Biom, Level, zwei Monarchen, Gegner nur im letzten Stand).
#[test]
fn zustaende_lassen_sich_laden() {
    for name in STAENDE {
        let s = state(name);
        assert_eq!(s.world.players.len(), 2, "{name}");
        assert_eq!(s.world.biome.id, "forest", "{name}");
        assert!(!s.world.level.chunks.is_empty(), "{name}: Level fehlt");
        assert_eq!(
            s.world.enemies.len(),
            if name == "nacht-gegner" { 6 } else { 0 },
            "{name}"
        );
        assert_eq!(s.cams.len(), s.seats.len(), "{name}");
    }
}

#[test]
fn bildvergleich_an_fuenf_staenden() {
    if !grafik_da() {
        return;
    }
    let mut report = String::new();
    for name in STAENDE {
        let rust = draw(&state(name));
        dump(name, &rust);
        let (mean, share) = diff(&rust, &go_bild(name));
        report += &format!("{name}: mittel {mean:.3}, abweichend {:.4}\n", share);
        assert!(
            mean <= MAX_MEAN,
            "{name}: mittlere Abweichung {mean:.3} über {MAX_MEAN}"
        );
        assert!(
            share <= MAX_SHARE,
            "{name}: {share:.4} der Pixel weichen ab, erlaubt {MAX_SHARE}"
        );
    }
    eprintln!("{report}");
}

/// Schwerpunkt der Pixel in den Zeilen `rows`, die sich ändern, wenn man die Figur aus dem Zustand nimmt: (x, y, Zahl).
fn change_center(full: &Image, without: &Image, rows: (u32, u32)) -> Option<(f64, f64, u64)> {
    let (mut sx, mut sy, mut n) = (0.0, 0.0, 0u64);
    for (i, (p, q)) in full
        .pix
        .as_chunks::<4>()
        .0
        .iter()
        .zip(without.pix.as_chunks::<4>().0)
        .enumerate()
    {
        let y = i as u32 / full.width;
        if p != q && (rows.0..rows.1).contains(&y) {
            sx += (i as u32 % full.width) as f64;
            sy += f64::from(y);
            n += 1;
        }
    }
    (n > 0).then(|| (sx / n as f64, sy / n as f64, n))
}

/// AC-02: Monarch, Truppen und Gegner stehen an den Positionen des Zustands: Nimmt man eine Figur aus dem Zustand, ändert
/// sich das Bild um ihren Fußpunkt (Bildschirm-x der Kamera der Zelle, in der sie steht).
#[test]
fn figuren_stehen_an_den_positionen_des_zustands() {
    if !grafik_da() {
        return;
    }
    let cfg = still::config(&root(), None);
    // Ein Stand mit allem: zwei Monarchen, Truppen, Gegner
    let base = state("nacht-gegner");
    let full = still::render(&base, &cfg).unwrap();
    let (_, cells) = base.frame();
    let cell = &cells[0]; // obere Zelle, Kamera bei cams[0]
    let (cy0, cy1) = (cell.rect.min.y as f64, cell.rect.max.y as f64);
    let mut checked = 0;
    let check = |what: &str, x: f64, without: StillState| {
        let img = still::render(&without, &cfg).unwrap();
        let (cx, cy, n) =
            change_center(&full, &img, (cy0 as u32, cy1 as u32)).unwrap_or_else(|| panic!("{what}: Bild unverändert"));
        let want = cell.camera.screen_x(x);
        assert!(
            (cx - want).abs() < 40.0,
            "{what}: Bild ändert sich bei x={cx:.0}, erwartet {want:.0} ({n} Pixel)"
        );
        assert!(cy > cy0 && cy < cy1, "{what}: y={cy:.0} außerhalb der Zelle");
    };
    // Monarch 0 (Zelle 0 zeigt ihn selbst), Truppen und Gegner im Sichtfeld der Zelle 0
    let mut s = base.clone();
    let p0 = s.world.players.remove(0);
    check("Monarch", p0.x, s);
    checked += 1;
    for t in &base.world.troops {
        let sx = cell.camera.screen_x(t.x);
        if !(60.0..f64::from(base.size.0) - 60.0).contains(&sx) || !troop_isolated(&base, t) {
            continue;
        }
        let mut s = base.clone();
        s.world.troops.retain(|o| o.id != t.id);
        check(&format!("Truppe {} ({})", t.id, t.kind), t.x, s);
        checked += 1;
    }
    for e in &base.world.enemies {
        if !enemy_visible(&base, e) {
            continue;
        }
        let mut s = base.clone();
        s.world.enemies.retain(|o| o.id != e.id);
        check(&format!("Gegner {} ({})", e.id, e.kind), e.x, s);
        checked += 1;
    }
    assert!(checked >= 6, "zu wenige Figuren geprüft: {checked}");
}

/// Eine Truppe ohne Nachbar in 3 Units (sonst überlagern sich die Änderungen).
fn troop_isolated(s: &StillState, t: &Troop) -> bool {
    let near = |x: f64| (x - t.x).abs() < 3.0;
    s.world.troops.iter().filter(|o| o.id != t.id && near(o.x)).count() == 0
        && s.world.players.iter().all(|p| !near(p.x))
        && s.world.enemies.iter().all(|e| !near(e.x))
}

fn enemy_visible(s: &StillState, e: &Enemy) -> bool {
    let (_, cells) = s.frame();
    let x = cells[0].camera.screen_x(e.x);
    (60.0..f64::from(s.size.0) - 60.0).contains(&x)
        && s.world.enemies.iter().all(|o| o.id == e.id || (o.x - e.x).abs() >= 3.0)
}

/// B-1073 AC-03: Mit Shader-Fluss ändert sich das Bild nur in der Fluss-Fläche der Zellen; dort trägt die Mehrheit der
/// Pixel die Farbe der Software-Referenz (`river_fragment`) aus `water_layout`; der Rest wird von Ebenen darüber
/// (Figuren, Bauten, Licht) gedeckt. Ohne Go-Golden mit Shader gibt es keinen Vergleich gegen Go.
#[test]
fn fluss_shader_im_standbild() {
    if !grafik_da() {
        return;
    }
    let base = state("mittag");
    let tiles = draw(&base);
    let shader = draw(&base.clone().with_river_shader());
    dump("mittag-fluss-shader", &shader);
    let (_, cells) = base.frame();
    let (mut inside, mut same, mut outside_diff) = (0u64, 0u64, 0u64);
    for c in &cells {
        let p = k3c_game::world::water::water_layout(c, base.clock);
        let mut r = k3c_game::world::water::RiverWater::default();
        r.build();
        r.set(&p);
        let u = r.uniforms();
        for y in 0..shader.height {
            for x in 0..shader.width {
                let (xi, yi) = (i64::from(x), i64::from(y));
                if xi < c.rect.min.x || xi >= c.rect.max.x || yi < c.rect.min.y || yi >= c.rect.max.y {
                    continue;
                }
                let i = ((y * shader.width + x) * 4) as usize;
                if yi < p.top {
                    outside_diff += u64::from(shader.pix[i..i + 4] != tiles.pix[i..i + 4]);
                    continue;
                }
                inside += 1;
                let f = k3c_platform::river_fragment(&u, [x as f32 + 0.5, y as f32 + 0.5]);
                let want = [0, 1, 2].map(|k| (f[k] * 255.0).round() as i32);
                same += u64::from((0..3).all(|k| (i32::from(shader.pix[i + k]) - want[k]).abs() <= 1));
            }
        }
    }
    assert_eq!(outside_diff, 0, "über der Fluss-Fläche ändert sich das Bild");
    let share = same as f64 / inside as f64;
    eprintln!("Fluss-Fläche {inside} Pixel, {share:.3} wie river_fragment");
    assert!(share > 0.6, "nur {share:.3} der Fluss-Pixel entsprechen der Referenz");
}
