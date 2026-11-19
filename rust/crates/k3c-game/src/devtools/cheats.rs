//! Port von `native/k3c-game/devtools/cheats.go`.
//!
//! Cheat-Menü als `view::roles::Overlay` (F9 bzw. View + Y, B-888): je Spieler ein eigenes Menü in seiner Zelle; ein
//! offenes Menü verbraucht die Eingabe seines Spielers (Monarch steht), die anderen spielen weiter. F9 kommt von der
//! Tastatur; spielt sie nicht, gilt der erste Spieler. Die Aktion geht über `DevSession` an den Raum (dort 🐛-Log mit
//! Spieler und Wert); nicht im Aufnahme-Modus.
//!
//! Abweichung von Go: `update_with` bekommt die Entwickler-Rolle der Session als Parameter (Go: Typ-Zusicherung im
//! Overlay, siehe `dev_of`), damit Tests sie ohne volle `Session` einsetzen.

use super::DevOverlay;
use super::cheatlist::cheat_groups;
use super::cheatmenu::{CheatResult, CheatState, action, label};
use crate::view::MAX_PLAYERS;
use crate::view::devkeys::DevSession;
use crate::view::frame::{Cue, CueKind, Frame, Seat};
use crate::view::input::{Action, DeviceID, Input};
use crate::view::roles::OverlayStep;

/// ERR_NO_DEV meldet eine Session ohne Entwickler-Rolle.
pub const ERR_NO_DEV: &str = "Cheats gehen nur im lokalen Spiel";

impl DevOverlay {
    /// update_with bedient die Cheat-Menüs aller Spieler mit der Eingabe `input` und dem Frame `f` des vorigen
    /// Updates; `dev` ist die Entwickler-Rolle der Session (`None`: nur lokal).
    pub fn update_with(&mut self, input: &Input, f: &Frame, dev: &mut Option<&mut dyn DevSession>) -> OverlayStep {
        let mut out = OverlayStep::default();
        self.cues.clear();
        if f.recording {
            return out;
        }
        if f.seats.is_empty() {
            self.toggles.clear(); // kein Spiel: die Schalter des Raums sind wieder aus
        }
        let mut seated = [false; MAX_PLAYERS];
        for seat in &f.seats {
            let Some(p) = usize::try_from(seat.player).ok().filter(|p| *p < MAX_PLAYERS) else {
                continue;
            };
            seated[p] = true;
            let acts = actions_of(input, &f.seats, seat);
            let r = self.menus[p].step(&acts, cheat_groups());
            let closed = r.closed;
            self.apply(&r, p, dev);
            out.consumed[p] = self.menus[p].open || closed;
        }
        for (p, m) in self.menus.iter_mut().enumerate() {
            if !seated[p] {
                *m = CheatState::default();
            }
        }
        out.cues = std::mem::take(&mut self.cues);
        out
    }

    fn cue(&mut self, kind: CueKind, player: usize, text: &str) {
        self.cues.push(Cue {
            kind,
            player: i64::try_from(player).unwrap_or(-1),
            text: text.to_string(),
            ..Default::default()
        });
    }

    /// apply meldet Cues und führt einen ausgelösten Eintrag über die Session aus.
    fn apply(&mut self, r: &CheatResult<'_>, player: usize, dev: &mut Option<&mut dyn DevSession>) {
        if r.opened {
            self.cue(CueKind::Confirm, player, "");
            k3c_platform::log::info("🐛 Cheat-Menü offen", &[("player", &player)]);
        } else if r.closed {
            self.cue(CueKind::Back, player, "");
        } else if r.moved {
            self.cue(CueKind::Move, player, "");
        }
        let Some(e) = r.fire else {
            return;
        };
        let a = action(&self.toggles, e);
        let res = match dev {
            Some(d) => d.dev(i64::try_from(player).unwrap_or(-1), &a),
            None => Err(ERR_NO_DEV.to_string()),
        };
        if let Err(msg) = res {
            self.menus[player].status = msg.clone();
            self.menus[player].bad = true;
            self.cue(CueKind::Denied, player, &msg);
            return;
        }
        if !e.toggle.is_empty() {
            self.toggles.insert(e.toggle.to_string(), a.on.unwrap_or(false));
        }
        self.menus[player].status = format!("Erledigt: {}", label(&self.toggles, e));
        self.menus[player].bad = false;
        self.cue(CueKind::Confirm, player, "");
    }
}

/// actions_of liefert die Aktionen des Spielers `seat`: die seines Geräts, dazu F9 eines Geräts ohne Platz, wenn er der
/// Ersatz ist (`keyboard_seat`).
pub fn actions_of(input: &Input, seats: &[Seat], seat: &Seat) -> Vec<Action> {
    let (di, _) = input.of(seat.device);
    let mut acts = di.actions;
    for d in &input.devices {
        if d.device == seat.device || !d.actions.contains(&Action::Cheats) || seat_of(seats, d.device).is_some() {
            continue;
        }
        if keyboard_seat(seats) == Some(seat.player) {
            acts.push(Action::Cheats);
            return acts;
        }
    }
    acts
}

/// keyboard_seat ist der Spieler, für den F9 gilt, wenn die Tastatur keinen Platz hat: der erste; `None` ohne Spieler.
/// Hat die Tastatur einen Platz, öffnet F9 das Menü dieses Spielers über sein eigenes Gerät (eine Tastatur, B-893).
pub fn keyboard_seat(seats: &[Seat]) -> Option<i64> {
    seats.first().map(|s| s.player)
}

fn seat_of(seats: &[Seat], d: DeviceID) -> Option<i64> {
    seats.iter().find(|s| s.device == d).map(|s| s.player)
}
