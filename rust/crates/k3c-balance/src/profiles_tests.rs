//! Tests zu den Profilen (Port von `engine/balance/profiles_test.go`, BAL3, BAL6, B-300). Tests, die Kennzahlen lesen,
//! warten auf RU28.1; die langen (Go `-short` überspringt sie) sind `ignore = "lang"`.

use k3c_data::balance_targets::Target;
use k3c_sim::dev::dev_add_stock;
use k3c_sim::island::step_island;
use k3c_sim::state::PlayerCommand;

use crate::bots::bot_by_name;
use crate::island_world::shared;
use crate::run::{Matrix, Scenario, TICK_HZ, new_island, run};
use crate::scope::validate_target;

fn matrix(bot: &str, players: i64) -> Matrix {
    Matrix {
        seeds: vec!["1".into()],
        players: vec![players],
        bots: vec![bot.into()],
        depths: vec![0],
        days: 1,
        ..Matrix::default()
    }
}

/// BAL3/AC-01: Jedes neue Profil liefert mit gleichem Seed und gleichen Daten byte-gleiche, gültige Kennzahlen.
#[test]
#[ignore = "lang"]
fn neue_profile_gleiche_bytes() {
    for (bot, players) in [
        ("walls", 2),
        ("economy", 2),
        ("wirtschaft", 2),
        ("coop2", 2),
        ("coop4", 4),
    ] {
        let m = matrix(bot, players);
        let r = run(&m).expect("Lauf");
        assert!(r.results[0].valid, "{bot} ungültig: {}", r.results[0].error);
        let a = r.json().expect("JSON");
        let b = run(&m).expect("Lauf").json().expect("JSON");
        assert_eq!(a, b, "{bot}: zwei Läufe mit gleichem Seed liefern verschiedenes JSON");
    }
}

/// Mauern zuerst baut an Tag 1 eine Mauer; Wirtschaft zuerst wirbt an Tag 1 an und baut keine Mauer.
#[test]
#[ignore = "wartet auf RU28"]
fn mauern_und_wirtschaft_zuerst() {
    // Liest `Metrics.first_wall_tick`, `days[0].dusk_tick` und `days[0].expense` (RU28.1).
}

/// B-158 › Ausnahmefälle: Ein Profil mit mehr Spielern als das Szenario ist ein Fehler beim Start.
#[test]
fn profil_zu_wenige_spieler() {
    for (bot, players) in [("coop2", 1), ("coop4", 2), ("coop4", 3)] {
        let err = run(&matrix(bot, players)).expect_err("Fehler beim Start");
        assert!(
            err.to_string().contains("braucht"),
            "{bot} mit {players} Spielern: {err}"
        );
        let g = Target {
            measure: "castleHeld".into(),
            name: "x".into(),
            rule: "x".into(),
            kind: "share".into(),
            players,
            bot: bot.into(),
            days: 1,
            lower: Some(0.0),
            ..Target::default()
        };
        let err = validate_target(&g).expect_err("Ziel mit zu wenigen Spielern");
        assert!(
            err.to_string().contains("braucht"),
            "Ziel {bot} mit {players} Spielern: {err}"
        );
    }
}

/// BAL6/AC-02 (B-300/AC-01): „Mauern zuerst“ spielt messbar anders als „sparsam“: Es baut an Tag 1 einen Turm, „sparsam“ nie.
#[test]
#[ignore = "wartet auf RU28"]
fn mauern_zuerst_anders_als_sparsam() {
    // Liest `Metrics.first_tower_tick` (RU28.1).
}

/// B-300/AC-01 (Beschluss BAL6.3, autonom): „Mauern zuerst“ zahlt den Hub-Ausbau und baut danach eine Mauer auf Stufe 2 aus.
/// Holz und Stein für Mauern, Türme, Hub-Stufe 2 und den Mauer-Ausbau liegen im Vorrat (Dev-Vorrat), kein Kupfer (B-631/AC-03:
/// das Material-Tor von Hub-Stufe 2 ist Stein).
///
/// Der Go-Test ist selbst rot (`go test -run TestMauernZuerstBautMauerAus`: „Hub-Stufe 2, Mauer auf Stufe 2 ausgebaut: false
/// (nach 10800 Ticks)“, vermutlich seit der Standard-Welt mit fünf Inseln, WL2/B-1215); Rust meldet dasselbe. Go gilt:
/// angehalten, bis der Go-Test grün ist.
#[test]
#[ignore = "wartet auf RU28"]
fn mauern_zuerst_baut_mauer_aus() {
    let sc = Scenario {
        seed: "1".into(),
        players: 2,
        days: 1,
        ..Scenario::default()
    };
    let mut isl = new_island(&sc).expect("Insel");
    let difficulty = &shared().expect("Daten").data.difficulty;
    for r in ["wood", "stone"] {
        assert!(dev_add_stock(&mut isl, difficulty, 0, r, 200).1, "Vorrat {r}");
    }
    for w in &mut isl.stages {
        for p in &mut w.players {
            p.gold = 30; // der Test prüft den Bot, nicht die Wirtschaft: genug Gold für Ausbau und Mauer-Ausbau
        }
    }
    let bot = bot_by_name("walls").expect("Profil walls");
    let mut cmds = vec![PlayerCommand::default(); 2];
    let (mut upgraded, mut tick) = (false, 0);
    while tick < 10 * 60 * TICK_HZ && isl.stages[0].cycle.phase == "day" && !upgraded {
        let w = &isl.stages[0];
        for p in &w.players {
            cmds[p.index as usize] = bot.decide(&isl, w, p);
        }
        step_island(&mut isl, &cmds, difficulty, 1.0 / TICK_HZ as f64);
        upgraded = isl.stages[0].events.iter().any(|ev| {
            ev.get("type").and_then(|v| v.as_str()) == Some("upgraded")
                && ev.get("kind").and_then(|v| v.as_str()) == Some("wall")
                && ev.get("level").and_then(|v| v.as_f64()) == Some(2.0)
        });
        tick += 1;
    }
    let w = &isl.stages[0];
    assert!(
        w.hub_level >= 2 && upgraded,
        "Hub-Stufe {}, Mauer auf Stufe 2 ausgebaut: {upgraded} (nach {tick} Ticks)",
        w.hub_level
    );
}
