//! Port von `native/k3c-game/actors/figures.go`.
//!
//! Figuren mit Zustand: je Figur der Simulation (Stufe, Art, ID) ein Eintrag mit Rig-Actor bzw. Sheet, Pose, Richtung
//! und dem Vor-Zustand. Listen gleicht die Einträge einmal je Frame mit der Welt ab; Draw liest sie nur (bis auf den
//! Pose-Puffer des Actors, siehe draw.rs). Ab `figure` steht der Rest in `figures_pose.rs`.
//!
//! Abweichung von Go: Ein Rig-Actor braucht in Rust eine Zeichenfläche (Textur-Upload), die Listen nicht hat. Listen
//! legt die Figur deshalb ohne Actor an (`pending`) und stellt die Pose vorläufig nur als Wert ein; `ensure_actors`
//! legt die Actors beim ersten Zeichnen (oder beim Start) an und stellt die Pose dann am Actor ein.

use k3c_det::fmax;
use k3c_sim::state::{Enemy, Troop, World as SimWorld};

use super::Actors;
use super::anim::{ANIM_ATTACK, ANIM_IDLE, MonarchFrame, Motion, Pose, Want, figure_want, monarch_want};
use super::catalog::{Look, UNIT_PX, skinned};
use crate::art::Actor;
use crate::view::frame::Frame;
use std::collections::BTreeMap;

/// class unterscheidet die Listen der Welt; IDs sind nur je Liste und Stufe eindeutig.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Class {
    #[default]
    Player,
    Troop,
    Enemy,
}

/// key ist die stabile Kennung einer Figur über die Frames.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Key {
    pub stage: i64,
    pub class: Class,
    pub id: i64,
}

// Höhe des Turm-Platzes (Browser-Client: 270 Spiel-Pixel) und Abstand, ab dem eine Truppe auf ihrem Turm steht.
pub const TOWER_LIFT: f64 = 270.0 / UNIT_PX;
pub const ON_TOWER_MAX: f64 = 0.3;
/// Bauer arbeitet, wenn er so nah am Ziel steht (Units)
pub const WORK_NEAR: f64 = 0.5;
/// fliehende Gegner (Browser: worldRenderer.ts)
pub const FLEE_ALPHA: f64 = 0.7;
/// unsichtbarer Monarch, Smoke Bomb (Browser: worldRenderer.ts, B-577)
pub const STEALTH_MAX: f64 = 0.35;

/// figure ist eine Figur mit Zustand.
pub struct Figure {
    pub look: Look,
    /// `None`: Sheet (look.sheet), kaputt oder noch nicht angelegt (`pending`)
    pub actor: Option<Actor>,
    /// Rig fehlt: zeichnet nichts (gemeldet beim Anlegen)
    pub broken: bool,
    /// Der Rig-Actor wird beim nächsten `ensure_actors` angelegt (Abweichung von Go, siehe Kopf)
    pub pending: bool,
    /// Monarch auf dem Reittier des Katalogs (für das Anlegen des Actors)
    pub mounted: bool,
    /// Rig-Pfad des Reittiers des Monarchen (leer = Katalog-Standard, RU62.9)
    pub mount: String,
    pub pose: Pose,
    pub left: bool,
    pub shift: f64,
    /// Fußpunkt über dem Boden in Units
    pub lift: f64,
    pub alpha: f64,
    /// Deckkraft der Anzeigen (Balken, Punkt, Gold): Container-Alpha des Browsers, ohne Sprite-Alpha
    pub ui: f64,
    pub generation: u64,
    /// Vor-Zustand gesetzt
    pub known: bool,
    pub x: f64,
    pub cd: f64,
    /// der Actor hat eine Pose (Umhang hängt)
    pub posed: bool,
    /// Pose seit der letzten Rechnung gewechselt
    pub dirty: bool,
    /// Darstellungszeit der Pose
    pub posed_at: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct WarnKey {
    pub what: String,
    pub name: String,
}

impl Figure {
    /// hasGait meldet, ob das Reittier der Figur die Gangart hat (ohne Rig: alle).
    pub fn has_gait(&self, g: &str) -> bool {
        let Some(m) = self.actor.as_ref().and_then(|a| a.mount.as_ref()) else {
            return true;
        };
        m.animations.get(g).is_some_and(|a| !a.gait.is_empty())
    }
}

impl Actors {
    /// Listen gleicht die Figuren mit den Welten des Frames ab: neue anlegen, Zustand → Pose, verschwundene
    /// entfernen, dann die Pose aller Rig-Figuren im Takt fortschreiben (pose.rs).
    pub fn listen(&mut self, f: &Frame) {
        if self.cat.is_none() {
            return;
        }
        if self.started && !f.paused {
            self.now += fmax(f.clock - self.last, 0.0);
        }
        self.last = f.clock;
        self.started = true;
        self.generation += 1;
        for st in &f.stages {
            let Some(w) = &st.world else {
                continue;
            };
            self.listen_players(w, st.index);
            for t in &w.troops {
                self.listen_troop(w, st.index, t);
            }
            let empty = BTreeMap::new();
            let skins = w.vals.as_ref().map_or(&empty, |v| &v.skins);
            for e in &w.enemies {
                self.listen_enemy(st.index, e, skins);
            }
        }
        let generation = self.generation;
        self.figs.retain(|_, fg| fg.generation == generation); // Reihenfolge egal: entfernt nur
        self.advance_all();
    }

    fn listen_players(&mut self, w: &SimWorld, stage: i64) {
        let mount_ref = self.cat.as_ref().map_or(0.0, |c| c.mount_ref);
        for p in &w.players {
            let Some(look) = self.cat.as_ref().map(|c| c.player(p.index)) else {
                return;
            };
            let k = Key {
                stage,
                class: Class::Player,
                id: p.id,
            };
            let mut fg = self.figure(k, look, true);
            let mount = self.cat.as_ref().map_or("", |c| c.mount_of(&p.mount));
            if fg.mount != mount {
                fg.mount = mount.to_string();
                fg.actor = None;
                fg.broken = false;
                fg.pending = !fg.look.sheet && self.lib.is_some();
            }
            let prev = MonarchFrame {
                known: fg.known,
                ..MonarchFrame::default()
            };
            let cur = MonarchFrame { known: true, vx: p.vx };
            let want = monarch_want(prev, cur, mount_ref, |g| fg.has_gait(g));
            self.apply(&mut fg, &want);
            if p.facing != 0 {
                fg.left = p.facing < 0;
            }
            fg.known = true;
            fg.x = p.x;
            fg.lift = 0.0;
            fg.alpha = monarch_alpha(p.stealth_for);
            fg.ui = fg.alpha;
            self.figs.insert(k, fg);
        }
    }

    fn listen_troop(&mut self, w: &SimWorld, stage: i64, t: &Troop) {
        let craft = self.cat.as_ref().is_some_and(|c| c.troops.contains_key(CRAFTSMAN));
        let Some(look) = self
            .cat
            .as_ref()
            .and_then(|c| c.troops.get(look_kind(t, craft)))
            .cloned()
        else {
            self.warn(
                WarnKey {
                    what: "truppe".to_string(),
                    name: t.data_kind().to_string(),
                },
                &"unbekannte Truppen-Art",
            );
            return;
        };
        let k = Key {
            stage,
            class: Class::Troop,
            id: t.id,
        };
        let mut fg = self.figure(k, look.clone(), false);
        let m = Motion {
            known: fg.known,
            prev_x: fg.x,
            x: t.x,
            prev_cd: fg.cd,
            cd: t.cooldown,
            attack_cooldown: false,
        };
        let (want, dir) = troop_want(t.data_kind(), m, working(t), nearest_enemy_dir(w, t.x));
        let (want, dir) = if w.festival.is_some() && w.biome.depth == 0 {
            {
                let g = super::festival::Guest {
                    kind: t.data_kind(),
                    id: t.id,
                    x: t.x,
                    hub_x: w.hub_x,
                    working: working(t),
                    now: self.now,
                };
                super::festival::festival_want(g, want, dir)
            }
        } else {
            (want, dir)
        };
        self.apply(&mut fg, &want);
        if dir != 0 {
            fg.left = dir < 0;
        }
        fg.known = true;
        fg.x = t.x;
        fg.cd = t.cooldown;
        fg.alpha = look.alpha;
        fg.lift = look.lift;
        if on_tower(w, t) {
            fg.lift += TOWER_LIFT;
        }
        self.figs.insert(k, fg);
    }

    fn listen_enemy(&mut self, stage: i64, e: &Enemy, skins: &BTreeMap<String, String>) {
        let Some(look) = self
            .cat
            .as_ref()
            .and_then(|c| c.enemy(&e.kind, e.boss))
            .map(|l| skinned(l, skins))
        else {
            self.warn(
                WarnKey {
                    what: "gegner".to_string(),
                    name: e.kind.clone(),
                },
                &"unbekannte Gegner-Art",
            );
            return;
        };
        let k = Key {
            stage,
            class: Class::Enemy,
            id: e.id,
        };
        let mut fg = self.figure(k, look.clone(), false);
        let (want, dir) = figure_want(&Motion {
            known: fg.known,
            prev_x: fg.x,
            x: e.x,
            prev_cd: fg.cd,
            cd: e.cooldown,
            attack_cooldown: true,
        });
        self.apply(&mut fg, &want);
        if dir != 0 {
            fg.left = dir < 0;
        }
        fg.known = true;
        fg.x = e.x;
        fg.cd = e.cooldown;
        fg.lift = look.lift;
        fg.alpha = look.alpha;
        fg.ui = 1.0;
        if e.fleeing {
            fg.alpha *= FLEE_ALPHA;
            fg.ui = FLEE_ALPHA;
        }
        self.figs.insert(k, fg);
    }
}

/// MonarchAlpha liefert die Deckkraft des Monarchen: fast durchsichtig, solange die Smoke Bomb wirkt.
pub fn monarch_alpha(stealth_for: f64) -> f64 {
    if stealth_for > 0.0 {
        return STEALTH_MAX;
    }
    1.0
}

/// TroopWant wählt die Animation einer Truppe: Kämpfer schlagen nach Cooldown, Landstreicher und Bauern nutzen ihn
/// fürs Warten; ein arbeitender Bauer schlägt in Schleife, ein stehender Bogenschütze schaut zum nächsten Gegner.
pub fn troop_want(kind: &str, mut m: Motion, working: bool, enemy_dir: i64) -> (Want, i64) {
    m.attack_cooldown = kind != "vagrant" && kind != "peasant";
    let (mut want, mut dir) = figure_want(&m);
    if dir == 0 && (kind == "archer" || kind == "eliteArcher") {
        dir = enemy_dir;
    }
    if kind == "peasant" && working && want.anim == ANIM_IDLE {
        want = Want {
            anim: ANIM_ATTACK.to_string(),
            ..Want::default()
        };
    }
    (want, dir)
}

/// Schlüssel des Handwerkers in `data/paper.json` (FG1.3): ein Bürger im Dienst mit Werkzeuggürtel.
const CRAFTSMAN: &str = "craftsman";

/// look_kind: Schlüssel der Grafik einer Truppe; der Handwerker (Bürger mit Hammer) hat eine eigene Figur, sonst gilt die
/// Art aus den Daten. `craft`: Der Katalog kennt `craftsman`, sonst bleibt es beim Bauern.
fn look_kind(t: &Troop, craft: bool) -> &str {
    if craft && t.hammer && t.kind == "citizen" && !t.wild {
        CRAFTSMAN
    } else {
        t.data_kind()
    }
}

/// working: Bauer bei der Arbeit (Holz hacken, bauen) am Ziel.
pub fn working(t: &Troop) -> bool {
    t.job
        .as_ref()
        .is_some_and(|j| (j.r#type == "gather" || j.r#type == "build") && (t.target_x - t.x).abs() < WORK_NEAR)
}

/// nearestEnemyDir: Richtung zum nächsten Gegner (0 = keiner da).
pub fn nearest_enemy_dir(w: &SimWorld, x: f64) -> i64 {
    let (mut best, mut dist) = (0, f64::INFINITY);
    for e in &w.enemies {
        let d = (e.x - x).abs();
        if d < dist {
            best = 1.0_f64.copysign(e.x - x) as i64;
            dist = d;
        }
    }
    best
}

/// onTower: Truppe steht auf ihrem Turm (viewRules.ts › isOnTower).
pub fn on_tower(w: &SimWorld, t: &Troop) -> bool {
    let Some(id) = t.tower_id else {
        return false;
    };
    w.sites
        .iter()
        .find(|s| s.id == id)
        .is_some_and(|s| (s.x - t.x).abs() < ON_TOWER_MAX)
}

#[cfg(test)]
#[path = "figures_tests.rs"]
mod figures_tests;
