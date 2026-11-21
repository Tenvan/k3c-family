//! Cheat-Fenster (RU56.11, B-1135): F4 und `--devwin` starten `k3c-devwin.exe` neben dem Spiel, die Schleife ruft einmal
//! je Takt `DevWin::update`, und `Game` ist der `Host` der Befehle (dieselben Cheat-Pfade wie die F-Tasten).
//! Ohne Fenster ist `devwin_tick` ein einziger Vergleich: keine Threads, keine Allokation.

use std::collections::BTreeMap;
use std::mem::take;
use std::time::Instant;

use k3c_devproto::{Catalog, Entry, GameInfo, IslandInfo, PlayerState, State, WorldInfo};
use k3c_platform::log;

use super::window::toggled_window;
use super::{Game, Mode};
use crate::devtools::cheatlist::{EVENTS, GRADES, PHASES, RESOURCES, bosses};
use crate::devwin::{Host, open_pipe, spawn};
use crate::local::worlds::game_data;
use crate::view::devkeys::{DevCmd, DevSession, TIMESCALES};
use crate::view::frame::{Cue, CueKind};

/// DevWinState ist der Zustand des Cheat-Fensters im Spiel: Verbindung, Prozess und ob der Start per Schalter schon
/// geprüft ist.
#[derive(Default)]
pub(super) struct DevWinState {
    pub(super) win: crate::devwin::DevWin,
    pub(super) child: Option<std::process::Child>,
    pub(super) boot: bool,
}

impl Game {
    /// F4: das Fenster öffnen, läuft es schon, nach vorn holen.
    pub(super) fn dev_window(&mut self) {
        if self.dw.win.connected() {
            self.dw.win.focus();
        } else if self.dw.child.is_some() {
            log::info("🐛 Cheat-Fenster läuft schon, verbindet", &[]);
        } else {
            self.open_devwin();
        }
    }

    /// Startet das Fenster als eigenen Prozess und will die Verbindung; fehlt die EXE, steht ein Hinweis im Spiel.
    fn open_devwin(&mut self) {
        let res = std::env::current_exe().map_err(|e| e.to_string()).and_then(|exe| {
            let (pid, pipe) = (std::process::id(), spawn::pipe_name(std::process::id(), spawn::nonce()));
            spawn::spawn(&spawn::exe_path(&exe), &pipe, pid).map(|child| (child, pipe))
        });
        match res {
            Ok((child, pipe)) => {
                self.dw.child = Some(child);
                let info = GameInfo {
                    name: "k3c-game".to_string(),
                    version: crate::menu::version::version(),
                };
                self.dw.win.start(open_pipe(spawn::pipe_path(&pipe)), info);
                log::info("🚀 Cheat-Fenster gestartet", &[("pipe", &pipe)]);
            }
            Err(e) => {
                log::warn("🚫 Cheat-Fenster nicht gestartet", &[("err", &e)]);
                self.cues.push(Cue {
                    kind: CueKind::Denied,
                    player: -1,
                    text: format!("Cheat-Fenster: {e}"),
                    ..Cue::default()
                });
            }
        }
    }

    /// Ein Takt des Fensters: beim ersten Aufruf `--devwin` bzw. `K3C_DEVWIN=1`, danach Befehle und Zustand; das
    /// Ende des Fenster-Prozesses beendet die Verbindung (F4 öffnet es neu).
    pub(super) fn devwin_tick(&mut self) {
        if !self.dw.boot {
            self.dw.boot = true;
            let env = std::env::var("K3C_DEVWIN").is_ok_and(|v| v == "1");
            if self.args.shots.is_empty() && (self.args.devwin || env) {
                self.open_devwin();
            }
        }
        if !self.dw.win.is_on() {
            return;
        }
        let mut dw = take(&mut self.dw.win);
        dw.update(self, Instant::now());
        self.dw.win = dw;
        if self
            .dw
            .child
            .as_mut()
            .is_some_and(|c| c.try_wait().map_or(true, |s| s.is_some()))
        {
            self.dw.child = None;
            self.dw.win.stop();
            log::info("🛑 Cheat-Fenster beendet", &[]);
        }
    }

    /// Zustand der Entwickler-Anzeigen für `dev_cmd`: nur im laufenden Spiel und nicht im Aufnahme-Modus.
    fn dev_cmd_allowed(&self, c: DevCmd) -> Result<(), String> {
        if !self.args.shots.is_empty() {
            return Err("Im Aufnahme-Modus gilt keine Entwickler-Taste".to_string());
        }
        let session = matches!(c, DevCmd::Save | DevCmd::Load | DevCmd::Pause | DevCmd::Speed);
        if session && (!self.sim.active() || self.mode != Mode::Play) {
            return Err("Nur im laufenden Spiel (nicht im Menü)".to_string());
        }
        Ok(())
    }
}

impl Host for Game {
    fn player_count(&self) -> i64 {
        i64::try_from(self.frame.seats.len()).unwrap_or(0)
    }

    fn dev_session(&mut self) -> Option<&mut dyn DevSession> {
        // nur im laufenden Spiel, nicht unter einem Menü (wie die F-Tasten; F6 hebt sonst dessen Pause auf)
        if !self.sim.active() || self.mode != Mode::Play {
            return None;
        }
        self.sim.dev_session()
    }

    fn learn(&mut self, player: i64, skill: &str) -> Result<(), String> {
        self.sim.learn(player, skill).map_err(|e| e.to_string())
    }

    fn respec(&mut self, player: i64) -> Result<(), String> {
        self.sim.respec(player).map_err(|e| e.to_string())
    }

    fn dev_cmd(&mut self, c: DevCmd) -> Result<(), String> {
        self.dev_cmd_allowed(c)?;
        if c == DevCmd::Save {
            // ehrliches ack: der Speicherfehler kommt zurück statt nur als Hinweis im Spiel
            return self.sim.save().map_err(|e| e.to_string());
        }
        let mut set = self.frame.settings.clone();
        if c == DevCmd::Fullscreen {
            set.window = toggled_window(&set.window).to_string();
        } else if !self.dev_command(c, &mut set) {
            return Err("Diese Taste kennt das Fenster nicht".to_string());
        }
        if set != self.frame.settings {
            self.sim.set_settings(set).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    fn goto(&mut self, _world: &str, _island: i64, _stage: i64) -> Result<(), String> {
        Err(
            "Direkter Wechsel von Welt, Insel und Stufe: die Session kennt nur den Inselwechsel am Hafen (Ticket)"
                .to_string(),
        )
    }

    fn focus(&mut self) -> Result<(), String> {
        Err("Spielfenster nach vorn: die Plattform bietet das noch nicht (Ticket)".to_string())
    }

    fn state(&mut self) -> State {
        let halted = self.sim.dev_session().is_some_and(|d| d.halted());
        let f = &self.frame;
        let world_of = |stage: i64| f.stage(stage).and_then(|st| st.world.as_deref());
        let first = f.seats.first();
        let players = f
            .seats
            .iter()
            .map(|s| PlayerState {
                gold: f.monarch(s.player).map_or(0, |m| m.gold),
                stock: world_of(s.stage).map_or_else(BTreeMap::new, |w| {
                    let k = &w.stock;
                    [
                        ("wood", k.wood),
                        ("stone", k.stone),
                        ("copper", k.copper),
                        ("iron", k.iron),
                        ("crystal", k.crystal),
                    ]
                    .into_iter()
                    .map(|(n, v)| (n.to_string(), v))
                    .collect()
                }),
            })
            .collect();
        State {
            world: self.sim.world(),
            island: f
                .route
                .iter()
                .position(|r| r.id == f.campaign.island)
                .map_or(-1, |i| i64::try_from(i).unwrap_or(-1)),
            stage: first.map_or(0, |s| s.stage),
            phase: first
                .and_then(|s| world_of(s.stage))
                .map(|w| w.cycle.phase.clone())
                .unwrap_or_default(),
            timescale: f.timescale.max(1),
            halted,
            players,
            debug_page: self.dev.debug,
            perf: f.settings.stats,
            river: f.settings.river.clone(),
            ticks_per_s: (self.dev.tps * 10.0).round() / 10.0,
        }
    }

    fn catalog(&self) -> Catalog {
        let entry = |id: &str, label: &str| Entry {
            id: id.to_string(),
            label: label.to_string(),
        };
        let current = self.sim.world();
        let islands: Vec<IslandInfo> = self
            .frame
            .route
            .iter()
            .map(|r| IslandInfo {
                name: r.name.clone(),
                stages: r.stages,
            })
            .collect();
        let worlds = self
            .sim
            .world_list()
            .into_iter()
            .map(|w| WorldInfo {
                islands: if w.id == current { islands.clone() } else { Vec::new() },
                id: w.id,
                name: w.name,
            })
            .collect();
        let mut cat = Catalog {
            worlds,
            bosses: bosses().iter().map(|(id, name)| entry(id, name)).collect(),
            events: EVENTS.iter().map(|(id, name)| entry(id, name)).collect(),
            resources: RESOURCES.iter().map(|(id, name)| entry(id, name)).collect(),
            phases: PHASES.iter().map(|(id, _)| (*id).to_string()).collect(),
            grades: GRADES.iter().map(|(id, _)| (*id).to_string()).collect(),
            timescales: TIMESCALES.to_vec(),
            effects: crate::world::fx::EFFECTS
                .iter()
                .map(|(id, name)| entry(&format!("fx:{id}"), name))
                .collect(),
            ..Catalog::default()
        };
        if let Ok(d) = game_data() {
            cat.skills = d.monarch.skills.keys().map(|id| entry(id, id)).collect();
            cat.units = d
                .enemies
                .iter()
                .map(|(id, e)| entry(&format!("enemy:{id}"), &format!("Gegner: {}", e.name)))
                .chain(
                    d.troops
                        .keys()
                        .map(|id| entry(&format!("troop:{id}"), &format!("Truppe: {id}"))),
                )
                .collect();
        }
        cat
    }
}
