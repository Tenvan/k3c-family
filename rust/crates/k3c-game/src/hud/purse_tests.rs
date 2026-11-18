//! Port von `native/k3c-game/hud/purse_test.go`.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU10, RU15, RU18, RU23)

use super::data::purse_max_gold;
use super::drawstock::ICON_SCALE;
use super::fonts::{ROLE_SHARED, font_px};
use super::purse::*;
use super::testutil::{hud, set_world, two_players};
use crate::view::settings::default_settings;

/// B-919/AC-02: Das Geldsack-Icon ist doppelt so groß wie ein Ressourcen-Icon, in jeder Größe 1 bis 3.
#[test]
fn purse_side_doppelt_so_gross_wie_ressourcen_icon() {
    for px in [14.0, 28.0, 42.0] {
        let (got, res) = (purse_side(px), px * ICON_SCALE);
        assert_eq!(got, 2.0 * res, "Schrift {px}: Sack {got}, Ressourcen-Icon {res}");
    }
    assert!(
        purse_scale(1) < purse_scale(2) && purse_scale(2) < purse_scale(3) && purse_scale(3) <= 1.0,
        "drei wachsende Größen: {PURSE_FILL:?}"
    );
    assert!(
        purse_scale(0) == purse_scale(1) && purse_scale(9) == purse_scale(3),
        "Größe außerhalb 1 bis 3 wird begrenzt"
    );
}

/// B-919/AC-01: Die Spielerkarte hat weder Lebensbalken noch Spielermarke; sie ist so hoch wie der Sack und trägt das Gold.
#[test]
fn card_ohne_healthbar_und_marke() {
    let (_, mut f) = two_players(0);
    f.settings = default_settings();
    let mut h = hud();
    h.listen(&f);
    let st = std::mem::take(&mut h.seats[0]);
    let k = 1.0;
    let (w, ht) = h.card_size(&st, None, 0, k);
    let side = purse_side(font_px(ROLE_SHARED, h.cells as i64) * k);
    assert!(ht >= side && w >= side, "Karte {w}×{ht} kleiner als der Sack {side}");
    assert!(!st.gold_s.contains("P1"), "Goldtext ohne Spielermarke: {:?}", st.gold_s);
}

/// B-922/AC-02 (ohne Skill): Das HUD zeigt Größe 1 und die Münz-Obergrenze des Spielers (`purse_max_of`). Die Sim-Seite
/// (gelernte Größe samt Dev-Bonus) liefert erst RU18; bis dahin gilt der Grundwert aus `economy.json` (Shim in
/// `pending.rs`, RU18: ersetzen).
#[test]
fn purse_groesse_und_obergrenze_ohne_skill() {
    let (_, mut f) = two_players(0);
    f.settings = default_settings();
    let mut h = hud();
    h.listen(&f);
    let suffix = format!("/{}", purse_max_gold());
    let st = &h.seats[0];
    assert!(
        st.purse == 1 && st.gold_s.ends_with(&suffix),
        "ohne Skill: Größe {}, Gold {:?}, Obergrenze {suffix}",
        st.purse,
        st.gold_s
    );
}

/// B-922/AC-02 (mit Skills): gelernte Größe 2 und 3. `purse_size` liest die Werte der Insel (`values`, RU10) und
/// stürzt mit gelerntem Skill bis dahin ab; die Obergrenze je Größe kommt von `purse_max_of` (RU18).
#[test]
#[ignore = "wartet auf RU10 und RU18"]
fn purse_groesse_und_obergrenze_mit_skills() {
    let (mut w, mut f) = two_players(0);
    f.settings = default_settings();
    let mut h = hud();
    for (i, id) in ["purseMedium", "purseLarge"].iter().enumerate() {
        w.players[0].skills.push((*id).to_string());
        set_world(&mut f, &w);
        h.listen(&f);
        let st = &h.seats[0];
        let want = k3c_sim::state::purse::purse_max_of(&w, &w.players[0], None);
        assert_eq!(st.purse, i as i64 + 2, "{id}: Größe {}", st.purse);
        assert!(
            st.gold_s.ends_with(&format!("/{want}")),
            "{id}: Gold {:?}, Obergrenze {want}",
            st.gold_s
        );
    }
}

/// Das Skill-Menü führt die Linie „Geldsack“ mit beiden Skills hinter den anderen Linien und hat Zeilen dafür.
#[test]
fn skill_menu_geldsack_linie() {
    use super::data::{line_index, skill_lines};
    use super::skillmenu::menu_entries;
    use super::skilltext::entry_text;
    assert!(
        line_index("purse") < skill_lines().len(),
        "Linie purse fehlt in monarch.json › lines"
    );
    let mut ids = Vec::new();
    for e in menu_entries(Vec::new(), &[], 1, &|_| true, false) {
        if e.line == "purse" {
            let txt = entry_text("de", &e, false, "");
            assert!(
                txt.contains("Geldsack") && txt.contains("Purse"),
                "Zeile {txt:?} ohne Linie und Namen"
            );
            ids.push(e.id);
        }
    }
    assert_eq!(ids, ["purseMedium", "purseLarge"], "Skills der Linie purse");
}
