//! Vollständiges Spiel mit einem Bot (Port von `engine/balance/fullgame_test.go`, BS3.4, 🧑-Wunsch): Ein einzelner Bot spielt
//! die Insel 1 Stufe für Stufe bis zum Sieg über den Endboss. Die Tests sichern, dass die Simulation durchspielbar ist,
//! unabhängig von der Balance der Münzskala.
//!
//! Jede Stufe ist ein eigener Test (`fullgame_1…`), jede baut auf der vorigen auf. Es sind lange Läufe (praktisch Balancing):
//! `#[ignore = "lang"]`, sie laufen in keiner Session und nicht in `check:diff`.
//!
//! Dev-Hilfen (Umgebung `K3C_FULLGAME`, wie in Go): „hilfen“ (Dev-Baukosten, voller Insel-Vorrat, Grad dev), „voll“ (zusätzlich
//! alle Stufen voll ausgebaut, Dev-Bau Stufe 5; Standard), „boss“ (alles davor, schwacher Endboss und geheilter Monarch) oder
//! „ohne“ (Grad normal, nichts geschenkt: der offene Meilenstein der Balance, Fehler gelten dort nur als Hinweis im Log).

use k3c_sim::island::{Island, step_island};

use crate::bot_abstieg::deepest_of;
use crate::bots::{Bot, bot_by_name, economy_view};
use crate::helps::after_step;
use crate::island_world::shared;
use crate::progress::FULLGAME_DAYS;
use crate::run::{MAX_TICKS_PER_DAY, Scenario, TICK_HZ, new_island};

/// Spielt einen Bot, bis `done` wahr ist oder `FULLGAME_DAYS` vorbei sind; liefert Insel, Tag und ob Hilfen galten.
fn fullgame(done: impl Fn(&Island) -> bool) -> (Island, i64, bool) {
    let mode = std::env::var("K3C_FULLGAME")
        .ok()
        .filter(|m| !m.is_empty())
        .unwrap_or_else(|| "voll".into());
    let helps = mode != "ohne";
    let sc = Scenario {
        seed: "k3c-dev".into(),
        players: 1,
        days: FULLGAME_DAYS,
        world: "standard".into(),
        helps: if helps { mode } else { String::new() },
        ..Scenario::default()
    };
    let mut isl = new_island(&sc).expect("Insel");
    let deepest = deepest_of(&isl);
    let descent = Bot::Descent {
        inner: Box::new(bot_by_name("wirtschaft").expect("Profil wirtschaft")),
        deepest,
    };
    let bot = Bot::BossFight {
        inner: Box::new(descent),
        deepest,
    };
    let difficulty = &shared().expect("Daten").data.difficulty;
    let mut cmds = vec![k3c_sim::state::PlayerCommand::default(); 1];
    let mut last = 0;
    for tick in 1..=FULLGAME_DAYS * MAX_TICKS_PER_DAY {
        if done(&isl) {
            break;
        }
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
        if day > FULLGAME_DAYS {
            break;
        }
    }
    let day = isl.stages[0].cycle.day;
    (isl, day, helps)
}

/// Meldet ein Ergebnis: mit Hilfen ein Fehler, ohne nur ein Hinweis im Log.
fn verdict(ok: bool, helps: bool, msg: &str) {
    if ok {
        println!("✅ {msg}");
    } else if helps {
        panic!("❌ {msg}");
    } else {
        println!("⚪ {msg} (ohne Hilfen: offener Meilenstein, kein Fehler)");
    }
}

fn boss_down(isl: &Island, id: &str) -> bool {
    isl.defeated_bosses.iter().any(|b| b == id)
}

/// Stufe 1: Der Miniboss des Waldes (Goblin-Anführer) fällt.
#[test]
#[ignore = "lang"]
fn fullgame_1_miniboss_wald() {
    let (isl, day, helps) = fullgame(|i| boss_down(i, "goblinLeader"));
    verdict(
        boss_down(&isl, "goblinLeader"),
        helps,
        &format!("Goblin-Anführer besiegt, Tag {day}"),
    );
}

/// Stufe 2: Der Spieler erreicht die tiefste Stufe (Kristallhöhle).
#[test]
#[ignore = "lang"]
fn fullgame_2_tiefste_stufe() {
    let at_bottom = |i: &Island| i.stage_of(0) == Some(i.stages.len() - 1);
    let (isl, day, helps) = fullgame(at_bottom);
    verdict(at_bottom(&isl), helps, &format!("tiefste Stufe erreicht, Tag {day}"));
}

/// Stufe 3: Alle vier Minibosse der Insel fallen.
#[test]
#[ignore = "lang"]
fn fullgame_3_alle_minibosse() {
    let all = |i: &Island| {
        ["goblinLeader", "trollKing", "ratQueen", "lavaGolem"]
            .iter()
            .all(|id| boss_down(i, id))
    };
    let (isl, day, helps) = fullgame(all);
    verdict(
        all(&isl),
        helps,
        &format!(
            "alle Minibosse besiegt, Tag {day} (Bosse: {})",
            isl.defeated_bosses.join(", ")
        ),
    );
}

/// Stufe 4: Der Endboss der Insel 1 fällt (komplettes Spiel).
#[test]
#[ignore = "lang"]
fn fullgame_4_endboss() {
    let (isl, day, helps) = fullgame(|i| i.endboss_defeated);
    verdict(
        isl.endboss_defeated,
        helps,
        &format!("Endboss besiegt, Tag {day} (Bosse: {})", isl.defeated_bosses.join(", ")),
    );
}

/// Schreibt zu Tagesbeginn, wo der Spieler steht und wie die Stufen aussehen (Go: `logFullgameDay`).
pub(crate) fn log_day(isl: &Island, day: i64) {
    let mut line = format!(
        "Tag {day}: Spieler auf Stufe {}",
        isl.stage_of(0).map_or(-1, |s| s as i64)
    );
    for (i, w) in isl.stages.iter().enumerate() {
        let e = economy_view(w);
        line += &format!(" | S{i} Hub {} Kämpfer {}", e.hub_level, e.fighters);
        if let Some(a) = w.aggression {
            line += &format!(" (Pool {} Welle {} Gegner {})", a as i64, w.wave, w.enemies.len());
        }
        for en in w.enemies.iter().filter(|en| en.boss && en.hp > 0.0) {
            line += &format!(" [{} {}/{} x{}]", en.kind, en.hp as i64, en.max_hp as i64, en.x as i64);
        }
    }
    for w in &isl.stages {
        for p in &w.players {
            line += &format!(" | Monarch Gold {} x{} (Hub x{})", p.gold, p.x as i64, w.hub_x as i64);
        }
    }
    println!("{line} | Bosse {}", isl.defeated_bosses.join(", "));
}
