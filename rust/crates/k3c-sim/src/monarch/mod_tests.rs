//! Tests zu `monarch/mod.rs` (Go: `monarch_test.go`, S1.1, B-118, docs/rules/monarch.md §§ 1–3): Schlag, Fund-Pool, Gating,
//! Respec, Presets.
//!
//! Die Tests über `emit` und `unit_x` (Schlag, Truhe: events.go) und `hit_event` warten auf RU19 und tragen `ignore`. Die
//! Tests mit Präfix `own_` haben kein Go-Gegenstück.

use serde_json::json;

use super::testutil::*;
use super::*;
use crate::crown::testutil::fell;
use crate::enemies::testutil::{DT, add_player, quiet_world, spawn};
use crate::island::testutil::{add_island_pool, add_pool_points, count, must_island, must_learn, test_data};
use crate::island::{add_island_player, step_island};
use crate::state::Pickup;
use crate::world::step;
use crate::world::values_room::default_values;

#[test]
#[ignore = "wartet auf RU19 (emit, unit_x, hit_event)"]
fn monarch_schlag_reichweite_und_abklingzeit() {
    let mut w = quiet_world();
    let pi = add_player(&mut w);
    let v = vals(&w);
    let px = w.players[pi].x;
    let far = spawn(&mut w, "goblin", px - 2.0);
    let cmd = strike_cmd();
    step_attack(&mut w, &v, pi, &cmd, DT);
    let (hp, max) = (w.enemies[far].hp, w.enemies[far].max_hp);
    let cd = w.players[pi].attack_cooldown;
    // Schlag ins Leere: keine Wirkung, aber Abklingzeit (B-321)
    assert!(
        hp == max && cd == 0.7,
        "Gegner in 2 Units getroffen oder keine Abklingzeit: HP {hp}, CD {cd}"
    );
    w.players[pi].attack_cooldown = 0.0;
    let e = spawn(&mut w, "goblin", px + 1.5);
    step_attack(&mut w, &v, pi, &cmd, DT);
    let (ehp, emax) = (w.enemies[e].hp, w.enemies[e].max_hp);
    let cd = w.players[pi].attack_cooldown;
    assert!(
        ehp == emax - 10.0 && w.enemies[far].hp == w.enemies[far].max_hp && cd == 0.7,
        "Schlag in 1,5 Units: HP {ehp}/{emax}, CD {cd}"
    );
    for _ in 0..18 {
        // 0,6 s: noch in der Abklingzeit
        step_attack(&mut w, &v, pi, &cmd, DT);
    }
    assert_eq!(w.enemies[e].hp, emax - 10.0, "zweiter Schlag vor 0,7 s");
    for _ in 0..5 {
        // 0,77 s: genau ein weiterer Schlag
        step_attack(&mut w, &v, pi, &cmd, DT);
    }
    assert_eq!(w.enemies[e].hp, emax - 20.0, "zweiter Schlag nach 0,7 s fehlt");
}

#[test]
#[ignore = "wartet auf RU19 (emit, unit_x, hit_event, step)"]
fn monarch_schlag_ereignisse() {
    let mut w = quiet_world();
    let pi = add_player(&mut w);
    let ev = events_of(&mut w, "strike", 3, &[strike_cmd()]);
    assert!(
        ev.len() == 1 && ev[0]["hit"] == json!(false) && w.players[pi].attack_cooldown != 0.0,
        "ohne Gegner: {ev:?}, CD {}",
        w.players[pi].attack_cooldown
    );
    w.players[pi].attack_cooldown = 0.0;
    let px = w.players[pi].x;
    let e = spawn(&mut w, "goblin", px + 1.0);
    step(&mut w, &[strike_cmd()], None, DT);
    let (pid, eid) = (json!(w.players[pi].id), json!(w.enemies[e].id));
    let strike = w.events.iter().find(|e| e["type"] == "strike" && e["from"] == pid);
    let hit = w
        .events
        .iter()
        .find(|e| e["type"] == "hit" && e["target"] == "enemy" && e["id"] == eid && e["damage"] == json!(10.0));
    assert!(
        strike.is_some_and(|s| s["x"] == json!(unit_x(px))) && hit.is_some(),
        "strike {strike:?}, hit {hit:?}, Ereignisse {:?}",
        w.events
    );
    fell(&mut w, pi);
    w.players[pi].attack_cooldown = 0.0;
    let ev = events_of(&mut w, "strike", 1, &[strike_cmd()]);
    assert!(
        !ev.iter().any(|e| e["from"] == pid),
        "ein Monarch ohne Krone schlägt zu"
    );
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story: must_island lädt die Standard-Welt)"]
fn monarch_pool_je_spieler_und_stufe() {
    let data = test_data();
    let mut isl = must_island(&data, "pool", &[0, 1]);
    let (p0, p1) = (add_island_player(&mut isl, 0), add_island_player(&mut isl, 1));
    add_island_pool(&mut isl, 0, 3);
    let (a, b) = (isl.stages[0].skill_points, isl.stages[1].skill_points);
    assert!(
        a == 3 && b == 3 && pool_of(&isl.stages[1]) == 3,
        "Pool gilt nicht in allen Stufen: {a}, {b}"
    );
    must_learn(&mut isl.stages[0], p0, &["taunt", "shieldBash", "armorAura"]);
    must_learn(&mut isl.stages[1], p1, &["fireball", "iceWall", "arcanePower"]);
    let (w0, w1) = (&isl.stages[0], &isl.stages[1]);
    assert!(
        available_points(w0, p0) == 0
            && available_points(w1, p1) == 0
            && w0.players[p0].skills.len() == 3
            && w1.players[p1].skills.len() == 3,
        "Verteilung nicht getrennt: {:?}, {:?}",
        w0.players[p0].skills,
        w1.players[p1].skills
    );
    let v = vals(&isl.stages[0]);
    assert!(
        learn_skill(&mut isl.stages[0], &v, p0, "thickSkin").is_err(),
        "vierter Skill ohne Punkt gelernt"
    );
    let p2 = add_island_player(&mut isl, 0); // Heiler-Preset verteilt beim Beitritt
    let w0 = &isl.stages[0];
    let skills = &w0.players[p2].skills;
    assert!(
        skills.len() as i64 + available_points(w0, p2) == 3 && skills == &["heal", "groupHeal"],
        "Spätbeitretender: Skills {skills:?}, frei {}",
        available_points(w0, p2)
    );
}

#[test]
fn monarch_tier_gating() {
    let mut w = quiet_world();
    let pi = add_player(&mut w);
    let v = vals(&w);
    add_pool_points(&mut w, 10);
    must_learn(&mut w, pi, &["taunt", "fireball"]); // der Zauberer-Skill zählt nicht für die Tank-Linie
    assert!(
        learn_skill(&mut w, &v, pi, "armorAura").is_err(),
        "Passiv (Tier 2) mit 1 Skill der Linie gelernt"
    );
    must_learn(&mut w, pi, &["shieldBash", "armorAura"]);
    assert!(
        learn_skill(&mut w, &v, pi, "ironWall").is_err(),
        "Tier 3 mit 3 Skills der Linie gelernt"
    );
    must_learn(&mut w, pi, &["thickSkin", "ironWall"]);
    assert!(
        learn_skill(&mut w, &v, pi, "lastStand").is_err(),
        "Tier 4 mit 5 Skills der Linie gelernt"
    );
    must_learn(&mut w, pi, &["regeneration", "lastStand"]);
    for bad in ["taunt", "shadowStrike"] {
        assert!(
            learn_skill(&mut w, &v, pi, bad).is_err(),
            "{bad}: schon gelernt bzw. unbekannt, trotzdem gelernt"
        );
    }
    assert_eq!(w.players[pi].slots, ["taunt", "fireball", "shieldBash", "ironWall"]);
}

#[test]
fn monarch_respec_nur_am_tag_an_der_burg() {
    let mut w = quiet_world();
    let pi = add_player(&mut w);
    let v = vals(&w);
    add_pool_points(&mut w, 2);
    must_learn(&mut w, pi, &["heal", "groupHeal"]);
    for phase in ["dusk", "night"] {
        w.cycle.phase = phase.into();
        assert!(respec(&mut w, &v, pi).is_err(), "Respec in Phase {phase}");
    }
    w.cycle.phase = "day".into();
    let x = w.players[pi].x;
    w.players[pi].x = w.castle.x + 10.0;
    assert!(respec(&mut w, &v, pi).is_err(), "Respec weit weg von der Burg");
    w.players[pi].x = x;
    let r = respec(&mut w, &v, pi);
    let p = &w.players[pi];
    assert!(
        r.is_ok() && p.skills.is_empty() && p.slots.is_empty() && available_points(&w, pi) == 2,
        "Respec am Tag an der Burg: {r:?}, Skills {:?}, frei {}",
        p.skills,
        available_points(&w, pi)
    );
}

#[test]
#[ignore = "wartet auf RU19 (emit, unit_x, step_island)"]
fn monarch_dritte_truhe_der_insel() {
    let data = test_data();
    let mut isl = must_island(&data, "truhen", &[0, 1]);
    let (p0, p1) = (add_island_player(&mut isl, 0), add_island_player(&mut isl, 1));
    let (x0, x1) = (isl.stages[0].players[p0].x, isl.stages[1].players[p1].x);
    let cmds = [PlayerCommand::default(), PlayerCommand::default()];
    let chest = |w: &mut World, x: f64| {
        let id = w.new_id();
        Pickup {
            id,
            kind: "chest".into(),
            x,
        }
    };
    let pickups = [chest(&mut isl.stages[0], x0), chest(&mut isl.stages[0], x0)];
    isl.stages[0].pickups = pickups.to_vec();
    isl.stages[1].pickups = Vec::new();
    step_island(&mut isl, &cmds, &data.difficulty, DT);
    assert!(
        isl.chests_opened == 2 && isl.skill_pool == 0,
        "2 Truhen: geöffnet {}, Pool {}",
        isl.chests_opened,
        isl.skill_pool
    );
    let third = chest(&mut isl.stages[1], x1);
    isl.stages[1].pickups = vec![third];
    step_island(&mut isl, &cmds, &data.difficulty, DT);
    let (a, b) = (isl.stages[0].skill_points, isl.stages[1].skill_points);
    assert!(
        isl.skill_pool == 1 && a == 1 && b == 1 && count(&isl.stages[1].events, "skillPoint") == 1,
        "3. Truhe der Insel: Pool {}, Stufen {a}/{b}",
        isl.skill_pool
    );
}

#[test]
fn monarch_preset_beim_beitritt() {
    let mut w = quiet_world();
    add_pool_points(&mut w, 2);
    let p = add_player(&mut w); // Index 0: Tank
    assert!(
        w.players[p].skills == ["taunt", "shieldBash"] && w.players[p].slots == w.players[p].skills,
        "Tank-Preset: {:?}, Slots {:?}",
        w.players[p].skills,
        w.players[p].slots
    );
    let q = add_player(&mut w);
    assert_eq!(w.players[q].skills, ["fireball", "iceWall"], "Zauberer-Preset");
    let mut w2 = quiet_world();
    add_pool_points(&mut w2, 1);
    let p = add_player(&mut w2);
    assert_eq!(w2.players[p].skills, ["taunt"], "Preset mit 1 Punkt");
    let q = add_player(&mut w2);
    assert!(
        apply_preset(&mut w2, q, "unbekannt").is_err(),
        "unbekanntes Preset angenommen"
    );
}

#[test]
fn monarch_daten_katalog() {
    let v = default_values();
    for id in ["tank", "mage", "healer"] {
        // Dieb-Linie ist Nicht-Ziel (keine Skills im Katalog)
        for s in &v.monarch.presets[id].active_skills {
            assert!(skill_by_id(&v, s).is_some(), "Preset {id}: Skill {s} fehlt in skills");
        }
    }
    for s in &v.skills.list {
        let valid = v.monarch.lines.contains_key(&s.line) && (s.kind == "active" || s.kind == "passive");
        assert!(
            valid,
            "Skill {}: Linie {:?} oder Art {:?} ungültig",
            s.id, s.line, s.kind
        );
    }
    let a = &v.monarch.attack;
    assert!(
        a.damage == 10.0 && a.range == 1.5 && a.cooldown == 0.7 && v.monarch.tier_points.len() == 4,
        "attack {a:?}, tierPoints {:?}",
        v.monarch.tier_points
    );
}

/// Jeder Tier jeder Linie ist mit dem Katalog erreichbar (B-216): Skills der Linie nach Tier lernen.
#[test]
fn monarch_jeder_tier_erreichbar() {
    for line in ["tank", "mage", "healer"] {
        let mut w = quiet_world();
        let pi = add_player(&mut w);
        let v = vals(&w);
        add_pool_points(&mut w, v.skills.list.len() as i64);
        for tier in 1..=4 {
            for s in v.skills.list.iter().filter(|s| s.line == line && s.tier == tier) {
                must_learn(&mut w, pi, &[s.id.as_str()]);
            }
        }
    }
}

/// Eigener Test (AC-03): Zwei Spieler lernen und setzen im selben Tick zurück; der Pool gehört der Insel, die Verteilung
/// dem Spieler, die Reihenfolge der Aufrufe ändert nichts.
#[test]
fn own_zwei_spieler_lernen_und_respec_im_selben_tick() {
    let run = |swap: bool| {
        let mut w = quiet_world();
        add_pool_points(&mut w, 4);
        let (a, b) = (add_player(&mut w), add_player(&mut w)); // Tank (taunt, shieldBash), Zauberer (fireball, iceWall)
        let v = vals(&w);
        let order = if swap { [b, a] } else { [a, b] };
        for pi in order {
            let id = if pi == a { "armorAura" } else { "arcanePower" };
            must_learn(&mut w, pi, &[id]);
        }
        let respec_a = respec(&mut w, &v, a); // Tag an der Burg
        let learn_a = learn_skill(&mut w, &v, a, "heal");
        (
            respec_a,
            learn_a,
            w.players[a].skills.clone(),
            w.players[b].skills.clone(),
            available_points(&w, a),
            available_points(&w, b),
        )
    };
    let first = run(false);
    assert_eq!(first, run(true), "Reihenfolge ändert das Ergebnis");
    let (respec_a, learn_a, sa, sb, free_a, free_b) = first;
    assert!(respec_a.is_ok() && learn_a.is_ok(), "{respec_a:?} {learn_a:?}");
    assert_eq!(sa, ["heal"]);
    assert_eq!(sb, ["fireball", "iceWall", "arcanePower"]);
    assert_eq!((free_a, free_b), (3, 1));
}

/// Eigener Test: `add_pool_points` ohne Insel zählt den Zähler der Welt, mit Insel den Pool der Insel und spiegelt ihn in
/// die Stufe; `chest_skill_point` zählt Truhen und gibt erst jede n-te einen Punkt (hier nur die ersten zwei, ohne Ereignis).
#[test]
fn own_pool_und_truhenzaehler() {
    let mut w = quiet_world();
    let v = vals(&w);
    let pi = add_player(&mut w);
    add_pool_points(&mut w, 2);
    assert_eq!((w.skill_points, pool_of(&w)), (2, 2));
    let mut r = IslandRef::default();
    crate::monarch::add_pool_points(&mut w, Some(&mut r), 3);
    assert_eq!((r.skill_pool, w.skill_points), (3, 3), "Pool der Insel gespiegelt");
    let every = v.monarch.skill_point_sources.chest_every;
    assert!(every > 2, "chestEvery {every}");
    for n in 1..=2 {
        chest_skill_point(&mut w, &v, Some(&mut r), pi);
        assert_eq!((r.chests_opened, r.skill_pool), (n, 3));
    }
    chest_skill_point(&mut w, &v, None, pi); // ohne Insel zählt nichts
    assert_eq!(r.chests_opened, 2);
}

/// Eigener Test (Ersatz für `monarch_pool_je_spieler_und_stufe` ohne `must_island`): Der Bezug `IslandRef` trägt Pool,
/// Truhenzähler, Händler-Besuche und die Taverne der anderen Stufen aus der Insel in die Stufe und zurück; der Pool wird
/// in alle Stufen gespiegelt.
#[test]
fn own_island_ref_traegt_pool_truhen_besuche_und_taverne() {
    use crate::island::Island;
    use crate::state::Site;
    let v = default_values();
    let diff = Default::default();
    let mut isl = Island {
        stages: vec![World::default(), World::default()],
        ..Island::default()
    };
    isl.stages[1].sites.push(Site {
        kind: "tavern".into(),
        state: "built".into(),
        ..Site::default()
    });
    let mut r = isl.stage_ref(0, &diff);
    assert!(r.tavern_elsewhere, "Taverne der anderen Stufe");
    assert!(
        !isl.stage_ref(1, &diff).tavern_elsewhere,
        "die eigene Stufe liest `w.sites`"
    );
    crate::monarch::add_pool_points(&mut isl.stages[0], Some(&mut r), 3);
    chest_skill_point(&mut isl.stages[0], &v, Some(&mut r), 0);
    r.merchant_visits += 1;
    isl.put_stage_ref(r);
    assert_eq!((isl.skill_pool, isl.chests_opened, isl.merchant_visits), (3, 1, 1));
    assert_eq!(
        (
            isl.stages[0].skill_points,
            isl.stages[1].skill_points,
            pool_of(&isl.stages[1])
        ),
        (3, 3, 3),
        "Pool in allen Stufen"
    );
    let r1 = isl.stage_ref(1, &diff);
    assert_eq!((r1.skill_pool, r1.chests_opened, r1.merchant_visits), (3, 1, 1));
}
