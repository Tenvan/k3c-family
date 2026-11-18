//! Port von `native/k3c-game/fx/fx.go` (Teil `stageFx`): die laufenden Effekte und Status-Markierungen einer Stufe.

use std::collections::BTreeMap;

use k3c_sim::state::{Event as SimEvent, World};

use super::effects::{
    Effect, Kind, MAX_LIVE_PIECES, RING_PX, UNIT_PX, coin_loss_for, config, damage_for, effect_for, flash_allowed,
    num_of, str_of,
};
use super::looks::skill_look;
use super::status::{AuraCache, Mark};
use crate::view::frame::Event;

/// `StageFx` sind die laufenden Effekte und Status-Markierungen einer Stufe.
#[derive(Debug)]
pub struct StageFx {
    pub index: i64,
    pub effects: Vec<Effect>,
    pub pieces: i64,
    pub spots: BTreeMap<String, f64>,
    pub marks: Vec<Mark>,
    pub last_flash: f64,
    /// Optionen › Blitze aus (Settings.FlashesOff, B-809): keine Blitz-Effekte
    pub no_flash: bool,
    pub auras: AuraCache,
}

impl StageFx {
    pub fn new(index: i64) -> StageFx {
        StageFx {
            index,
            effects: Vec::new(),
            pieces: 0,
            spots: BTreeMap::new(),
            marks: Vec::new(),
            last_flash: -1.0,
            no_flash: false,
            auras: AuraCache::default(),
        }
    }

    /// `expire` lässt abgelaufene Effekte fallen.
    pub fn expire(&mut self, now: f64) {
        let mut keep = std::mem::take(&mut self.effects);
        keep.retain(|e| now - e.start < e.look.duration && now >= e.start);
        self.pieces = keep.iter().map(Effect::pieces).sum();
        self.effects = keep;
    }

    /// `event` legt die Effekte eines Ereignisses an: Zuordnung aus `effect_for`, dazu Schadenszahl und Skill-Wirkung.
    pub fn event(&mut self, lang: &str, e: &Event, w: &World, now: f64) {
        let typ = e.type_();
        let px = |index: i64| match w.players.iter().find(|p| p.index == index) {
            Some(p) => (p.x, true),
            None => (0.0, false),
        };
        let (fx, ok) = effect_for(lang, &e.data, typ, &px, &mut self.spots);
        if ok {
            self.spawn(fx, now);
        }
        if typ == "hit" {
            let (fx, ok) = damage_for(&e.data);
            if ok {
                self.spawn(fx, now);
            }
            let (fx, ok) = coin_loss_for(&e.data);
            // Schadenszahl hat Vorrang
            if ok && self.pieces + fx.pieces() < MAX_LIVE_PIECES {
                self.spawn(fx, now);
            }
        }
        if typ == "skillCast" {
            let (fx, ok) = cast_for(&e.data);
            if ok {
                self.spawn(fx, now);
            }
        }
    }

    /// `spawn` hängt einen Effekt an: höchstens `MAX_LIVE_PIECES` Teile, Blitze höchstens 3 je Sekunde (abgeschaltet
    /// keine), derselbe Text an derselben Stelle nur einmal (ein gehaltener Skill meldet castFailed je Takt).
    pub fn spawn(&mut self, mut e: Effect, now: f64) {
        if self.pieces + e.pieces() > MAX_LIVE_PIECES {
            return;
        }
        if !e.text.is_empty() && e.kind == Kind::NoTarget {
            for o in &self.effects {
                if o.text == e.text && o.x - e.x < 1.0 && e.x - o.x < 1.0 {
                    return;
                }
            }
        }
        if config(e.kind).flash {
            if self.no_flash || !flash_allowed(now, self.last_flash) {
                return;
            }
            self.last_flash = now;
        }
        e.start = now;
        self.pieces += e.pieces();
        self.effects.push(e);
    }
}

/// `cast_for` ist der Ring eines gewirkten Skills am Wirkort, ein Flächenzauber mit dem Radius des Skills.
pub fn cast_for(d: &SimEvent) -> (Effect, bool) {
    let l = skill_look(&str_of(d, "skill"));
    let (x, has_x) = num_of(d, "x");
    let Some(mut l) = l.filter(|_| has_x) else {
        return (Effect::none(), false);
    };
    let (r, _) = num_of(d, "radius");
    if r > 0.0 {
        l.radius = r * UNIT_PX;
    }
    let mut e = Effect::new(Kind::Skill, x, RING_PX);
    e.look = l;
    (e, true)
}
