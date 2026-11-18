//! Port von `native/k3c-game/hud/skillmenu.go`: Skill-Menü je Spieler (Spiegel von `src/scenes/hud/skillMenuLogic.ts`,
//! S3.2, B-124): Einträge aus dem Katalog nach Linie und Tier, gelernt und lernbar aus der Simulation (`can_learn`,
//! `can_respec`), Respec als letzter Eintrag, wenn er geht. Eingabe als Flanken (`Action`): Skill-Menü-Taste öffnet,
//! links/rechts (auch hoch/runter) wählt, Bestätigen lernt, B/Menu/Esc/Home schließt. Steuerkreuz ↓ liefert `Down`
//! und `SkillMenu` zugleich: bei offenem Menü zählt es als „weiter“ (`view/input.rs`). Reine Logik ohne Zeichnen.

use std::sync::OnceLock;

use super::data::{SkillInfo, line_index, skill_catalog};
use crate::view::input::Action;

/// MenuEntry ist ein Eintrag: Skill oder Respec (id `RESPEC_ID`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MenuEntry {
    pub id: String,
    pub line: String,
    pub tier: i64,
    pub passive: bool,
    pub learned: bool,
    pub learnable: bool,
}

/// RESPEC_ID ist die ID des Respec-Eintrags.
pub const RESPEC_ID: &str = "respec";

/// sorted_skills ist der Katalog nach Linie (Reihenfolge in monarch.json › lines) und Tier, sonst in Datei-Reihenfolge.
fn sorted_skills() -> &'static [SkillInfo] {
    static SORTED: OnceLock<Vec<SkillInfo>> = OnceLock::new();
    SORTED.get_or_init(|| {
        let mut out = skill_catalog().to_vec();
        out.sort_by_key(|s| (line_index(&s.line), s.tier)); // stabil
        out
    })
}

/// menu_entries schreibt die Einträge in `out` (geleert): lernbar nur mit freien Punkten und laut `can_learn`; Respec
/// am Ende, wenn `respec` gilt.
pub fn menu_entries(
    mut out: Vec<MenuEntry>,
    learned: &[String],
    points: i64,
    can_learn: &dyn Fn(&str) -> bool,
    respec: bool,
) -> Vec<MenuEntry> {
    out.clear();
    for s in sorted_skills() {
        let has = learned.contains(&s.id);
        out.push(MenuEntry {
            id: s.id.clone(),
            line: s.line.clone(),
            tier: s.tier,
            passive: s.kind == "passive",
            learned: has,
            learnable: !has && points > 0 && can_learn(&s.id),
        });
    }
    if respec {
        out.push(MenuEntry {
            id: RESPEC_ID.to_string(),
            learnable: true,
            ..MenuEntry::default()
        });
    }
    out
}

/// MenuState ist das Menü eines Spielers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MenuState {
    pub open: bool,
    pub cursor: i64,
}

/// MenuResult ist, was ein Update im Menü auslöst.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MenuResult {
    pub opened: bool,
    pub closed: bool,
    pub moved: bool,
    pub denied: bool,
    /// Skill-ID zum Lernen ("" = keiner)
    pub learn: String,
    pub respec: bool,
}

impl MenuState {
    /// step wertet die Flanken `acts` eines Updates für das Menü mit den Einträgen `entries` aus.
    pub fn step(&mut self, acts: &[Action], entries: &[MenuEntry]) -> MenuResult {
        let mut r = MenuResult::default();
        if !self.open {
            // Öffnen verbraucht das Update: Steuerkreuz ↓ bewegt nicht zugleich den Cursor
            if acts.contains(&Action::SkillMenu) {
                self.open = true;
                self.cursor = 0;
                r.opened = true;
            }
            return r;
        }
        let down = acts.contains(&Action::Down);
        for a in acts {
            if self.open {
                self.act(*a, down, entries, &mut r);
            }
        }
        if self.open && !entries.is_empty() {
            self.cursor = self.cursor.min(entries.len() as i64 - 1);
        }
        r
    }

    /// act führt eine Aktion bei offenem Menü aus.
    fn act(&mut self, a: Action, down: bool, entries: &[MenuEntry], r: &mut MenuResult) {
        let n = entries.len() as i64;
        match a {
            // K bzw. Num 5 schließt; Steuerkreuz ↓ zählt als `Down`
            Action::SkillMenu if !down => self.close(r),
            Action::Back | Action::Menu | Action::Home => self.close(r),
            Action::Left | Action::Up if n > 0 => {
                self.cursor = (self.cursor - 1 + n) % n;
                r.moved = true;
            }
            Action::Right | Action::Down if n > 0 => {
                self.cursor = (self.cursor + 1) % n;
                r.moved = true;
            }
            Action::Confirm => self.confirm(entries, r),
            _ => {}
        }
    }

    fn close(&mut self, r: &mut MenuResult) {
        self.open = false;
        r.closed = true;
    }

    fn confirm(&self, entries: &[MenuEntry], r: &mut MenuResult) {
        let e = usize::try_from(self.cursor).ok().and_then(|i| entries.get(i));
        match e {
            Some(e) if e.learnable && e.id == RESPEC_ID => r.respec = true,
            Some(e) if e.learnable => r.learn = e.id.clone(),
            _ => r.denied = true,
        }
    }
}

/// SlotView ist ein Skill-Slot: Skill ("" = frei) und Abklingzeit in Sekunden (0 = bereit).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SlotView {
    pub skill: String,
    pub cooldown: f64,
}

/// slot_views liest die Slots 1 bis 4 (fehlende Felder: frei bzw. bereit).
pub fn slot_views(slots: &[String], cooldowns: &[f64]) -> [SlotView; 4] {
    std::array::from_fn(|i| SlotView {
        skill: slots.get(i).cloned().unwrap_or_default(),
        cooldown: cooldowns.get(i).map_or(0.0, |c| c.max(0.0)),
    })
}
