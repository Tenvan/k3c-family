//! Port von `native/k3c-game/menu/keys.go`: Menü-Tasten, gemeinsamer Zustand der Screens (`Base`), Meldung (`Note`)
//! und die Auswahl-Hilfen `move_sel`, `first`, `window`, `cycle`.
//!
//! Abweichung: Go bettet `base` in jeden Screen ein und ruft `b.run(in, s, h)`. Hier liefert der Screen über den Trait
//! [`Handler`] seine `Base` und `handle`, und die freie Funktion [`run`] hat dieselbe Schleife.

use super::pending::{TXT_KEYBOARD, TXT_PAD, TextKey, tr, tr1};
use crate::view::frame::{Cue, CueKind};
use crate::view::input::{Action, DeviceID, DeviceKind, Input};
use crate::view::screen::{Screen, Step, StepKind};
use crate::view::session::{Fehler, Session};
use crate::view::settings::DEFAULT_LANGUAGE;

/// Key ist eine Menü-Taste nach dem Zusammenfassen der Aktionen: B und Menu gehen beide zurück (Esc liefert beide im
/// selben Update, das zählt einmal), View löscht in der Spielstand-Liste, Home (View + Menu, Pos1) führt zur Wurzel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Key {
    #[default]
    None,
    Up,
    Down,
    Left,
    Right,
    OK,
    Back,
    Home,
    Delete,
}

/// Ordnet eine Aktion ihrer Menü-Taste zu; alles andere (Seiten, Skill-Menü, Vollbild) ist `Key::None`.
pub fn key_of(a: Action) -> Key {
    match a {
        Action::Up => Key::Up,
        Action::Down => Key::Down,
        Action::Left => Key::Left,
        Action::Right => Key::Right,
        Action::Confirm => Key::OK,
        Action::Back | Action::Menu => Key::Back,
        Action::Home => Key::Home,
        Action::View => Key::Delete,
        _ => Key::None,
    }
}

/// Die Meldung eines Screens: Schlüssel (Text in der Sprache) oder fertiger Text der Session (deutsch).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Note {
    pub key: TextKey,
    pub arg: String,
    /// gesetzt: Text der Session, ersetzt `key`
    pub raw: String,
    /// Fehler (rot) statt Hinweis
    pub bad: bool,
    pub set: bool,
}

impl Note {
    /// Liefert die Meldung in der Sprache `lang` ("" ohne Meldung).
    pub fn text(&self, lang: &str) -> String {
        if !self.set {
            return String::new();
        }
        if !self.raw.is_empty() {
            return self.raw.clone();
        }
        tr1(lang, self.key, &self.arg)
    }
}

/// Base ist der gemeinsame Zustand der Screens: Cues des Updates, das zuletzt bedienende Gerät, Meldung und Version
/// (jede Änderung erhöht `ver`; `draw` baut seine Texte nur bei neuer Version oder Sprache neu).
#[derive(Debug, Default)]
pub struct Base {
    pub cues: Vec<Cue>,
    pub by: DeviceID,
    pub by_ok: bool,
    pub note: Note,
    pub ver: i64,
}

/// Handler bedient eine Taste eines Geräts und sagt, was danach geschieht (Stay, Push … mit dem nächsten Screen).
pub trait Handler {
    fn base(&mut self) -> &mut Base;
    fn handle(&mut self, d: DeviceID, k: Key, s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>);
}

/// Gibt die Tasten aller Geräte in Druck-Reihenfolge an `h`, bis ein Schritt den Screen verlässt.
pub fn run<H: Handler>(h: &mut H, input: &Input, s: &mut dyn Session) -> Step {
    h.base().cues.clear();
    for di in &input.devices {
        let mut back = false;
        for &a in &di.actions {
            let k = key_of(a);
            if k == Key::None || (k == Key::Back && back) {
                continue; // ein zweites Back desselben Geräts im selben Update zählt nicht
            }
            back = back || k == Key::Back;
            let b = h.base();
            if !b.by_ok || b.by != di.device {
                b.by = di.device;
                b.by_ok = true;
                b.ver += 1;
            }
            let (kind, next) = h.handle(di.device, k, s);
            if kind != StepKind::Stay {
                return Step {
                    kind,
                    next,
                    cues: std::mem::take(&mut h.base().cues),
                };
            }
        }
    }
    Step {
        cues: std::mem::take(&mut h.base().cues),
        ..Step::default()
    }
}

impl Base {
    /// Merkt einen Cue für dieses Update vor.
    pub fn cue(&mut self, kind: CueKind, player: i64, text: &str) {
        self.cues.push(Cue {
            kind,
            player,
            text: text.to_string(),
            device: DeviceID::default(),
        });
    }

    /// Meldet einen Fehler der Session: rote Meldung und `CueKind::Denied` mit dem Text.
    pub fn fail(&mut self, err: &Fehler) {
        let text = err.to_string();
        self.note = Note {
            raw: text.clone(),
            bad: true,
            set: true,
            ..Note::default()
        };
        self.cue(CueKind::Denied, -1, &text);
        self.ver += 1;
    }

    /// Meldet eine abgelehnte Eingabe mit Text `k`.
    pub fn deny(&mut self, k: TextKey, arg: &str) {
        self.note = Note {
            key: k,
            arg: arg.to_string(),
            bad: true,
            set: true,
            ..Note::default()
        };
        self.cue(CueKind::Denied, -1, &tr1(DEFAULT_LANGUAGE, k, arg));
        self.ver += 1;
    }

    /// Zeigt einen Hinweis (kein Fehler).
    pub fn inform(&mut self, k: TextKey, arg: &str) {
        self.note = Note {
            key: k,
            arg: arg.to_string(),
            set: true,
            ..Note::default()
        };
        self.ver += 1;
    }
}

/// Nennt das Gerät für die Anzeige (Tastatur, Pad n).
pub fn device_name(lang: &str, d: DeviceID) -> String {
    if d.kind == DeviceKind::Gamepad {
        return tr1(lang, TXT_PAD, &(d.index + 1).to_string());
    }
    tr(lang, TXT_KEYBOARD)
}

/// Liefert die nächste wählbare Zeile ab `sel` in Richtung `dir` (-1 hoch, 1 runter) unter `n` Zeilen. Am Rand bleibt
/// die Auswahl (`wrap` false) oder springt ans andere Ende; ausgegraute Zeilen (`off`) werden übersprungen. Ist keine
/// andere Zeile wählbar, bleibt `sel`.
pub fn move_sel(sel: i64, dir: i64, n: i64, wrap: bool, off: Option<&dyn Fn(i64) -> bool>) -> i64 {
    let mut i = sel;
    for _ in 0..n {
        i += dir;
        if wrap {
            i = (i % n + n) % n;
        } else if i < 0 || i >= n {
            return sel;
        }
        if !off.is_some_and(|f| f(i)) {
            return i;
        }
    }
    sel
}

/// Liefert die erste wählbare Zeile (0, wenn keine wählbar ist).
pub fn first(n: i64, off: Option<&dyn Fn(i64) -> bool>) -> i64 {
    for i in 0..n {
        if !off.is_some_and(|f| f(i)) {
            return i;
        }
    }
    0
}

/// Liefert die erste sichtbare Zeile einer Liste mit `n` Einträgen, von denen `vis` sichtbar sind, sodass `sel`
/// möglichst mittig sichtbar ist.
pub fn window(sel: i64, n: i64, vis: i64) -> i64 {
    if n <= vis || vis <= 0 {
        return 0;
    }
    0.max((sel - vis / 2).min(n - vis))
}

/// Verschiebt den Index `i` in einer Liste mit `n` Einträgen um `dir` und springt am Ende zum Anfang (A in Wahl-Zeilen).
pub fn cycle(i: i64, dir: i64, n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    ((i + dir) % n + n) % n
}
