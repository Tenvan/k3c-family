//! Raum schließen auf Wunsch eines Geräts (B-581, BD9.1). Port von `closeroom.go`. Ablauf wie das Aufräumen eines leeren
//! Raums (`locked_sweep`): speichern, Report schreiben, schließen, Test-Stand löschen; verbundene Geräte bekommen
//! `room_closed` und gehen zurück zur Raumliste. Danach ist der Code frei und `delete_save` möglich.
use super::env::{Env, Store};
use super::log::{Level, Log};
use super::manager::{Manager, valid_save_name};
use super::types::Room;
use crate::fehler::Fehler;

impl<S: Store, L: Log> Manager<S, L> {
    /// Schließt den offenen Raum mit dem Spielstand `name`. Ungültiger Name: `BadRequest`, kein Raum mit diesem Stand:
    /// `RoomNotFound`.
    pub fn close_room(&mut self, name: &str) -> Result<(), Fehler> {
        if !valid_save_name(name) {
            return Err(Fehler::BadRequest);
        }
        let (rooms, mut env) = self.split();
        let found = rooms.iter().find(|(_, r)| r.name == name).map(|(c, _)| c.clone());
        let Some(mut r) = found.and_then(|code| rooms.remove(&code)) else {
            env.log.info(
                "🚫 Raum schließen abgelehnt, kein Raum offen",
                &[("ns", &"room"), ("save", &name)],
            );
            return Err(Fehler::RoomNotFound);
        };
        r.close_by_device(&mut env);
        Ok(())
    }
}

impl Room {
    /// Go `closeByDevice`: speichert, schreibt den Report, schließt, löscht den Test-Stand und meldet `closed(false)`.
    pub fn close_by_device(&mut self, env: &mut Env) {
        let geraete = self.connected();
        self.log(
            env,
            Level::Info,
            "🛑 Raum schließt",
            &[("grund", &"vom Gerät geschlossen"), ("geraete", &geraete)],
        );
        self.save_sync(env);
        self.write_report(env);
        self.closed = true;
        self.drop_test_save(env);
        for d in self.devices.values_mut().filter(|d| d.connected) {
            d.peer.closed(false);
        }
    }
}
