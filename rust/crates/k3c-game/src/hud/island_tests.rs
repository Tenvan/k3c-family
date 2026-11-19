//! B-1125: Das HUD rechnet Beutel-Obergrenze und Aktions-Hinweis mit der Insel-Sicht der Sim (`World::island_view`),
//! wie Go mit `w.island`.

use k3c_sim::merchant::merchant_x;
use k3c_sim::state::purse::purse_max_of;
use k3c_sim::state::{IslandView, Merchant, World};

use super::data::purse_max_gold;
use super::hints::{HintKind, action_hint};
use super::testutil::{StubView, hud, set_world, two_players};
use crate::view::settings::default_settings;

fn view(stock_bonus: i64) -> IslandView {
    IslandView {
        stages: 1,
        stock_bonus,
        ..IslandView::default()
    }
}

/// AC-01: Mit Dev-Bonus Lager zeigt die Spielerkarte das erhöhte Maximum; ohne Insel bleibt der Grundwert.
#[test]
fn spielerkarte_zeigt_dev_bonus_im_maximum() {
    let (mut w, mut f) = two_players(0);
    f.settings = default_settings();
    let mut h = hud();
    h.listen(&f);
    let base = format!("/{}", purse_max_gold());
    assert!(
        h.seats[0].gold_s.ends_with(&base),
        "ohne Insel: {:?}",
        h.seats[0].gold_s
    );

    w.island_view = Some(view(50));
    set_world(&mut f, &w);
    h.listen(&f);
    let want = purse_max_of(&w, &w.players[0], w.island_ref_view().as_ref());
    assert_eq!(want, purse_max_gold() + 50, "Sim: Grundwert plus Dev-Bonus");
    assert!(
        h.seats[0].gold_s.ends_with(&format!("/{want}")),
        "mit Bonus: {:?}",
        h.seats[0].gold_s
    );
}

/// AC-02: Am Händler (Kaufen) erscheint der Hinweis nur, solange das Lager Platz hat; bei vollem Lager schweigt er.
#[test]
fn volles_lager_am_haendler_gibt_keinen_kaufen_hinweis() {
    let (mut w, _) = two_players(0);
    w.merchant = Some(Merchant {
        resource: "wood".to_string(),
        ..Merchant::default()
    });
    w.players[0].gold = 5;
    w.players[0].x = merchant_x(&w, "buy");
    let kind = |w: &World| action_hint("de", w, &w.players[0], &StubView).kind;
    assert_eq!(kind(&w), HintKind::Buy, "ohne Insel gilt der Vorrat nie als voll");

    w.island_view = Some(view(0));
    w.stock.wood = 300; // eine Stufe: Maximum 300 je Hub
    assert_eq!(kind(&w), HintKind::None, "volles Lager");
    w.stock.wood = 299;
    assert_eq!(kind(&w), HintKind::Buy, "Platz im Lager");
}
