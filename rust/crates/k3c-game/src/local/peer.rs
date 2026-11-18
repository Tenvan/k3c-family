//! Das lokale Gerät im Raum (`room::Peer`). Port des Rumpfs von `peer` aus `native/k3c-game/local/step.go`; RU27.2
//! ergänzt Welten, Ereignisse und Stufen des Takts.
//!
//! Abweichung von Go: Der Raum besitzt den Peer (`Box<dyn Peer>`), die Session liest ihn über ein geteiltes
//! `Arc<Mutex<PeerData>>`. `Peer::state` bekommt nur JSON (`serde_json::Value`), keine Welt; die Session setzt deshalb den
//! `Manager::snapshot` auf `Null` und liest die Welten der Stufen später direkt aus dem Raum (RU27.2).

use std::sync::{Arc, Mutex};

use k3c_level::types::Layout;
use k3c_room::room::{MonarchState, Peer, Seat};

use super::watched::lock;

/// Was der Raum dem Gerät zuletzt gemeldet hat.
#[derive(Debug, Default)]
pub struct PeerData {
    /// Plätze des Geräts (nach `joined` und jedem `seats`)
    pub seats: Vec<Seat>,
    /// Raum wurde geschlossen
    pub closed: bool,
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

    fn level(&mut self, _stage: i64, _depth: i64, _layout: &Layout) {}

    fn state(&mut self, _tick: i64, _stage: i64, _state: &serde_json::Value) {}

    fn seats(&mut self, you: &[Seat], _monarchs: &[MonarchState]) {
        lock(&self.data).seats = you.to_vec();
    }

    fn replaced(&mut self) {}

    fn closed(&mut self, _final: bool) {
        lock(&self.data).closed = true;
    }
}
