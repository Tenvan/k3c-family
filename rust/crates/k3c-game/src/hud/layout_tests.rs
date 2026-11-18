//! Port von `native/k3c-game/hud/layout_test.go`.

use super::draw::rect_of;
use super::fonts::{REF_HEIGHT, scale_of};
use super::layout::*;
use super::radar::radar_size;
use crate::view::cell::{Cell, Rectangle, WORLD_SHARE, rect, stats_rect, stats_rect_lines};

/// sizes_at: Größen wie im Spiel (Browser-Test hudLayout.test.ts, 28 px fett, Innenabstand 8/4 px) beim Maßstab k.
pub(super) fn sizes_at(k: f64, cell_w: f64) -> Items {
    let mut inp: Items = [Size::default(); ITEM_COUNT];
    let mut set = |id: ItemId, w: f64, h: f64| inp[id] = Size { w: w * k, h: h * k };
    set(ITEM_PLAYER, 224.0, 92.0); // Geldsack-Karte (cardSize): Sack 84 px + Gold, B-919
    set(ITEM_SKILLS, 832.0, 42.0);
    set(ITEM_BANNER, 640.0, 42.0);
    set(ITEM_SHARED, 416.0, 42.0);
    set(ITEM_CLOCK, 352.0, 42.0);
    set(ITEM_FIGHT, 288.0, 42.0);
    set(ITEM_CITIZENS, 448.0, 42.0);
    set(ITEM_PLACE, 448.0, 38.0);
    set(ITEM_TRAVEL, 512.0, 56.0);
    set(ITEM_JOIN, 480.0, 40.0);
    set(ITEM_BOSS, 672.0, 80.0);
    set(ITEM_EVENTS, 320.0, 42.0);
    let (w, h) = radar_size(cell_w, k);
    inp[ITEM_RADAR] = Size { w, h };
    inp
}

/// cell_rect wie game/layout.go › CellRect: eine Zelle füllt das Fenster, zwei liegen übereinander.
fn cell_rect(win: Rectangle, n: i64, i: i64) -> Rectangle {
    if n <= 1 {
        return win;
    }
    let mid = win.min.y + win.dy() / 2;
    if i == 0 {
        return rect(win.min.x, win.min.y, win.max.x, mid);
    }
    rect(win.min.x, mid, win.max.x, win.max.y)
}

fn to_win(area: Rect) -> Rectangle {
    rect(
        area.x as i64,
        area.y as i64,
        (area.x + area.w) as i64,
        (area.y + area.h) as i64,
    )
}

fn tick_free(area: Rect) -> Rect {
    rect_of(stats_rect(to_win(area)))
}

fn windows() -> [Rectangle; 2] {
    [rect(0, 0, 1280, 720), rect(0, 0, 1920, 1080)]
}

#[test]
fn layout_ohne_ueberschneidung_pflicht_da() {
    for win in windows() {
        for n in [1, 2] {
            for i in 0..n {
                check_cell(win, n, i);
            }
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn check_cell(win: Rectangle, n: i64, i: i64) {
    let name = format!("{}x{}, {} Spieler, Zelle {}", win.dx(), win.dy(), n, i);
    let area = rect_of(cell_rect(win, n, i));
    let k = win.dy() as f64 / REF_HEIGHT;
    let mut free = Vec::new();
    if i == 0 {
        free.push(tick_free(area));
    }
    let inp = sizes_at(k, area.w);
    let out = layout(&inp, area, &free, k);
    for id in 0..ITEM_COUNT {
        if !out.ok[id] {
            assert!(ANCHORS[id].rank != RANK_MUST, "{name}: Pflicht {id} fehlt");
            continue;
        }
        let r = out.at[id];
        assert!(r.inside(area), "{name}: Element {id} {r:?} nicht in der Zelle {area:?}");
        for f in &free {
            assert!(!r.overlaps(*f), "{name}: Element {id} verdeckt die Takt-Anzeige");
        }
        for o in id + 1..ITEM_COUNT {
            assert!(
                !(out.ok[o] && r.overlaps(out.at[o])),
                "{name}: Element {id} schneidet {o}"
            );
        }
    }
}

#[test]
fn layout_ohne_freiflaeche_am_anker() {
    let (area, k) = (Rect::new(0.0, 0.0, 1920.0, 1080.0), 1.0);
    let inp = sizes_at(k, area.w);
    let out = layout(&inp, area, &[], k);
    for id in [ITEM_PLAYER, ITEM_SKILLS, ITEM_CLOCK] {
        let want = anchor_rect(id, area, inp[id].w, inp[id].h, k);
        assert_eq!(out.at[id], want, "{id}: {:?}, erwartet Anker {want:?}", out.at[id]);
    }
    assert!(
        out.at[ITEM_PLAYER].x == 24.0 && out.at[ITEM_FIGHT].x + out.at[ITEM_FIGHT].w == 1920.0 - 24.0,
        "Spielerkarte links, Kampf rechts mit 24 px Rand: {:?} {:?}",
        out.at[ITEM_PLAYER],
        out.at[ITEM_FIGHT]
    );
    assert!(
        out.at[ITEM_SHARED].y >= out.at[ITEM_PLAYER].y + out.at[ITEM_PLAYER].h,
        "Vorrat steht unter der Spielerkarte"
    );
}

#[test]
fn layout_takt_anzeige_verschiebt_kampf() {
    let (area, k) = (Rect::new(0.0, 0.0, 1280.0, 360.0), 720.0 / REF_HEIGHT);
    let inp = sizes_at(k, area.w);
    let free = tick_free(area);
    let out = layout(&inp, area, &[free], k);
    assert!(
        out.ok[ITEM_FIGHT] && !out.at[ITEM_FIGHT].overlaps(free),
        "Kampf-Zeile weicht der Takt-Anzeige aus: {:?}",
        out.at[ITEM_FIGHT]
    );
    assert!(
        out.at[ITEM_FIGHT] != anchor_rect(ITEM_FIGHT, area, inp[ITEM_FIGHT].w, inp[ITEM_FIGHT].h, k),
        "Kampf-Zeile steht noch an ihrem Anker"
    );
}

#[test]
fn layout_zu_klein_faellt_nach_rang() {
    let (area, k) = (Rect::new(0.0, 0.0, 400.0, 110.0), 1.0);
    let mut inp: Items = [Size::default(); ITEM_COUNT];
    inp[ITEM_PLAYER] = Size { w: 400.0, h: 46.0 };
    inp[ITEM_CLOCK] = Size { w: 400.0, h: 46.0 };
    inp[ITEM_RADAR] = Size { w: 400.0, h: 46.0 };
    let out = layout(&inp, area, &[], k);
    assert!(
        out.ok[ITEM_PLAYER] && out.ok[ITEM_CLOCK] && !out.ok[ITEM_RADAR],
        "Pflicht und Rang 1 bleiben, Radar fällt weg: {:?}",
        out.ok
    );
}

#[test]
fn layout_nicht_gezeigt_bleibt_leer() {
    let mut inp: Items = [Size::default(); ITEM_COUNT];
    inp[ITEM_PLAYER] = Size { w: 100.0, h: 20.0 };
    let out = layout(&inp, Rect::new(0.0, 0.0, 800.0, 600.0), &[], 1.0);
    for id in 0..ITEM_COUNT {
        assert!(id == ITEM_PLAYER || !out.ok[id], "Element {id} ohne Größe gezeigt");
    }
}

#[test]
fn scale_of_zellen() {
    let c = Cell {
        rect: rect(0, 360, 1280, 720),
        ..Cell::default()
    };
    let k = scale_of(&c, 2);
    assert!(k == 720.0 / 1080.0, "zwei Zellen à 360 px: Maßstab {k}");
    let c = Cell {
        rect: rect(0, 0, 1920, 1080),
        ..Cell::default()
    };
    let k = scale_of(&c, 1);
    assert!(k == 1.0, "1080 px: Maßstab {k}");
}

/// B-915: Das Radar sitzt an der Unterkante der Zelle (im Fluss) und überlappt weder die Takt-/Stresstest-Anzeige
/// (auch mit zwei Zeilen) noch ein anderes Element, mit einem und mit zwei Spielern.
#[test]
fn layout_radar_unterkante_ohne_anzeige() {
    for win in windows() {
        for n in [1, 2] {
            for i in 0..n {
                check_radar_cell(win, n, i);
            }
        }
    }
}

/// check_radar_cell prüft eine Zelle mit den echten Größen (Geldsack-Karte, Boss, Events): Radar und Vorrats-Zeile
/// bleiben in der Zelle, das Radar im Fluss.
fn check_radar_cell(win: Rectangle, n: i64, i: i64) {
    let area = rect_of(cell_rect(win, n, i));
    let k = win.dy() as f64 / REF_HEIGHT;
    let mut free = Vec::new();
    let mut stats = Rect::default();
    if i == 0 {
        free.push(tick_free(area));
        stats = rect_of(stats_rect_lines(to_win(area), 2));
    }
    let inp = sizes_at(k, area.w);
    let out = layout(&inp, area, &free, k);
    let name = format!("{win:?}, {n} Spieler, Zelle {i}");
    assert!(
        out.ok[ITEM_RADAR] && out.ok[ITEM_SHARED],
        "{name}: Radar {} oder Vorrat {} fehlt",
        out.ok[ITEM_RADAR],
        out.ok[ITEM_SHARED]
    );
    let r = out.at[ITEM_RADAR];
    assert!(
        !(r.y < area.y + area.h * WORLD_SHARE || !r.inside(area) || !out.at[ITEM_SHARED].inside(area)),
        "{name}: Radar {r:?} nicht im Fluss der Zelle {area:?} bzw. Vorrat {:?} draußen",
        out.at[ITEM_SHARED]
    );
    assert!(
        !(stats.w > 0.0 && r.overlaps(stats)),
        "{name}: Radar {r:?} überlappt die Anzeige {stats:?}"
    );
    for o in 0..ITEM_COUNT {
        assert!(
            !(o != ITEM_RADAR && out.ok[o] && r.overlaps(out.at[o])),
            "{name}: Radar schneidet Element {o}"
        );
    }
}
