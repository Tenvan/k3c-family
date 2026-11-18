//! Tests zu `spell_tower.rs` (Go: `building_spell_tower_test.go`, W3.2, B-116/AC-01, Sprint W3 AC-01): Der Zaubertum
//! (Turm-Stufe `spell.from_level`) schießt selbst und trifft alle Gegner im Radius um das Ziel (Q31); die Schützen steigen
//! ab und bleiben Kämpfer. Werte aus tower.spell. Tests über `day_world`, `level_of` und `step` warten auf RU18.
#![allow(clippy::unimplemented)] // Rumpf des Tests, der `upgrade_at` (RU18.3) braucht

use super::barracks::fighters;
use super::spell_tower::{is_spell_tower, step_spell_towers};
use super::testutil::*;
use crate::hub::level_of;
use crate::state::World;
use crate::world::vals;

/// Go: `spellWorld`: Turm der Linie 1 links auf Stufe `level`, Gegner fest an Abständen zum Turm (nach außen), zwei Spieler.
fn spell_world(level: i64, offsets: &[f64]) -> (World, usize, Vec<usize>) {
    let mut w = day_world();
    add_player(&mut w);
    add_player(&mut w);
    w.hub_level = level;
    let tower = line_site(&mut w, "tower", -1, 1);
    built(&mut w.sites[tower]);
    w.sites[tower].level = level;
    let tx = w.sites[tower].x;
    let mut enemies = vec![];
    for dx in offsets {
        let e = spawn(&mut w, "skeleton", tx - dx);
        w.enemies[e].hp = 1000.0;
        w.enemies[e].max_hp = 1000.0;
        enemies.push(e);
    }
    (w, tower, enemies)
}

#[test]
fn zaubertum_flaechenschaden_im_takt() {
    let sp = vals(&bare_world()).buildings["tower"].spell.clone();
    let near = sp.range_units / 2.0;
    // Ziel, im Radius, außerhalb des Radius (in Reichweite), außerhalb der Reichweite.
    let offsets = [
        near,
        near + sp.radius_units - 0.5,
        near + sp.radius_units + 0.5,
        sp.range_units + sp.radius_units + 1.0,
    ];
    let (mut w, _, es) = spell_world(sp.from_level, &offsets);
    step_spell_towers(&mut w, DT);
    for (i, want) in [1000.0 - sp.damage, 1000.0 - sp.damage, 1000.0, 1000.0]
        .into_iter()
        .enumerate()
    {
        assert_eq!(w.enemies[es[i]].hp, want, "Gegner {i} nach dem ersten Schuss");
    }
    let mut tm = DT;
    while tm < sp.interval_seconds - DT / 2.0 {
        step_spell_towers(&mut w, DT);
        tm += DT;
    }
    assert_eq!(
        w.enemies[es[0]].hp,
        1000.0 - sp.damage,
        "vor Ablauf des Takts erneut getroffen"
    );
    step_spell_towers(&mut w, DT);
    assert_eq!(
        w.enemies[es[0]].hp,
        1000.0 - 2.0 * sp.damage,
        "nach {} s kein zweiter Schuss",
        sp.interval_seconds
    );
}

#[test]
fn turm_unter_zaubertum_stufe_schiesst_nicht() {
    let sp = vals(&bare_world()).buildings["tower"].spell.clone();
    let (mut w, _, es) = spell_world(sp.from_level - 1, &[2.0]);
    let mut tm = 0.0;
    while tm < 2.0 * sp.interval_seconds {
        step_spell_towers(&mut w, DT);
        tm += DT;
    }
    assert_eq!(
        w.enemies[es[0]].hp,
        1000.0,
        "Turm-Stufe {} macht Flächenschaden",
        sp.from_level - 1
    );
}

/// Ausbau auf die Zaubertum-Stufe: der Schütze steigt ab, bleibt Kämpfer, der Turm nimmt keinen mehr auf.
#[test]
fn zaubertum_schuetzen_steigen_ab() {
    use super::upgrades_sites_tests::upgrade_at;
    use crate::enemies::testutil::run;
    use crate::unit::archer::free_tower;
    use crate::unit::jobs::make_archer;
    use crate::unit::spawn_vagrant;
    let sp = vals(&bare_world()).buildings["tower"].spell.clone();
    let (mut w, tower, _) = spell_world(sp.from_level - 1, &[]);
    w.hub_level = sp.from_level;
    let (hub, tx) = (w.hub_x, w.sites[tower].x);
    let archer = spawn_vagrant(&mut w, hub, tx);
    make_archer(&mut w, archer);
    let tower_id = w.sites[tower].id;
    (w.troops[archer].anchor_x, w.troops[archer].tower_id) = (tx, Some(tower_id));
    let players: Vec<usize> = (0..w.players.len()).collect();
    assert!(
        upgrade_at(&mut w, &players, tower) >= 0.0,
        "Ausbau auf die Zaubertum-Stufe nicht fertig"
    );
    run(&mut w, 1.0);
    let a = &w.troops[archer];
    assert!(
        a.tower_id.is_none() && a.kind == "archer" && fighters(&w) == 1,
        "Schütze: Turm {:?}, Art {}, Kämpfer {}",
        a.tower_id,
        a.kind,
        fighters(&w)
    );
    let v = vals(&w);
    assert_ne!(
        free_tower(&w, &v, archer),
        Some(tower_id),
        "Zaubertum bietet weiter Plätze für Schützen"
    );
    let ei = spawn(&mut w, "skeleton", tx - 2.0);
    w.enemies[ei].hp = 1000.0;
    w.enemies[ei].max_hp = 1000.0;
    crate::world::step(&mut w, &[], None, DT); // der Schuss läuft im Takt der Sim, sofort, ohne Flugzeit
    assert_eq!(w.enemies[ei].hp, 1000.0 - sp.damage, "Zaubertum schießt im Spiel nicht");
}

/// Eigener Test (ohne Go-Gegenstück, kein `level_of`-Platzhalter nötig): Ein Turm unter der Zaubertum-Stufe oder ungebaut
/// ist kein Zaubertum, die Zahl der Zaubertürme folgt `level_of`.
#[test]
fn is_spell_tower_nach_stufe() {
    let mut w = bare_world();
    let hub = w.hub_x;
    let ti = built_site(&mut w, "tower", hub - 80.0);
    let from = vals(&w).buildings["tower"].spell.from_level;
    w.sites[ti].level = from - 1;
    assert!(!is_spell_tower(&w, &w.sites[ti]));
    w.sites[ti].level = from;
    assert!(is_spell_tower(&w, &w.sites[ti]) && level_of(&w, &w.sites[ti]) >= from);
    w.sites[ti].state = "unpaid".into();
    assert!(!is_spell_tower(&w, &w.sites[ti]));
}
