//! Port von `native/k3c-game/game/scene.go`: die gesteuerte Szene (Aufnahme-Modus) und die Schalter des Programms.

use super::flags::{FlagSet, Kind, Spec, spec};
use super::modes::{MODE_FLAGS, Measure};
use super::script::parse_script;
use crate::view::MAX_PLAYERS;
use crate::view::input::{Action, Command, DeviceID, DeviceKind};
use k3c_platform::log;

/// Grenzen der Schalter.
pub const MAX_SHOTS: i64 = 64;
/// Vier Stunden Spielzeit in Takten.
pub const MAX_TICK: i64 = 30 * 60 * 60 * 4;
pub const MIN_WIDTH: i64 = 320;
pub const MAX_WIDTH: i64 = 3840;
/// Aufnahme ohne --input: Spieler 1 läuft rechts, Spieler 2 sprintet links.
pub const DEF_SCRIPT: &str = "0@0=R,1@0=LS";

/// Hold ist ein Eintrag der festen Eingabe-Folge: Ab Takt `tick` (seit Start der Szene) hält Spieler `player` den
/// Befehl `cmd`; mit `press` drückt er stattdessen genau in Takt `tick` die Menü-Aktion `action` (Flanke, der Befehl
/// bleibt).
#[derive(Clone, Debug)]
pub struct Hold {
    pub tick: i64,
    pub player: i64,
    pub cmd: Command,
    pub press: bool,
    pub action: Action,
}

/// Scene ist die gesteuerte Szene: was zu sehen ist, ohne Menü und ohne Zufall von außen.
#[derive(Clone, Debug, Default)]
pub struct Scene {
    /// neues Spiel mit diesem Seed (leer und ohne Load: Menü)
    pub seed: String,
    /// Stufe (Tiefe), in der die Spieler beginnen
    pub depth: i64,
    /// statt Seed: Spielstand aus dem Ordner --saves
    pub load: String,
    /// 1 oder 2 (Geräte keys1, pad1)
    pub players: i64,
    /// Kamera-x je Zelle in Units, NaN = folgt dem Monarchen
    pub cams: Vec<f64>,
    /// Takte, die die Simulation vor dem ersten Bild vorläuft
    pub tick: i64,
    /// feste Eingabe-Folge; gesetzt: ersetzt Tastatur und Gamepads
    pub script: Vec<Hold>,
    /// Aufnahme eines Menüs nach dem Vorlauf: "", main, saves, options, lobby, pause, keys, licenses (`MENUS`)
    pub menu: String,
}

/// Die Werte von --menu: main, saves, lobby und licenses ohne Spiel (ein Spiel der Szene endet nach dem Vorlauf mit
/// Speichern), options und keys über dem Hauptmenü bzw. mit Szene über der Pause, pause über dem stehenden Spiel.
pub const MENUS: [&str; 7] = ["main", "saves", "options", "lobby", "pause", "keys", "licenses"];

/// Args sind die Schalter des Programms.
#[derive(Clone, Debug, Default)]
pub struct Args {
    /// Ordner der Spielstände
    pub saves: String,
    /// Aufnahme-Modus: Ordner der Bilder; leer = Fenster
    pub shots: String,
    /// Zahl der Bilder
    pub count: i64,
    /// Takte zwischen zwei Bildern
    pub every: i64,
    /// Größe der Zeichenfläche (Aufnahme) bzw. des Fensters
    pub w: i64,
    pub h: i64,
    pub scene: Scene,
    /// Eingabe aus `scene.script` statt von den Geräten
    pub script: bool,
    /// --flow: Ablauf ab dem Hauptmenü ohne Szenen-Spiel; die Eingabe-Folge bedient Menüs und Spiel, die Aufnahme läuft
    /// auch ohne laufendes Spiel weiter (Ende-zu-Ende, NG9.1).
    pub flow: bool,
    /// --at: Aufnahme genau nach diesen Takten seit dem Start (aufsteigend); ersetzt --tick, --count und --every.
    pub at: Vec<i64>,
    /// Messmodus (--measure, `measure.go`)
    pub measure: Measure,
    /// --assets: Grafik-Ordner aller Level ausgeben und beenden (`assets.go`)
    pub assets: bool,
    /// --grafik: Grafik-Ordner statt public/grafik (leer = Standard)
    pub grafik: String,
    /// --river shader|tiles: Fluss-Fassung in die Einstellungen schreiben
    pub river: String,
    /// --stats: Leistungsanzeige in die Einstellungen schreiben
    pub stats: bool,
    /// --stall: dauerhaft die Ladeanzeige (nur Aufnahme-Modus)
    pub stall: bool,
    /// --devwin (oder K3C_DEVWIN=1): das Cheat-Fenster beim Start öffnen (RU56)
    pub devwin: bool,
}

impl Scene {
    /// Meldet, ob das Menü der Szene über dem laufenden (pausierten) Spiel liegt.
    pub fn in_game(&self) -> bool {
        self.menu == "pause" || ((self.menu == "options" || self.menu == "keys") && self.direct())
    }

    /// Meldet, ob das Spiel ohne Menü startet.
    pub fn direct(&self) -> bool {
        !self.seed.is_empty() || !self.load.is_empty()
    }

    /// Die Geräte der Spieler der Szene: keys1, dann Pads 1, 2 … (eine Tastatur hat nur ein Spieler, B-893).
    pub fn devices(&self) -> Vec<DeviceID> {
        (0..self.players.max(0))
            .map(|i| {
                if i > 0 {
                    DeviceID::new(DeviceKind::Gamepad, i - 1)
                } else {
                    DeviceID::default()
                }
            })
            .collect()
    }
}

/// Nennt die Schalter.
pub const USAGE: &str = concat!(
    "k3c-game [--saves <ordner>] [--seed <s> | --load <name> | --flow] [--depth <n>] [--players 1|2] [--cam <x>[,<x>]] ",
    "[--tick <n>] [--input <p@t=LRSPA1234-|a:aktion,…>] [--menu main|saves|options|lobby|pause|keys|licenses] [--size <B>x<H>] ",
    "[--shots <ordner> [--count <n>] [--every <takte>] [--at <t>,…]] ",
    "[--measure <s> [--warmup <s>] [--report <datei>] [--novsync]] [--assets] ",
    "[--grafik <ordner>] [--river shader|tiles] [--stats] [--stall] [--devwin]"
);

/// Die Schalter der Szene (Go: `newFlagSet`; `saves` bekommt `def_saves` als Standard).
const BASE_FLAGS: &[Spec] = &[
    spec("saves", Kind::Str, ""),
    spec("shots", Kind::Str, ""),
    spec("count", Kind::Int, "8"),
    spec("every", Kind::Int, "3"),
    spec("size", Kind::Str, "1280x720"),
    spec("seed", Kind::Str, ""),
    spec("depth", Kind::Int, "0"),
    spec("load", Kind::Str, ""),
    spec("players", Kind::Int, "2"),
    spec("cam", Kind::Str, ""),
    spec("tick", Kind::Int, "0"),
    spec("input", Kind::Str, ""),
    spec("menu", Kind::Str, ""),
];

/// Schalter, die erst nach dem Einlesen ausgewertet werden.
struct RawArgs {
    cams: String,
    input: String,
    size: String,
    at: String,
}

/// Legt Schalter an und liest `argv` ein.
fn read_flags(argv: &[String], def_saves: &str) -> Result<(Args, RawArgs), String> {
    let specs: Vec<Spec> = BASE_FLAGS.iter().chain(MODE_FLAGS).copied().collect();
    let mut fs = FlagSet::new(&specs);
    fs.set_default("saves", def_saves);
    if let Err(reason) = fs.parse(argv) {
        return Err(bad_flags(&reason, fs.args()));
    }
    if !fs.args().is_empty() {
        return Err(bad_flags("<nil>", fs.args()));
    }
    Ok(collect(&fs))
}

fn bad_flags(reason: &str, rest: &[String]) -> String {
    format!("🚫 ungültige Schalter ({reason} [{}]); gültig: {USAGE}", rest.join(" "))
}

/// Überträgt die gelesenen Werte in `Args`.
fn collect(fs: &FlagSet) -> (Args, RawArgs) {
    let a = Args {
        saves: fs.str("saves").to_string(),
        shots: fs.str("shots").to_string(),
        count: fs.int("count"),
        every: fs.int("every"),
        scene: Scene {
            seed: fs.str("seed").to_string(),
            depth: fs.int("depth"),
            load: fs.str("load").to_string(),
            players: fs.int("players"),
            tick: fs.int("tick"),
            menu: fs.str("menu").to_string(),
            ..Scene::default()
        },
        flow: fs.bool("flow"),
        measure: Measure {
            seconds: fs.float("measure"),
            warmup: fs.float("warmup"),
            report: fs.str("report").to_string(),
            no_vsync: fs.bool("novsync"),
            frames: fs.str("measure-frames").to_string(),
            load: fs.str("measure-load").to_string(),
            pacing: fs.str("pacing").to_string(),
        },
        assets: fs.bool("assets"),
        grafik: fs.str("grafik").to_string(),
        river: fs.str("river").to_string(),
        stats: fs.bool("stats"),
        stall: fs.bool("stall"),
        devwin: fs.bool("devwin"),
        ..Args::default()
    };
    let r = RawArgs {
        cams: fs.str("cam").to_string(),
        input: fs.str("input").to_string(),
        size: fs.str("size").to_string(),
        at: fs.str("at").to_string(),
    };
    (a, r)
}

/// Wertet die Schalter aus (rein, ohne Dateizugriff); `def_saves` ist der Standard für --saves.
pub fn parse_args(argv: &[String], def_saves: &str) -> Result<Args, String> {
    let (mut a, mut r) = read_flags(argv, def_saves)?;
    if !a.measure.pacing.is_empty() {
        log::info(
            "🚦 --pacing wirkt nicht mehr: Rust hat nur die Zeitsammlung k3c_platform::Clock",
            &[],
        );
    }
    a.apply_defaults(&mut r);
    (a.w, a.h) = parse_size(&r.size)?;
    a.scene.cams = parse_cams(&r.cams)?;
    a.scene.script = if r.input.is_empty() {
        Vec::new()
    } else {
        parse_script(&r.input)?
    };
    a.apply_at(&r.at)?;
    a.check()?;
    Ok(a)
}

impl Args {
    /// Setzt Seed und Eingabe-Folge, die eine Aufnahme ohne eigene Angabe braucht (nicht bei --flow).
    fn apply_defaults(&mut self, r: &mut RawArgs) {
        let shots = !self.shots.is_empty();
        let menu = self.scene.menu.as_str();
        if !self.scene.direct() && !self.flow && ((shots && menu.is_empty()) || menu == "pause") {
            self.scene.seed = "k3c".to_string(); // Aufnahme einer Szene bzw. der Pause braucht ein Spiel
        }
        self.script = !r.input.is_empty() || shots;
        if r.input.is_empty() && shots && self.scene.menu.is_empty() && !self.flow {
            r.input = DEF_SCRIPT.to_string();
        }
    }

    /// Prüft die Grenzen der Schalter.
    fn check(&self) -> Result<(), String> {
        let s = &self.scene;
        let max = MAX_PLAYERS as i64;
        if s.players < 1 || s.players > max {
            Err(format!("🚫 --players 1 bis {max}, nicht {}", s.players))
        } else if !s.seed.is_empty() && !s.load.is_empty() {
            Err("🚫 --seed und --load schließen sich aus".to_string())
        } else if !s.load.is_empty() && !is_save_name(&s.load) {
            Err(format!(
                "🚫 --load {:?}: Name a–z, 0–9, -, höchstens 32 Zeichen",
                s.load
            ))
        } else if s.depth < 0 || s.tick < 0 || s.tick > MAX_TICK {
            Err(format!("🚫 --depth ab 0, --tick 0 bis {MAX_TICK}"))
        } else if s.cams.len() as i64 > s.players {
            Err(format!("🚫 --cam: {} Werte für {} Spieler", s.cams.len(), s.players))
        } else if !s.menu.is_empty() && !MENUS.contains(&s.menu.as_str()) {
            Err(format!("🚫 --menu {:?}: einer von {}", s.menu, MENUS.join(", ")))
        } else {
            self.check_shots()
        }
    }

    /// Prüft Zahl und Abstand der Bilder und die Spieler der Eingabe-Folge.
    fn check_shots(&self) -> Result<(), String> {
        if self.count < 1 || self.count > MAX_SHOTS || self.every < 1 || self.every > MAX_TICK {
            return Err(format!("🚫 --count 1 bis {MAX_SHOTS}, --every ab 1"));
        }
        for h in &self.scene.script {
            if h.player >= self.scene.players {
                return Err(format!(
                    "🚫 --input: Spieler {} bei {} Spielern",
                    h.player, self.scene.players
                ));
            }
        }
        self.check_modes()
    }
}

/// Gültiger Spielstand-Name `^[a-z0-9-]{1,32}$` (wie `engine/store`).
fn is_save_name(s: &str) -> bool {
    (1..=32).contains(&s.len())
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// Liest „BxH“.
pub fn parse_size(s: &str) -> Result<(i64, i64), String> {
    let parsed = s.split_once('x').and_then(|(ws, hs)| {
        let (w, h) = (ws.parse::<i64>().ok()?, hs.parse::<i64>().ok()?);
        let ok = (MIN_WIDTH..=MAX_WIDTH).contains(&w) && (MIN_WIDTH * 9 / 16..=MAX_WIDTH).contains(&h);
        ok.then_some((w, h))
    });
    parsed.ok_or_else(|| format!("🚫 --size {s:?}: Breite×Höhe wie 1280x720, Breite {MIN_WIDTH} bis {MAX_WIDTH}"))
}

/// Liest „x[,x]“; ein leerer Eintrag oder „-“ folgt dem Monarchen.
pub fn parse_cams(s: &str) -> Result<Vec<f64>, String> {
    if s.is_empty() {
        return Ok(Vec::new());
    }
    s.split(',')
        .map(|part| {
            if part.is_empty() || part == "-" {
                return Ok(f64::NAN);
            }
            part.parse::<f64>()
                .ok()
                .filter(|x| x.is_finite())
                .ok_or_else(|| format!("🚫 --cam {s:?}: Welt-x in Units, Einträge durch Komma"))
        })
        .collect()
}

#[cfg(test)]
#[path = "scene_tests.rs"]
mod scene_tests;
