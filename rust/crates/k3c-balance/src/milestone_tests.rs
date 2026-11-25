//! Meilenstein-Lauf coop2 (Port von `engine/balance/milestone_test.go`, BS3.6): dasselbe wie `sim_test {bots: coop2, world:
//! standard, days: 40}`. Werte-Varianten wie bei `sim_test` über die Umgebung `K3C_VARY` (Semikolon, `datei.json:a.b=wert`);
//! `K3C_SEEDS` (Komma) ersetzt den Seed `k3c-dev`. Die Tests melden nur, der erste schlägt bei ungültigen Läufen fehl.
//! Es sind lange Läufe (praktisch Balancing): `#[ignore = "lang"]`, sie laufen in keiner Session.

use k3c_sim::island::step_island;

use crate::bot_abstieg::deepest_of;
use crate::bots::{Bot, bot_by_name};
use crate::fullgame_tests::log_day;
use crate::helps::after_step;
use crate::island_world::shared;
use crate::progress::{FULLGAME_DAYS, progress_of};
use crate::run::{MAX_TICKS_PER_DAY, Matrix, Scenario, TICK_HZ, new_island, run};
use crate::variant::with_variants;

/// Der Meilenstein-Lauf; meldet Endboss-Tag, Kennzahlen und die Leiter je Seed.
#[test]
#[ignore = "lang"]
fn meilenstein_coop2() {
    let seeds: Vec<String> = match std::env::var("K3C_SEEDS") {
        Ok(s) if !s.is_empty() => s.split(',').map(String::from).collect(),
        _ => vec!["k3c-dev".into()],
    };
    let specs: Vec<String> = match std::env::var("K3C_VARY") {
        Ok(v) if !v.is_empty() => v.split(';').map(String::from).collect(),
        _ => Vec::new(),
    };
    let m = Matrix {
        seeds: seeds.clone(),
        players: vec![2],
        bots: vec!["coop2".into()],
        depths: vec![0],
        days: FULLGAME_DAYS,
        world: "standard".into(),
        ..Matrix::default()
    };
    let rep = with_variants(&specs, || run(&m)).expect("Lauf");
    for res in &rep.results {
        let Some(metrics) = res.metrics.as_ref().filter(|_| res.valid) else {
            panic!("❌ Lauf {} ungültig: {}", res.scenario.seed, res.error);
        };
        let json = serde_json::to_string(metrics).unwrap_or_default();
        println!(
            "Seed {} Endboss-Tag: {:?} | {json}",
            res.scenario.seed, metrics.endboss_day
        );
        if let Some(p) = progress_of(&rep, &res.scenario, &seeds) {
            println!("Blocker: {}", p.blocker);
            for r in &p.ladder {
                let day = r.median_day.map_or("–".to_string(), |d| d.to_string());
                println!("  {:<40} {}/{} Tag {day}", r.label, r.reached, r.runs);
            }
        }
    }
}

/// Spielt den Meilenstein-Lauf wie `run_one` und schreibt zu jedem Tagesbeginn den Stand, zusätzlich bei Gegnern mit Boss
/// alle 30 s Spielzeit die Bosse der Stufe, in der ein Monarch steht (Tag-für-Tag-Diagnose, B-645); nur mit `K3C_DIAG=1`.
#[test]
#[ignore = "lang"]
fn diagnose_coop2() {
    if std::env::var("K3C_DIAG").is_err() {
        println!("nur mit K3C_DIAG=1");
        return;
    }
    let sc = Scenario {
        seed: "k3c-dev".into(),
        players: 2,
        bot: "coop2".into(),
        days: FULLGAME_DAYS,
        world: "standard".into(),
        ..Scenario::default()
    };
    let mut isl = new_island(&sc).expect("Insel");
    let deepest = deepest_of(&isl);
    let descent = Bot::Descent {
        inner: Box::new(bot_by_name(&sc.bot).expect("Profil coop2")),
        deepest,
    };
    let bot = Bot::BossFight {
        inner: Box::new(descent),
        deepest,
    };
    let difficulty = &shared().expect("Daten").data.difficulty;
    let mut cmds = vec![k3c_sim::state::PlayerCommand::default(); sc.players as usize];
    let mut last = 0;
    for tick in 1..=sc.days * MAX_TICKS_PER_DAY {
        for w in &isl.stages {
            for p in &w.players {
                cmds[p.index as usize] = bot.decide(&isl, w, p);
            }
        }
        step_island(&mut isl, &cmds, difficulty, 1.0 / TICK_HZ as f64);
        after_step(&mut isl, &sc.world, &sc.helps, tick, TICK_HZ);
        let day = isl.stages[0].cycle.day;
        if day != last {
            last = day;
            log_day(&isl, day);
        }
        if tick % (20 * TICK_HZ) == 0 && (22..=31).contains(&day) {
            log_titan(&isl, tick);
        }
        if day > sc.days {
            break;
        }
    }
}

/// Die Lage der beiden tiefsten Stufen: Gegner (Boss mit HP), Kämpfer, Monarchen mit Gold (Go: `logTitan`).
fn log_titan(isl: &k3c_sim::island::Island, tick: i64) {
    let mut line = format!("t{}s", tick / TICK_HZ);
    for i in isl.stages.len().saturating_sub(2)..isl.stages.len() {
        let w = &isl.stages[i];
        line += &format!(
            " | S{i} Tag {} K{} Gegner {}",
            w.cycle.day,
            crate::bots::economy_view(w).fighters,
            w.enemies.len()
        );
        for en in w.enemies.iter().filter(|en| en.boss) {
            line += &format!(" [{} {}/{} x{}]", en.kind, en.hp as i64, en.max_hp as i64, en.x as i64);
        }
        for p in &w.players {
            line += &format!(" M{} G{} x{}", p.index, p.gold, p.x as i64);
        }
    }
    println!("{line}");
}
