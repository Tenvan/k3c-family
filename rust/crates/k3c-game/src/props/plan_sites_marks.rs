//! Port von `native/k3c-game/props/plan_sites.go` (Teil 2: Wartegrund, Risse, Schutt): `site_status`, `status`, `cracks`,
//! `rubble`.

use k3c_sim::state::Site;

use super::plan::{DARK, Item, Planner, STONE_GRAY, Shape, WHITE, px};
use super::sites::{TextKey, TextWhat, WaitReason};

/// `WAIT_COLOR`: Farbe von Symbol und Text je Wartegrund (siteStatusView.ts › COLOR), Index = `WaitReason`.
pub const WAIT_COLOR: [[[u8; 4]; 2]; 4] = [
    [[0; 4], [0; 4]],
    [[76, 201, 240, 255], WHITE],
    [[255, 143, 163, 255], [255, 143, 163, 255]],
    [[230, 57, 70, 255], [255, 107, 107, 255]],
];

/// `CRACK_LINES`: Risse als Linienzüge in Anteilen der Breite (−0,5 … 0,5) und Höhe (0 … 1); je Stufe mehr.
pub const CRACK_LINES: [&[[f64; 2]]; 5] = [
    &[[-0.2, 0.95], [-0.12, 0.75], [-0.22, 0.6], [-0.15, 0.45]],
    &[[0.25, 0.85], [0.18, 0.65], [0.28, 0.5]],
    &[[-0.38, 0.4], [-0.3, 0.28], [-0.36, 0.15]],
    &[[0.05, 0.55], [0.12, 0.4], [0.04, 0.25], [0.1, 0.1]],
    &[[0.36, 0.35], [0.3, 0.2]],
];

/// `CRACKS_PER_STAGE`: Zahl der Risse je Schadensstufe.
pub const CRACKS_PER_STAGE: [usize; 4] = [0, 2, 4, 5];

/// `RUBBLE_STONES`: Steine des Trümmerhaufens (Mitte als Anteil der Breite, Breite und Höhe in Spiel-Pixeln).
pub const RUBBLE_STONES: [[f64; 3]; 6] = [
    [-0.42, 22.0, 12.0],
    [-0.3, 18.0, 16.0],
    [-0.18, 26.0, 10.0],
    [0.32, 20.0, 14.0],
    [0.44, 16.0, 10.0],
    [0.2, 24.0, 9.0],
];

const BLACK: [u8; 4] = [0, 0, 0, 255];

impl Planner {
    /// `site_status`: Wartegrund eines Platzes (Text „Name: Grund“, Symbol darüber); `n` = Tiefe bzw. Stufe für das
    /// Material.
    pub fn site_status(&mut self, s: &Site, r: WaitReason, state: &str, n: i64, x: f64, y: f64) {
        let n = if s.state == "built" { s.level.max(1) } else { n };
        let key = TextKey {
            kind: s.kind.clone(),
            state: state.to_string(),
            n,
            m: i64::from(r as u8),
            ..TextKey::new(TextWhat::Status)
        };
        self.with_text(key, |p, text| p.status(x, y, r, text));
    }

    /// `status`: Text mit Unterkante y, darüber das Symbol (Kreis Bauer, Quadrat Material, Dreieck Gefahr).
    pub fn status(&mut self, x: f64, y: f64, r: WaitReason, text: &str) {
        let c = WAIT_COLOR[r as usize];
        self.label(x, y, text, c[1]);
        let sy = y + 1.25 + px(18.0);
        match r {
            WaitReason::Worker => {
                self.top.push(Item {
                    shape: Shape::Circle,
                    x,
                    y: sy,
                    w: px(12.0),
                    clr: c[0],
                    ..Default::default()
                });
                self.top.push(Item {
                    shape: Shape::Ring,
                    x,
                    y: sy,
                    w: px(12.0),
                    size: px(3.0),
                    clr: BLACK,
                    ..Default::default()
                });
            }
            WaitReason::Material => {
                Planner::rect(&mut self.top, x, sy - px(12.0), px(24.0), px(24.0), c[0]);
                self.top.push(Item {
                    shape: Shape::Frame,
                    x,
                    y: sy - px(12.0),
                    w: px(24.0),
                    h: px(24.0),
                    size: px(3.0),
                    clr: BLACK,
                    ..Default::default()
                });
            }
            _ => self.top.push(Item {
                shape: Shape::Tri,
                x,
                y: sy - px(12.0),
                w: px(28.0),
                h: px(26.0),
                clr: c[0],
                ..Default::default()
            }),
        }
    }

    /// `cracks`: Risse über einem Gebäude der Breite w und Höhe h (damageView.ts › drawDamage).
    pub fn cracks(&mut self, x: f64, w: f64, h: f64, stage: i64) {
        let width = if stage == 3 { px(4.0) } else { px(3.0) };
        let clr = [23, 23, 23, 217];
        for line in &CRACK_LINES[..CRACKS_PER_STAGE[stage as usize]] {
            for i in 1..line.len() {
                let (a, b) = (line[i - 1], line[i]);
                self.top.push(Item {
                    shape: Shape::Line,
                    x: x + a[0] * w,
                    y: a[1] * h,
                    x2: x + b[0] * w,
                    y2: b[1] * h,
                    size: width,
                    clr,
                    ..Default::default()
                });
            }
        }
    }

    /// `rubble`: Trümmer am Fuß einer Ruine (damageView.ts › drawRubble).
    pub fn rubble(&mut self, x: f64, w: f64) {
        for s in RUBBLE_STONES {
            Planner::rect(&mut self.top, x + s[0] * w, 0.0, px(s[1]), px(s[2]), STONE_GRAY);
            self.top.push(Item {
                shape: Shape::Frame,
                x: x + s[0] * w,
                w: px(s[1]),
                h: px(s[2]),
                size: px(2.0),
                clr: DARK,
                ..Default::default()
            });
        }
    }
}
