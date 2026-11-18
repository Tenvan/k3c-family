//! Der Manager hält alle Räume: anlegen, beitreten, aufräumen, schließen. Port von `engine/room/manager.go`.
//!
//! Gos `sync.Mutex`, `context` und Goroutinen entfallen: der Manager ist ein Struct mit `&mut self`-Methoden, den Takt
//! gibt der Aufrufer vor (`tick_all`, `sweep_if_due` in `run.rs`). Uhr, Store und Log gehören dem Manager und kommen als
//! `Env` in jede Raum-Methode. Aufruf-Reihenfolgen und Texte wie in Go.
use std::collections::BTreeMap;
use std::sync::Arc;

use k3c_data::Data;
use k3c_rng::Rng;
use k3c_sim::island::{Island, IslandOptions, default_options_for, set_options};

use super::env::{Env, SnapshotFn, Store};
use super::log::{Level, Log, list_text, short};
use super::run::Failure;
use super::stages::{free_monarchs, start_stage};
use super::types::{MAX_ROOMS, MAX_SEED_RUNES, Options, Peer, Room};
use super::{Info, valid_slots};
use crate::fehler::Fehler;
use crate::store::{Stamp, is_slot};

/// Großbuchstaben ohne I und O (`docs/protocol.md` › Begriffe).
pub const CODE_LETTERS: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ";

/// Hält alle Räume eines Servers. `S` ist der Spielstand-Speicher (`Saves` oder ein Test-Speicher), `L` das Log.
pub struct Manager<S: Store, L: Log> {
    pub store: S,
    pub log: L,
    /// Uhr des Managers (Wanduhr für Fristen und Stempel); der Aufrufer gibt sie vor.
    pub now: Box<dyn FnMut() -> Stamp>,
    /// Spiel-Daten (Schwierigkeitsgrade usw.).
    pub data: Arc<Data>,
    /// Dev-Mode (`K3C_DEV`): Grad dev wählbar, Standardgrad eines neuen Raums ist dann dev.
    pub dev: bool,
    /// Baut den Zustand einer Stufe für `Peer::state`; `None` = die Welt selbst als JSON (Tests).
    pub snapshot: Option<SnapshotFn>,
    pub rooms: BTreeMap<String, Room>,
    /// Die letzten abgestürzten Räume (höchstens 10).
    pub failures: Vec<Failure>,
    /// Nach `close`: keine neuen Räume.
    pub shut: bool,
    pub(super) rng: Rng,
    pub(super) last_sweep: Option<Stamp>,
    pub(super) last_stats: Option<Stamp>,
}

/// Ist `name` ein gültiger Spielstandname (`^[a-z0-9-]{1,32}$`)?
pub fn valid_save_name(name: &str) -> bool {
    is_slot(name)
}

/// Der Seed eines neuen Stands: `create.seed`, ohne Angabe der Name (B-095); zu lang: `Fehler::BadRequest`.
pub fn seed_of(name: &str, opts: &Options) -> Result<String, Fehler> {
    if opts.seed.is_empty() {
        return Ok(name.to_string());
    }
    if opts.seed.chars().count() > MAX_SEED_RUNES {
        return Err(Fehler::BadRequest);
    }
    Ok(opts.seed.clone())
}

impl<S: Store, L: Log> Manager<S, L> {
    /// Legt einen Manager ohne Räume an. `seed` seedet den Zufall der Raum-Codes (der Aufrufer wählt ihn).
    pub fn new(store: S, log: L, now: impl FnMut() -> Stamp + 'static, seed: &str, data: Arc<Data>) -> Self {
        Manager {
            store,
            log,
            now: Box::new(now),
            data,
            dev: false,
            snapshot: None,
            rooms: BTreeMap::new(),
            failures: Vec::new(),
            shut: false,
            rng: Rng::new(seed),
            last_sweep: None,
            last_stats: None,
        }
    }

    /// Teilt den Manager in die Räume und die Umgebung für einen Raum-Aufruf (die Uhr wird einmal gelesen).
    pub fn split(&mut self) -> (&mut BTreeMap<String, Room>, Env<'_>) {
        let now = (self.now)();
        let env = Env {
            now,
            data: &self.data,
            store: &mut self.store,
            log: &mut self.log,
            dev: self.dev,
            snapshot: self.snapshot,
        };
        (&mut self.rooms, env)
    }

    /// Ruft `f` mit dem Raum `code` und seiner Umgebung auf (`None`, wenn es den Raum nicht gibt).
    pub fn with_room<R>(&mut self, code: &str, f: impl FnOnce(&mut Room, &mut Env) -> R) -> Option<R> {
        let (rooms, mut env) = self.split();
        rooms.get_mut(code).map(|r| f(r, &mut env))
    }

    /// Ein neuer Raum-Code aus vier Buchstaben, der noch nicht vergeben ist.
    pub fn code(&mut self) -> String {
        let letters: Vec<char> = CODE_LETTERS.chars().collect();
        loop {
            let c: String = (0..4)
                .map(|_| letters[self.rng.int(0, letters.len() as i64 - 1) as usize])
                .collect();
            if !self.rooms.contains_key(&c) {
                return c;
            }
        }
    }

    /// Meldet einen abgelehnten Beitritt (Fehler-Code aus `docs/protocol.md`); `ziel` ist Spielstand-Name oder Raum-Code.
    fn log_rejected<T>(&mut self, op: &str, id: &str, ziel: &str, res: &Result<T, Fehler>) {
        if let Err(e) = res {
            self.log.warn(
                "🚫 Beitritt abgelehnt",
                &[
                    ("ns", &"room"),
                    ("op", &op),
                    ("device", &short(id)),
                    ("ziel", &ziel),
                    ("code", e),
                ],
            );
        }
    }

    /// Erstellt einen Raum mit neuem (`fresh`) oder gespeichertem Spielstand und nimmt das Gerät auf. Prüf-Reihenfolge aus
    /// `docs/protocol.md` › Beitreten; ist der gespeicherte Stand schon offen, tritt das Gerät diesem Raum bei. Die Antwort
    /// ist der Raum-Code.
    #[allow(clippy::too_many_arguments)] // wie Go: Gerät, Peer, Name, neu, Tiefe, Slots, Optionen
    pub fn create(
        &mut self,
        id: &str,
        peer: Box<dyn Peer>,
        name: &str,
        fresh: bool,
        depth: i64,
        slots: &[i64],
        opts: Options,
    ) -> Result<String, Fehler> {
        valid_slots(slots)?;
        let (device, own_slots) = (id.to_string(), slots.to_vec());
        self.create_with(id, name, fresh, depth, opts, slots, move |r, env| {
            r.join(env, &device, peer, &own_slots)
        })
    }

    /// `create` mit einem beliebigen Eintritt des Geräts (Plätze oder Beobachter, `observer`, RU24).
    #[allow(clippy::too_many_arguments)] // wie Go `createWith`
    pub fn create_with(
        &mut self,
        id: &str,
        name: &str,
        fresh: bool,
        depth: i64,
        opts: Options,
        slots: &[i64],
        enter: impl FnOnce(&mut Room, &mut Env) -> Result<(), Fehler>,
    ) -> Result<String, Fehler> {
        if !valid_save_name(name) {
            return Err(Fehler::BadRequest);
        }
        let res = if self.shut {
            Err(Fehler::Closed)
        } else {
            self.create_room(id, name, fresh, depth, opts, slots, enter)
        };
        self.log_rejected("create", id, name, &res);
        res
    }

    #[allow(clippy::too_many_arguments)] // wie Go `create`
    fn create_room(
        &mut self,
        id: &str,
        name: &str,
        fresh: bool,
        depth: i64,
        opts: Options,
        slots: &[i64],
        enter: impl FnOnce(&mut Room, &mut Env) -> Result<(), Fehler>,
    ) -> Result<String, Fehler> {
        if let Some(code) = self.rooms.iter().find(|(_, r)| r.name == name).map(|(c, _)| c.clone()) {
            if fresh {
                return Err(Fehler::SaveExists);
            }
            let (rooms, mut env) = self.split();
            if let Some(r) = rooms.get_mut(&code) {
                enter(r, &mut env)?;
            }
            return Ok(code);
        }
        let isl = self.open(name, fresh, depth, &opts)?;
        let monarchs = free_monarchs(&isl).ok_or(Fehler::BadRequest)?;
        if self.rooms.len() >= MAX_ROOMS {
            return Err(Fehler::TooManyRooms);
        }
        let code = self.code();
        let start = start_stage(&isl, depth);
        let room = Room::new(&code, name, isl, opts.clone(), start, monarchs);
        self.rooms.insert(code.clone(), room);
        let raeume = self.rooms.len();
        let (rooms, mut env) = self.split();
        let Some(r) = rooms.get_mut(&code) else {
            return Err(Fehler::RoomNotFound);
        };
        r.log(
            &mut env,
            Level::Info,
            "🏰 Raum erstellt",
            &[
                ("device", &short(id)),
                ("neu", &fresh),
                ("tiefe", &depth),
                ("slots", &list_text(slots)),
                ("optionen", &opts),
                ("raeume", &raeume),
            ],
        );
        enter(r, &mut env)?;
        Ok(code)
    }

    /// Lädt den Spielstand in seiner gespeicherten Welt oder legt eine neue Insel in der Welt `opts.world` an. Die Spieler
    /// eines geladenen Stands sind im Raum freie Monarchen.
    fn open(&mut self, name: &str, fresh: bool, depth: i64, opts: &Options) -> Result<Island, Fehler> {
        let loaded = self.store.load(name);
        match (fresh, loaded) {
            (true, Ok(_)) => Err(Fehler::SaveExists),
            (false, Err(Fehler::NotFound)) => Err(Fehler::SaveNotFound),
            (_, Err(e)) if e != Fehler::NotFound => Err(e),
            (true, _) => {
                let seed = seed_of(name, opts)?;
                let mut isl = self.create_in(&seed, depth, opts)?;
                isl.id = format!("{name}-{}", (self.now)().format_compact_ms());
                let options = self.island_options(opts);
                set_options(&mut isl, options, self.dev).map_err(|_| Fehler::BadRequest)?; // ungültiger Wert oder dev ohne Dev-Mode
                Ok(isl)
            }
            (false, Ok(data)) => {
                let mut isl = self.load_in(&data)?;
                self.live_grade(&mut isl, name);
                Ok(isl)
            }
            (false, Err(e)) => Err(e),
        }
    }

    /// Setzt den Grad dev im Live-Modus auf normal (nur im Speicher; der Stand bleibt, bis er neu gespeichert wird).
    pub fn live_grade(&mut self, isl: &mut Island, name: &str) {
        if isl.options.grade == "dev" && !self.dev {
            self.log.warn(
                "🐛 Spielstand mit Grad dev im Live-Modus: Grad auf normal gesetzt",
                &[("save", &name)],
            );
            isl.options.grade = "normal".to_string();
        }
    }

    /// Füllt fehlende Felder: Grad nach Modus (dev im Dev-Mode, sonst normal), Ziel und Niederlage-Modus nach Grad.
    pub fn island_options(&self, o: &Options) -> IslandOptions {
        let grade = match (o.grade.is_empty(), self.dev) {
            (false, _) => o.grade.clone(),
            (true, true) => "dev".to_string(),
            (true, false) => "normal".to_string(),
        };
        let mut r = default_options_for(&grade, &self.data.difficulty);
        if r.grade != grade {
            r.grade = grade; // unbekannter Grad: `set_options` lehnt ihn ab
        }
        if !o.goal.is_empty() {
            r.goal = o.goal.clone();
        }
        if !o.defeat.is_empty() {
            r.defeat = o.defeat.clone();
        }
        r
    }

    /// Tritt einem Raum per Code bei oder verbindet wieder.
    pub fn join(&mut self, id: &str, peer: Box<dyn Peer>, code: &str, slots: &[i64]) -> Result<(), Fehler> {
        valid_slots(slots)?;
        let res = if self.shut {
            Err(Fehler::Closed)
        } else {
            let (rooms, mut env) = self.split();
            match rooms.get_mut(code) {
                Some(r) if r.closed => Err(Fehler::RoomNotFound),
                Some(r) => r.join(&mut env, id, peer, slots),
                None => Err(Fehler::RoomNotFound),
            }
        };
        self.log_rejected("join", id, code, &res);
        res
    }

    /// Der Raum mit diesem Code.
    pub fn room(&self, code: &str) -> Option<&Room> {
        self.rooms.get(code)
    }

    /// Die Raumliste, nach Code sortiert.
    pub fn rooms(&self) -> Vec<Info> {
        self.rooms.values().map(Room::info).collect()
    }

    /// Prüft die Fristen aller Räume nach Wanduhr: wartende Monarchen nach `WAIT_FOR_SECS` frei, leere Räume nach
    /// `EMPTY_FOR_SECS` speichern und aufräumen (Code wird frei).
    pub fn sweep(&mut self) {
        let (rooms, mut env) = self.split();
        let now = env.now;
        let mut removed = Vec::new();
        for (code, r) in rooms.iter_mut() {
            if locked_sweep(r, &mut env, now) {
                removed.push(code.clone());
            }
        }
        for code in removed {
            rooms.remove(&code);
        }
    }

    /// Speichert alle Räume, meldet ihren Geräten `room_closed` und räumt sie auf (Server fährt herunter). Danach nimmt
    /// der Manager keine Räume mehr an (`room_closed`).
    pub fn close(&mut self) {
        self.shut = true;
        let (rooms, mut env) = self.split();
        for r in std::mem::take(rooms).values_mut() {
            close_final(r, &mut env);
        }
    }
}

/// Prüft die Fristen eines Raums; `true`: der Raum ist aufgeräumt und muss aus dem Manager.
fn locked_sweep(r: &mut Room, env: &mut Env, now: Stamp) -> bool {
    let (_, remove) = r.sweep(env, now);
    if !remove {
        return false;
    }
    if r.is_test() {
        r.log(env, Level::Info, "🧹 Test-Raum leer, wird sofort aufgeräumt", &[]);
    } else {
        r.log(
            env,
            Level::Info,
            "🧹 Raum leer seit Frist, wird aufgeräumt",
            &[("frist", &"10m0s")],
        );
    }
    r.save_sync(env);
    r.write_report(env);
    r.closed = true;
    r.drop_test_save(env);
    true
}

fn close_final(r: &mut Room, env: &mut Env) {
    r.log(env, Level::Info, "🛑 Raum schließt (Server fährt herunter)", &[]);
    r.save_sync(env);
    r.write_report(env);
    r.closed = true;
    for d in r.devices.values_mut().filter(|d| d.connected) {
        d.peer.closed(true);
    }
}
