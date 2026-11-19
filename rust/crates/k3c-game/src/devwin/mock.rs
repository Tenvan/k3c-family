//! Spielattrappe für die Tests der Spielseite (Einheits- und Integrationstests): zeichnet auf, was die Cheat-Pfade
//! erreicht, ohne ein Spiel zu starten.

use k3c_devproto::{Catalog, State};

use super::Host;
use crate::view::devkeys::{DevAction, DevCmd, DevSession};

/// MockSession ist eine `DevSession`, die alles aufzeichnet.
#[derive(Default)]
pub struct MockSession {
    /// Dev-Aktionen mit Spieler, wie sie `DevSession::dev` erreichten.
    pub log: Vec<(i64, DevAction)>,
    pub halted: bool,
    pub steps: i64,
    pub reloads: i64,
    /// Gesetzt: `dev` lehnt ab.
    pub reject: Option<String>,
}

impl DevSession for MockSession {
    fn dev(&mut self, player: i64, a: &DevAction) -> Result<(), String> {
        self.log.push((player, a.clone()));
        self.reject.clone().map_or(Ok(()), Err)
    }
    fn halt(&mut self, on: bool) {
        self.halted = on;
    }
    fn halted(&self) -> bool {
        self.halted
    }
    fn step_once(&mut self) {
        self.steps += 1;
    }
    fn reload(&mut self) -> Result<(), String> {
        self.reloads += 1;
        Ok(())
    }
}

/// MockHost ist ein `Host` mit Aufzeichnung.
#[derive(Default)]
pub struct MockHost {
    pub players: i64,
    pub session: Option<MockSession>,
    pub cmds: Vec<DevCmd>,
    pub learned: Vec<(i64, String)>,
    pub respecs: Vec<i64>,
    pub gotos: Vec<(String, i64, i64)>,
    pub focused: i64,
    pub state: State,
    pub catalog: Catalog,
    /// Wie oft `state` gebaut wurde.
    pub state_calls: std::cell::Cell<i64>,
}

impl MockHost {
    /// new baut ein Spiel mit `players` Spielern und Entwickler-Rolle.
    pub fn new(players: i64) -> MockHost {
        MockHost {
            players,
            session: Some(MockSession::default()),
            ..MockHost::default()
        }
    }

    /// log sind die aufgezeichneten Dev-Aktionen.
    pub fn log(&self) -> &[(i64, DevAction)] {
        self.session.as_ref().map_or(&[], |s| s.log.as_slice())
    }
}

impl Host for MockHost {
    fn player_count(&self) -> i64 {
        self.players
    }
    fn dev_session(&mut self) -> Option<&mut dyn DevSession> {
        self.session.as_mut().map(|s| s as &mut dyn DevSession)
    }
    fn learn(&mut self, player: i64, skill: &str) -> Result<(), String> {
        self.learned.push((player, skill.to_string()));
        Ok(())
    }
    fn respec(&mut self, player: i64) -> Result<(), String> {
        self.respecs.push(player);
        Ok(())
    }
    fn dev_cmd(&mut self, c: DevCmd) -> Result<(), String> {
        self.cmds.push(c);
        match c {
            DevCmd::Debug => self.state.debug_page = (self.state.debug_page + 1) % 3,
            DevCmd::Stats => self.state.perf = !self.state.perf,
            _ => {}
        }
        Ok(())
    }
    fn goto(&mut self, world: &str, island: i64, stage: i64) -> Result<(), String> {
        self.gotos.push((world.to_string(), island, stage));
        Ok(())
    }
    fn focus(&mut self) -> Result<(), String> {
        self.focused += 1;
        Ok(())
    }
    fn state(&self) -> State {
        self.state_calls.set(self.state_calls.get() + 1);
        self.state.clone()
    }
    fn catalog(&self) -> Catalog {
        self.catalog.clone()
    }
}
