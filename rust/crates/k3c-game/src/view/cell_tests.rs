//! Port von `native/k3c-game/view/cell_test.go`; `two_halves_stacked` ist neu (Kriterium der Session RU7.4).

use super::*;

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn cell_splits_world_and_river() {
    let mut c = Cell {
        index: 1,
        player: 1,
        rect: rect(0, 540, 1920, 1080),
        ..Cell::default()
    };
    let got = c.ground_y();
    assert_eq!(got, 900, "GroundY = {got}, erwartet 900");
    assert_eq!(c.world_rect(), rect(0, 540, 1920, 900), "WorldRect");
    assert_eq!(c.river_rect(), rect(0, 900, 1920, 1080), "RiverRect");
    c.camera = new_camera(c.rect, 10.0, 32.0);
    let y = c.screen_y(2.0);
    assert!(near(y, 900.0 - 64.0), "ScreenY(2) = {y}");
}

#[test]
fn camera_maps_world_to_screen() {
    let k = new_camera(rect(960, 0, 1920, 540), 100.0, 20.0);
    assert!(
        near(k.center, 1440.0) && near(k.half, 480.0),
        "Center/Half = {}/{}",
        k.center,
        k.half
    );
    let x = k.screen_x(100.0);
    assert!(near(x, 1440.0), "Mitte: {x}");
    let x = k.screen_x(101.0);
    assert!(near(x, 1460.0), "eine Unit rechts: {x}");
    let wx = k.world_x(k.screen_x(87.5));
    assert!(near(wx, 87.5), "WorldX(ScreenX) = {wx}");
    let x = k.parallax_x(5.0, 0.0);
    assert!(near(x, 5.0 * 20.0 + 1440.0), "Faktor 0 steht: {x}");
    let x = k.parallax_x(100.0, 1.0);
    assert!(near(x, k.screen_x(100.0)), "Faktor 1 wie Boden: {x}");
    let (l, r) = k.span();
    assert!(near(l, 76.0) && near(r, 124.0), "Span = {l} … {r}");
    assert!(
        k.visible(125.0, 1.5) && !k.visible(126.0, 1.5) && k.visible(75.0, 1.0),
        "Visible falsch am Rand"
    );
}

#[test]
fn camera_without_scale() {
    let k = Camera {
        x: 3.0,
        ..Camera::default()
    };
    assert!(k.world_x(500.0) == 3.0, "WorldX ohne Maßstab");
    let (l, r) = k.span();
    assert!(l == 3.0 && r == 3.0, "Span ohne Maßstab");
}

#[test]
fn window_cell_covers_window() {
    let c = window_cell(rect(0, 0, 1280, 720));
    assert!(
        c.index == -1 && c.player == -1 && near(c.camera.center, 640.0),
        "WindowCell = {c:?}"
    );
}

// Die Leistungsanzeige hat zwei Zeilen: gleiche Oberkante und Breite, weiter unten zu Ende; eine Zeile ist StatsRect.
#[test]
fn stats_rect_lines_grow_down() {
    let win = rect(0, 0, 1280, 720);
    let (one, two) = (stats_rect_lines(win, 1), stats_rect_lines(win, 2));
    assert!(
        one == stats_rect(win) && stats_rect_lines(win, 0) == one,
        "eine Zeile: {one:?}"
    );
    assert!(
        two.min == one.min && two.dx() == one.dx() && two.dy() > one.dy(),
        "zwei Zeilen: {two:?} gegen {one:?}"
    );
    let (w, h, k) = stats_text_size_lines(2);
    let r = stats_rect_lines(win, 2);
    assert!(
        w * k == r.dx() && h * k == r.dy(),
        "Textbild {w}x{h}·{k} passt nicht zu {r:?}"
    );
}

// Zwei Spieler: Zellen übereinander, je eigene Kamera; Welt und Fluss jeder Hälfte bleiben in ihrer Zelle, und das
// Rechteck für Canvas::clip liegt in Fensterkoordinaten.
#[test]
fn two_halves_stacked() {
    let (win_w, win_h) = (1920, 1080);
    let cells: Vec<Cell> = (0..2)
        .map(|i| {
            let r = rect(0, i * win_h / 2, win_w, (i + 1) * win_h / 2);
            Cell {
                index: i,
                player: i,
                rect: r,
                camera: new_camera(r, 10.0 + 50.0 * i as f64, 32.0),
            }
        })
        .collect();
    assert_eq!(cells[0].ground_y(), 360, "obere Hälfte: Boden");
    assert_eq!(cells[1].ground_y(), 900, "untere Hälfte: Boden");
    for c in &cells {
        assert_eq!(c.world_rect().min, c.rect.min, "Welt beginnt oben in Zelle {}", c.index);
        assert_eq!(c.river_rect().max, c.rect.max, "Fluss endet unten in Zelle {}", c.index);
        assert_eq!(
            c.world_rect().max.y,
            c.river_rect().min.y,
            "Welt und Fluss grenzen in Zelle {}",
            c.index
        );
        // Gleiches Welt-x der Kamera steht in jeder Hälfte in der Fenstermitte; x hängt nicht von der Lage ab.
        assert!(near(c.camera.screen_x(c.camera.x), 960.0), "Mitte in Zelle {}", c.index);
        assert!(
            near(c.camera.screen_x(c.camera.x + 1.0), 992.0),
            "eine Unit = 32 px in Zelle {}",
            c.index
        );
    }
    assert_eq!(cells[0].rect.max.y, cells[1].rect.min.y, "Hälften grenzen ohne Lücke");
    assert_eq!(
        cells[1].rect.to_rect(),
        k3c_platform::Rect::new(0.0, 540.0, 1920.0, 540.0),
        "Clip untere Hälfte"
    );
    assert!(
        near(cells[1].screen_y(1.0), 900.0 - 32.0),
        "ScreenY in der unteren Hälfte"
    );
}
