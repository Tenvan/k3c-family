//! Port von `native/k3c-game/actors/combatmarks_test.go`.

use k3c_sim::state::Crown;

use super::*;

/// Der Wimpel zeigt im Bild nach rechts; gespiegelt, wenn der Stock links der Mitte steht.
#[test]
#[ignore = "wartet auf RU13.5"] // load_offset (marks.go) liegt bis dahin in pending.rs
fn banner_flag_points_away_from_back() {
    for left in [false, true] {
        let flip = banner_flip(left);
        assert_eq!(
            flip,
            load_offset(left) * BANNER_OFFSET < 0.0,
            "left={left}: Wimpel zeigt zum Monarchen hin (flip={flip})"
        );
    }
}

#[test]
fn variant_anim_faelle() {
    assert!(
        variant_anim(0) == "idle"
            && variant_anim(1) == "idle@spieler1"
            && variant_anim(4) == "idle@spieler4"
            && variant_anim(5) == "idle",
        "Variante je Spieler 1 bis 4, sonst Grundvariante"
    );
}

#[test]
fn crown_carrier_faelle() {
    let w = SimWorld {
        players: vec![
            Player {
                id: 1,
                index: 0,
                crown: Some(Crown {
                    state: CROWN_PLAYER.to_string(),
                    carrier: 2,
                    ..Crown::default()
                }),
                ..Player::default()
            },
            Player {
                id: 2,
                index: 1,
                ..Player::default()
            },
        ],
        ..SimWorld::default()
    };
    assert!(
        !crown_carrier(&w, &w.players[0]) && crown_carrier(&w, &w.players[1]),
        "Träger der fremden Krone falsch erkannt"
    );
}
