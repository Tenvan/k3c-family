//! Port von `native/k3c-game/audio/mix.go`: der Mix-Planer. Er leitet aus dem Frame ab, was in diesem Takt klingt, wie
//! laut und für welche Spielerhälfte; reine Logik ohne Gerät, mit Zustand über Takte (Musik, Ducking, Hub).
//!
//! Abweichungen von Go: `plan` gibt `&Mix` auf den wiederverwendeten Puffer zurück; Slots der Effekte sind
//! `Cow<'static, str>` (Tabellen-Slots ohne Kopie, Skill-Slots als `String`); `Settings::normalized` wird für die zwei
//! Lautstärken nachgebildet, damit kein Takt die Eingabe-Profile kopiert.

use std::borrow::Cow;
use std::sync::Arc;

use k3c_sim::state::World;

use super::duck::{DuckWindow, duck_gain, extend_duck};
use super::events::{
    Ear, RIDE_GAIN, SKILL_CAST_EVENT, SLOT_HUB, attenuation, cue_slot, event_cue, event_x, night_gain, riding,
};
use super::music::{Fader, MusicState, MusicView, MusicVoice, TPS, next_music_state};
use super::register::Register;
use crate::view::cell::{Cell, world_share};
use crate::view::frame::{Event, Frame};

// Sichtbare Breite einer Zelle in Units, wenn der Frame keine Zelle des Spielers nennt (Frame.cells leer, z. B. in
// Tests): Ein Fenster 16:9 zeigt VIEW_UNITS (game/layout.go, 420/32) über `world_share` der Höhe, also etwa 35 Units je
// Spieler; im Split-Screen liegen die Zellen übereinander, jede ist halb so hoch und zeigt doppelt so viel Breite.
pub const VIEW_UNITS: f64 = 420.0 / 32.0;
pub const ASPECT: f64 = 16.0 / 9.0;

/// Sichtbare Breite einer Zelle in Units (Look-Profil: `world_share`).
pub fn span_per_seat() -> f64 {
    ASPECT * VIEW_UNITS / world_share()
}

/// Ein Effekt-Auftrag des Takts: Slot und Pegel 0 … 1 vor der Bus-Lautstärke.
#[derive(Clone, Debug, PartialEq)]
pub struct Request {
    pub slot: Cow<'static, str>,
    pub gain: f64,
}

/// Eine laufende Schleife (Hufschlag) je Monarch (`key` = `Player::index`) mit Pegel vor dem Bus.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Loop {
    pub key: i64,
    pub gain: f64,
}

/// Das Ergebnis eines Takts: was klingt, wie laut, welche Musik. Die Wiedergabe setzt es nur um.
#[derive(Clone, Debug, Default)]
pub struct Mix {
    /// je Slot höchstens einmal je Takt (Drosselung), der lauteste Auftrag gilt
    pub effects: Vec<Request>,
    /// gewünschte Hufschlag-Schleifen; fehlende enden
    pub loops: Vec<Loop>,
    /// [0] laufendes bzw. einblendendes Stück, [1] ausblendendes (mit Ducking)
    pub music: [MusicVoice; 2],
    /// Lautstärke der Effekte: Einstellung 0 … 1 mal Mischpegel (`Register::mix`)
    pub effect_bus: f64,
    /// Lautstärke der Musik: Einstellung 0 … 1 mal Mischpegel (`Register::mix`)
    pub music_bus: f64,
}

impl Mix {
    /// Lautstärke eines Effekts oder einer Schleife mit Pegel `gain` am Bus.
    pub fn effect_volume(&self, gain: f64) -> f64 {
        gain * self.effect_bus
    }

    /// Lautstärke eines Musikstücks mit Pegel `gain` am Bus.
    pub fn music_volume(&self, gain: f64) -> f64 {
        gain * self.music_bus
    }
}

/// Merkt die Hub-Stufe einer Stufe für den Ausbau-Klang (events.ts › hubRaised).
#[derive(Clone, Copy, Debug)]
struct HubMark {
    stage: i64,
    level: i64,
}

/// Der Planer; hält Puffer über Takte (`clear()` statt neu anlegen).
#[derive(Debug)]
pub struct Planner {
    pub reg: Arc<Register>,
    pub mix: Mix,
    /// Zahl der Listen-Aufrufe: Zeit für Übergänge, läuft auch in Menü und Pause
    pub tick: u64,
    pub state: MusicState,
    pub fade: Fader,
    pub duck: DuckWindow,
    hubs: Vec<HubMark>,
    next: Vec<HubMark>,
    ears: Vec<Ear>,
}

/// Lautstärke-Einstellung 0 … 1 (NaN = Standard 1), wie `Settings::normalized` für Musik und Effekte.
fn volume(v: f64) -> f64 {
    if v.is_nan() { 1.0 } else { v.clamp(0.0, 1.0) }
}

impl Planner {
    pub fn new(reg: Arc<Register>) -> Planner {
        Planner {
            reg,
            mix: Mix::default(),
            tick: 0,
            state: MusicState::None,
            fade: Fader::default(),
            duck: DuckWindow::default(),
            hubs: Vec::new(),
            next: Vec::new(),
            ears: Vec::new(),
        }
    }

    /// Rechnet den Mix eines Frames; der Mix gilt bis zum nächsten Aufruf (Puffer werden wiederverwendet).
    #[allow(clippy::cast_precision_loss)]
    pub fn plan(&mut self, f: &Frame) -> &Mix {
        self.tick += 1;
        let t = self.tick as f64 / TPS as f64;
        self.mix.effects.clear();
        self.mix.loops.clear();
        let mx = self.reg.mixing();
        self.mix.effect_bus = volume(f.settings.effects) * mx.effects;
        self.mix.music_bus = volume(f.settings.music) * mx.music;
        for c in &f.cues {
            let slot = cue_slot(c.kind);
            if !slot.is_empty() {
                self.add(Cow::Borrowed(slot), 1.0);
            }
        }
        let mut fallen = false;
        for e in &f.events {
            fallen = fallen || e.type_() == "castleFallen";
            self.event(f, e, t);
        }
        self.hub_levels(f);
        self.music(f, fallen, t);
        if !f.paused {
            self.loops(f);
        }
        &self.mix
    }

    /// Legt einen Effekt-Auftrag an; derselbe Slot im selben Takt klingt einmal, mit dem lautesten Pegel.
    pub fn add(&mut self, slot: Cow<'static, str>, gain: f64) {
        if gain <= 0.0 {
            return;
        }
        if let Some(r) = self.mix.effects.iter_mut().find(|r| r.slot == slot) {
            r.gain = r.gain.max(gain);
            return;
        }
        self.mix.effects.push(Request { slot, gain });
    }

    /// Bildet ein Ereignis auf einen Klang ab, gedämpft nach Abstand zu den Spielern seiner Stufe und Tageszeit.
    fn event(&mut self, f: &Frame, e: &Event, t: f64) {
        let typ = e.type_();
        let cue = if typ == SKILL_CAST_EVENT {
            let skill = e.data.get("skill").and_then(|v| v.as_str()).unwrap_or("");
            self.reg.skill_slot(skill).map(|s| (Cow::Owned(s), false))
        } else {
            event_cue(typ).map(|c| (Cow::Borrowed(c.slot), c.global))
        };
        let Some((slot, global)) = cue else { return };
        if slot.is_empty() {
            return;
        }
        if global {
            // Warnung senkt die Musik, auch wenn der Klang fehlt
            self.duck = extend_duck(&self.duck, t, self.reg.mixing().duck);
        }
        let w = f.stage(e.stage).and_then(|s| s.world.as_deref());
        let (x, has_x) = event_x(e, w);
        let g = attenuation(x, has_x, self.ears_of(f, e.stage), global) * night_gain(phase_of(w));
        self.add(slot, g);
    }

    /// Die Ausschnitte der lokalen Spieler, die in Stufe `stage` stehen: der sichtbare Bereich ihrer Zelle
    /// (`Frame::cells`, Kamera), ohne Zelle die feste Breite um ihren Monarchen.
    #[allow(clippy::cast_precision_loss)]
    pub fn ears_of(&mut self, f: &Frame, stage: i64) -> &[Ear] {
        self.ears.clear();
        let span = span_per_seat() * f.seats.len().max(1) as f64;
        for s in &f.seats {
            if s.stage != stage {
                continue;
            }
            if let Some(e) = cell_ear(&f.cells, s.player) {
                self.ears.push(e);
            } else if let Some(m) = f.monarch(s.player) {
                self.ears.push(Ear { center: m.x, span });
            }
        }
        &self.ears
    }

    /// Spielt den Hub-Ausbau-Klang, wenn die Hub-Stufe einer Stufe gegenüber dem vorigen Frame steigt
    /// (audioCore.ts › onHubLevel); eine neue oder wiederkehrende Stufe klingt beim ersten Sehen nicht.
    fn hub_levels(&mut self, f: &Frame) {
        self.next.clear();
        for st in &f.stages {
            let Some(w) = st.world.as_deref() else { continue };
            let lv = w.hub_level;
            if self.hubs.iter().any(|h| h.stage == st.index && lv > h.level) {
                self.add(Cow::Borrowed(SLOT_HUB), 1.0);
            }
            self.next.push(HubMark {
                stage: st.index,
                level: lv,
            });
        }
        std::mem::swap(&mut self.hubs, &mut self.next);
    }

    /// Bestimmt den Zustand und blendet über; Ducking wirkt auf beide Stücke.
    fn music(&mut self, f: &Frame, fallen: bool, t: f64) {
        self.state = next_music_state(self.state, &view_of(f), fallen);
        let mut slot = self.state.slot();
        if !self.reg.music.contains_key(slot) {
            slot = ""; // Lücke: Stille, das laufende Stück blendet aus
        }
        self.fade.target(slot);
        self.fade.step();
        let g = duck_gain(t, &self.duck);
        self.mix.music = self.fade.voices();
        self.mix.music[0].gain *= g;
        self.mix.music[1].gain *= g;
    }

    /// Sammelt den Hufschlag je reitendem Monarch der Stufen mit lokalen Spielern (events.ts › rideLoops).
    fn loops(&mut self, f: &Frame) {
        for st in &f.stages {
            let Some(w) = st.world.as_deref() else { continue };
            let night = night_gain(&w.cycle.phase);
            for m in &w.players {
                if !riding(Some(m)) {
                    continue;
                }
                let g = attenuation(m.x, true, self.ears_of(f, st.index), false) * night * RIDE_GAIN;
                if g > 0.0 {
                    self.mix.loops.push(Loop { key: m.index, gain: g });
                }
            }
        }
    }
}

/// Der sichtbare Bereich der Zelle des Spielers `player` (None ohne Zelle oder Maßstab).
pub fn cell_ear(cells: &[Cell], player: i64) -> Option<Ear> {
    let c = cells.iter().find(|c| c.player == player && c.camera.scale > 0.0)?;
    let (l, r) = c.camera.span();
    Some(Ear {
        center: (l + r) / 2.0,
        span: r - l,
    })
}

/// Die Tageszeit der Welt ("" ohne Welt).
pub fn phase_of(w: Option<&World>) -> &str {
    w.map_or("", |w| w.cycle.phase.as_str())
}

/// Der Auszug für die Musik (music.ts › musicViewOf) über alle Stufen mit lokalen Spielern: Ohne Stufe läuft das Menü
/// (Lobby); Gegner zählen in allen Stufen, Tiefe nur, wenn alle Spieler unter Tage stehen, die Tageszeit der ersten
/// Stufe gilt (der Zyklus läuft auf der Insel gemeinsam).
pub fn view_of(f: &Frame) -> MusicView<'_> {
    if f.stages.is_empty() {
        return MusicView {
            lobby: true,
            ..MusicView::default()
        };
    }
    let mut v = MusicView {
        depth: -1,
        ..MusicView::default()
    };
    for st in &f.stages {
        let Some(w) = st.world.as_deref() else { continue };
        if v.phase.is_empty() {
            v.phase = w.cycle.phase.as_str();
        }
        v.enemies += i64::try_from(w.enemies.len()).unwrap_or(i64::MAX);
        if v.depth < 0 || st.depth < v.depth {
            v.depth = st.depth;
        }
    }
    v.depth = v.depth.max(0);
    v
}

#[cfg(test)]
#[path = "mix_tests.rs"]
mod mix_tests;
