//! Tests zu `endboss.rs` (Port von `engine/sim/boss_endboss_test.go`, K2.1c, B-130/AC-02, AC-03): Bau, Warten,
//! Auslösung, Phasen, Skalierung, Belohnung, keine Rückkehr, Determinismus. Die Go-Tests laufen über `CreateIsland` und
//! `Step`; sie tragen `ignore` (Insel RU10, Takt RU14, Ereignisse RU19). Die Fälle ohne Insel stehen in `real_tests`.

use super::data::end_boss;
use super::endboss::{EndbossFight, boss_ability_of, park_endboss, phase_tempo, step_endboss};
use super::testutil::{boss_stage, boss_values, count_kind};
use crate::enemies::testutil::{count_events, end_island, step_for};
use crate::state::island_ref::IslandRef;
use crate::state::{Enemy, World};

const WARDEN_ID: &str = "crystalHeartWarden";

/// Go: `warden`: der Endboss der Kristallhöhle in der Stufe (Index).
fn warden(w: &World) -> Option<usize> {
    w.enemies.iter().position(|e| e.kind == WARDEN_ID)
}

// (a) Ohne Spieler am Bau wartet der Endboss, auch nach langer Zeit: kein Gegner, nichts kann ihn treffen. Ein
// Spieler genau auf triggerUnits, ein toter und ein getrennter Spieler am Bau lösen ihn nicht aus.
#[test]
#[ignore = "wartet auf RU15 (end_island)"]
fn endboss_wartet_ohne_zeitdruck() {
    let (mut isl, _x) = end_island(3);
    let w = &mut isl.stages[0];
    for _ in 0..20 {
        step_for(w, 60.0);
        assert!(
            warden(w).is_none() && count_events(w, "bossSpawned") == 0,
            "Endboss ohne Ankunft aktiv"
        );
    }
}

// (b) Ein Spieler kommt an: Der Boss erscheint einmal am Bau, läuft nicht und ist verwundbar; mit zwei Spielern an
// verschiedenen Orten genügt einer.
#[test]
#[ignore = "wartet auf RU15 (end_island)"]
fn endboss_wird_bei_ankunft_ausgeloest() {
    let (mut isl, x) = end_island(2);
    let w = &mut isl.stages[0];
    step_for(w, 10.0);
    let e = warden(w).expect("Endboss");
    assert!(w.enemies[e].x == x && w.enemies[e].home_x == x, "einmal am Bau");
    assert_eq!(count_events(w, "bossSpawned"), 1);
}

// (c) Phasen: Phase 1 ruft Kristallspinnen, Phase 2 (unter 60 %) schlägt im Radius 5 alle 6 s und beschwört nicht
// mehr, Phase 3 (unter 25 %) schlägt mit +50 % Tempo alle 4 s; jeder Wechsel wird genau einmal gemeldet.
#[test]
#[ignore = "wartet auf RU15 (end_island)"]
fn endboss_phasen() {
    let (mut isl, _x) = end_island(2);
    let w = &mut isl.stages[0];
    step_for(w, 10.5);
    assert!(count_kind(w, "crystalSpider") >= 2, "Phase 1: Kristallspinnen");
    assert_eq!(count_events(w, "bossPhase"), 0);
    // Phase 2 und 3: Schläge im Takt 6 s bzw. 4 s, je ein `bossPhase`: braucht Spieler und Takt (RU18: `step_players`).
    panic!("RU18: strikesOf, protect (Step: step_players)");
}

// (d) HP bei 3 Spielern = 2 × HP bei 1 Spieler (1 + 0,5 × 2); später beitretende Spieler ändern sie nicht.
#[test]
#[ignore = "wartet auf RU15 (end_island)"]
fn endboss_hp_skaliert_mit_spielern() {
    let (mut isl, _x) = end_island(3);
    step_for(&mut isl.stages[0], 1.0);
    let e = warden(&isl.stages[0]).expect("Endboss");
    assert_eq!(isl.stages[0].enemies[e].hp, isl.stages[0].enemies[e].max_hp);
}

// (e) Belohnung: 500 Gold als Münzen am Bau, 250 Kristall in den Insel-Vorrat, 3 Skill-Punkte in den Pool der Insel
// (für alle Spieler); Merker gesetzt.
#[test]
#[ignore = "wartet auf RU15 (end_island)"]
fn endboss_belohnung() {
    let (mut isl, _x) = end_island(2);
    let w = &mut isl.stages[0];
    step_for(w, 1.0);
    let e = warden(w).expect("Endboss");
    w.enemies[e].hp = 0.0;
    step_for(w, 1.0);
    assert_eq!(count_events(w, "bossDefeated"), 1);
}

// (f) Ein aktiver Endboss übersteht einen Burgfall; nach seinem Tod kehrt er nie zurück, auch nicht nach einem
// Burgfall mit einem Spieler am Bau.
#[test]
#[ignore = "wartet auf RU15 (end_island)"]
fn endboss_kehrt_nie_zurueck() {
    let (mut isl, _x) = end_island(1);
    let w = &mut isl.stages[0];
    step_for(w, 1.0);
    let e = warden(w).expect("Endboss");
    w.enemies[e].hp -= 1.0;
    w.castle.hp = 0.0;
    step_for(w, 0.1);
    assert!(warden(w).is_some(), "aktiver Endboss nach dem Burgfall verschwunden");
}

// (g) Gleicher Seed, gleicher Verlauf: zwei Spieler am Bau, alle drei Phasen, Sieg.
#[test]
#[ignore = "wartet auf RU15 (end_island)"]
fn endboss_deterministisch() {
    let trace = || {
        let (mut isl, _x) = end_island(2);
        step_for(&mut isl.stages[0], 120.0);
        format!("{} {}", isl.stages[0].enemies.len(), isl.stages[0].coins.len())
    };
    assert_eq!(trace(), trace(), "Endboss-Verlauf nicht deterministisch");
}

// Daten: der Endboss der Kristallhöhle (Tiefe 4, dazu je einer für Tiefe 0–3, B-981) mit Werten laut bosse.md § 1;
// kaputte Phasen oder Baue werden beim Laden abgelehnt.
#[test]
fn endboss_daten() {
    let d = k3c_data::load_all(&k3c_data::Source::Embedded).expect("eingebettete Daten");
    let b = end_boss(&d.bosses, 4).expect("Endboss der Tiefe 4");
    assert!(
        d.bosses.end_bosses.len() == 5
            && b.id == WARDEN_ID
            && b.hp_factor == 30.0
            && b.phases.len() == 3
            && b.phases[2].below_hp_percent == 25.0
            && b.phases[2].tempo == 1.5
            && b.reward.gold == 42
            && b.reward.material == 250,
        "Endbosse {:?}",
        d.bosses.end_bosses
    );
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../data/bosses.json");
    let raw = std::fs::read_to_string(path).expect("data/bosses.json");
    let depths: Vec<i64> = d.biomes.iter().map(|b| b.depth).collect();
    let cases = [
        (r#""triggerUnits": 15"#, r#""triggerUnits": 0"#),
        (r#""belowHpPercent": 25"#, r#""belowHpPercent": 70"#),
        (r#""tempo": 1.5"#, r#""tempo": -1"#),
        (r#""kind": "end""#, r#""kind": "mini""#),
        (
            r#"{ "ability": { "type": "summon", "enemy": "crystalSpider""#,
            r#"{ "ability": { "type": "summon", "enemy": "nope""#,
        ),
    ];
    for (bad, repl) in cases {
        let broken = raw.replacen(bad, repl, 1);
        assert_ne!(broken, raw, "Testdaten: {bad} nicht gefunden");
        assert!(
            k3c_data::bosses::load(&broken, &d.enemies, &depths).is_err(),
            "{repl}: ungültige Daten geladen"
        );
    }
}

mod real_tests {
    //! Fälle ohne Insel und Takt: Tempo der Phasen, aktuelle Fähigkeit, Burgfall (Parken), Auslösung ohne Bau.

    use super::*;

    #[test]
    fn phase_tempo_nimmt_das_der_phase_davor() {
        let v = boss_values();
        let b = end_boss(&v.bosses, 0).expect("Endboss Wald");
        assert_eq!(phase_tempo(b, 0), 1.0, "keine Angabe: normal");
        assert_eq!(phase_tempo(b, 1), 1.0);
        assert_eq!(phase_tempo(b, 2), 1.5, "Phase 3 beschleunigt");
        assert_eq!(phase_tempo(b, 9), 1.5, "über das Ende: die letzte Angabe");
    }

    #[test]
    fn boss_ability_of_endboss_folgt_der_phase() {
        let v = boss_values();
        let b = end_boss(&v.bosses, 0).expect("Endboss Wald");
        let mut isl = IslandRef::default();
        assert_eq!(
            boss_ability_of(b, Some(&isl)).r#type,
            b.ability.r#type,
            "ohne Kampf: die eigene"
        );
        isl.endboss = Some(EndbossFight::default());
        assert_eq!(boss_ability_of(b, Some(&isl)).r#type, "summon", "Phase 1");
        isl.endboss.as_mut().expect("Kampf").phase = 1;
        assert_eq!(boss_ability_of(b, Some(&isl)).r#type, "aoe", "Phase 2");
        isl.endboss.as_mut().expect("Kampf").phase = 2;
        assert_eq!(
            boss_ability_of(b, Some(&isl)).r#type,
            "aoe",
            "Phase 3 ohne eigene: die davor"
        );
        let mini = super::super::data::mini_boss(&v.bosses, 0).expect("Miniboss");
        assert_eq!(
            boss_ability_of(mini, Some(&isl)).r#type,
            "summon",
            "Miniboss: die einzige"
        );
    }

    #[test]
    fn step_endboss_ohne_insel_oder_bau_tut_nichts() {
        let (mut w, mut isl) = boss_stage(0, 1);
        let v = boss_values();
        step_endboss(&mut w, &v, None, 0.1);
        step_endboss(&mut w, &v, Some(&mut isl), 0.1); // kein `lair` im Level
        assert!(w.enemies.is_empty() && isl.endboss.is_none());
    }

    #[test]
    fn aktiver_endboss_uebersteht_den_burgfall() {
        let (mut w, mut isl) = boss_stage(0, 1);
        let v = boss_values();
        let id = w.new_id();
        w.enemies.push(Enemy {
            id,
            kind: "forestChieftain".into(),
            hp: 100.0,
            max_hp: 100.0,
            boss: true,
            ..Enemy::default()
        });
        isl.endboss = Some(EndbossFight {
            boss_id: id,
            ..EndbossFight::default()
        });
        park_endboss(&mut w, Some(&mut isl)); // Burgfall räumt die Gegner ab
        assert!(w.enemies.is_empty());
        step_endboss(&mut w, &v, Some(&mut isl), 0.1);
        assert_eq!(w.enemies.len(), 1, "der Endboss ist zurück");
        assert_eq!(w.enemies[0].id, id);
        assert!(isl.endboss.as_ref().is_some_and(|f| f.parked.is_none()));
    }

    #[test]
    fn endboss_einer_anderen_stufe_bleibt_liegen() {
        let (mut w, mut isl) = boss_stage(0, 1);
        let v = boss_values();
        isl.stage = 1;
        isl.endboss = Some(EndbossFight {
            boss_id: 7,
            home: 0,
            parked: Some(Enemy {
                id: 7,
                ..Enemy::default()
            }),
            ..EndbossFight::default()
        });
        step_endboss(&mut w, &v, Some(&mut isl), 0.1);
        assert!(w.enemies.is_empty(), "Go: `f.home != w` kehrt zurück");
        park_endboss(&mut w, Some(&mut isl));
    }

    #[test]
    fn mini_boss_zaehlt_nicht_als_warden() {
        let (w, _) = boss_stage(0, 1);
        assert!(warden(&w).is_none());
        assert_eq!(count_kind(&w, WARDEN_ID), 0);
    }
}
