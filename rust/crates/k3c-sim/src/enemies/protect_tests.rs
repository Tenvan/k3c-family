//! Grad „leicht“: die ersten `protectedNights` Nächte ohne Verlust (Go: `difficulty_protect_test.go`, S6.1, B-148,
//! AC-02). Der Test des Zustandsprüfers (`protectSnapshot`) braucht `CreateIsland`, `castleFallen` und `buildSite`; er
//! wartet auf RU10 und RU14. Die Entscheidung selbst (`protected_night`) prüft der Test in `island_ref.rs`.

use super::testutil::*;

/// Was eine Nacht nicht kosten darf: Gold beider Spieler, Vorrat, Gebäude, Truppen.
#[derive(Debug, PartialEq)]
struct ProtectState {
    gold: [i64; 2],
    stock: (i64, i64, i64),
    built: usize,
    troops: usize,
}

fn snapshot(w: &crate::state::World) -> ProtectState {
    let mut gold = [0; 2];
    for (i, p) in w.players.iter().take(2).enumerate() {
        gold[i] = p.gold;
    }
    ProtectState {
        gold,
        stock: (w.stock.wood, w.stock.stone, w.stock.copper),
        built: w.sites.iter().filter(|s| s.state == "built").count(),
        troops: w.troops.len(),
    }
}

/// Insel mit zwei Spielern im Grad `grade`, Nacht `day`; Goldraub an beiden Spielern, tödlicher Schaden an einem Gebäude,
/// dann fällt die Burg. Liefert den Stand davor und danach.
fn forced_attack(grade: &str, day: i64) -> (ProtectState, ProtectState) {
    let mut isl = island_with(grade, 2);
    let w = &mut isl.stages[0];
    w.cycle.phase = "night".into();
    w.cycle.day = day;
    w.stock.wood = 10;
    w.stock.stone = 8;
    w.stock.copper = 6;
    for p in &mut w.players {
        p.gold = 20;
        p.shield = 1e9;
        p.shield_for = 1e9; // Treffer-Münzen zählen nicht als Raub: der Schild fängt Treffer ab
    }
    let site = build_site(w, "tower");
    let id = w.new_id();
    let hub_x = w.hub_x;
    w.troops.push(crate::state::Troop {
        id,
        kind: "archer".into(),
        x: hub_x,
        hp: 10.0,
        max_hp: 10.0,
        ..Default::default()
    });
    let before = snapshot(w);
    for pi in 0..w.players.len() {
        let at = w.players[pi].x;
        let greed = spawn(w, "greed", at);
        let t = super::Target {
            id: w.players[pi].id,
            x: w.players[pi].x,
            kind: super::TargetKind::Player,
            player: Some(pi),
        };
        let v = w.vals.clone().expect("Wertesatz");
        super::target::attack(w, &v, greed, &t, None); // die Insel liefert RU10 (`IslandRef`)
    }
    let (sid, smax) = (w.sites[site].id, w.sites[site].max_hp);
    crate::state::common::apply_damage_by(w, sid, smax * 10.0, "goblin");
    w.castle.hp = 0.0;
    // `castleFallen` (RU15/RU18) folgt dem Burgfall.
    (before, snapshot(w))
}

#[test]
#[ignore = "wartet auf RU10 (island_with)"]
fn protected_night_easy_keeps_everything() {
    let (before, after) = forced_attack("easy", 1);
    assert_eq!(after, before, "easy, Nacht 1: Verlust");
}

fn assert_lost(name: &str, before: &ProtectState, after: &ProtectState) {
    for i in 0..2 {
        assert!(
            after.gold[i] < before.gold[i],
            "{name}: Spieler {i} behält Gold {}",
            after.gold[i]
        );
    }
    assert!(
        after.stock != before.stock && after.built < before.built && after.troops < before.troops,
        "{name}: ohne Verlust {before:?} -> {after:?}"
    );
}

#[test]
#[ignore = "wartet auf RU10 (island_with)"]
fn protected_night_normal_loses() {
    let (before, after) = forced_attack("normal", 1);
    assert_lost("normal, Nacht 1", &before, &after);
}

#[test]
#[ignore = "wartet auf RU10 (island_with)"]
fn protected_night_easy_second_night_loses() {
    let (before, after) = forced_attack("easy", 2);
    assert_lost("easy, Nacht 2", &before, &after);
}
