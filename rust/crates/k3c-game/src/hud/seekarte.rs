//! Port von `native/k3c-game/hud/seekarte.go`: Seekarte am Hafen (B-962): Solange der Wechselpunkt der Insel offen ist
//! (`World::island_switch`), zeigt jede Zelle die Etappen der Welt untereinander, die Inseln als Knoten (abgeschlossen,
//! Ziel, gesperrt). Der Cursor je Spieler blättert die Ziele (Links/Rechts), Bestätigen sendet die Wahl an die
//! Simulation (`Session::choose_island`); die gewählte Insel (`SwitchState::choice`) gilt für alle und ist in jeder Zelle
//! markiert. Das HUD rechnet keine Regel: Ziele, Wahl und Abgeschlossene kommen aus dem Zustand, die Etappen aus
//! `Frame::route`. Reine Logik ohne Zeichnen.
//!
//! Abweichung von Go: `SeaShown::route` merkt den Zeiger auf den ersten Eintrag der Route und ihre Länge als
//! `(usize, usize)` (`as_ptr`), verglichen wie in Go (keine tiefe Gleichheit, kein `unsafe`).

use super::drawsea::SeaSizes;
use super::glyphs::glyph_of;
use super::keyline::{KeyLine, arg, new_key_line};
use super::seekarte_look::{SeaLook, look_of};
use super::skilltext::move_arg;
use super::texts::{tf, tr};
use crate::view::frame::RouteIsland;
use crate::view::input::Action;
use k3c_sim::state::SwitchState;

/// SeaKind ist der Zustand eines Knotens der Seekarte.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SeaKind {
    /// noch nicht anfahrbar
    #[default]
    Locked,
    /// abgeschlossen
    Done,
    /// wählbares Ziel
    Target,
}

/// SeaNode ist eine Insel der Karte: `chosen` = gewähltes Ziel (für alle), `cursor` = Cursor dieses Spielers, `current`
/// = die Insel, an der das Schiff liegt. `look` ist ihr Aussehen im Diorama (`seekarte_look.rs`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SeaNode {
    pub id: String,
    pub name: String,
    pub kind: SeaKind,
    pub chosen: bool,
    pub cursor: bool,
    pub current: bool,
    pub look: SeaLook,
}

/// SeaRow ist eine Etappe mit ihren Inseln in Listenreihenfolge.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SeaRow {
    pub etappe: i64,
    pub title: String,
    pub nodes: Vec<SeaNode>,
}

/// SeaShown ist der Stand, aus dem Zeilen und Texte zuletzt gebaut wurden; nur bei Änderung wird neu gebaut.
#[derive(Clone, Debug, Default)]
pub struct SeaShown {
    pub set: bool,
    pub lang: String,
    pub profile: String,
    pub choice: String,
    /// Zeiger auf den ersten Eintrag der Route und ihre Länge (local hält sie je Spiel stabil)
    pub route: (usize, usize),
    pub targets: Vec<String>,
    pub done_id: Vec<String>,
}

/// SeaView ist die Seekarte eines Spielers. `open`: der Wechselpunkt ist offen und es gibt Ziele. `ids` sind die Ziele
/// in Reihenfolge der Simulation, `cursor` ihr Index; `choice` ist die zuletzt gelesene Wahl (ändert sie sich, springt
/// der Cursor darauf). Texte, Zeilen und Messungen (`sizes` bei Skalierung `sizes_k`) gelten, bis sich `shown` ändert.
/// `hold`: der Spieler hat am Steg bestätigt und hält A noch; bis zum Loslassen bleibt seine Eingabe verbraucht
/// (`seekarte_input.rs`).
#[derive(Clone, Debug, Default)]
pub struct SeaView {
    pub open: bool,
    pub hold: bool,
    pub ids: Vec<String>,
    pub cursor: usize,
    pub choice: String,
    pub rows: Vec<SeaRow>,
    pub title: String,
    pub hint: String,
    /// Bedienhinweis mit Tasten-Icons (`keyline.rs`); `hint` ist derselbe als Text
    pub line: KeyLine,
    pub shown: SeaShown,
    pub sizes: SeaSizes,
    pub sizes_k: f64,
    pub fallback: Vec<RouteIsland>,
}

/// SeaResult ist das Ergebnis eines Eingabeschritts: `moved` = Cursor bewegt, `pick` = Insel-ID der bestätigten Wahl.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SeaResult {
    pub moved: bool,
    pub pick: String,
}

impl SeaView {
    /// refresh liest die Karte aus dem Wechselzustand `sw` und der Welt `route`; ohne offenen Wechselpunkt (`sw` None)
    /// schließt sie. Neu gebaut wird nur bei Änderung von Sprache, Profil, Route, Zielen, Wahl oder Abgeschlossenen;
    /// sonst setzt es nur den Cursor (keine Allokation je Frame).
    pub fn refresh(&mut self, lang: &str, profile: &str, sw: Option<&SwitchState>, route: &[RouteIsland]) {
        let Some(sw) = sw.filter(|s| s.open && !s.targets.is_empty()) else {
            self.close();
            return;
        };
        self.open = true;
        let mut fallback = std::mem::take(&mut self.fallback);
        let mut route = route;
        if route.is_empty() {
            // ohne Welt-Angaben nur die Ziele
            fallback.clear();
            fallback.extend(sw.targets.iter().map(|t| RouteIsland {
                id: t.id.clone(),
                name: t.name.clone(),
                etappe: t.etappe,
                ..RouteIsland::default()
            }));
            route = &fallback;
        }
        if self.same(lang, profile, sw, route) {
            if self.choice != sw.choice {
                self.choice.clone_from(&sw.choice);
            }
        } else {
            self.remember(lang, profile, sw, route);
            self.ids.clear();
            self.ids.extend(sw.targets.iter().map(|t| t.id.clone()));
            if self.choice != sw.choice || self.cursor >= self.ids.len() {
                self.choice.clone_from(&sw.choice);
                self.cursor = self.ids.iter().position(|i| *i == sw.choice).unwrap_or(0);
            }
            self.line = sea_line(lang, profile, self.ids.len() > 1);
            self.title = tr(lang, "sea.title").to_string();
            self.hint.clone_from(&self.line.plain);
            self.build(lang, sw, route);
            self.sizes_k = 0.0;
        }
        self.mark_cursor();
        self.fallback = fallback;
    }

    /// close schließt die Karte und behält die Puffer.
    fn close(&mut self) {
        let mut shown = std::mem::take(&mut self.shown);
        shown.set = false;
        let (mut rows, mut ids, mut fallback) = (
            std::mem::take(&mut self.rows),
            std::mem::take(&mut self.ids),
            std::mem::take(&mut self.fallback),
        );
        rows.clear();
        ids.clear();
        fallback.clear();
        *self = SeaView {
            rows,
            ids,
            shown,
            fallback,
            ..SeaView::default()
        };
    }

    /// same: Der Stand, aus dem die Karte gebaut ist, ist unverändert.
    fn same(&self, lang: &str, profile: &str, sw: &SwitchState, route: &[RouteIsland]) -> bool {
        let sh = &self.shown;
        sh.set
            && sh.lang == lang
            && sh.profile == profile
            && sh.choice == sw.choice
            && sh.route == (route.as_ptr() as usize, route.len())
            && sh.targets.len() == sw.targets.len()
            && sh.done_id == sw.completed
            && sw.targets.iter().zip(&sh.targets).all(|(t, id)| t.id == *id)
    }

    /// remember merkt den Stand der Karte (Vecs werden wiederverwendet).
    fn remember(&mut self, lang: &str, profile: &str, sw: &SwitchState, route: &[RouteIsland]) {
        let sh = &mut self.shown;
        sh.set = true;
        sh.lang.clear();
        sh.lang.push_str(lang);
        sh.profile.clear();
        sh.profile.push_str(profile);
        sh.choice.clone_from(&sw.choice);
        sh.route = (route.as_ptr() as usize, route.len());
        sh.targets.clear();
        sh.targets.extend(sw.targets.iter().map(|t| t.id.clone()));
        sh.done_id.clone_from(&sw.completed);
    }

    /// build legt die Zeilen nach Etappe (aufsteigend) und die Knoten nach Listenreihenfolge an. Abgeschlossen ist, was
    /// `SwitchState::completed` nennt (die Simulation zählt die aktuelle Insel dazu), Ziel, was in `targets` steht, alles
    /// andere ist gesperrt. Die aktuelle Insel nennt die Simulation als letzte der Abgeschlossenen.
    fn build(&mut self, lang: &str, sw: &SwitchState, route: &[RouteIsland]) {
        self.rows.clear();
        for isl in route {
            let i = match self.rows.iter().position(|r| r.etappe == isl.etappe) {
                Some(i) => i,
                None => {
                    self.rows.push(SeaRow {
                        etappe: isl.etappe,
                        title: tf(lang, "sea.etappe", &[("n", isl.etappe.into())]),
                        nodes: Vec::new(),
                    });
                    self.rows.len() - 1
                }
            };
            let kind = if self.ids.contains(&isl.id) {
                SeaKind::Target
            } else if sw.completed.contains(&isl.id) {
                SeaKind::Done
            } else {
                SeaKind::Locked
            };
            self.rows[i].nodes.push(SeaNode {
                id: isl.id.clone(),
                name: isl.name.clone(),
                kind,
                chosen: isl.id == sw.choice,
                current: sw.completed.last().is_some_and(|c| *c == isl.id),
                look: look_of(isl),
                ..SeaNode::default()
            });
        }
        self.rows.sort_by_key(|r| r.etappe); // stabil
    }

    /// mark_cursor setzt den Cursor-Marker auf das Ziel unter dem Cursor und die Wahl-Marker (ohne Allokation).
    fn mark_cursor(&mut self) {
        let under = self.ids.get(self.cursor);
        for r in &mut self.rows {
            for n in &mut r.nodes {
                n.cursor = n.kind == SeaKind::Target && under.is_some_and(|u| *u == n.id);
                n.chosen = n.id == self.choice;
            }
        }
    }

    /// step wertet die Flanken eines Updates aus: Links/Rechts blättern die Ziele im Kreis (bei einem Ziel nichts),
    /// Bestätigen wählt das Ziel unter dem Cursor.
    pub fn step(&mut self, acts: &[Action]) -> SeaResult {
        let mut r = SeaResult::default();
        if !self.open || self.ids.is_empty() {
            return r;
        }
        let n = self.ids.len();
        for a in acts {
            match a {
                Action::Left if n > 1 => {
                    self.cursor = (self.cursor + n - 1) % n;
                    r.moved = true;
                }
                Action::Right if n > 1 => {
                    self.cursor = (self.cursor + 1) % n;
                    r.moved = true;
                }
                Action::Confirm => r.pick.clone_from(&self.ids[self.cursor.min(n - 1)]),
                _ => {}
            }
        }
        r
    }
}

/// sea_line ist der Bedienhinweis mit den Tasten des Eingabe-Profils; bei einem Ziel ohne Pfeile (nichts zu blättern).
pub fn sea_line(lang: &str, profile: &str, several: bool) -> KeyLine {
    let ok = arg("ok", vec![glyph_of(lang, "confirm", profile)]);
    if several {
        return new_key_line(tr(lang, "sea.hint"), &[move_arg(lang, profile), ok]);
    }
    new_key_line(tr(lang, "sea.hintOne"), &[ok])
}
