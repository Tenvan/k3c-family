//! Port von `native/k3c-game/fx/status.go`.
//!
//! Status-Effekte der Skills (Spiegel von src/scenes/effects/statusLooks.ts, B-675): Auren und Sturm am Boden, Schild
//! und Last Stand am Monarchen, Verlangsamung, Betäubung und Spott über dem Gegner; dazu der Warnkreis vor dem
//! Flächenschlag eines Bosses (kampfView.ts › WarnView). Nur Optik aus dem Zustand, Bewegung allein aus der Zeit.

use k3c_sim::state::{Player, World};

use super::effects::UNIT_PX;
use super::looks::{shield_max, skill_look};
use super::pending::Aura;

/// `StatusKind` in Zeichen-Reihenfolge (Boden zuerst).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusKind {
    Aura,
    Storm,
    Shield,
    LastStand,
    Slow,
    Stun,
    Taunt,
}

/// Alle Arten in Zeichen-Reihenfolge (Go: `range statusKinds`).
pub const STATUS_KINDS: [StatusKind; 7] = [
    StatusKind::Aura,
    StatusKind::Storm,
    StatusKind::Shield,
    StatusKind::LastStand,
    StatusKind::Slow,
    StatusKind::Stun,
    StatusKind::Taunt,
];

/// `Mark` ist eine Markierung: x in Units, y Höhe über dem Boden in Welt-px (Mitte bzw. Boden), Radius in Welt-px,
/// Farbe und Anteil (Schild-Balken).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mark {
    pub kind: StatusKind,
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: u32,
    pub share: f64,
}

// Werte der Looks (STATUS_LOOKS): Farben DB32, Größen und Höhen in Welt-px, Takte in Sekunden.
pub const STUN_COLOR: u32 = 0xfbf236;
pub const SLOW_COLOR: u32 = 0x5fcde4;
pub const TAUNT_COLOR: u32 = 0xdf7126;
pub const STORM_COLOR: u32 = 0xfbf236;
pub const STORM_FILL: u32 = 0x5b6ee1;
pub const SHIELD_COLOR: u32 = 0x9badb7;
pub const LAST_STAND_COLOR: u32 = 0xac3232;
pub const STUN_ORBIT: f64 = 16.0;
pub const SLOW_SIZE: f64 = 9.0;
pub const TAUNT_HEIGHT: f64 = 18.0;
pub const STATUS_Y: f64 = 104.0;
pub const STATUS_BOSS_Y: f64 = 190.0;
pub const TAUNT_Y: f64 = 112.0;
pub const TAUNT_BOSS_Y: f64 = 200.0;
pub const SHIELD_RADIUS: f64 = 46.0;
pub const SHIELD_Y: f64 = 48.0;
pub const LAST_STAND_RADIUS: f64 = 40.0;
pub const AURA_FALLBACK: u32 = 0xffffff;

/// `AuraCache` merkt `sim.AurasOf` je Monarch, solange seine Skills gleich bleiben: Die Auren hängen nur von den
/// gelernten Skills und den Werten der Welt ab, AurasOf legt aber je Aufruf einen neuen Slice an.
/// Abweichung von Go: Die Welt kommt je Aufruf als Parameter; ihre Identität ist die Adresse (`world`), der Schlüssel
/// je Monarch ist sein Index.
#[derive(Debug, Default)]
pub struct AuraCache {
    pub entries: Vec<AuraEntry>,
}

/// `AuraEntry` ist der gemerkte Stand eines Monarchen.
#[derive(Debug, Default)]
pub struct AuraEntry {
    pub world: usize,
    pub index: i64,
    pub skills: Vec<String>,
    pub auras: Vec<Aura>,
}

impl AuraCache {
    /// `of` liefert die Auren des Monarchen p in der Welt w (`source`).
    pub fn of(&mut self, w: &World, p: &Player, source: &dyn Fn(&World, &Player) -> Vec<Aura>) -> &[Aura] {
        if p.crown.is_some() {
            // gefallen: keine Auren (wie sim.AurasOf), nichts merken
            return &[];
        }
        let wid = w as *const World as usize;
        if let Some(i) = self.entries.iter().position(|e| e.index == p.index) {
            let e = &mut self.entries[i];
            if e.world != wid || e.skills != p.skills {
                e.world = wid;
                e.skills.clone_from(&p.skills);
                e.auras = source(w, p);
            }
            return &self.entries[i].auras;
        }
        self.entries.push(AuraEntry {
            world: wid,
            index: p.index,
            skills: p.skills.clone(),
            auras: source(w, p),
        });
        &self.entries[self.entries.len() - 1].auras
    }
}

/// `status_marks` schreibt (`auras_of` liefert die sichtbaren Passive eines Monarchen, sim.AurasOf; Tests geben eigene) alle Markierungen der Welt w in out (geleert), nach Art sortiert (Boden zuerst, stabil).
pub fn status_marks(mut out: Vec<Mark>, w: &World, auras_of: &dyn Fn(&Player) -> Vec<Aura>) -> Vec<Mark> {
    out.clear();
    for kind in STATUS_KINDS {
        match kind {
            StatusKind::Aura => out = aura_marks(out, w, auras_of),
            StatusKind::Storm => {
                for s in &w.storms {
                    if s.left > 0.0 {
                        out.push(Mark {
                            kind: StatusKind::Storm,
                            x: s.x,
                            y: 0.0,
                            radius: s.radius * UNIT_PX,
                            color: STORM_COLOR,
                            share: 1.0,
                        });
                    }
                }
            }
            StatusKind::Shield | StatusKind::LastStand => out = player_marks(out, w, kind),
            _ => out = enemy_marks(out, w, kind),
        }
    }
    out
}

/// `aura_marks`: Aura-Ringe der lebenden Monarchen am Boden (Holy Ground an der Burg), mindestens eine Unit groß.
fn aura_marks(mut out: Vec<Mark>, w: &World, auras_of: &dyn Fn(&Player) -> Vec<Aura>) -> Vec<Mark> {
    for p in &w.players {
        if p.crown.is_some() {
            continue;
        }
        for a in auras_of(p) {
            let mut x = p.x;
            if a.castle {
                x = w.castle.x;
            }
            let c = skill_look(&a.skill).map_or(AURA_FALLBACK, |l| l.color);
            out.push(Mark {
                kind: StatusKind::Aura,
                x,
                y: 0.0,
                radius: (a.radius * UNIT_PX).max(UNIT_PX),
                color: c,
                share: 1.0,
            });
        }
    }
    out
}

/// `player_marks`: Schild-Blase mit Balken bzw. Last Stand der lebenden Monarchen.
fn player_marks(mut out: Vec<Mark>, w: &World, kind: StatusKind) -> Vec<Mark> {
    for p in &w.players {
        if p.crown.is_some() {
            continue;
        }
        if kind == StatusKind::Shield && p.shield_for > 0.0 && p.shield > 0.0 {
            out.push(Mark {
                kind: StatusKind::Shield,
                x: p.x,
                y: SHIELD_Y,
                radius: SHIELD_RADIUS,
                color: SHIELD_COLOR,
                share: 1.0_f64.min(p.shield / shield_max()),
            });
        } else if kind == StatusKind::LastStand && p.last_stand_for > 0.0 {
            out.push(Mark {
                kind: StatusKind::LastStand,
                x: p.x,
                y: SHIELD_Y,
                radius: LAST_STAND_RADIUS,
                color: LAST_STAND_COLOR,
                share: 1.0,
            });
        }
    }
    out
}

/// `enemy_marks`: Verlangsamung (links), Betäubung (Mitte), Spott (rechts) über dem Kopf lebender Gegner.
fn enemy_marks(mut out: Vec<Mark>, w: &World, kind: StatusKind) -> Vec<Mark> {
    for e in &w.enemies {
        if e.hp <= 0.0 {
            continue;
        }
        let (mut y, mut ty) = (STATUS_Y, TAUNT_Y);
        if e.boss {
            (y, ty) = (STATUS_BOSS_Y, TAUNT_BOSS_Y);
        }
        if kind == StatusKind::Slow && e.slow_for > 0.0 {
            out.push(Mark {
                kind: StatusKind::Slow,
                x: e.x - 18.0 / UNIT_PX,
                y,
                radius: SLOW_SIZE,
                color: SLOW_COLOR,
                share: 1.0,
            });
        } else if kind == StatusKind::Stun && e.stun > 0.0 {
            out.push(Mark {
                kind: StatusKind::Stun,
                x: e.x,
                y,
                radius: STUN_ORBIT,
                color: STUN_COLOR,
                share: 1.0,
            });
        } else if kind == StatusKind::Taunt && e.taunt_for > 0.0 {
            out.push(Mark {
                kind: StatusKind::Taunt,
                x: e.x + 18.0 / UNIT_PX,
                y: ty,
                radius: TAUNT_HEIGHT,
                color: TAUNT_COLOR,
                share: 1.0,
            });
        }
    }
    out
}

/// `WARN_WINDOW`: Sekunden vor dem Flächenschlag, in denen der Warnkreis kräftiger wird.
pub const WARN_WINDOW: f64 = 3.0;

/// `urgency` ist die Dringlichkeit des Warnkreises: 0 = Schlag noch weit, 1 = jetzt.
pub fn urgency(inn: f64) -> f64 {
    1.0 - WARN_WINDOW.min(0.0_f64.max(inn)) / WARN_WINDOW
}
