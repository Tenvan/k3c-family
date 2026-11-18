//! Begleitschutz (Go: `unit_escort_test.go`, B-647 Weg A): Ist ein Monarch beim kämpfenden Endboss, marschieren die
//! Kämpfer der Stufe zu ihm und treffen ihn; ohne Monarch in der Nähe bleiben sie auf ihrem Posten.

use super::escort::{escort_post, escort_post_at};
use crate::enemies::testutil::*;
use crate::state::Troop;

#[test]
fn escort_post_at_liegt_auf_der_seite_des_hubs_in_reichweite() {
    let w = bare_world();
    let v = w.vals.clone().expect("Wertesatz");
    let archer = Troop {
        kind: "archer".into(),
        ..Troop::default()
    };
    let warrior = Troop {
        kind: "warrior".into(),
        ..Troop::default()
    };
    let boss_x = w.hub_x + 300.0; // Boss rechts vom Hub: die Truppen kommen von links
    let a = escort_post_at(&w, &v, &archer, boss_x);
    assert_eq!(a, boss_x - (v.troops["archer"].range * 0.8).max(1.0));
    let k = escort_post_at(&w, &v, &warrior, boss_x);
    assert_eq!(k, boss_x - (v.troops["warrior"].range * 0.8).max(1.0));
    let left = w.hub_x - 300.0; // Boss links: die Truppen kommen von rechts
    assert!(escort_post_at(&w, &v, &archer, left) > left);
    let blind = Troop {
        kind: "unbekannt".into(),
        ..Troop::default()
    };
    assert_eq!(
        escort_post_at(&w, &v, &blind, boss_x),
        boss_x - 1.0,
        "mindestens eine Unit Abstand"
    );
}

#[test]
#[ignore = "wartet auf RU10"] // der Bezug zur Insel (`IslandRef`) kommt mit dem Insel-Schritt (`None` hier ist Platzhalter)
fn truppen_begleiten_den_monarchen_zum_endboss() {
    let (mut isl, lair) = end_island(1);
    let w = &mut isl.stages[0];
    let v0 = w.vals.clone().expect("Wertesatz");
    let (archers, warriors) = unverwundbare_truppen(w);
    // Ohne Monarch am Boss: kein Begleitschutz (der Boss ist gar nicht erschienen).
    assert!(
        escort_post(w, &v0, None, &w.troops[archers[0]]).is_none(),
        "Begleitschutz ohne Endboss"
    );
    w.players[0].x = lair - 10.0; // näher als triggerUnits: der Boss erscheint und der Begleitschutz rastet ein
    for _ in 0..2 {
        step(w, None, DT);
    }
    let boss = w.enemies.iter().position(|e| e.boss).expect("Endboss nicht erschienen");
    w.enemies[boss].hp = 1e9;
    w.enemies[boss].max_hp = 1e9; // der Test prüft den Marsch, nicht den Sieg
    for _ in 0..120 {
        step(w, None, DT);
    }
    let v = w.vals.clone().expect("Wertesatz");
    for &a in &archers {
        let d = (w.troops[a].x - w.enemies[boss].x).abs();
        assert!(d <= v.troops["archer"].range + 1.0, "Schütze steht {d:.1} vom Boss");
    }
    for &k in &warriors {
        let d = (w.troops[k].x - w.enemies[boss].x).abs();
        assert!(d <= v.troops["warrior"].range + 2.0, "Krieger steht {d:.1} vom Boss");
    }
    assert!(w.enemies[boss].hp < 1e9, "kein Truppenschaden am Boss");
    // Der Begleitschutz bleibt, auch wenn der Monarch weggeht, und endet mit dem Tod des Bosses.
    w.players[0].x = w.hub_x;
    step(w, None, DT);
    assert!(
        escort_post(w, &v0, None, &w.troops[archers[0]]).is_some(),
        "Begleitschutz hörte mit dem Monarchen auf"
    );
    w.enemies[boss].hp = 0.0;
    assert!(
        escort_post(w, &v0, None, &w.troops[archers[0]]).is_none(),
        "Begleitschutz trotz totem Boss"
    );
}

/// Go: `troopsAt`: drei Schützen rechts und drei Krieger links vom Hub, unverwundbar: Der Test prüft den Marsch; ein
/// gefallener Kämpfer rüstet sich in Bossnähe nicht neu (kampf.md § 4). Liefert die Indizes.
fn unverwundbare_truppen(w: &mut crate::state::World) -> (Vec<usize>, Vec<usize>) {
    let hub = w.hub_x;
    let (mut archers, mut warriors) = (Vec::new(), Vec::new());
    for _ in 0..3 {
        for (kind, dx, list) in [("archer", 5.0, &mut archers), ("warrior", -5.0, &mut warriors)] {
            let id = w.new_id();
            w.troops.push(Troop {
                id,
                kind: kind.into(),
                x: hub + dx,
                anchor_x: hub + dx,
                hp: 1e9,
                max_hp: 1e9,
                ..Troop::default()
            });
            list.push(w.troops.len() - 1);
        }
    }
    (archers, warriors)
}
