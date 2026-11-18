//! Port von `native/k3c-game/local/session.go`: Die Sitzung führt das native Spiel im selben Prozess über `k3c-room`: ein
//! Gerät mit bis zu `MAX_PLAYERS` Slots, synchron getaktet aus der Spielschleife (kein `Manager::run`, kein Thread je
//! Raum), Spielstände über `k3c_room::store`. Je laufendem Spiel gibt es einen eigenen `Manager` mit genau einem Raum;
//! stürzt der Takt ab, wird er verworfen, ohne zu speichern.
//!
//! Abweichungen von Go: Der Raum liegt im Manager (`Session::code` ist sein Schlüssel). `Manager` kennt weder `new_code`
//! (der Code ist zufällig, nicht `LOCL`), noch `worlds`, `world_list`, `close_room` (RU24.4); `close_room` unten ist
//! `Manager.CloseRoom` aus Go. Takt (`step.rs`) und Kampagne (`campaign.rs`) stehen seit RU27.2 daneben, Spielstände (`saves`) folgen in RU50.1. Aufgeteilt (≤ 400
//! Zeilen): `config.rs` (Config, Log, Uhr), `error.rs` (UserError, describe), `view_impl.rs` (`impl view::session::Session`).

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use k3c_room::fehler::Fehler as RoomFehler;
use k3c_room::room::{Env, Level, Log as _, Manager, Options, Room};
use k3c_room::store::Saves;
use k3c_sim::state::{PlayerCommand, World};

use super::campaign::Campaign;
use super::config::{Config, LogHandle, Zeit, clock, default_saves_dir, wall_clock};
use super::error::{UserError, describe, err_no_game, user_err};
use super::peer::{LocalPeer, PeerData, SharedPeer, StageState};
use super::route::RouteCache;
use super::settings::{SETTINGS_FILE, load_settings};
use super::watched::{SaveResults, SharedResults, WatchedStore};
use super::worlds::{game_data, worlds_fs};
use crate::view::MAX_PLAYERS;
use crate::view::frame::{Cue, CueKind, Outcome, Seat};
use crate::view::input::DeviceID;
use crate::view::settings::Settings;

/// Das eine Gerät des Prozesses im Raum.
pub const DEVICE_ID: &str = "local";
/// Verbindungs-Kennung des Geräts im Raum (je Spiel ein neuer Manager, also genügt eine feste Zahl).
pub const CONN: u64 = 1;

/// Der Manager eines Spiels.
pub type GameManager = Manager<WatchedStore, LogHandle>;

/// Ein geöffnetes Spiel: sein Manager, der Code des Raums und die Sicht auf den Peer.
pub struct Started {
    pub m: GameManager,
    pub code: String,
    pub peer: SharedPeer,
}

/// Session erfüllt `view::session::Session` und `Sim` (Takt in `step.rs`). Alle Methoden laufen in der Schleife. Die Felder
/// von `saves` und `dev` (RU50.1, Halt) stehen schon hier, damit die Struktur nur einmal entsteht.
pub struct Session {
    pub(super) dir: PathBuf,
    pub(super) saves: Saves,
    pub(super) results: SharedResults,
    pub(super) log: LogHandle,
    pub(super) now: Arc<dyn Fn() -> Zeit>,
    pub(super) m: Option<GameManager>,
    /// Code des Raums im Manager (leer ohne Spiel)
    pub(super) code: String,
    pub(super) peer: Option<SharedPeer>,
    /// Spielstand des laufenden Spiels
    pub(super) name: String,
    /// Welt-ID des laufenden Spiels (leer = Standard-Welt)
    pub(super) world: String,
    /// Ordner der lokalen Welten (`worlds_fs()` beim Öffnen)
    pub(super) worlds_dir: Option<PathBuf>,
    /// Inseln der Welt (`Frame::route`, route.rs)
    pub(super) route: RouteCache,
    /// Slot (= lokaler Spieler) → Gerät
    pub(super) devs: BTreeMap<i64, DeviceID>,
    /// Geräte des letzten Spiels, für `load` und `continue_game`
    pub(super) last: Vec<DeviceID>,
    pub(super) paused: bool,
    /// Halt der Entwickler-Taste F7 (dev)
    pub(super) halt: bool,
    /// im Halt: der nächste Step rechnet einen Takt
    pub(super) single: bool,
    /// Rechenzeit des letzten Takts (`Frame::step_ms`)
    pub(super) step_ms: f64,
    /// Aufrufe von `step`, Grundlage von `Frame::clock`
    pub(super) steps: u64,
    pub(super) cues: Vec<Cue>,
    /// Ausgang des laufenden bzw. zuletzt beendeten Spiels (`Frame::outcome`)
    pub(super) ended: Outcome,
    /// Aufnahme-Modus: Der Raum speichert synchron im Takt, `CueSaved` liegt darum ohne Warten im Frame des Takts
    #[allow(dead_code)]
    pub(super) recording: bool,
    pub(super) settings: Settings,
    pub(super) settings_path: PathBuf,
    /// Puffer von `command`, je Aufruf geleert
    pub(super) cmd: BTreeMap<i64, PlayerCommand>,
    /// Puffer von `Frame::seats`
    pub(super) seat_buf: Vec<Seat>,
    /// Die Stufen des letzten Takts (Welten, Ereignisse, Zeiten; Go: der `peer`)
    pub(super) stages: StageState,
    /// Stand der Kampagne (`campaign.rs`)
    pub(super) camp: Campaign,
}

/// Der Zustand der Stufen kommt aus dem Raum selbst, nicht als JSON durch `Peer::state` (RU27.2).
fn no_snapshot(_: &World, _: i64, _: bool) -> serde_json::Value {
    serde_json::Value::Null
}

impl Session {
    /// Legt die Session an und liest die Einstellungen (`SETTINGS_FILE`; fehlt oder kaputt: Standardwerte); der Ordner
    /// entsteht erst beim ersten Speichern.
    pub fn open(cfg: Config) -> Session {
        let dir = PathBuf::from(if cfg.dir.is_empty() {
            default_saves_dir()
        } else {
            cfg.dir
        });
        let mut log = LogHandle::new(cfg.log);
        let now: Arc<dyn Fn() -> Zeit> = cfg.now.map_or_else(|| Arc::new(wall_clock) as _, Arc::from);
        let settings_path = dir.join(SETTINGS_FILE);
        let (settings, err) = load_settings(&settings_path);
        if let Some(err) = err {
            let datei = settings_path.display().to_string();
            log.warn(
                "🩹 Einstellungen unlesbar, Standardwerte",
                &[("datei", &datei), ("err", &err)],
            );
        }
        Session {
            saves: Saves::new(dir.clone(), clock(&now)),
            dir,
            results: Arc::new(Mutex::new(SaveResults::default())),
            log,
            now,
            m: None,
            code: String::new(),
            peer: None,
            name: String::new(),
            world: String::new(),
            worlds_dir: worlds_fs(),
            route: RouteCache::default(),
            devs: BTreeMap::new(),
            last: Vec::new(),
            paused: false,
            halt: false,
            single: false,
            step_ms: 0.0,
            steps: 0,
            cues: Vec::new(),
            ended: Outcome::None,
            recording: cfg.recording,
            settings,
            settings_path,
            cmd: BTreeMap::new(),
            seat_buf: Vec::new(),
            stages: StageState::default(),
            camp: Campaign::default(),
        }
    }

    /// Legt den Manager eines Spiels an (Go `manager`): Dev = `settings.dev_mode`. Die Spieldaten lädt der erste Aufruf.
    /// `Manager` hat noch kein Feld für den Ordner der Welten, weitere Welten gehen darum nur über die Standard-Daten.
    pub(super) fn manager(&mut self) -> Result<GameManager, UserError> {
        let data = game_data()?;
        let store = WatchedStore::new(
            Saves::new(self.dir.clone(), clock(&self.now)),
            Arc::clone(&self.results),
        );
        let mut m = Manager::new(store, self.log.clone(), clock(&self.now), "local", data);
        m.dev = self.settings.dev_mode;
        m.snapshot = Some(no_snapshot);
        Ok(m)
    }

    /// Ruft `f` mit dem Raum des laufenden Spiels und seiner Umgebung auf (`None` ohne Spiel).
    pub(super) fn with_room<R>(&mut self, f: impl FnOnce(&mut Room, &mut Env) -> R) -> Option<R> {
        self.m.as_mut()?.with_room(&self.code, f)
    }

    /// Öffnet den Raum mit den Geräten `devices` (Slot i = Gerät i); `depth` ist die Start-Tiefe eines neuen Spiels.
    pub(super) fn start(
        &mut self,
        name: &str,
        fresh: bool,
        depth: i64,
        opts: Options,
        devices: &[DeviceID],
    ) -> Result<(), UserError> {
        if devices.is_empty() || devices.len() > MAX_PLAYERS {
            return Err(user_err(format!("1 bis {MAX_PLAYERS} Geräte nötig"), None));
        }
        if devices.iter().enumerate().any(|(i, d)| devices[..i].contains(d)) {
            return Err(user_err("Ein Gerät kann nur einen Spieler steuern", None));
        }
        if let Err(e) = self.close_game() {
            self.log.warn("💥 Altes Spiel nicht sauber beendet", &[("err", &e)]);
        }
        let mut m = self.manager()?;
        let shared: SharedPeer = Arc::new(Mutex::new(PeerData::default()));
        let peer = LocalPeer::new(CONN, Arc::clone(&shared));
        let slots: Vec<i64> = (0..devices.len() as i64).collect();
        let world = opts.world.clone();
        let code = m
            .create(DEVICE_ID, Box::new(peer), name, fresh, depth, &slots, opts)
            .map_err(|e| describe(&e, name))?;
        self.adopt(Started { m, code, peer: shared }, name, fresh, world, devices);
        Ok(())
    }

    /// Übernimmt den geöffneten Raum als laufendes Spiel (Go: der Rest von `start` nach `Create`); `world` ist die Welt eines
    /// neuen Spiels, bei einem geladenen die des Spielstands.
    pub(super) fn adopt(&mut self, st: Started, name: &str, fresh: bool, world: String, devices: &[DeviceID]) {
        let world = if fresh {
            world
        } else {
            self.saves
                .list()
                .into_iter()
                .find(|e| e.name == name)
                .map(|e| e.world)
                .unwrap_or_default()
        };
        self.devs = (0..).zip(devices.iter().copied()).collect();
        (self.m, self.code, self.peer, self.name) = (Some(st.m), st.code, Some(st.peer), name.to_string());
        (self.paused, self.ended, self.world) = (false, Outcome::None, world);
        self.stages = StageState::default();
        self.last = devices.to_vec();
        self.sync_stages();
        self.begin_campaign(name, fresh);
        self.log.info(
            "🚀 Spiel gestartet",
            &[("save", &name), ("neu", &fresh), ("spieler", &devices.len())],
        );
    }

    /// Speichert und beendet das laufende Spiel (Go `Close`).
    pub(super) fn close_game(&mut self) -> Result<(), UserError> {
        let Some(mut m) = self.m.take() else {
            return Ok(());
        };
        let name = self.name.clone();
        let res = close_room(&mut m, &self.code);
        self.reset();
        res.map_err(|e| describe(&e, &name))?;
        self.log.info("🛑 Spiel beendet", &[("save", &name)]);
        Ok(())
    }

    /// Vergisst das laufende Spiel, ohne zu speichern.
    pub(super) fn reset(&mut self) {
        (self.m, self.peer, self.paused, self.halt, self.single) = (None, None, false, false, false);
        self.code.clear();
        self.name.clear();
        self.devs.clear();
        self.stages = StageState::default();
    }

    /// Speichert das laufende Spiel sofort (synchron).
    pub(super) fn save_game(&mut self) -> Result<(), UserError> {
        if self.m.is_none() {
            return Err(err_no_game());
        }
        let name = self.name.clone();
        match self.with_room(|r, env| r.save_now(env)) {
            Some(Ok(_)) => Ok(()),
            Some(Err(e)) => Err(describe(&e, &name)),
            None => Err(describe(&RoomFehler::RoomNotFound, &name)),
        }
    }

    /// Hält die Simulation an bzw. lässt sie weiterlaufen; die gehaltenen Eingaben verfallen dabei (wie im Raum).
    pub(super) fn pause_game(&mut self, on: bool) {
        if self.m.is_none() || self.paused == on {
            return;
        }
        match self.with_room(|r, env| r.pause(env, DEVICE_ID, CONN, on)) {
            Some(Ok(())) => {}
            Some(Err(e)) => return self.log.error("💥 Pause abgelehnt", &[("err", &e)]),
            None => return,
        }
        self.paused = on;
        self.cue(if on { CueKind::Pause } else { CueKind::Resume }, -1, "");
        self.log.info("🚦 Pause", &[("an", &on)]);
    }

    /// Nimmt das Gerät `d` als weiteren Spieler auf; spielt es schon, liefert es seinen Spieler.
    pub(super) fn join_device(&mut self, d: DeviceID) -> Result<i64, UserError> {
        if self.m.is_none() {
            return Err(err_no_game());
        }
        if let Some((slot, _)) = self.devs.iter().find(|(_, dv)| **dv == d) {
            return Ok(*slot);
        }
        let Some(slot) = (0..MAX_PLAYERS as i64).find(|s| !self.devs.contains_key(s)) else {
            return Err(user_err(format!("Höchstens {MAX_PLAYERS} Spieler"), None));
        };
        let name = self.name.clone();
        let code = self.code.clone();
        let res = self
            .m
            .as_mut()
            .and_then(|m| m.rooms.get_mut(&code))
            .map(|r| r.add_slot(DEVICE_ID, CONN, slot));
        res.unwrap_or(Err(RoomFehler::RoomNotFound))
            .map_err(|e| describe(&e, &name))?;
        self.devs.insert(slot, d);
        self.sync_stages();
        self.last = self.devices();
        self.cue(CueKind::Joined, slot, "");
        self.log
            .info("👑 Spieler kommt", &[("player", &slot), ("device", &d.to_string())]);
        Ok(slot)
    }

    /// Lässt den Spieler gehen; sein Monarch ist frei. Der letzte Spieler bleibt (Spiel über `close` beenden).
    pub(super) fn leave_player(&mut self, player: i64) -> Result<(), UserError> {
        if self.m.is_none() {
            return Err(err_no_game());
        }
        if !self.devs.contains_key(&player) {
            return Err(user_err(format!("Spieler {} spielt nicht", player + 1), None));
        }
        if self.devs.len() == 1 {
            return Err(user_err(
                "Der letzte Spieler kann nicht gehen, das Spiel über das Menü beenden",
                None,
            ));
        }
        let name = self.name.clone();
        match self.with_room(|r, env| r.remove_slot(env, DEVICE_ID, CONN, player)) {
            Some(Ok(_)) => {}
            Some(Err(e)) => return Err(describe(&e, &name)),
            None => return Err(describe(&RoomFehler::RoomNotFound, &name)),
        }
        self.devs.remove(&player);
        self.sync_stages();
        self.last = self.devices();
        self.cue(CueKind::Left, player, "");
        self.log.info("👋 Spieler geht", &[("player", &player)]);
        Ok(())
    }

    /// Die Geräte der Spieler nach Slot.
    pub(super) fn devices(&self) -> Vec<DeviceID> {
        (0..MAX_PLAYERS as i64)
            .filter_map(|s| self.devs.get(&s).copied())
            .collect()
    }

    pub(super) fn cue(&mut self, kind: CueKind, player: i64, text: &str) {
        self.cues.push(Cue {
            kind,
            player,
            text: text.to_string(),
            device: DeviceID::default(),
        });
    }
}

/// `Manager.CloseRoom` aus Go (`closeroom.go`, `k3c-room` hat es noch nicht): Ablauf wie das Aufräumen eines leeren Raums:
/// speichern, Report schreiben, schließen, Test-Stand löschen; verbundene Geräte bekommen `closed`. Danach ist der Code
/// frei.
fn close_room(m: &mut GameManager, code: &str) -> Result<(), RoomFehler> {
    let (rooms, mut env) = m.split();
    let Some(mut r) = rooms.remove(code) else {
        env.log.info(
            "🚫 Raum schließen abgelehnt, kein Raum offen",
            &[("ns", &"room"), ("save", &code)],
        );
        return Err(RoomFehler::RoomNotFound);
    };
    let geraete = r.connected();
    r.log(
        &mut env,
        Level::Info,
        "🛑 Raum schließt",
        &[("grund", &"vom Gerät geschlossen"), ("geraete", &geraete)],
    );
    r.save_sync(&mut env);
    r.write_report(&mut env);
    r.closed = true;
    r.drop_test_save(&mut env);
    for d in r.devices.values_mut().filter(|d| d.connected) {
        d.peer.closed(false);
    }
    Ok(())
}
