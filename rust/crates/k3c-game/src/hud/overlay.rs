//! Port von `native/k3c-game/hud/overlay.go`: Skill-Menü als `view::roles::Overlay`: game ruft `update` in jedem Update
//! im Spiel vor Pause, Beitritt und Befehlen. Jeder Spieler hat sein eigenes Menü; ein offenes Menü verbraucht die
//! Eingabe seines Spielers (Monarch steht, kein Pause-Menü), die anderen spielen weiter. Lernen und Respec gehen über
//! die Session; die Wirkung zeigt der nächste Frame.

use super::Hud;
use super::skillmenu::{MenuResult, MenuState};
use crate::view::MAX_PLAYERS;
use crate::view::frame::{Cue, CueKind, Frame, Seat};
use crate::view::input::{Action, Input};
use crate::view::roles::{Overlay, OverlayStep};
use crate::view::session::Session;

impl Overlay for Hud {
    /// Bedient die Skill-Menüs aller Spieler mit der Eingabe `input` und dem Frame `f` des vorigen Updates.
    fn update(&mut self, input: &Input, f: &Frame, s: &mut dyn Session) -> OverlayStep {
        let mut out = OverlayStep::default();
        self.cue_buf.clear();
        let mut seated = [false; MAX_PLAYERS];
        for seat in &f.seats {
            let Some(p) = usize::try_from(seat.player).ok().filter(|p| *p < MAX_PLAYERS) else {
                continue;
            };
            seated[p] = true;
            let menu = self.update_seat(input, f, s, seat);
            // Seekarte am Hafen: Bestätigen wird verbraucht, danach bis zum Loslassen von A (B-1006)
            out.consumed[p] = self.update_sea(input, f, s, seat, menu) || menu;
        }
        for (p, st) in self.seats.iter_mut().enumerate() {
            if !seated[p] {
                st.menu = MenuState::default();
            }
        }
        self.save_guide(s);
        out.cues = std::mem::take(&mut self.cue_buf);
        out
    }
}

impl Hud {
    /// save_guide schreibt neu gesehene Hinweise der ersten Nacht in die Einstellungen (`Settings::guide_seen`, B-809),
    /// damit „Anleitung gesehen“ einen Neustart überlebt; ein Fehler betrifft nur das Speichern (Log).
    fn save_guide(&mut self, s: &mut dyn Session) {
        if self.pending.0 == 0 {
            return;
        }
        let mut cur = s.settings();
        cur.guide_seen |= i64::from(self.pending.0);
        self.pending.0 = 0;
        if let Err(e) = s.set_settings(cur) {
            k3c_platform::log::warn("💥 Anleitung gesehen nicht gespeichert", &[("err", &e)]);
        }
    }

    /// update_seat bedient das Menü eines Spielers; Rückgabe: Eingabe verbraucht (Menü offen oder eben geschlossen).
    fn update_seat(&mut self, input: &Input, f: &Frame, s: &mut dyn Session, seat: &Seat) -> bool {
        let Some(st) = usize::try_from(seat.player).ok().and_then(|p| self.seats.get_mut(p)) else {
            return false;
        };
        let (di, ok) = input.of(seat.device);
        let world = f.stage(seat.stage).and_then(|g| g.world.as_deref());
        let m = f.monarch(seat.player);
        let (Some(world), Some(m)) = (world, m) else {
            st.menu = MenuState::default(); // Spieler ohne Monarch oder Stufe: Menü zu
            return false;
        };
        if !ok || seat.lost {
            st.menu = MenuState::default(); // Spieler ohne Eingabe: Menü zu
            return false;
        }
        if !st.menu.open && !di.actions.contains(&Action::SkillMenu) {
            return false;
        }
        st.refresh_entries(world, m);
        let r = st.menu.step(&di.actions, &st.entries);
        apply(&mut self.cue_buf, &r, seat.player, s);
        st.menu.open || r.closed
    }
}

/// apply führt Lernen und Respec über die Session aus und meldet die Cues.
pub(super) fn apply(cues: &mut Vec<Cue>, r: &MenuResult, player: i64, s: &mut dyn Session) {
    let mut cue = |kind: CueKind, text: &str| {
        cues.push(Cue {
            kind,
            player,
            text: text.to_string(),
            ..Cue::default()
        });
    };
    if r.opened {
        cue(CueKind::Confirm, "");
    } else if r.closed {
        cue(CueKind::Back, "");
    } else if r.moved {
        cue(CueKind::Move, "");
    }
    let res = if r.denied {
        cue(CueKind::Denied, "");
        return;
    } else if !r.learn.is_empty() {
        s.learn(player, &r.learn)
    } else if r.respec {
        s.respec(player)
    } else {
        return;
    };
    match res {
        Err(e) => {
            k3c_platform::log::info("🚫 Skill-Menü abgelehnt", &[("player", &player), ("err", &e)]);
            cue(CueKind::Denied, &e.to_string());
        }
        Ok(()) => cue(CueKind::Confirm, ""),
    }
}
