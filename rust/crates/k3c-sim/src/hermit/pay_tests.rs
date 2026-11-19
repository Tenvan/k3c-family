//! Zahlung an der Hütte (RU62.10, AC-05): Abzug, Bau im Hub beginnt sofort, zweite Zahlung und zu wenig Münzen ändern nichts,
//! beide Monarchen, Hub-Stufe, Reichweite, kein Münzeinwurf am Platz selbst, Bauzeit und Stufe 2 fehlt.

use super::BUILDING_STATE;
use super::testutil::{add_deep_player, count, hall, hermit_data, hermit_island, hut, pay_at_hut, run, tick};
use crate::state::hermit_view::HermitPhase;

#[test]
fn zahlung_zieht_ab_und_der_bau_beginnt_sofort() {
    let data = hermit_data("swordsmith");
    let (_, mut isl) = hermit_island(&data, 1);
    let m = add_deep_player(&mut isl);
    assert_eq!(hall(&isl).state, "unpaid");
    assert_eq!(isl.stages[1].hermit.as_ref().map(|h| h.phase), Some(HermitPhase::Hut));
    let ev = pay_at_hut(&mut isl, &data, m);
    assert_eq!(count(&ev, "hermitPaid"), 1);
    assert_eq!(count(&ev, "hermitBuilding"), 1, "der Bau beginnt im selben Takt");
    assert_eq!(isl.stages[1].players[0].gold, 80, "20 Münzen abgezogen");
    let s = hall(&isl);
    assert_eq!((s.state.as_str(), s.max_hp), (BUILDING_STATE, 250.0));
    assert!(isl.hermit.paid);
    let h = isl.stages[0].hermit.as_ref().expect("Zustand");
    assert_eq!(h.phase, HermitPhase::Walking, "Figur unterwegs");
    assert!(
        isl.stages[1].coins.is_empty(),
        "an der Hütte fällt keine Münze auf den Boden"
    );
}

#[test]
fn bauwerk_steht_nach_der_bauzeit_und_hat_volle_hp() {
    let data = hermit_data("swordsmith");
    let (_, mut isl) = hermit_island(&data, 1);
    let m = add_deep_player(&mut isl);
    pay_at_hut(&mut isl, &data, m);
    let ev = run(&mut isl, &data, 10.0);
    assert!(count(&ev, "buildProgress") >= 1);
    assert_eq!(hall(&isl).state, BUILDING_STATE, "nach 10 s von 20 s noch im Bau");
    let p = isl.stages[0].hermit.as_ref().map_or(0.0, |h| h.progress);
    assert!(p > 0.4 && p < 0.6, "Fortschritt {p}");
    let ev = run(&mut isl, &data, 11.0);
    assert_eq!(count(&ev, "built"), 1);
    let s = hall(&isl);
    assert_eq!((s.state.as_str(), s.hp, s.max_hp), ("built", 250.0, 250.0));
    let h = isl.stages[1].hermit.as_ref().expect("Zustand");
    assert!(h.bonus && h.phase == HermitPhase::Built);
}

#[test]
fn zweite_zahlung_wird_abgelehnt_die_huette_ist_leer() {
    let data = hermit_data("swordsmith");
    let (_, mut isl) = hermit_island(&data, 1);
    let m = add_deep_player(&mut isl);
    pay_at_hut(&mut isl, &data, m);
    let ev = pay_at_hut(&mut isl, &data, m);
    assert_eq!(count(&ev, "hermitRejected"), 1);
    assert_eq!(count(&ev, "hermitPaid"), 0);
    assert_eq!(isl.stages[1].players[0].gold, 80, "kein zweiter Abzug");
    run(&mut isl, &data, 25.0);
    let ev = pay_at_hut(&mut isl, &data, m);
    assert_eq!(
        (count(&ev, "hermitRejected"), isl.stages[1].players[0].gold),
        (1, 80),
        "auch nach dem Bau"
    );
}

#[test]
fn zu_wenig_muenzen_kein_abzug_einmal_abgelehnt() {
    let data = hermit_data("swordsmith");
    let (_, mut isl) = hermit_island(&data, 1);
    let m = add_deep_player(&mut isl);
    isl.stages[1].players[0].gold = 19;
    let mut rejected = 0;
    tick(&mut isl, &data, &[]);
    for _ in 0..5 {
        rejected += count(&tick(&mut isl, &data, &[m]), "hermitRejected");
    }
    assert_eq!(rejected, 1, "gehaltene Taste: ein Ereignis je Druck");
    assert_eq!(isl.stages[1].players[0].gold, 19);
    assert_eq!((hall(&isl).state.as_str(), isl.hermit.paid), ("unpaid", false));
    isl.stages[1].players[0].gold = 20;
    pay_at_hut(&mut isl, &data, m);
    assert_eq!(hall(&isl).state, BUILDING_STATE, "genau der Preis reicht");
}

#[test]
fn jeder_der_beiden_monarchen_kann_zahlen() {
    for second in [false, true] {
        let data = hermit_data("priestess");
        let (_, mut isl) = hermit_island(&data, 0);
        let first = add_deep_player(&mut isl);
        let other = add_deep_player(&mut isl);
        let payer = if second { other } else { first };
        pay_at_hut(&mut isl, &data, payer);
        let gold: Vec<i64> = isl.stages[1].players.iter().map(|p| p.gold).collect();
        let want = if second { vec![100, 80] } else { vec![80, 100] };
        assert_eq!(gold, want, "nur der Zahler verliert Münzen");
        assert_eq!(hall(&isl).state, BUILDING_STATE);
    }
}

#[test]
fn zahlen_beide_im_selben_takt_zahlt_nur_der_erste() {
    let data = hermit_data("swordsmith");
    let (_, mut isl) = hermit_island(&data, 0);
    let a = add_deep_player(&mut isl);
    let b = add_deep_player(&mut isl);
    tick(&mut isl, &data, &[]);
    let ev = tick(&mut isl, &data, &[a, b]);
    assert_eq!((count(&ev, "hermitPaid"), count(&ev, "hermitRejected")), (1, 1));
    let gold: Vec<i64> = isl.stages[1].players.iter().map(|p| p.gold).collect();
    assert_eq!(gold, vec![80, 100], "der Spieler mit dem kleineren Index zahlt");
}

#[test]
fn ausserhalb_der_reichweite_keine_zahlung() {
    let data = hermit_data("swordsmith");
    let (_, mut isl) = hermit_island(&data, 0);
    let m = add_deep_player(&mut isl);
    let x = hut(&isl).x;
    isl.stages[1].players[0].x = x + hut(&isl).range + 2.0;
    run(&mut isl, &data, 1.0); // der Beitritts-Tastendruck wirft keine Münze (pay_cooldown)
    let ev = pay_at_hut(&mut isl, &data, m);
    assert_eq!((count(&ev, "hermitPaid"), count(&ev, "hermitRejected")), (0, 0));
    assert_eq!(isl.stages[1].players[0].gold, 99, "weit weg fällt eine Münze zu Boden");
    assert_eq!(hall(&isl).state, "unpaid");
}

#[test]
fn vor_hub_stufe_2_nimmt_die_huette_nichts_an() {
    let data = hermit_data("swordsmith");
    let (_, mut isl) = hermit_island(&data, 0);
    let m = add_deep_player(&mut isl);
    isl.stages[0].hub_level = 1;
    let ev = pay_at_hut(&mut isl, &data, m);
    assert_eq!(count(&ev, "hermitRejected"), 1);
    assert_eq!(
        (isl.stages[1].players[0].gold, hall(&isl).state.as_str()),
        (100, "unpaid")
    );
    isl.stages[0].hub_level = 2;
    pay_at_hut(&mut isl, &data, m);
    assert_eq!(hall(&isl).state, BUILDING_STATE);
}

#[test]
fn der_platz_im_hub_nimmt_keine_muenzen_an() {
    let data = hermit_data("swordsmith");
    let (_, mut isl) = hermit_island(&data, 1);
    let x = hall(&isl).x;
    isl.stages[0].players[0].x = x;
    for _ in 0..30 {
        tick(&mut isl, &data, &[0]);
    }
    let s = hall(&isl);
    assert_eq!(
        (s.state.as_str(), s.paid_gold),
        ("unpaid", 0),
        "nur die Hütte bezahlt das Bauwerk"
    );
}

#[test]
fn ohne_stufe_2_gibt_es_nichts_zu_zahlen() {
    let data = hermit_data("swordsmith");
    let (_, mut isl) = hermit_island(&data, 1);
    isl.stages.truncate(1); // Stufe 2 ist verloren: die Hütte ist nicht erreichbar
    for _ in 0..3 {
        tick(&mut isl, &data, &[0]);
    }
    assert_eq!((hall(&isl).state.as_str(), isl.hermit.paid), ("unpaid", false));
    assert!(isl.stages.iter().all(|w| w.hermit_hut().is_none()));
}
