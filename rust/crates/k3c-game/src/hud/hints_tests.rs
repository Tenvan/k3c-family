//! Port von `native/k3c-game/hud/hints_test.go` und `guide_test.go`.
//!
//! `anleitung_aus` und `anleitung_gesehen_gespeichert` laufen mit `Listen` (RU20.4); der Teil von
//! `anleitung_gesehen_gespeichert`, den in Go `hud.Update` (Overlay, RU23) erledigt, ist unten nachgebaut. Der Lage-Teil
//! von `anleitung_mit_bild` braucht `guideLayout` (RU20.5): ignoriert, der Zeichner-Sprint schaltet ihn ein. Der Teil von
//! `anleitung_mit_bild`, der nur `guide_image` prüft, läuft als `anleitung_bild_wahl`.

use super::hints::*;
use super::testutil::{hud, set_world, two_players};
use crate::view::frame::Frame;
use k3c_sim::state::{Coin, CycleInfo, Site, Troop, World};

fn guide_world(phase: &str) -> World {
    World {
        cycle: CycleInfo {
            phase: phase.to_string(),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn coin(id: i64, x: f64) -> Coin {
    Coin {
        id,
        x,
        ..Default::default()
    }
}

fn troop(id: i64, x: f64, kind: &str) -> Troop {
    Troop {
        id,
        x,
        kind: if kind == "vagrant" || kind == "peasant" {
            "citizen"
        } else {
            kind
        }
        .to_string(),
        wild: kind == "vagrant",
        ..Default::default()
    }
}

/// Ein Hinweis je Spieler (actionHints.test.ts, B-319, W11.4): Ausbilden und Tauschen auch am Preisschild, der Bau des
/// Endbosses nur abseits, gefallen keiner; Tasten-Aktionen (Schlag, Skills, Lernen, Respec) nie.
#[test]
fn aktions_hinweis_wahl() {
    // (Name, gefallen, train, side, res, at_tag, at_lair, erwartet)
    let cases = [
        ("nichts", false, "", "", "", false, false, HintKind::None),
        ("Bau des Endbosses", false, "", "", "", false, true, HintKind::Lair),
        (
            "am Preisschild kein Bau-Hinweis",
            false,
            "",
            "",
            "",
            true,
            true,
            HintKind::None,
        ),
        (
            "Ausbilden am Preisschild",
            false,
            "builder",
            "",
            "",
            true,
            true,
            HintKind::Train,
        ),
        ("Verkaufen", false, "", "sell", "iron", false, true, HintKind::Sell),
        ("Kaufen", false, "", "buy", "stone", true, false, HintKind::Buy),
        ("gefallen", true, "miner", "", "", false, true, HintKind::None),
    ];
    for (name, down, train, side, res, at_tag, at_lair, want) in cases {
        let (got, _) = choose_hint(down, train, side, res, at_tag, at_lair);
        assert_eq!(got, want, "{name}");
    }
}

fn key(lang: &str, kind: HintKind, name: &str) -> HintKey {
    HintKey {
        lang: lang.to_string(),
        kind,
        name: name.to_string(),
    }
}

#[test]
fn aktions_hinweis_texte() {
    let cases = [
        ("Bergmann", key("de", HintKind::Train, "miner"), "Bergmann ausbilden"),
        (
            "Handwerker",
            key("de", HintKind::Train, "craftsman"),
            "Handwerker ausbilden",
        ),
        ("Stein kaufen", key("de", HintKind::Buy, "stone"), "Stein kaufen"),
        ("Eisen verkaufen", key("de", HintKind::Sell, "iron"), "Eisen verkaufen"),
        ("englisch", key("en", HintKind::Train, "miner"), "train Miner"),
        (
            "unbekanntes Material",
            key("de", HintKind::Buy, "mithril"),
            "mithril kaufen",
        ),
        (
            "Bau",
            key("de", HintKind::Lair, ""),
            "Bau des Endbosses: Ankunft löst ihn aus",
        ),
        ("keiner", key("de", HintKind::None, ""), ""),
    ];
    for (name, k, want) in cases {
        assert_eq!(hint_text(&k), want, "{name}");
    }
}

/// Geführte erste Nacht (guideHints.test.ts, S6.3, B-148).
#[test]
fn guide_reihenfolge() {
    let mut w = guide_world("dusk");
    w.coins = vec![coin(1, 12.0)];
    w.sites = vec![Site {
        id: 2,
        x: 8.0,
        state: "unpaid".to_string(),
        ..Default::default()
    }];
    w.troops = vec![troop(3, 11.0, "vagrant")];
    let bit = |id: GuideId| 1u8 << id as u8;
    let steps = [
        (0, GuideId::Coin, 12.0, 1),
        (bit(GuideId::Coin), GuideId::Dusk, 10.0, -1),
        (bit(GuideId::Coin) | bit(GuideId::Dusk), GuideId::Recruit, 11.0, 3),
        (
            bit(GuideId::Coin) | bit(GuideId::Dusk) | bit(GuideId::Recruit),
            GuideId::None,
            0.0,
            0,
        ),
    ];
    for (seen, id, x, target) in steps {
        let got = next_guide(&w, 10.0, false, GuideSeen(seen));
        assert!(
            got.id == id && got.x == x && (got.id == GuideId::None || got.target == target),
            "gesehen {seen:b}: {got:?}, erwartet {id:?} {x} {target}"
        );
    }
    assert_eq!(
        next_guide(&w, 10.0, true, GuideSeen(0)).id,
        GuideId::None,
        "ein gefallener Monarch bekommt keinen Hinweis"
    );
}

#[test]
fn guide_reichweite() {
    let mut w = guide_world("day");
    w.coins = vec![coin(1, 40.0)];
    assert_eq!(
        next_guide(&w, 10.0, false, GuideSeen(0)).id,
        GuideId::None,
        "Münze außer Reichweite"
    );
    w.coins = vec![coin(1, 14.0), coin(2, 9.0)];
    assert_eq!(
        next_guide(&w, 10.0, false, GuideSeen(0)).target,
        2,
        "die nächste zuerst"
    );
    w.coins.clear();
    w.troops = vec![troop(2, 10.0, "peasant")];
    assert!(
        next_guide(&w, 10.0, false, GuideSeen(0)).id == GuideId::None
            && next_guide(&guide_world("night"), 5.0, false, GuideSeen(0)).id == GuideId::None,
        "Bauern und Nacht ohne Hinweis"
    );
}

#[test]
fn guide_erledigt_und_zwei_spieler() {
    let mut seen = GuideSeen::default();
    let mut w = guide_world("day");
    w.coins = vec![coin(1, 5.0)];
    w.troops = vec![troop(3, 50.0, "vagrant")];
    let p1 = step_guide(GuideHint::default(), &w, 4.0, false, &mut seen);
    let p2 = step_guide(GuideHint::default(), &w, 51.0, false, &mut seen);
    assert!(
        p1.id == GuideId::Coin && p2.id == GuideId::Recruit,
        "je Zelle ein eigener Hinweis: {p1:?} {p2:?}"
    );
    let again = step_guide(p1, &w, 4.0, false, &mut seen);
    assert!(again == p1 && seen == GuideSeen(0), "ignorierter Hinweis bleibt stehen");
    w.coins.clear();
    let got = step_guide(p1, &w, 4.0, false, &mut seen);
    assert!(
        got.id == GuideId::None && seen.has(GuideId::Coin),
        "aufgehoben: weg und gesehen, {got:?}"
    );
    w.coins = vec![coin(5, 4.0)];
    assert_ne!(
        step_guide(GuideHint::default(), &w, 4.0, false, &mut seen).id,
        GuideId::Coin,
        "gesehener Hinweis kommt nicht wieder"
    );
    w.troops[0].serve();
    step_guide(p2, &w, 51.0, false, &mut seen);
    assert!(seen.has(GuideId::Recruit), "Landstreicher ist Bauer: erledigt");
    let dusk = step_guide(GuideHint::default(), &guide_world("dusk"), 0.0, false, &mut seen);
    step_guide(dusk, &guide_world("night"), 0.0, false, &mut seen);
    assert!(seen.has(GuideId::Dusk), "Nacht da: erledigt");
    assert!(
        guide_height(&dusk) == GUIDE_PLAYER_UNITS && guide_height(&p1) == GUIDE_OBJECT_UNITS,
        "Höhe: über dem Monarchen bzw. über dem Objekt"
    );
}

/// guideImage-Teil von TestAnleitungMitBild: Münz-Hinweis mit Münze, Nacht mit Mond, Landstreicher ohne; fehlen die
/// Vorlagen zum Zeichnen, steht der Text allein.
#[test]
fn anleitung_bild_wahl() {
    let cases = [
        (GuideId::Coin, true, GuideIcon::Coin),
        (GuideId::Dusk, true, GuideIcon::Moon),
        (GuideId::Recruit, true, GuideIcon::None),
        (GuideId::Coin, false, GuideIcon::None),
        (GuideId::Dusk, false, GuideIcon::None),
    ];
    for (id, ready, want) in cases {
        assert_eq!(guide_image(id, ready), want, "Hinweis {id:?}, Vorlagen {ready}");
    }
}

/// guide_frame ist ein Spieler mit einer Münze in Reichweite des Preisschilds.
fn guide_frame() -> (World, Frame) {
    let (mut w, mut f) = two_players(0);
    f.seats.truncate(1);
    w.coins = vec![coin(900, w.players[0].x + 2.0)];
    set_world(&mut f, &w);
    (w, f)
}

/// Anleitung aus (Settings.GuideOff, B-809 AC-02): kein Hinweis; an: der Münz-Hinweis. Go: `TestAnleitungAus`.
#[test]
fn anleitung_aus() {
    let (_, mut f) = guide_frame();
    let mut h = hud();
    h.listen(&f);
    assert!(
        !h.seats[0].guide_s.is_empty() && h.seats[0].guide.id == GuideId::Coin,
        "Anleitung an: Münz-Hinweis erwartet, {:?} {:?}",
        h.seats[0].guide,
        h.seats[0].guide_s
    );
    f.settings.guide_off = true;
    h.listen(&f);
    assert!(
        h.seats[0].guide_s.is_empty() && h.seats[0].guide.id == GuideId::None,
        "Anleitung aus: kein Hinweis, {:?} {:?}",
        h.seats[0].guide,
        h.seats[0].guide_s
    );
}

/// „Anleitung gesehen“ (B-809 AC-03): Ein erledigter Hinweis geht in `Settings.guide_seen`; aus den Einstellungen
/// gelesen kommt er nach einem Neustart (neues Hud) nicht wieder. In Go speichert `hud.Update` (Overlay, RU23) die
/// Bits über `Session.SetSettings` und setzt `pending` zurück; der Test macht dasselbe von Hand
/// (`guide_seen |= pending`, `pending = 0`). Go: `TestAnleitungGesehenGespeichert`.
#[test]
fn anleitung_gesehen_gespeichert() {
    let (mut w, mut f) = guide_frame();
    let mut h = hud();
    h.listen(&f);
    w.coins.clear();
    set_world(&mut f, &w);
    h.listen(&f);
    assert_eq!(
        h.pending,
        GuideSeen(1 << GuideId::Coin as u8),
        "gesehen, noch nicht gespeichert"
    );
    f.settings.guide_seen |= i64::from(h.pending.0);
    h.pending = GuideSeen(0);
    h.listen(&f);
    assert_eq!(h.pending, GuideSeen(0), "nur neue Hinweise speichern");
    w.coins = vec![coin(901, w.players[0].x + 1.0)];
    set_world(&mut f, &w);
    let mut again = hud();
    again.listen(&f);
    assert_ne!(
        again.seats[0].guide.id,
        GuideId::Coin,
        "nach Neustart: gesehener Hinweis kommt nicht wieder"
    );
    f.settings.guide_seen = 0;
    again.listen(&f);
    assert_eq!(
        again.seats[0].guide.id,
        GuideId::Coin,
        "zurückgesetzt (Anleitung wieder an): Hinweis kommt wieder"
    );
}
