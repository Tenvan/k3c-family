//! Hilfen der Menü-Tests (Go: `fake_test.go`): die Schein-Session, Eingaben und Cue-Prüfung.
#![allow(dead_code)] // Hilfen für die Screens aus RU9.2 bis RU9.5 und RU12

use std::cell::RefCell;
use std::rc::Rc;

use super::{Menus, Shared, new_shared};
use crate::view::frame::{Cue, CueKind, Seat};
use crate::view::input::{Action, DeviceID, DeviceInput, DeviceKind, Input};
use crate::view::screen::{Screen, Step};
use crate::view::session::{Fehler, NewGame, SaveInfo, Session, WorldInfo};
use crate::view::settings::{Settings, default_settings};

/// FakeSession ist eine Schein-`Session`: Spielstände im Speicher, Fehler je Methode setzbar, Aufrufe gezählt.
#[derive(Default)]
pub struct FakeSession {
    pub saves: Vec<SaveInfo>,
    pub list_err: Option<Fehler>,
    pub new_err: Option<Fehler>,
    pub load_err: Option<Fehler>,
    pub del_err: Option<Fehler>,
    pub leave_err: Option<Fehler>,
    pub close_err: Option<Fehler>,
    pub set_err: Option<Fehler>,
    pub active: bool,
    pub news: Vec<NewGame>,
    pub loads: Vec<String>,
    pub devs: Vec<Vec<DeviceID>>,
    pub conts: i64,
    pub deleted: Vec<String>,
    pub left: Vec<i64>,
    pub closed: i64,
    pub settings: Vec<Settings>,
    /// aktuelle Einstellungen; `None` = Standard
    pub cur: Option<Settings>,
    pub seats: Vec<Seat>,
    /// `None` = nur Standard
    pub worlds: Option<Vec<WorldInfo>>,
    /// Welt des laufenden Spiels
    pub world: String,
}

/// Der Schein-Fehler (Go: `errFake`).
pub fn err_fake() -> Fehler {
    Fehler::Session("Schein-Fehler".to_string())
}

impl Session for FakeSession {
    fn new_game(&mut self, g: NewGame) -> Result<(), Fehler> {
        self.news.push(g);
        match &self.new_err {
            None => {
                self.active = true;
                Ok(())
            }
            Some(e) => Err(e.clone()),
        }
    }

    fn continue_game(&mut self, devices: &[DeviceID]) -> Result<(), Fehler> {
        self.conts += 1;
        self.devs.push(devices.to_vec());
        if self.saves.iter().any(|e| e.loadable) {
            self.active = true;
            return Ok(());
        }
        Err(Fehler::Session("Kein ladbarer Spielstand vorhanden".to_string()))
    }

    fn load(&mut self, name: &str, devices: &[DeviceID]) -> Result<(), Fehler> {
        self.loads.push(name.to_string());
        self.devs.push(devices.to_vec());
        match &self.load_err {
            None => {
                self.active = true;
                Ok(())
            }
            Some(e) => Err(e.clone()),
        }
    }

    fn list(&mut self) -> Result<Vec<SaveInfo>, Fehler> {
        match &self.list_err {
            Some(e) => Err(e.clone()),
            None => Ok(self.saves.clone()),
        }
    }

    fn delete(&mut self, name: &str) -> Result<(), Fehler> {
        if let Some(e) = &self.del_err {
            return Err(e.clone());
        }
        self.deleted.push(name.to_string());
        self.saves.retain(|e| e.name != name);
        Ok(())
    }

    fn save(&mut self) -> Result<(), Fehler> {
        Ok(())
    }

    fn close(&mut self) -> Result<(), Fehler> {
        self.closed += 1;
        self.active = false;
        match &self.close_err {
            Some(e) => Err(e.clone()),
            None => Ok(()),
        }
    }

    fn active(&self) -> bool {
        self.active
    }

    fn pause(&mut self, _on: bool) {}

    fn join(&mut self, _d: DeviceID) -> Result<i64, Fehler> {
        Ok(0)
    }

    fn leave(&mut self, player: i64) -> Result<(), Fehler> {
        self.left.push(player);
        match &self.leave_err {
            Some(e) => Err(e.clone()),
            None => Ok(()),
        }
    }

    fn rebind(&mut self, _player: i64, _d: DeviceID) -> Result<(), Fehler> {
        Ok(())
    }

    fn learn(&mut self, _player: i64, _skill: &str) -> Result<(), Fehler> {
        Ok(())
    }

    fn respec(&mut self, _player: i64) -> Result<(), Fehler> {
        Ok(())
    }

    fn choose_island(&mut self, _player: i64, _island: &str) -> Result<(), Fehler> {
        Ok(())
    }

    fn set_settings(&mut self, s: Settings) -> Result<(), Fehler> {
        self.settings.push(s.clone());
        self.cur = Some(s);
        match &self.set_err {
            Some(e) => Err(e.clone()),
            None => Ok(()),
        }
    }

    fn settings(&self) -> Settings {
        match &self.cur {
            Some(s) if *s != Settings::default() => s.clone(),
            _ => default_settings(),
        }
    }

    fn seats(&self) -> Vec<Seat> {
        self.seats.clone()
    }

    fn world_list(&self) -> Vec<WorldInfo> {
        match &self.worlds {
            Some(w) => w.clone(),
            None => vec![WorldInfo {
                id: "standard".to_string(),
                name: "Standard".to_string(),
            }],
        }
    }

    fn world(&self) -> String {
        self.world.clone()
    }
}

pub const KEYS1: DeviceID = DeviceID::new(DeviceKind::Keyboard, 0);
pub const PAD1: DeviceID = DeviceID::new(DeviceKind::Gamepad, 0);
pub const PAD2: DeviceID = DeviceID::new(DeviceKind::Gamepad, 1);
pub const PAD3: DeviceID = DeviceID::new(DeviceKind::Gamepad, 2);

/// press ist die Eingabe eines Updates: Gerät `d` drückt die Aktionen `a`.
pub fn press(d: DeviceID, a: &[Action]) -> Input {
    Input {
        devices: vec![DeviceInput {
            device: d,
            actions: a.to_vec(),
            ..Default::default()
        }],
    }
}

/// step_through gibt die Eingaben nacheinander (je ein Update) und liefert den letzten Schritt.
pub fn step_through(sc: &mut dyn Screen, s: &mut dyn Session, ins: &[Input]) -> Step {
    let mut st = Step::default();
    for i in ins {
        st = sc.update(i, s);
    }
    st
}

/// has_cue meldet, ob der Schritt einen Cue der Art `k` trägt.
pub fn has_cue(st: &Step, k: CueKind) -> bool {
    st.cues.iter().any(|c: &Cue| c.kind == k)
}

/// save baut einen Eintrag der Spielstand-Liste.
pub fn save(name: &str, day: i64, ok: bool) -> SaveInfo {
    let mut e = SaveInfo {
        name: name.to_string(),
        day,
        depths: vec![0],
        loadable: ok,
        ..Default::default()
    };
    if !ok {
        e.problem = "Spielstand nicht ladbar: Version 3".to_string();
    }
    e
}

/// new_menus baut den gemeinsamen Zustand ohne Grafik und Tastenbelegung (Go: `newMenus`).
pub fn new_menus() -> Shared {
    new_shared(None, None)
}

/// Lesezugriff auf den Zustand, kurz.
pub fn with_menus<R>(m: &Rc<RefCell<Menus>>, f: impl FnOnce(&mut Menus) -> R) -> R {
    f(&mut m.borrow_mut())
}

#[test]
fn fake_session_fulfils_contract() {
    let mut s = FakeSession::default();
    let _: &mut dyn Session = &mut s;
}
