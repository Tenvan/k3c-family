//! Port von native/k3c-rig/rig/gait.go: Gangarten des Vierbeiners wie tools/paper/rig/gangarten.ts: Standphase am
//! Boden, Schwungbahn, Zwei-Knochen-IK, Rückengelenk, Kopfnicken, Schweif und die Pose des Reiters.

use super::curve::{ik, spline};
use super::fehler::Fehler;
use super::figure::{TAU, zero};
use super::load::Rig;
use super::matrix::{M, Point, apply, invert, mul, rotation, scale, translate};

// Beine in fester Reihenfolge: hinten fern, hinten nah, vorn fern, vorn nah.
pub const LEG_HF: usize = 0;
pub const LEG_HN: usize = 1;
pub const LEG_VF: usize = 2;
pub const LEG_VN: usize = 3;
pub const LEG_COUNT: usize = 4;

/// LEG_NAMES nennt die Beine in der Reihenfolge der Bänder bein-<name>.
pub const LEG_NAMES: [&str; LEG_COUNT] = ["hf", "hn", "vf", "vn"];

// Hüfte je Bein in Einheiten des Tiers (x; y = HIP_Y) und ob es ein Vorderbein ist.
const LEG_X: [f64; LEG_COUNT] = [-26.0, -30.0, 30.0, 26.0];
const LEG_FRONT: [bool; LEG_COUNT] = [false, false, true, true];

const HIP_Y: f64 = -56.0;
const SPINE_PIVOT_Y: f64 = -66.0;
const RIDE_PIVOT_Y: f64 = -80.0;

type Wave = fn(f64) -> f64;

struct Gait {
    freq: f64,
    stance: f64,
    reach: f64,
    lift: [f64; 2], // vorn, hinten
    curl: [f64; 2],
    offs: [f64; LEG_COUNT],
    path_front: Option<&'static [Point]>,
    path_back: Option<&'static [Point]>,
    rider_level: f64,
    dy: Wave,
    pitch: Wave,
    nod: Wave,
    lean: Wave,
    spine: Wave,
    tail: Wave,
    rider_y: fn(f64, f64) -> f64,
}

fn c1(ph: f64, at: f64) -> f64 {
    (TAU * (ph - at)).cos()
}

fn c2(ph: f64, at: f64) -> f64 {
    (2.0 * TAU * (ph - at)).cos()
}

const PATH_FRONT: [Point; 7] = [
    [-1.0, 0.0],
    [-1.35, 0.45],
    [-0.7, 1.0],
    [0.3, 0.95],
    [1.35, 0.55],
    [1.15, 0.12],
    [1.0, 0.0],
];
const PATH_BACK: [Point; 6] = [
    [-1.0, 0.0],
    [-1.45, 0.35],
    [-0.9, 0.85],
    [0.2, 0.75],
    [1.1, 0.35],
    [1.0, 0.0],
];

// Stand (Reit-Idle): alle Hufe am Boden, nur Atmen, leichtes Nicken und Schweifpendeln.
static STAND: Gait = Gait {
    freq: 0.5,
    stance: 1.0,
    reach: 0.0,
    lift: [0.0; 2],
    curl: [0.0; 2],
    offs: [0.0; LEG_COUNT],
    path_front: None,
    path_back: None,
    rider_level: 0.0,
    dy: |ph| 1.0 + 0.6 * (TAU * ph).sin(),
    pitch: |_| 0.0,
    nod: |ph| 1.5 * (TAU * (ph - 0.2)).sin(),
    lean: |_| 2.0,
    spine: |ph| 0.4 * (TAU * ph).sin(),
    rider_y: |dy, _| -0.8 * (dy - 1.0),
    tail: |ph| 6.0 + 3.0 * (TAU * ph).sin(),
};

static SCHRITT: Gait = Gait {
    freq: 0.9,
    stance: 0.65,
    reach: 11.0,
    lift: [9.0, 7.0],
    curl: [35.0, 15.0],
    offs: [0.5, 0.0, 0.75, 0.25], // hf, hn, vf, vn
    path_front: None,
    path_back: None,
    rider_level: 0.0,
    dy: |ph| 1.0 + c2(ph, 0.3),
    pitch: |ph| (2.0 * TAU * ph).sin(),
    nod: |ph| 6.0 * c2(ph, 0.3),
    lean: |_| 3.0,
    spine: |ph| 1.5 * (2.0 * TAU * ph).sin(),
    rider_y: |dy, ph| -0.7 * (dy - 1.0) + 0.6 * (2.0 * TAU * ph).sin(),
    tail: |ph| 6.0 + 4.0 * (TAU * ph).sin(),
};

static TRAB: Gait = Gait {
    freq: 1.5,
    stance: 0.45,
    reach: 12.0,
    lift: [16.0, 12.0],
    curl: [45.0, 20.0],
    offs: [0.5, 0.0, 0.0, 0.5], // hf, hn, vf, vn
    path_front: None,
    path_back: None,
    rider_level: 0.0,
    dy: |ph| 1.0 + 2.5 * c2(ph, 0.225),
    pitch: |ph| (2.0 * TAU * ph).sin(),
    nod: |ph| 4.0 * c2(ph, 0.25),
    lean: |_| 6.0,
    spine: |ph| 2.5 * c2(ph, 0.1),
    rider_y: |dy, ph| -0.4 * (dy - 1.0) - 4.0 * f64::max(0.0, (TAU * (ph - 0.1)).sin()),
    tail: |ph| 12.0 + 6.0 * (2.0 * TAU * ph).sin(),
};

static GALOPP: Gait = Gait {
    freq: 2.1,
    stance: 0.25,
    reach: 22.0,
    lift: [26.0, 18.0],
    curl: [80.0, 35.0],
    offs: [0.0, 0.08, 0.3, 0.38], // hf, hn, vf, vn
    path_front: Some(&PATH_FRONT),
    path_back: Some(&PATH_BACK),
    rider_level: 0.7,
    dy: |ph| 8.0 - 6.0 * c1(ph, 0.8),
    pitch: |ph| 6.0 * (TAU * (ph - 0.2)).sin(),
    nod: |ph| 10.0 * c1(ph, 0.45),
    lean: |ph| 14.0 + 3.0 * (TAU * (ph - 0.95)).sin(),
    spine: |ph| 7.0 * c1(ph, 0.2),
    rider_y: |dy, _| -0.55 * (dy - 8.0),
    tail: |ph| 28.0 + 10.0 * (TAU * (ph - 0.2)).sin(),
};

fn gait(name: &str) -> Result<&'static Gait, Fehler> {
    match name {
        "stand" => Ok(&STAND),
        "schritt" => Ok(&SCHRITT),
        "trab" => Ok(&TRAB),
        "galopp" => Ok(&GALOPP),
        _ => Err(Fehler::Rig(format!(
            "animation-unbekannt: Gangart „{name}“ (stand, schritt, trab, galopp)"
        ))),
    }
}

/// `gait_freq` liefert die Frequenz einer Gangart (Zyklen je Sekunde).
pub fn gait_freq(name: &str) -> Result<f64, Fehler> {
    Ok(gait(name)?.freq)
}

/// RIDER_PARTS nennt die Teile des Reiters, die eine Gangart setzt (Reihenfolge von GaitPose.rider).
pub const RIDER_PARTS: [&str; 8] = [
    "oberschenkel-v",
    "unterschenkel-v",
    "oberarm-v",
    "unterarm-v",
    "oberarm-h",
    "unterarm-h",
    "kopf",
    "krone",
];

/// GaitPose ist die Pose des Vierbeiners und seines Reiters bei einer Phase der Gangart.
#[derive(Debug, Clone, Default)]
pub struct GaitPose {
    /// je Teil des Vierbeiners
    pub angles: Vec<f64>,
    /// Ride-Matrix: Hub
    pub dy: f64,
    /// Ride-Matrix: Neigung
    pub pitch: f64,
    /// Fahrtwind der Gangart (Einheiten/s)
    pub wind: f64,
    /// Hub des Reiters im Sattel
    pub rider_y: f64,
    /// Neigung des Reiters um den Sitzpunkt
    pub lean: f64,
    pub rider: [f64; RIDER_PARTS.len()],
}

/// LegIndex: Teile-Index von Oberbein, Unterbein und Huf je Bein, dazu Hals, Schweif und Rückengelenk (-1 = fehlt).
#[derive(Debug, Clone, Default)]
pub struct LegIndex {
    pub upper: [i64; LEG_COUNT],
    pub lower: [i64; LEG_COUNT],
    pub hoof: [i64; LEG_COUNT],
    pub neck: i64,
    pub tail: i64,
    pub back: i64,
    /// Längen der beiden IK-Knochen.
    pub l1: f64,
    pub l2: f64,
}

impl Rig {
    pub(super) fn leg_indices(&self) -> LegIndex {
        let mut x = LegIndex {
            neck: self.index_of("hals"),
            tail: self.index_of("schweif"),
            back: self.index_of("rumpf-hinten"),
            l1: 25.0,
            l2: 31.0,
            ..LegIndex::default()
        };
        for (i, n) in LEG_NAMES.iter().enumerate() {
            x.upper[i] = self.index_of(&format!("oberbein-{n}"));
            x.lower[i] = self.index_of(&format!("unterbein-{n}"));
            x.hoof[i] = self.index_of(&format!("huf-{n}"));
        }
        // Oberbein = Knie − Hüfte, Unterbein bis zum Boden (tools/paper/rig/proportionen.ts › beinLaengen)
        let (h, k) = (x.upper[LEG_HF], x.lower[LEG_HF]);
        if h >= 0 && k >= 0 {
            let (hip, knee) = (self.parts[h as usize].pivot[1], self.parts[k as usize].pivot[1]);
            (x.l1, x.l2) = (knee - hip, -hip - (knee - hip));
        }
        x
    }

    /// `gait_pose` rechnet die Pose des Vierbeiners (und des Reiters) bei Phase ph (0..1); `out.angles` wird
    /// wiederverwendet.
    pub fn gait_pose(&self, name: &str, ph: f64, out: &mut GaitPose) -> Result<(), Fehler> {
        let g = gait(name)?;
        let x = &self.legs;
        let (dy, pitch, spine, reach) = ((g.dy)(ph), (g.pitch)(ph), (g.spine)(ph), g.reach * self.scale);
        let front = ride_matrix(dy, pitch, self.scale);
        let back = mul(front, rotation(spine, 0.0, SPINE_PIVOT_Y));
        zero(&mut out.angles, self.parts.len());
        for i in 0..LEG_COUNT {
            if x.upper[i] < 0 || x.lower[i] < 0 || x.hoof[i] < 0 {
                return Err(Fehler::Rig(format!(
                    "pflicht-teil-fehlt: vierbeiner {} braucht oberbein-, unterbein- und huf-{}",
                    self.id, LEG_NAMES[i]
                )));
            }
            let base = if LEG_FRONT[i] { front } else { back };
            let u = (((ph - g.offs[i]) % 1.0) + 1.0) % 1.0;
            let (target, curl) = g.hoof_target(apply(&base, [LEG_X[i], HIP_Y]), reach, u, LEG_FRONT[i]);
            let lt = apply(&invert(&base), target);
            let (u1, u2) = ik(lt[0] - LEG_X[i], lt[1] - HIP_Y, LEG_FRONT[i], x.l1, x.l2);
            let mut hoof = -(u1 + u2) - pitch + curl;
            if !LEG_FRONT[i] {
                hoof -= spine;
            }
            out.angles[x.upper[i] as usize] = u1;
            out.angles[x.lower[i] as usize] = u2;
            out.angles[x.hoof[i] as usize] = hoof;
        }
        let (lean, nod, s) = ((g.lean)(ph) - g.rider_level * pitch, (g.nod)(ph), (TAU * ph).sin());
        set_if(&mut out.angles, x.neck, nod);
        set_if(&mut out.angles, x.tail, (g.tail)(ph));
        set_if(&mut out.angles, x.back, spine);
        (out.dy, out.pitch, out.lean, out.rider_y) = (dy, pitch, lean, (g.rider_y)(dy, ph));
        out.wind = 0.0;
        if g.reach > 0.0 {
            out.wind = reach * 2.0 / (g.stance / g.freq);
        }
        out.rider = [
            -40.0 - lean,
            50.0,
            -35.0 - lean + 0.6 * nod,
            -40.0,
            -30.0 - lean,
            -40.0,
            -lean * 0.7 + 2.0 * s,
            -2.0 * s,
        ];
        Ok(())
    }

    /// `rider_angles` schreibt die Winkel des Reiters aus einer Gangart-Pose in angles (je Teil dieses Rigs, sonst 0).
    pub fn rider_angles(&self, p: &GaitPose, angles: &mut Vec<f64>) {
        zero(angles, self.parts.len());
        for (i, &idx) in self.rider.iter().enumerate() {
            set_if(angles, idx, p.rider[i]);
        }
    }
}

impl Gait {
    /// `hoof_target`: Ziel des Hufs in Welt-Einheiten (Boden y = 0) und Hufkrümmung, nach Stand- oder Schwungphase.
    fn hoof_target(&self, hip: Point, reach: f64, u: f64, front: bool) -> (Point, f64) {
        let fi = usize::from(!front);
        if u < self.stance {
            return ([hip[0] + reach * (1.0 - 2.0 * u / self.stance), 0.0], 0.0);
        }
        let k = (u - self.stance) / (1.0 - self.stance);
        if self.path_front.is_some() {
            let path = if front { self.path_front } else { self.path_back }.unwrap_or(&[]);
            let p = spline(path, k);
            return (
                [hip[0] + reach * p[0], -p[1] * self.lift[fi]],
                self.curl[fi] * (std::f64::consts::PI * k.powf(0.7)).sin(),
            );
        }
        let e = 0.5 - 0.5 * (std::f64::consts::PI * k).cos();
        (
            [
                hip[0] + reach * (2.0 * e - 1.0),
                -(std::f64::consts::PI * k).sin() * self.lift[fi],
            ],
            self.curl[fi] * (std::f64::consts::PI * k).sin(),
        )
    }
}

/// `ride_matrix`: translate(0, dy) · rotate(pitch um (0, −80)) in Welt-Einheiten, darin scale(massstab).
pub fn ride_matrix(dy: f64, pitch: f64, scale_: f64) -> M {
    mul(
        translate(0.0, dy),
        mul(rotation(pitch, 0.0, RIDE_PIVOT_Y), scale(scale_)),
    )
}

fn set_if(angles: &mut [f64], i: i64, v: f64) {
    if i >= 0 {
        angles[i as usize] = v;
    }
}
