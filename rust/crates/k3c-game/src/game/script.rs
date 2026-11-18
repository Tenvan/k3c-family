//! Port von `native/k3c-game/game/script.go`: die feste Eingabe-Folge des Aufnahme-Modus (`--input`): gehaltene
//! Befehle und Menü-Aktionen je Spieler und Takt.

use k3c_platform::InputState;

use super::scene::{Hold, MAX_TICK, Scene};
use crate::view::MAX_PLAYERS;
use crate::view::input::{Action, Command, DeviceID, DeviceInput, Input, Source};

/// Die Menü-Aktionen der Eingabe-Folge („a:<name>“), Go: `actionNames`.
const ACTION_NAMES: [(&str, Action); 13] = [
    ("up", Action::Up),
    ("down", Action::Down),
    ("left", Action::Left),
    ("right", Action::Right),
    ("confirm", Action::Confirm),
    ("back", Action::Back),
    ("menu", Action::Menu),
    ("view", Action::View),
    ("prev", Action::PrevPage),
    ("next", Action::NextPage),
    ("home", Action::Home),
    ("skillmenu", Action::SkillMenu),
    ("fullscreen", Action::Fullscreen),
];

/// Liest die Eingabe-Folge „spieler@takt=befehl,…“. Befehl (gehalten ab dem Takt) aus L (links), R (rechts), S
/// (Sprint), P (zahlen), A (Schlag), 1–4 (Skill) oder „-“ (nichts); oder „a:<aktion>“: Menü-Aktion genau in diesem
/// Takt (up, down, left, right, confirm, back, menu, view, prev, next, home, skillmenu, fullscreen), der gehaltene
/// Befehl bleibt. Mehrere Aktionen im selben Takt kommen in der Reihenfolge der Folge.
/// Beispiel: „0@0=R,1@0=LS,0@90=-,0@95=a:skillmenu,0@100=a:right“.
pub fn parse_script(s: &str) -> Result<Vec<Hold>, String> {
    if s.is_empty() {
        return Ok(Vec::new());
    }
    s.split(',')
        .map(|part| parse_hold(part).map_err(|e| format!("🚫 --input {part:?}: {e}")))
        .collect()
}

/// Liest einen Eintrag „spieler@takt=befehl“ bzw. „spieler@takt=a:aktion“.
pub fn parse_hold(s: &str) -> Result<Hold, String> {
    let bad = || "erwartet spieler@takt=befehl".to_string();
    let (who, cmd) = s.split_once('=').ok_or_else(bad)?;
    let (ps, ts) = who.split_once('@').ok_or_else(bad)?;
    let p: i64 = ps.parse().map_err(|_| bad())?;
    let t: i64 = ts.parse().map_err(|_| bad())?;
    if p < 0 || p >= MAX_PLAYERS as i64 || !(0..=MAX_TICK).contains(&t) {
        return Err(bad());
    }
    let mut h = Hold {
        tick: t,
        player: p,
        cmd: Command::default(),
        press: false,
        action: Action::Up,
    };
    if let Some(name) = cmd.strip_prefix("a:") {
        let Some((_, a)) = ACTION_NAMES.iter().find(|(n, _)| *n == name) else {
            return Err(format!(
                "unbekannte Aktion {name:?} (up down left right confirm back menu view prev next home skillmenu fullscreen)"
            ));
        };
        h.press = true;
        h.action = *a;
        return Ok(h);
    }
    if cmd == "-" {
        return Ok(h);
    }
    for r in cmd.chars() {
        apply(&mut h.cmd, r)?;
    }
    if cmd.is_empty() {
        return Err("leerer Befehl, „-“ für nichts".to_string());
    }
    Ok(h)
}

/// Setzt den Buchstaben `r` im Befehl `c`.
pub fn apply(c: &mut Command, r: char) -> Result<(), String> {
    match r {
        'L' => c.move_ -= 1.0,
        'R' => c.move_ += 1.0,
        'S' => c.sprint = true,
        'P' => c.pay = true,
        'A' => c.attack = true,
        '1'..='4' => c.skill = i64::from(r as u8 - b'0'),
        _ => return Err(format!("unbekannter Befehl {r:?} (L R S P A 1–4 - oder a:aktion)")),
    }
    Ok(())
}

/// Die gehaltenen Befehle aller Spieler im Takt `tick`: je Spieler der letzte Befehls-Eintrag mit Tick ≤ `tick` (bei
/// gleichem Takt der spätere in der Folge); Aktionen zählen nicht.
pub fn commands_at(script: &[Hold], tick: i64) -> [Command; MAX_PLAYERS] {
    let mut out = [Command::default(); MAX_PLAYERS];
    let mut best = [-1_i64; MAX_PLAYERS];
    for h in script {
        let Ok(p) = usize::try_from(h.player) else { continue };
        if p < MAX_PLAYERS && !h.press && h.tick <= tick && h.tick >= best[p] {
            out[p] = h.cmd;
            best[p] = h.tick;
        }
    }
    out
}

/// Hängt die Aktionen des Spielers `player` im Takt `tick` in Reihenfolge der Folge an `dst`.
pub fn actions_at(dst: &mut Vec<Action>, script: &[Hold], player: i64, tick: i64) {
    dst.extend(
        script
            .iter()
            .filter(|h| h.press && h.player == player && h.tick == tick)
            .map(|h| h.action),
    );
}

/// Die Eingabe-Quelle der festen Folge: je `poll` ein Takt (ab dem Start, auch im Vorlauf), Geräte wie
/// `Scene::devices`. Die Plattform-Eingabe bleibt unbeachtet.
pub struct Scripted {
    script: Vec<Hold>,
    devs: Vec<DeviceID>,
    tick: i64,
}

/// Go: `newScripted`: die Quelle der Folge einer Szene.
pub fn scripted(s: &Scene) -> Box<dyn Source> {
    Box::new(Scripted {
        script: s.script.clone(),
        devs: s.devices(),
        tick: 0,
    })
}

impl Source for Scripted {
    fn poll(&mut self, _raw: &InputState) -> Input {
        let cmds = commands_at(&self.script, self.tick);
        let mut devices = Vec::with_capacity(self.devs.len());
        for (i, d) in self.devs.iter().enumerate() {
            let mut actions = Vec::new();
            actions_at(&mut actions, &self.script, i as i64, self.tick);
            devices.push(DeviceInput {
                device: *d,
                command: cmds.get(i).copied().unwrap_or_default(),
                actions,
            });
        }
        self.tick += 1;
        Input { devices }
    }
}

#[cfg(test)]
#[path = "script_tests.rs"]
mod script_tests;
