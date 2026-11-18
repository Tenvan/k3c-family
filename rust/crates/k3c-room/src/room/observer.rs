//! Beobachter (B-352, TR4.2): ein Gerät ohne Platz, z. B. das Gerät von `sim_test` mit Clients. Port von `observer.go`.
//! Es bekommt Level und Zustand jeder Stufe, in der ein Monarch steht (ohne Monarchen die Startstufe), belegt keinen
//! Monarchen und nimmt keine Eingabe an (`input`, `add_slot`, `pause` → `bad_request`). Es zählt nicht als verbundenes
//! Gerät: Ein Raum nur mit Beobachtern rechnet nicht (wie ein leerer), wird aber nicht aufgeräumt, solange ein Beobachter
//! verbunden ist.
use std::collections::BTreeMap;

use super::env::{Env, Store};
use super::log::{Level, Log, short};
use super::manager::Manager;
use super::stages::observer_stages;
use super::types::{Device, Options, Peer, Room};
use crate::fehler::Fehler;

impl<S: Store, L: Log> Manager<S, L> {
    /// Go `Observe`: tritt dem Raum per Code als Beobachter bei; hatte das Gerät dort Plätze, sind sie danach frei.
    pub fn observe(&mut self, id: &str, peer: Box<dyn Peer>, code: &str) -> Result<(), Fehler> {
        let res = if self.shut {
            Err(Fehler::Closed)
        } else {
            let (rooms, mut env) = self.split();
            match rooms.get_mut(code) {
                Some(r) => r.locked_observe(&mut env, id, peer),
                None => Err(Fehler::RoomNotFound),
            }
        };
        self.log_rejected("observe", id, code, &res);
        res
    }

    /// Go `CreateObserved`: `create` ohne Platz; das Gerät legt den Raum an (oder tritt dem offenen Stand bei) und
    /// beobachtet.
    pub fn create_observed(
        &mut self,
        id: &str,
        peer: Box<dyn Peer>,
        name: &str,
        fresh: bool,
        depth: i64,
        opts: Options,
    ) -> Result<String, Fehler> {
        let device = id.to_string();
        self.create_with(id, name, fresh, depth, opts, &[], move |r, env| {
            r.locked_observe(env, &device, peer)
        })
    }
}

impl Room {
    fn locked_observe(&mut self, env: &mut Env, id: &str, peer: Box<dyn Peer>) -> Result<(), Fehler> {
        if self.closed {
            return Err(Fehler::RoomNotFound);
        }
        self.observe(env, id, peer);
        Ok(())
    }

    /// Go `observe`: nimmt das Gerät als Beobachter auf; ein früherer Eintrag desselben Geräts gibt seine Monarchen frei.
    pub fn observe(&mut self, env: &mut Env, id: &str, peer: Box<dyn Peer>) {
        let mut old = self.devices.remove(id);
        if let Some(o) = old
            .as_mut()
            .filter(|o| o.connected && o.peer.conn_id() != peer.conn_id())
        {
            o.peer.replaced();
        }
        let mut d = Device::new(peer, BTreeMap::new());
        d.observer = true;
        self.devices.insert(id.to_string(), d);
        let (device, devices, watching) = (short(id), self.connected(), self.watching());
        self.log(
            env,
            Level::Info,
            "🔌 Beobachter im Raum",
            &[("device", &device), ("geraete", &devices), ("beobachter", &watching)],
        );
        let world = self.isl.world_id().to_string();
        if let Some(d) = self.devices.get_mut(id) {
            d.peer.joined(&self.code, &self.name, &world, &[]);
        }
        self.push_state(env, id, None);
        match old {
            Some(o) if !o.slots.is_empty() => {
                for idx in o.slots.values() {
                    self.release(*idx);
                }
                self.after_disconnect(env); // Plätze melden und speichern wie beim Verlassen
            }
            _ => self.broadcast_seats(),
        }
    }

    /// Go `unobserve`: Der Beobachter geht (`leave` oder Abbruch). Ist danach kein Gerät mehr da, läuft die Frist
    /// `EMPTY_FOR_SECS`.
    pub fn unobserve(&mut self, env: &mut Env, id: &str) {
        self.devices.remove(id);
        let (device, watching) = (short(id), self.watching());
        self.log(
            env,
            Level::Info,
            "👋 Beobachter hat den Raum verlassen",
            &[("device", &device), ("beobachter", &watching)],
        );
        if self.connected() == 0 && self.watching() == 0 {
            // Frist ab jetzt, auch nach langem Beobachten eines leeren Raums
            self.empty_since = Some(env.now);
        }
    }

    /// Go `watching`: zählt die verbundenen Beobachter.
    pub fn watching(&self) -> usize {
        self.devices.values().filter(|d| d.connected && d.observer).count()
    }

    /// Go `observerStages`: die Stufen aller Monarchen, aufsteigend; ohne Monarchen die Startstufe.
    pub fn observer_stages(&self) -> Vec<usize> {
        observer_stages(&self.isl, self.monarchs.len(), self.start)
    }
}
