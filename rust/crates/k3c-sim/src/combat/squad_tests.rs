//! Tests zu `squad.rs` (Go hat für `combat_squad.go` keinen eigenen Test; die Gruppen deckten bisher `combat_test.go` und
//! die Golden ab). Das Soll ist aus dem Go-Text von Hand abgeleitet; Tests mit `emit` (RU19) oder `crowned` (RU15.4) tragen
//! `ignore`.

use super::data::{COMBAT_GENERAL, COMBAT_GROUPS};
use super::squad::{
    count_fetching, free_warriors, group_count, guarded_leader, leaders_of, nearest_troop, send_groups, step_banners,
    step_groups, take_banner,
};
use super::testutil::{combat_stage, troop, vals};

#[test]
fn nearest_troop_bei_gleichstand_die_fruehere() {
    let (mut w, _, _) = combat_stage(COMBAT_GROUPS);
    let a = troop(&mut w, "warrior", -3.0, 0.0);
    let b = troop(&mut w, "warrior", 3.0, 0.0);
    let c = troop(&mut w, "warrior", 8.0, 0.0);
    assert_eq!(
        nearest_troop(&w, [a, b, c], w.hub_x),
        Some(a),
        "gleich weit: die frühere"
    );
    assert_eq!(nearest_troop(&w, [b, a, c], w.hub_x), Some(b));
    assert_eq!(nearest_troop(&w, [c], w.hub_x), Some(c));
    assert_eq!(nearest_troop(&w, [], w.hub_x), None);
}

#[test]
fn free_warriors_nur_freie_soldaten_in_reihenfolge() {
    let (mut w, _, _) = combat_stage(COMBAT_GROUPS);
    let free = troop(&mut w, "warrior", 0.0, 1.0);
    let elite = troop(&mut w, "eliteWarrior", 0.0, 1.0);
    let archer = troop(&mut w, "archer", 0.0, 1.0);
    let grouped = troop(&mut w, "warrior", 0.0, 1.0);
    w.troops[grouped].group = 77;
    let fetching = troop(&mut w, "warrior", 0.0, 1.0);
    w.troops[fetching].shield_fetch = true;
    assert_eq!(free_warriors(&w), vec![free, elite]);
    assert_eq!(count_fetching(&w), 1);
    assert!(!free_warriors(&w).contains(&archer));
}

#[test]
fn leaders_of_nach_seite_und_group_count() {
    let (mut w, _, _) = combat_stage(COMBAT_GROUPS);
    let r = troop(&mut w, "warrior", 0.0, 1.0);
    let l = troop(&mut w, "warrior", 0.0, -1.0);
    w.troops[r].leader = true;
    w.troops[l].leader = true;
    assert_eq!(leaders_of(&w, 1.0), vec![r]);
    assert_eq!(leaders_of(&w, -1.0), vec![l]);
    let rid = w.troops[r].id;
    w.troops[r].group = rid;
    assert_eq!(group_count(&w, r), 0, "der Führer zählt sich nicht");
    assert!(!guarded_leader(&w, r));
    let m = troop(&mut w, "warrior", 0.0, 1.0);
    w.troops[m].group = rid;
    assert_eq!(group_count(&w, r), 1);
    assert!(guarded_leader(&w, r));
    assert!(!guarded_leader(&w, m), "Mitglieder sind keine Führer");
}

#[test]
fn step_groups_gibt_mitglieder_ohne_fuehrer_frei() {
    let (mut w, _, _) = combat_stage(COMBAT_GROUPS);
    let v = vals(&w);
    let m = troop(&mut w, "warrior", 0.0, 1.0);
    w.troops[m].group = 12345; // der Führer ist gefallen
    step_groups(&mut w, &v);
    assert_eq!(w.troops[m].group, 0);
}

#[test]
fn step_groups_vergibt_schilde_an_den_naechsten_freien_soldaten() {
    let (mut w, _, _) = combat_stage(COMBAT_GROUPS);
    let v = vals(&w);
    let far = troop(&mut w, "warrior", 30.0, 1.0);
    let near = troop(&mut w, "warrior", 5.0, 1.0);
    let mid = troop(&mut w, "warrior", 10.0, 1.0);
    w.combat.shields = 2;
    step_groups(&mut w, &v);
    assert!(w.troops[near].shield_fetch && w.troops[mid].shield_fetch && !w.troops[far].shield_fetch);
    assert_eq!(w.combat.shields, 0);
    w.combat.shields = 5; // kein weiterer freier Soldat außer far: nur noch ein Schild geht weg
    step_groups(&mut w, &v);
    assert!(w.troops[far].shield_fetch);
    assert_eq!(w.combat.shields, 4, "ohne freien Soldaten bleibt der Rest liegen");
}

#[test]
fn step_groups_fuellt_je_tick_einen_soldaten_der_seite_bis_group_size() {
    let (mut w, _, _) = combat_stage(COMBAT_GROUPS);
    let v = vals(&w); // groupSize 3
    let l = troop(&mut w, "warrior", 5.0, 1.0);
    let lid = w.troops[l].id;
    w.troops[l].leader = true;
    w.troops[l].group = lid;
    let right: Vec<usize> = (0..4).map(|i| troop(&mut w, "warrior", 6.0 + i as f64, 1.0)).collect();
    let left = troop(&mut w, "warrior", 5.5, -1.0); // andere Seite: nie Kandidat
    for tick in 1..=3 {
        step_groups(&mut w, &v);
        assert_eq!(group_count(&w, l), tick, "Tick {tick}");
    }
    step_groups(&mut w, &v);
    assert_eq!(group_count(&w, l), 3, "groupSize erreicht");
    assert!(w.troops[left].group == 0 && w.troops[right[3]].group == 0);
    assert!(
        w.troops[right[0]].group == lid && w.troops[right[1]].group == lid && w.troops[right[2]].group == lid,
        "die nächsten zuerst"
    );
}

#[test]
fn step_groups_laesst_die_gruppe_mit_ziel_im_einsatz() {
    let (mut w, _, _) = combat_stage(COMBAT_GROUPS);
    let v = vals(&w);
    let l = troop(&mut w, "warrior", 10.0, 1.0);
    let lid = w.troops[l].id;
    w.troops[l].leader = true;
    w.troops[l].group = lid;
    w.troops[l].sortie = 1.0;
    step_groups(&mut w, &v); // Portal bei Hub + 40 liegt voraus
    assert_eq!(w.troops[l].sortie, 1.0);
}

#[test]
#[ignore = "wartet auf RU19"] // `emit` (groupReturned)
fn step_groups_beendet_den_einsatz_ohne_ziel() {
    let (mut w, _, _) = combat_stage(COMBAT_GROUPS);
    let v = vals(&w);
    w.portals.clear();
    let l = troop(&mut w, "warrior", 10.0, 1.0);
    w.troops[l].leader = true;
    w.troops[l].group = w.troops[l].id;
    w.troops[l].sortie = 1.0;
    step_groups(&mut w, &v);
    assert_eq!(w.troops[l].sortie, 0.0);
}

#[test]
#[ignore = "wartet auf RU19"] // `emit` (attackOrdered, bannerTaken)
fn send_groups_und_take_banner_melden_und_setzen() {
    let (mut w, _, _) = combat_stage(COMBAT_GROUPS);
    let l = troop(&mut w, "warrior", 0.0, 1.0);
    w.troops[l].leader = true;
    send_groups(&mut w, 1.0);
    assert_eq!(w.troops[l].sortie, 1.0);
    w.players[1].banner_out = true;
    take_banner(&mut w, 1, -1);
    assert_eq!((w.players[1].banner, w.players[1].banner_out), (-1, false));
}

#[test]
fn step_banners_loest_trupps_ohne_banner_auf() {
    let (mut w, _, _) = combat_stage(COMBAT_GENERAL);
    let v = vals(&w);
    let pid = w.players[0].id;
    let m = troop(&mut w, "warrior", 0.0, 1.0);
    w.troops[m].group = pid; // der Monarch hat kein Banner
    let o = troop(&mut w, "warrior", 0.0, 1.0);
    w.troops[o].group = 424_242; // gar kein Monarch
    step_banners(&mut w, &v);
    assert_eq!((w.troops[m].group, w.troops[o].group), (0, 0));
}

#[test]
fn step_banners_fuellt_den_trupp_je_art_einen_je_tick() {
    let (mut w, _, _) = combat_stage(COMBAT_GENERAL);
    let v = vals(&w);
    w.players[0].banner = 1;
    for _ in 0..7 {
        troop(&mut w, "warrior", 0.0, 1.0);
        troop(&mut w, "archer", 0.0, 1.0);
    }
    for _ in 0..8 {
        step_banners(&mut w, &v);
    }
    let pid = w.players[0].id;
    let (s, a) = w.troops.iter().filter(|t| t.group == pid).fold((0, 0), |(s, a), t| {
        if super::is_soldier(t) { (s + 1, a) } else { (s, a + 1) }
    });
    assert_eq!(
        (s, a),
        (v.economy.combat.troop_soldiers, v.economy.combat.troop_archers)
    );
}
