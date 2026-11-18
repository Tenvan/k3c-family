//! Zweiter Teil von `native/k3c-game/hud/state.go` (nach dem Abschnitt „Farben der Berufssymbole“): Bürger-Labels und
//! Skill-Menü-Zustand des Spielers.

use super::pending::{available_points, build_menu, can_learn, can_respec, menu_entries, new_menu_key};
use super::state::{Label, SeatState};
use super::values::PlaceKey;
use super::values_texts::{merchant_text, profession_text};
use k3c_sim::state::{Player, World};

/// Farben der Berufssymbole (citizenView.ts › ICON_COLOR).
pub fn profession_color(profession: &str) -> Option<u32> {
    match profession {
        "miner" => Some(0x8d99ae),
        "builder" => Some(0xe9c46a),
        "craftsman" => Some(0xb87333),
        _ => None,
    }
}

/// Höhe des Berufs-Labels über dem Boden in Units.
pub const BADGE_UNITS: f64 = 150.0 / 32.0;
/// Höhe des Händler-Labels über dem Boden in Units.
pub const MERCHANT_UNITS: f64 = 116.0 / 32.0;
/// Hub-Mitte + 8 Units (docs/rules/buerger.md).
pub const MERCHANT_SHIFT: f64 = 8.0;

/// merchant_label ist der kurze Händler-Text über dem Hub (Schlüssel: Sprache und Rohstoff in id).
pub fn merchant_label(k: &PlaceKey) -> String {
    merchant_text(&k.lang, &k.id, 0, 0, true)
}

impl SeatState {
    /// citizen_labels: Händler im Hub zuerst (Vorrang beim Budget `MAX_LABELS`), dann der Beruf über jedem Bauern mit
    /// Beruf (citizenView.ts, Text ohne Figur).
    pub fn citizen_labels(&mut self, lang: &str, w: &World) {
        self.labels.clear();
        if let Some(m) = &w.merchant {
            let k = PlaceKey {
                lang: lang.to_string(),
                id: m.resource.clone(),
                name: String::new(),
                player: 0,
            };
            let text = self.m_merchant.get(k, merchant_label).to_string();
            self.labels.push(Label {
                x: w.castle.x + MERCHANT_SHIFT,
                y: MERCHANT_UNITS,
                text,
                dot: 0,
            });
        }
        for t in &w.troops {
            if t.profession.is_empty() {
                continue;
            }
            let dot = profession_color(&t.profession).unwrap_or(0xcccccc);
            self.labels.push(Label {
                x: t.x,
                y: BADGE_UNITS,
                text: profession_text(lang, &t.profession, true),
                dot,
            });
        }
    }

    /// skills: Einträge des offenen Skill-Menüs und sein Text.
    pub fn skills(&mut self, lang: &str, w: &World, m: &Player) {
        if !self.menu.open {
            return;
        }
        let points = self.refresh_entries(w, m);
        let k = new_menu_key(
            lang,
            &self.profile,
            self.menu.cursor,
            points,
            &self.entries,
            &self.slots,
        );
        if self.m_menu.as_ref() != Some(&k) {
            self.menu_v = build_menu(&k, &self.entries);
            self.m_menu = Some(k);
        }
    }

    /// refresh_entries liest die Einträge des Menüs aus der Simulation und liefert die freien Punkte.
    pub fn refresh_entries(&mut self, w: &World, m: &Player) -> i64 {
        let points = available_points(w, m);
        let can = |id: &str| can_learn(w, m, id);
        let respec = m.crown.is_none() && can_respec(w, m);
        let out = std::mem::take(&mut self.entries);
        self.entries = menu_entries(out, &m.skills, points, &can, respec);
        points
    }
}
