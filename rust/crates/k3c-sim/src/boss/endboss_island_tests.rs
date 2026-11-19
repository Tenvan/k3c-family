//! Endboss auf der Insel (Go: `boss_endboss_test.go`, K2.1c, B-130/AC-02, AC-03): Auslösung, Phasen, Skalierung,
//! Belohnung, keine Rückkehr. Die Insel tickt mit `step_island` (Go: `Step(w)` mit `w.island`).

use super::data::end_boss;
use super::testutil::count_kind;
use crate::enemies::testutil::end_island;
use crate::island::testutil::{DT, test_data};
use crate::island::{Island, step_island};
use crate::state::{Event, World};

const WARDEN_ID: &str = "crystalHeartWarden";

/// Go: `warden`: der Endboss der Kristallhöhle in der Stufe (Index).
fn warden(w: &World) -> Option<usize> {
    w.enemies.iter().position(|e| e.kind == WARDEN_ID)
}

/// Go: `protect`: hält die Spieler handlungsfähig: Krone auf, viel Gold und Last Stand gegen den letzten Treffer.
fn protect(w: &mut World) {
    for p in &mut w.players {
        (p.crown, p.gold, p.last_stand_for) = (None, 1000, 1.0);
    }
}

/// Go: `endSteps`: tickt die Insel `seconds` lang (Stufe 0 wie in Go `Step(w)`); `keep` hält die Spieler am Leben.
/// Liefert alle Ereignisse der Stufe 0.
fn end_steps(isl: &mut Island, data: &k3c_data::Data, seconds: f64, keep: bool) -> Vec<Event> {
    let mut events = Vec::new();
    let mut tm = 0.0;
    while tm < seconds - 1e-9 {
        if keep {
            protect(&mut isl.stages[0]);
        }
        step_island(isl, &[], &data.difficulty, DT);
        events.extend(isl.stages[0].events.iter().cloned());
        tm += DT;
    }
    events
}

fn count_ev(events: &[Event], typ: &str) -> usize {
    events.iter().filter(|e| e["type"] == typ).count()
}

fn trigger_units() -> f64 {
    end_boss(&test_data().bosses, 4).map_or(0.0, |b| b.lair.trigger_units)
}

// (b) Ein Spieler kommt an: Der Boss erscheint einmal am Bau, läuft nicht und ist verwundbar; mit zwei Spielern an
// verschiedenen Orten genügt einer.
#[test]
fn endboss_wird_bei_ankunft_ausgeloest() {
    let data = test_data();
    let (mut isl, x) = end_island(2);
    isl.stages[0].players[1].x = x - trigger_units() + 1.0;
    let events = end_steps(&mut isl, &data, 10.0, true);
    let w = &isl.stages[0];
    let e = warden(w).expect("Endboss");
    assert!(
        count_ev(&events, "bossSpawned") == 1 && w.enemies[e].x == x && w.enemies[e].home_x == x,
        "erwartet einmal am Bau {x}"
    );
    let (id, hp) = (w.enemies[e].id, w.enemies[e].hp);
    crate::state::common::apply_damage_by(&mut isl.stages[0], id, 10.0, "test", None);
    let e = warden(&isl.stages[0]).expect("Endboss");
    assert_eq!(isl.stages[0].enemies[e].hp, hp - 10.0, "aktiver Endboss unverwundbar");
    let hub = isl.stages[0].hub_x;
    isl.stages[0].players[1].x = hub; // er bleibt aktiv, auch wenn der Spieler geht
    let events = end_steps(&mut isl, &data, 30.0, true);
    assert!(
        warden(&isl.stages[0]).is_some() && count_ev(&events, "bossSpawned") == 0,
        "Endboss nach dem Weggehen nicht mehr aktiv oder neu erschienen"
    );
}

// (d) HP bei 3 Spielern = 2 × HP bei 1 Spieler (1 + 0,5 × 2); später beitretende Spieler ändern sie nicht.
#[test]
fn endboss_hp_skaliert_mit_spielern() {
    let data = test_data();
    let hp_with = |players: i64| {
        let (mut isl, x) = end_island(players);
        isl.stages[0].players[0].x = x;
        end_steps(&mut isl, &data, DT, true);
        let e = warden(&isl.stages[0]);
        crate::island::add_island_player(&mut isl, 0);
        end_steps(&mut isl, &data, DT, true);
        let e = e.map(|i| &isl.stages[0].enemies[i]).expect("Endboss");
        assert_eq!(e.hp, e.max_hp, "{players} Spieler");
        e.max_hp
    };
    let (one, three) = (hp_with(1), hp_with(3));
    let per = data.waves.boss_hp_per_extra_player.unwrap_or(0.0);
    assert!(
        (three - one * (1.0 + 2.0 * per)).abs() <= 1.0,
        "HP 1 Spieler {one}, 3 Spieler {three}"
    );
}

// (e) Belohnung: 42 Gold als Münzen am Bau, 250 Kristall in den Insel-Vorrat, 3 Skill-Punkte in den Pool der Insel
// (für alle Spieler); Merker gesetzt.
#[test]
fn endboss_belohnung() {
    let data = test_data();
    let (mut isl, x) = end_island(2);
    isl.stock = Default::default();
    isl.sync_stock();
    isl.stages[0].players[0].x = x - 10.0;
    end_steps(&mut isl, &data, DT, true);
    let e = warden(&isl.stages[0]).expect("Endboss");
    let (pool, coins) = (isl.skill_pool, isl.stages[0].coins.len());
    isl.stages[0].enemies[e].hp = 0.0;
    let events = end_steps(&mut isl, &data, DT, true);
    let w = &isl.stages[0];
    let near = w.coins.iter().filter(|c| (c.x - x).abs() <= 1.5).count();
    assert!(
        count_ev(&events, "bossDefeated") == 1 && w.coins.len() - coins == 42 && near == 42,
        "{}× bossDefeated, {} neue Münzen ({near} am Bau), erwartet 42",
        count_ev(&events, "bossDefeated"),
        w.coins.len() - coins
    );
    assert!(
        isl.stock.crystal == 250 && isl.skill_pool - pool == 3 && w.skill_points == isl.skill_pool,
        "Kristall {}, Pool +{} (Stufe {})",
        isl.stock.crystal,
        isl.skill_pool - pool,
        w.skill_points
    );
    assert!(
        isl.endboss_defeated && isl.defeated_bosses.iter().any(|b| b == WARDEN_ID),
        "Merker fehlt: {:?}",
        isl.defeated_bosses
    );
}

// (f) Ein aktiver Endboss übersteht einen Burgfall; nach seinem Tod kehrt er nie zurück, auch nicht nach einem
// Burgfall mit einem Spieler am Bau.
#[test]
fn endboss_kehrt_nie_zurueck() {
    let data = test_data();
    let (mut isl, x) = end_island(1);
    isl.stages[0].players[0].x = x;
    end_steps(&mut isl, &data, DT, true);
    let e = warden(&isl.stages[0]).expect("Endboss");
    let id = isl.stages[0].enemies[e].id;
    isl.stages[0].enemies[e].hp -= 1.0;
    let max = isl.stages[0].enemies[e].max_hp;
    isl.stages[0].castle.hp = 0.0;
    end_steps(&mut isl, &data, 2.0 * DT, true);
    let e = warden(&isl.stages[0]).filter(|&i| isl.stages[0].enemies[i].id == id);
    assert!(
        e.is_some_and(|i| isl.stages[0].enemies[i].hp == max - 1.0),
        "aktiver Endboss nach dem Burgfall verschwunden oder geheilt"
    );
    let e = warden(&isl.stages[0]).expect("Endboss");
    isl.stages[0].enemies[e].hp = 0.0;
    end_steps(&mut isl, &data, DT, true);
    for _ in 0..3 {
        isl.stages[0].castle.hp = 0.0;
        let mut events = end_steps(&mut isl, &data, 60.0, false);
        isl.stages[0].players[0].x = x;
        events.extend(end_steps(&mut isl, &data, 60.0, false));
        assert!(
            warden(&isl.stages[0]).is_none()
                && count_ev(&events, "bossSpawned") == 0
                && count_ev(&events, "castleFallen") == 1,
            "besiegter Endboss zurück ({}× bossSpawned, {} Burgfälle)",
            count_ev(&events, "bossSpawned"),
            count_ev(&events, "castleFallen")
        );
    }
    assert_eq!(isl.defeated_bosses.len(), 1, "besiegt {:?}", isl.defeated_bosses);
}

/// Go: `strikesOf`: Zeitpunkte der Flächenschläge des Bosses (`strike` von ihm), Zahl der Phasenwechsel und der Treffer am
/// Spieler mit Index 1.
fn strikes_of(isl: &mut Island, data: &k3c_data::Data, boss_id: i64, seconds: f64) -> (Vec<f64>, usize, usize) {
    let (mut times, mut phases, mut outside) = (Vec::new(), 0, 0);
    let mut tm = 0.0;
    while tm < seconds - 1e-9 {
        protect(&mut isl.stages[0]);
        step_island(isl, &[], &data.difficulty, DT);
        let w = &isl.stages[0];
        for ev in &w.events {
            match ev["type"].as_str().unwrap_or("") {
                "strike" if ev["from"] == boss_id => times.push(w.time),
                "bossPhase" => phases += 1,
                "hit" if ev["target"] == "player" && ev["id"] == 1 => outside += 1,
                _ => {}
            }
        }
        tm += DT;
    }
    (times, phases, outside)
}

/// Go: `gaps`: Abstände der Zeitpunkte, gerundet auf 0,1 s.
fn gaps(times: &[f64]) -> Vec<f64> {
    times
        .windows(2)
        .map(|p| ((p[1] - p[0]) * 10.0).round() / 10.0)
        .collect()
}

// (c) Phasen: Phase 1 ruft Kristallspinnen, Phase 2 (unter 60 %) schlägt im Radius 5 alle 6 s und beschwört nicht
// mehr, Phase 3 (unter 25 %) schlägt mit +50 % Tempo alle 4 s; jeder Wechsel wird genau einmal gemeldet.
#[test]
fn endboss_phasen() {
    let data = test_data();
    let (mut isl, x) = end_island(2);
    (isl.stages[0].players[0].x, isl.stages[0].players[1].x) = (x + 4.0, x + 6.0); // hinter dem Bau
    end_steps(&mut isl, &data, DT, true);
    let boss = warden(&isl.stages[0]).expect("Endboss nicht ausgelöst");
    let id = isl.stages[0].enemies[boss].id;
    let events = end_steps(&mut isl, &data, 10.5, true);
    let spiders = count_kind(&isl.stages[0], "crystalSpider");
    assert!(
        spiders >= 2 && count_ev(&events, "bossPhase") == 0,
        "Phase 1: {spiders} Kristallspinnen"
    );
    isl.stages[0].enemies.retain(|e| e.id == id);
    let e = &mut isl.stages[0].enemies[0];
    e.hp = (e.max_hp * 0.59).floor();
    let (times, phases, outside) = strikes_of(&mut isl, &data, id, 25.0);
    let spiders = count_kind(&isl.stages[0], "crystalSpider");
    assert!(
        phases == 1 && gaps(&times) == [6.0, 6.0, 6.0] && spiders == 0,
        "Phase 2: {phases} Wechsel, Schläge alle {:?} s, {spiders} Spinnen",
        gaps(&times)
    );
    assert_eq!(outside, 0, "Flächenschlag trifft außerhalb von Radius 5");
    let e = &mut isl.stages[0].enemies[0];
    e.hp = (e.max_hp * 0.24).floor();
    let (times, phases, _) = strikes_of(&mut isl, &data, id, 17.0);
    let speed = isl.stages[0].enemies[0].speed;
    assert!(
        phases == 1 && gaps(&times) == [4.0, 4.0, 4.0] && speed == 0.0,
        "Phase 3: {phases} Wechsel, Schläge alle {:?} s, Tempo {speed}",
        gaps(&times)
    );
    assert_eq!(strikes_of(&mut isl, &data, id, 5.0).1, 0, "Phase 3 erneut gemeldet");
}
