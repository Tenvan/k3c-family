//! Port von `native/k3c-game/fx/looks.go`.
//!
//! Eigener Effekt je Skill (Spiegel von src/scenes/effects/skillLooks.ts, B-578): Farbe (Palette DB32), Größe, Dauer,
//! Partikelzahl und Bewegung. Ein Skill ohne Eintrag zeigt den Ring seiner Linie (lineColors). Nur Optik.

use std::collections::BTreeMap;
use std::f64::consts::PI;
use std::sync::OnceLock;

use k3c_data::Source;
use k3c_data::monarch::{self, SkillData};

use super::effects::{Kind, Look, Motion, config, look};

/// `line_colors`: Farbe des Skill-Rings je Linie (effects.ts › LINE_COLORS).
pub fn line_colors() -> &'static BTreeMap<&'static str, u32> {
    static M: OnceLock<BTreeMap<&'static str, u32>> = OnceLock::new();
    M.get_or_init(|| {
        BTreeMap::from([
            ("tank", 0x6fa8ff),
            ("mage", 0xc070ff),
            ("healer", 0x70f0a0),
            ("thief", 0x9a9a9a),
            ("purse", 0xf2c94c),
        ])
    })
}

/// `skill_looks`: eigener Look je Skill.
pub fn skill_looks() -> &'static BTreeMap<&'static str, Look> {
    use Motion::{Burst, Fall, Rise, Wave};
    static M: OnceLock<BTreeMap<&'static str, Look>> = OnceLock::new();
    M.get_or_init(|| {
        BTreeMap::from([
            ("taunt", look(0xdf7126, 40.0, 0.7, 6, Wave)),
            ("shieldBash", look(0xfbf236, 24.0, 0.4, 5, Burst)),
            ("ironWall", look(0x9badb7, 44.0, 0.9, 6, Rise)),
            ("lastStand", look(0xac3232, 46.0, 0.9, 8, Rise)),
            ("armorAura", look(0x639bff, 36.0, 0.6, 4, Wave)),
            ("thickSkin", look(0x8f563b, 30.0, 0.5, 4, Burst)),
            ("regeneration", look(0x6abe30, 28.0, 0.7, 5, Rise)),
            ("guardian", look(0x5b6ee1, 40.0, 0.7, 6, Wave)),
            ("fortified", look(0x847e87, 32.0, 0.5, 4, Fall)),
            ("fireball", look(0xdf7126, 34.0, 0.45, 8, Burst)),
            ("iceWall", look(0x5fcde4, 38.0, 0.7, 6, Wave)),
            ("lightningStorm", look(0xfbf236, 36.0, 0.5, 6, Fall)),
            ("meteor", look(0xac3232, 54.0, 0.8, 10, Fall)),
            ("arcanePower", look(0x76428a, 32.0, 0.6, 6, Rise)),
            ("spellEcho", look(0xd77bba, 30.0, 0.6, 6, Burst)),
            ("frostArmor", look(0xcbdbfc, 32.0, 0.6, 5, Fall)),
            ("elementalMastery", look(0x306082, 34.0, 0.6, 6, Burst)),
            ("heal", look(0x99e550, 30.0, 0.6, 5, Rise)),
            ("groupHeal", look(0x6abe30, 44.0, 0.7, 8, Wave)),
            ("divineShield", look(0xeec39a, 42.0, 0.8, 6, Rise)),
            ("resurrection", look(0xffffff, 50.0, 0.9, 8, Rise)),
            ("healingAura", look(0x37946e, 36.0, 0.7, 5, Wave)),
            ("blessings", look(0xd9a066, 32.0, 0.7, 6, Fall)),
            ("holyGround", look(0x8f974a, 40.0, 0.7, 6, Wave)),
            ("backstab", look(0xd95763, 22.0, 0.3, 4, Burst)),
            ("smokeBomb", look(0x696a6a, 48.0, 0.9, 8, Rise)),
        ])
    })
}

/// `skills` ist aus monarch.json › skills, was die Effekte brauchen (einmal gelesen).
pub fn skills() -> &'static BTreeMap<String, SkillData> {
    static S: OnceLock<BTreeMap<String, SkillData>> = OnceLock::new();
    S.get_or_init(read_skills)
}

fn read_skills() -> BTreeMap<String, SkillData> {
    let res = Source::from_env()
        .read(monarch::FILE)
        .and_then(|raw| monarch::load(&raw));
    match res {
        Ok(m) => m.skills,
        Err(e) => {
            k3c_platform::log::error(
                "💥 Effekt-Werte nicht lesbar",
                &[("datei", &monarch::FILE), ("err", &e)],
            );
            BTreeMap::new()
        }
    }
}

/// `shield_max` ist der größte Schild eines Skills (effect.type shield) als volle Balkenbreite.
pub fn shield_max() -> f64 {
    static M: OnceLock<f64> = OnceLock::new();
    *M.get_or_init(|| {
        let mut m = 1.0_f64;
        for s in skills().values() {
            if s.effect.r#type == "shield" {
                m = m.max(s.effect.hp);
            }
        }
        m
    })
}

/// `skill_look` liefert den Effekt eines Skills: eigener Look, sonst der Ring seiner Linie; unbekannt: None.
pub fn skill_look(id: &str) -> Option<Look> {
    if let Some(l) = skill_looks().get(id) {
        return Some(*l);
    }
    let line = skills().get(id).map(|s| s.line.as_str()).unwrap_or("");
    let c = *line_colors().get(line)?;
    let base = config(Kind::Skill).look;
    Some(look(c, base.radius, base.duration, base.particles, Motion::Burst))
}

/// `particle_offset` ist der Ziel-Versatz des Partikels i von n je Bewegung (Welt-px; y nach unten positiv).
pub fn particle_offset(m: Motion, i: i64, n: i64, radius: f64) -> (f64, f64) {
    let mut spread = 0.0;
    if n > 1 {
        spread = i as f64 / (n - 1) as f64 - 0.5;
    }
    match m {
        Motion::Rise => return (spread * radius * 1.6, -radius * 2.0),
        Motion::Fall => return (spread * radius * 1.6, radius * 1.2),
        Motion::Wave => {
            let side = if i % 2 == 0 { -1.0 } else { 1.0 };
            return (side * radius * (1.2 + (i / 2) as f64 * 0.5), 30.0);
        }
        Motion::Burst => {}
    }
    let a = i as f64 / n as f64 * PI * 2.0;
    (a.cos() * radius * 1.4, a.sin() * radius * 1.4 - 12.0)
}

/// `Piece` ist ein Teil eines Effekts zum Fortschritt p: Versatz (Welt-px), Deckkraft, Radius (Welt-px).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub dx: f64,
    pub dy: f64,
    pub alpha: f64,
    pub radius: f64,
}

/// `piece_at` ist Teil i des Effekts (i = 0: Ring, wächst auf das 1,8-Fache; danach Partikel, schrumpfen auf die
/// Hälfte) beim Fortschritt p (0..1, linear wie die Tweens im Browser).
pub fn piece_at(l: Look, i: i64, p: f64) -> Piece {
    let k = 1.0_f64.min(0.0_f64.max(p));
    if i == 0 {
        return Piece {
            dx: 0.0,
            dy: 0.0,
            alpha: 0.8 * (1.0 - k),
            radius: l.radius * (1.0 + 0.8 * k),
        };
    }
    let (dx, dy) = particle_offset(l.motion, i - 1, l.particles, l.radius);
    Piece {
        dx: dx * k,
        dy: dy * k,
        alpha: 0.8 * (1.0 - k),
        radius: 5.0 * (1.0 - 0.5 * k),
    }
}
