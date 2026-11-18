//! Das lokale Gerät im Raum (`room::Peer`) und die Stufen eines Takts. Port von `peer` und `stageTime` aus
//! `native/k3c-game/local/step.go` (Teilung an der Stelle des Abschnittskommentars, Leitfaden § 2).
//!
//! Abweichung von Go: Der Raum besitzt den Peer (`Box<dyn Peer>`), die Session liest ihn über ein geteiltes
//! `Arc<Mutex<PeerData>>`. `Peer::state` bekommt nur JSON (`serde_json::Value`), keine Welt; die Session setzt deshalb den
//! `Manager::snapshot` auf `Null`. Der Peer merkt sich nur, welche Stufen in diesem Takt einen Zustand bekamen
//! (`PeerData::active`); die Welten dieser Stufen liest `Session::step` danach aus dem Raum und hält sie in `StageState`
//! (Go: `peer.worlds`), die nur der Session gehört (`Rc`, nicht geteilt).

use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use k3c_level::types::Layout;
use k3c_room::room::{MonarchState, Peer, Seat};
use k3c_sim::state::World;

use super::watched::lock;
use crate::view::frame::{Event, Stage};

/// Was der Raum dem Gerät zuletzt gemeldet hat.
#[derive(Default)]
pub struct PeerData {
    /// Plätze des Geräts (nach `joined` und jedem `seats`)
    pub seats: Vec<Seat>,
    /// Raum wurde geschlossen
    pub closed: bool,
    /// Takt des letzten Zustands (Go: `peer.tick`)
    pub tick: u64,
    /// Stufen, die seit `begin` einen Zustand bekamen, in der Reihenfolge des Raums
    pub active: Vec<i64>,
    /// Stufen, die seit `begin` ein Level bekamen (neue Stufe oder neue Insel: ihre Welt ist eine andere)
    pub replaced: Vec<i64>,
    /// Test-Naht: läuft in `state` (Go: `peer.onState`)
    pub on_state: Option<Box<dyn FnMut() + Send>>,
}

impl PeerData {
    /// Vergisst die Stufen des letzten Takts (Go: `peer.begin` leert die Welten).
    pub fn begin(&mut self) {
        self.active.clear();
        self.replaced.clear();
    }
}

/// Geteilte Sicht der Session auf den Peer.
pub type SharedPeer = Arc<Mutex<PeerData>>;

/// Das eine Gerät des Prozesses (`conn` ist seine Verbindungs-Kennung im Raum).
pub struct LocalPeer {
    conn: u64,
    data: SharedPeer,
}

impl LocalPeer {
    pub fn new(conn: u64, data: SharedPeer) -> LocalPeer {
        LocalPeer { conn, data }
    }
}

impl Peer for LocalPeer {
    fn conn_id(&self) -> u64 {
        self.conn
    }

    fn joined(&mut self, _room: &str, _name: &str, _world: &str, you: &[Seat]) {
        lock(&self.data).seats = you.to_vec();
    }

    fn level(&mut self, stage: i64, _depth: i64, _layout: &Layout) {
        lock(&self.data).replaced.push(stage);
    }

    fn state(&mut self, tick: i64, stage: i64, _state: &serde_json::Value) {
        // Die Naht läuft ohne Sperre (sie darf panicken, ohne den Peer zu vergiften)
        let hook = lock(&self.data).on_state.take();
        if let Some(mut f) = hook {
            f();
            lock(&self.data).on_state = Some(f);
        }
        let mut d = lock(&self.data);
        d.tick = u64::try_from(tick).unwrap_or(0);
        d.active.push(stage);
    }

    fn seats(&mut self, you: &[Seat], _monarchs: &[MonarchState]) {
        lock(&self.data).seats = you.to_vec();
    }

    fn replaced(&mut self) {}

    fn closed(&mut self, _final: bool) {
        lock(&self.data).closed = true;
    }
}

/// Die Spielzeit einer Stufe nach einem Takt (Go: `stageTime`; die Welt-Identität ersetzt `PeerData::replaced`).
#[derive(Clone, Copy, Debug)]
struct StageTime {
    stage: i64,
    t: f64,
}

/// Die Stufen des letzten Takts (Go: `peer.worlds`, `events`, `times`, `stageBuf`).
#[derive(Default)]
pub struct StageState {
    /// Stufe → Welt des letzten Zustands (eine Kopie aus dem Raum, siehe `Session::step`)
    pub worlds: BTreeMap<i64, Rc<World>>,
    /// Ereignisse seit dem letzten Frame
    pub events: Vec<Event>,
    /// Stufen mit neuer Welt (Level gesendet) im letzten Takt
    pub replaced: Vec<i64>,
    times: Vec<StageTime>,
    time_buf: Vec<StageTime>,
    stage_buf: Vec<Stage>,
}

impl StageState {
    /// Leert die Stufen vor einem Takt: Stufen ohne Spieler bekommen keinen Zustand mehr.
    pub fn begin(&mut self) {
        self.worlds.clear();
        self.replaced.clear();
    }

    /// Merkt sich die Welt der Stufe und ihre Ereignisse; `saved` ist kein Ereignis der Simulation (Speichern meldet die
    /// Session als Cue).
    pub fn absorb(&mut self, tick: u64, stage: i64, w: &World) {
        self.worlds.insert(stage, Rc::new(w.clone()));
        for e in &w.events {
            if e.get("type").and_then(|t| t.as_str()) != Some("saved") {
                self.events.push(Event {
                    stage,
                    tick,
                    data: e.clone(),
                });
            }
        }
    }

    /// Meldet nach einem Takt, ob eine Stufe mit derselben Welt wie im vorigen Takt stehen geblieben ist (Spielzeit
    /// gleich), und merkt sich die Zeiten. `sim::step` zählt die Zeit in jedem Schritt weiter; steht sie, ruht die Insel.
    /// Eine Stufe mit neuer Welt (`replaced`, z. B. nach einem Inselwechsel) steht nie.
    pub fn stalled(&mut self) -> bool {
        let mut out = false;
        self.time_buf.clear();
        for (&i, w) in &self.worlds {
            let fresh = self.replaced.contains(&i);
            out = out || (!fresh && self.times.iter().any(|old| old.stage == i && old.t == w.time));
            self.time_buf.push(StageTime { stage: i, t: w.time });
        }
        std::mem::swap(&mut self.times, &mut self.time_buf);
        out
    }

    /// Die Stufen mit Welt nach Index (im Puffer).
    pub fn stages(&mut self) -> &[Stage] {
        self.stage_buf.clear();
        for (&i, w) in &self.worlds {
            self.stage_buf.push(Stage {
                index: i,
                depth: w.biome.depth,
                level: w.biome.id.clone(),
                world: Some(Rc::clone(w)),
            });
        }
        &self.stage_buf
    }

    /// Kapazität des Stufen-Puffers (Test: bleibt über Takte gleich).
    #[cfg(test)]
    pub fn stage_capacity(&self) -> usize {
        self.stage_buf.capacity()
    }

    /// Meldet, ob ein Platz in einer Stufe mit offenem Wechselpunkt steht (Go: `peer.atGate`).
    pub fn at_gate(&self, seats: &[Seat]) -> bool {
        seats
            .iter()
            .any(|st| self.worlds.get(&st.stage).is_some_and(|w| w.island_switch.is_some()))
    }
}
