//! Port von `native/k3c-game/hud/hints_test.go` und `guide_test.go`.
//!
//! `anleitung_aus`, `anleitung_gesehen_gespeichert` und `anleitung_mit_bild` (Lage-Teil) brauchen `hud` (RU20.4) und
//! `guideLayout` (RU20.5): ignoriert, der Zeichner-Sprint schaltet sie ein. Der Teil von `anleitung_mit_bild`, der nur
//! `guide_image` prüft, läuft als `anleitung_bild_wahl`.

use super::hints::*;
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
        kind: kind.to_string(),
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
    w.troops[0].kind = "peasant".to_string();
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

/// Anleitung aus (Settings.GuideOff, B-809 AC-02): kein Hinweis; an: der Münz-Hinweis. Go: `guide_test.go`, braucht
/// `hud::Listen` und `seatState` (RU20.4).
#[test]
#[ignore = "wartet auf RU20"]
fn anleitung_aus() {
    unimplemented!("RU20.4: hud.Listen, seats[0].guide/guideS (Go: TestAnleitungAus)");
}

/// „Anleitung gesehen“ (B-809 AC-03): erledigter Hinweis geht über das Overlay in Settings.GuideSeen, nach Neustart
/// kommt er nicht wieder. Go: `guide_test.go`, braucht `hud.Update`/`Listen` (RU20.4).
#[test]
#[ignore = "wartet auf RU20"]
fn anleitung_gesehen_gespeichert() {
    unimplemented!("RU20.4: hud.Update, Settings.GuideSeen (Go: TestAnleitungGesehenGespeichert)");
}

/// Lage-Teil von TestAnleitungMitBild: Bild links neben dem Text, beide zusammen mittig (`guideLayout`, RU20.5).
#[test]
#[ignore = "wartet auf RU20"]
fn anleitung_mit_bild() {
    unimplemented!("RU20.5: guide_layout (Go: TestAnleitungMitBild, zweiter Teil)");
}
