//! Port von `native/k3c-game/devtools/cheatmenu.go`.
//!
//! Bedienung des Cheat-Menüs je Spieler (F9 bzw. View + Y, B-888): links/rechts wechselt die Gruppe, hoch/runter den
//! Eintrag, Bestätigen löst aus, B, Menu, Pos1 oder erneut F9 schließt. Reine Logik ohne Zeichnen; die Aktion führt das
//! Overlay über die Session aus.

use std::collections::BTreeMap;

use super::cheatlist::{CheatEntry, CheatGroup};
use crate::view::devkeys::DevAction;
use crate::view::input::Action;

/// CheatState ist das Menü eines Spielers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CheatState {
    pub open: bool,
    pub group: usize,
    pub row: usize,
    /// Ergebnis der letzten Aktion (Anzeige)
    pub status: String,
    /// `status` ist ein Fehler
    pub bad: bool,
}

/// CheatResult ist, was ein Update im Menü auslöst.
#[derive(Debug, Default)]
pub struct CheatResult<'a> {
    pub opened: bool,
    pub closed: bool,
    pub moved: bool,
    /// auszulösender Eintrag
    pub fire: Option<&'a CheatEntry>,
}

impl CheatState {
    /// step wertet die Flanken `acts` eines Updates für das Menü mit den Gruppen `groups` aus.
    pub fn step<'a>(&mut self, acts: &[Action], groups: &'a [CheatGroup]) -> CheatResult<'a> {
        let mut r = CheatResult::default();
        for &a in acts {
            if !self.open {
                if a == Action::Cheats {
                    self.open = true;
                    r.opened = true;
                    self.status.clear();
                    return r; // Öffnen verbraucht das Update
                }
                continue;
            }
            self.act(a, groups, &mut r);
        }
        r
    }

    /// act führt eine Aktion bei offenem Menü aus.
    fn act<'a>(&mut self, a: Action, groups: &'a [CheatGroup], r: &mut CheatResult<'a>) {
        let n = groups.len();
        match a {
            Action::Cheats | Action::Back | Action::Menu | Action::Home => {
                self.open = false;
                r.closed = true;
            }
            _ if n == 0 => {}
            Action::Left | Action::PrevPage => {
                self.group = (self.group + n - 1) % n;
                self.row = 0;
                r.moved = true;
            }
            Action::Right | Action::NextPage => {
                self.group = (self.group + 1) % n;
                self.row = 0;
                r.moved = true;
            }
            Action::Up | Action::Down => self.step_row(a, groups, r),
            Action::Confirm => r.fire = groups.get(self.group).and_then(|g| g.entries.get(self.row)),
            _ => {}
        }
    }

    /// step_row bewegt die Auswahl um einen Eintrag (hoch von oben: letzter Eintrag).
    fn step_row(&mut self, a: Action, groups: &[CheatGroup], r: &mut CheatResult<'_>) {
        let rows = groups.get(self.group).map_or(0, |g| g.entries.len());
        if rows == 0 {
            return;
        }
        let d = if a == Action::Up { rows - 1 } else { 1 };
        self.row = (self.row + d) % rows;
        r.moved = true;
    }
}

/// Toggles ist der Stand der Schalter (raumweit, wie der Raum sie hält; Start aus).
pub type Toggles = BTreeMap<String, bool>;

/// action liefert die Aktion des Eintrags; ein Schalter bekommt den umgekehrten Stand.
pub fn action(t: &Toggles, e: &CheatEntry) -> DevAction {
    let mut a = e.act.clone();
    if !e.toggle.is_empty() {
        a.on = Some(!t.get(e.toggle).copied().unwrap_or(false));
    }
    a
}

/// label liefert die Beschriftung des Eintrags, bei Schaltern mit Stand.
pub fn label(t: &Toggles, e: &CheatEntry) -> String {
    if e.toggle.is_empty() {
        return e.label.clone();
    }
    if t.get(e.toggle).copied().unwrap_or(false) {
        format!("{}: an", e.label)
    } else {
        format!("{}: aus", e.label)
    }
}
