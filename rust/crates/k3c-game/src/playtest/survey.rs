//! Fragebogen am Spielende (WZ66.3, B-1222): drei Fragen je Spieler, bedient mit dem eigenen Pad (oder der Tastatur),
//! Ergebnis atomar nach `<ordner>/survey.json`. Reine Logik; gezeichnet wird in `survey_draw`.
//!
//! Bedienung (Pad / Tastatur): links, rechts ändern die Auswahl (Steuerkreuz oder linker Stick / Pfeile oder A, D); A
//! (Enter, Leertaste) bestätigt; B (Rücktaste) überspringt die Frage; Start (Esc) überspringt den ganzen Bogen. Ohne
//! Eingabe schließt der Bogen eines Spielers nach 60 s. Nicht beantwortet heißt `null`.

use std::fs;
use std::io;
use std::path::Path;

use k3c_platform::{InputState, Key, PadAxis, PadButton};
use serde_json::{Value, json};

use crate::view::input::{DeviceID, DeviceKind};

/// Takte (30 Hz) ohne Eingabe, nach denen der Bogen eines Spielers schließt (60 s).
pub const IDLE_TICKS: u32 = 60 * 30;
/// Name der Datei im Session-Ordner.
pub const FILE: &str = "survey.json";

/// Die drei Fragen mit ihren Antworten (Anzeigetext in Großbuchstaben ohne Umlaute, passend zur Pixel-Schrift).
pub const QUESTIONS: [&str; 3] = ["SPASS", "SCHWIERIGKEIT", "NOCHMAL SPIELEN?"];
pub const OPTIONS: [&[&str]; 3] = [
    &["1", "2", "3", "4", "5"],
    &["ZU LEICHT", "PASST", "ZU SCHWER"],
    &["JA", "NEIN"],
];
/// Vorauswahl je Frage (Index): drei Sterne, passt, ja.
const START_SEL: [usize; 3] = [2, 1, 0];
/// Werte in `survey.json` für Schwierigkeit.
const DIFFICULTY: [&str; 3] = ["zu_leicht", "passt", "zu_schwer"];

/// Eine Eingabe eines Spielers am Bogen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Press {
    Left,
    Right,
    Confirm,
    /// B: diese Frage überspringen
    SkipQuestion,
    /// Start: den ganzen Bogen überspringen
    SkipAll,
}

/// Gehaltene Tasten eines Geräts im vorigen Takt (Flankenerkennung).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Held([bool; 5]);

/// Der Bogen eines Spielers.
#[derive(Clone, Debug)]
pub struct Sheet {
    pub player: i64,
    pub device: DeviceID,
    /// aktuelle Frage 0 … 2; 3 = fertig
    pub q: usize,
    /// Auswahl je Frage (Index in `OPTIONS`)
    pub sel: [usize; 3],
    /// Antworten (Index in `OPTIONS`), `None` = nicht beantwortet
    pub answers: [Option<usize>; 3],
    idle: u32,
    prev: Option<Held>,
}

impl Sheet {
    pub fn new(player: i64, device: DeviceID) -> Sheet {
        Sheet {
            player,
            device,
            q: 0,
            sel: START_SEL,
            answers: [None; 3],
            idle: 0,
            prev: None,
        }
    }

    pub fn finished(&self) -> bool {
        self.q >= QUESTIONS.len()
    }

    /// Wendet eine Eingabe an.
    pub fn press(&mut self, p: Press) {
        if self.finished() {
            return;
        }
        self.idle = 0;
        let n = OPTIONS[self.q].len();
        match p {
            Press::Left => self.sel[self.q] = (self.sel[self.q] + n - 1) % n,
            Press::Right => self.sel[self.q] = (self.sel[self.q] + 1) % n,
            Press::Confirm => {
                self.answers[self.q] = Some(self.sel[self.q]);
                self.q += 1;
            }
            Press::SkipQuestion => self.q += 1,
            Press::SkipAll => self.q = QUESTIONS.len(),
        }
    }

    /// Liest die Eingabe eines Takts: Flanken der Tasten des Geräts; nach 60 s ohne Eingabe schließt der Bogen.
    pub fn step(&mut self, input: &InputState) {
        if self.finished() {
            return;
        }
        let now = held(input, self.device);
        // Was beim Öffnen schon gehalten ist (z. B. A vom Spiel), wirkt erst nach dem Loslassen
        let prev = self.prev.replace(now).unwrap_or(now);
        const ORDER: [Press; 5] = [
            Press::Left,
            Press::Right,
            Press::Confirm,
            Press::SkipQuestion,
            Press::SkipAll,
        ];
        let mut any = false;
        for (i, p) in ORDER.into_iter().enumerate() {
            if now.0[i] && !prev.0[i] {
                self.press(p);
                any = true;
                if self.finished() {
                    return;
                }
            }
        }
        if !any {
            self.idle += 1;
            if self.idle >= IDLE_TICKS {
                self.q = QUESTIONS.len();
            }
        }
    }
}

/// Gehaltene Bogen-Tasten des Geräts: links, rechts, bestätigen, B, Start.
fn held(input: &InputState, d: DeviceID) -> Held {
    if d.kind == DeviceKind::Keyboard {
        let k = |ks: &[Key]| ks.iter().any(|&x| input.held(x));
        return Held([
            k(&[Key::ArrowLeft, Key::A]),
            k(&[Key::ArrowRight, Key::D]),
            k(&[Key::Enter, Key::NumpadEnter, Key::Space]),
            k(&[Key::Backspace]),
            k(&[Key::Escape]),
        ]);
    }
    let Some(p) = u32::try_from(d.index).ok().and_then(|id| input.pad(id)) else {
        return Held::default();
    };
    let x = p.axis(PadAxis::LX);
    Held([
        p.held(PadButton::Left) || x < -0.5,
        p.held(PadButton::Right) || x > 0.5,
        p.held(PadButton::A),
        p.held(PadButton::B),
        p.held(PadButton::Menu),
    ])
}

/// Der Fragebogen aller Spieler.
#[derive(Clone, Debug)]
pub struct Survey {
    /// Unix-ms beim Öffnen des Bogens
    pub ts: u64,
    pub sheets: Vec<Sheet>,
}

impl Survey {
    /// Ein Bogen je Spieler `(Slot + 1, Gerät)`.
    pub fn new(players: &[(i64, DeviceID)], ts: u64) -> Survey {
        Survey {
            ts,
            sheets: players.iter().map(|&(p, d)| Sheet::new(p, d)).collect(),
        }
    }

    /// Ein Takt für alle Spieler; `true`, wenn alle fertig sind.
    pub fn step(&mut self, input: &InputState) -> bool {
        for s in &mut self.sheets {
            s.step(input);
        }
        self.sheets.iter().all(Sheet::finished)
    }

    /// `{"ts": …, "players": [{"player": 1, "fun": 4, "difficulty": "passt", "again": true}, …]}`; fehlt eine Antwort,
    /// steht `null`.
    pub fn to_json(&self) -> Value {
        let players: Vec<Value> = self
            .sheets
            .iter()
            .map(|s| {
                let a = &s.answers;
                json!({
                    "player": s.player,
                    "fun": a[0].map(|i| i + 1),
                    "difficulty": a[1].map(|i| DIFFICULTY[i]),
                    "again": a[2].map(|i| i == 0),
                })
            })
            .collect();
        json!({ "ts": self.ts, "players": players })
    }

    /// Schreibt `<dir>/survey.json` atomar (erst `survey.json.tmp`, dann umbenennen): nie eine halbe Datei.
    pub fn write(&self, dir: &Path) -> io::Result<()> {
        let tmp = dir.join(format!("{FILE}.tmp"));
        let text = serde_json::to_string_pretty(&self.to_json()).map_err(io::Error::other)?;
        fs::write(&tmp, text + "\n")?;
        fs::rename(&tmp, dir.join(FILE)).inspect_err(|_| {
            let _ = fs::remove_file(&tmp);
        })
    }
}
