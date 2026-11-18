//! Port von `native/k3c-game/fx/fx.go` (Paket fx): Effekte der Welt (Sprint NG6, B-799).
//!
//! Zeichnet die Effekte der Welt wie der Browser (src/scenes/effects/): Ring und Partikel zu den Ereignissen der
//! Simulation (Treffer, Tod, Münzen, Bau, gefallen, Schlag, „kein Ziel“), Skill-Wirkungen am Wirkort und beim Lernen
//! passiver Skills, Status-Effekte an Figuren, Warnkreise vor Flächenschlägen, Geschosse und Schadenszahlen.
//!
//! Aufbau: Listen (Listener) legt je Frame die Effekte der Ereignisse an und lässt abgelaufene fallen (Zeit nur aus
//! `Frame::clock`, kein Zufall); effects, looks und status rechnen rein und getestet, paint und paintmarks zeichnen.
//! Draw ändert keinen Zustand außer gemessenen Textbreiten.
//!
//! Abweichung von Go: `Drawer::draw` hat nur `&self`; der `Painter` (Vorlagen, gemessene Textbreiten) liegt in einer
//! `RefCell`, die Bibliothek (Schrift, Münz-Sheet) ebenso. `sim.AurasOf` kommt als `AuraSource` in `Fx::new`
//! (Standard: `pending::aura_of`, bis RU18 liefert), damit Tests und `still` eine feste Quelle setzen können.

use std::cell::RefCell;
use std::rc::Rc;

use k3c_platform::Canvas;
use k3c_sim::state::{Player, World};

use crate::art::Library;
use crate::view::cell::Cell;
use crate::view::frame::{Frame, Stage};
use crate::view::layer::{Drawer, Layer, Listener};
use crate::view::roles::AssetSource;
use looks::skills;
use paint::Painter;
use pending::{Aura, aura_of};
use stage::StageFx;
use status::status_marks;

pub mod effects;
pub mod looks;
pub mod paint;
pub mod paintmarks;
pub mod pending;
pub mod stage;
pub mod status;
pub mod texts;

#[cfg(test)]
mod effects_tests;
#[cfg(test)]
mod fx_tests;
#[cfg(test)]
mod status_tests;
#[cfg(test)]
mod testutil;
#[cfg(test)]
mod textrule_tests;

/// Das Münz-Sheet des Münzschauers (data/paper.json › muenze); props meldet es ebenfalls an, die Library lädt es einmal.
pub const COIN_ASSET: &str = "icons/muenze";
pub const COIN_ANIM: &str = "idle";

/// `AuraSource` ist `sim.AurasOf`: die sichtbaren Passive eines Monarchen.
pub type AuraSource = Rc<dyn Fn(&World, &Player) -> Vec<Aura>>;

/// Die Quelle von Go (`sim.AurasOf`), bis RU18 sie liefert.
pub fn default_aura_source() -> AuraSource {
    Rc::new(aura_of)
}

/// `Learned` ist der zuletzt gesehene Stand der gelernten Skills eines Monarchen (für den Lern-Ring passiver Skills).
#[derive(Debug, Default, Clone)]
pub struct Learned {
    pub set: bool,
    pub stage: i64,
    pub skills: Vec<String>,
}

/// `Fx` ist der Beitrag des Pakets: Drawer (`Layer::EFFECTS`), Listener und AssetSource (Go: `drawer`).
pub struct Fx {
    /// `None` (Tests): es fehlen nur die Texte und die Münzen
    pub lib: Option<Rc<RefCell<Library>>>,
    /// Stufen des laufenden Frames
    pub stages: Vec<StageFx>,
    /// alle früheren (Wiederverwendung)
    pub spare: Vec<StageFx>,
    /// je Monarch-Index
    pub learned: Vec<Learned>,
    pub cells: usize,
    pub paint: RefCell<Painter>,
    aura_source: AuraSource,
}

/// `new` liefert den Drawer der Effekte; lib (Grafik) darf `None` sein, dann fehlen nur die Texte.
pub fn new(lib: Option<Rc<RefCell<Library>>>, aura_source: AuraSource) -> Fx {
    Fx {
        lib,
        stages: Vec::new(),
        spare: Vec::new(),
        learned: Vec::new(),
        cells: 0,
        paint: RefCell::new(Painter::default()),
        aura_source,
    }
}

impl AssetSource for Fx {
    /// Assets meldet die Münze für jedes Level.
    fn assets(&self, _level: &str, _w: &World) -> Vec<String> {
        vec![COIN_ASSET.to_string()]
    }
}

impl Listener for Fx {
    /// Listen legt die Effekte der Ereignisse des Frames an.
    fn listen(&mut self, f: &Frame) {
        self.paint.get_mut().outline.sweep();
        let lang = if f.settings.language == crate::view::settings::LANGUAGE_ENGLISH {
            crate::view::settings::LANGUAGE_ENGLISH
        } else {
            crate::view::settings::DEFAULT_LANGUAGE
        };
        self.cells = f.seats.len().min(crate::view::MAX_PLAYERS);
        let mut keep: Vec<StageFx> = Vec::new();
        for st in &f.stages {
            let Some(w) = st.world.as_deref() else {
                continue;
            };
            let mut s = self.stage_for(st.index);
            s.expire(f.clock);
            s.no_flash = f.settings.flashes_off;
            for e in f.events_of(st.index) {
                s.event(lang, e, w, f.clock);
            }
            self.learn_rings(&mut s, st, w, f.clock);
            let src = Rc::clone(&self.aura_source);
            let cache = RefCell::new(&mut s.auras);
            let marks = std::mem::take(&mut s.marks);
            s.marks = status_marks(marks, w, &|p: &Player| cache.borrow_mut().of(w, p, &*src).to_vec());
            keep.push(s);
        }
        // Stufen, die der Frame nicht mehr nennt, bleiben als Vorrat (Go: `spare` kennt alle)
        self.spare.append(&mut self.stages);
        self.stages = keep;
    }
}

impl Fx {
    /// `stage_for` liefert die Effekte der Stufe i aus dem letzten Frame oder neue (nach einer Lücke leer); die Stufe
    /// verlässt die Listen und kommt am Ende des Frames zurück (Go teilt einen Zeiger).
    fn stage_for(&mut self, i: i64) -> StageFx {
        if let Some(n) = self.stages.iter().position(|s| s.index == i) {
            return self.stages.remove(n);
        }
        if let Some(n) = self.spare.iter().position(|s| s.index == i) {
            let mut s = self.spare.remove(n);
            s.effects.clear();
            s.pieces = 0;
            return s;
        }
        StageFx::new(i)
    }

    /// `stage` liefert die Effekte der Stufe i des laufenden Frames (`None`, wenn Listen sie nicht kennt).
    pub fn stage(&self, i: i64) -> Option<&StageFx> {
        self.stages.iter().find(|s| s.index == i)
    }

    /// `learn_rings`: Ein passiver Skill, der neu gelernt ist, zeigt seinen Ring am Monarchen; nach einem Wechsel der
    /// Stufe zählt der erste Stand nicht als Lernen.
    fn learn_rings(&mut self, s: &mut StageFx, st: &Stage, w: &World, now: f64) {
        for p in &w.players {
            if p.index < 0 {
                continue;
            }
            let idx = usize::try_from(p.index).unwrap_or(0);
            while self.learned.len() <= idx {
                self.learned.push(Learned::default());
            }
            let l = &mut self.learned[idx];
            if l.set && l.stage == st.index {
                for id in &p.skills {
                    if !l.skills.contains(id)
                        && skills().get(id).is_some_and(|k| k.kind == "passive")
                        && let Some(lk) = looks::skill_look(id)
                    {
                        let mut e = effects::Effect::new(effects::Kind::Skill, p.x, effects::RING_PX);
                        e.look = lk;
                        s.spawn(e, now);
                    }
                }
            }
            l.set = true;
            l.stage = st.index;
            l.skills.clone_from(&p.skills);
        }
    }
}

impl Drawer for Fx {
    fn layer(&self) -> Layer {
        Layer::EFFECTS
    }

    /// Draw zeichnet Status, Warnkreise, Geschosse und Effekte der Stufe, in der der Spieler der Zelle steht.
    fn draw(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        let Some(seat) = f.seat(c.player) else {
            return;
        };
        let Some(w) = f.world(c) else {
            return;
        };
        let mut paint = self.paint.borrow_mut();
        let st = paint.stencils(dst);
        let s = self.stage(seat.stage);
        let k = scale_of(c, self.cells);
        if let Some(s) = s {
            st.marks(dst, &s.marks, c, f.clock, true);
        }
        paint.warn(dst, w, c);
        paint.projectiles(dst, w, c);
        if let Some(s) = s {
            let mut lib = self.lib.as_ref().map(|l| l.borrow_mut());
            paint.effects(dst, &s.effects, lib.as_deref_mut(), c, f.clock, k);
            st.marks(dst, &s.marks, c, f.clock, false);
        }
    }
}

/// `scale_of` ist Fensterhöhe / 1080 (Texte in fester Größe wie im Browser); die Zellen liegen übereinander.
#[allow(clippy::cast_precision_loss)]
pub fn scale_of(c: &Cell, cells: usize) -> f64 {
    let h = (c.rect.dy() * cells.max(1) as i64) as f64;
    if h <= 0.0 {
        return 1.0;
    }
    h / 1080.0
}
