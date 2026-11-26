//! Go `progress_test.go`.

use std::fs;

use super::super::datafiles;
use super::super::hints::default_hints;
use super::super::insights::{Meta, build_insights};
use super::super::insights_progress::{find_previous, mark_arrows, write_result};
use super::super::names::load_world;
use super::super::report_types::{
    BossMetrics, DawnMetrics, Matrix, Metrics, Report, RunResult, Scenario, StageMetrics,
};
use super::super::summary::{Summary, read_summary};
use super::super::targets::{ROLE_GOAL, ROLE_GUARD, Targets, default_targets, load_targets};
use super::super::test_helpers::{metrics_with, repo_root, results, seeds, share_target, temp_root};
use super::{GoalView, blocker, load_bosses, milestones, progress_of, test_match};
use crate::balance::evaluate::{STATUS_NARROW, STATUS_OK};

/// B-648/AC-01: Das Urteil hängt nur an den Zielen mit Rolle goal; verfehlte guard-Korridore ändern es nie.
#[test]
fn urteil_haengt_nur_an_goal() {
    let mut goal = share_target();
    goal.role = ROLE_GOAL.to_owned();
    let mut guard = share_target();
    guard.name = "g".to_owned();
    guard.role = ROLE_GUARD.to_owned();
    (guard.lower, guard.upper) = (Some(95.0), Some(100.0));
    let mut ts = Targets {
        seeds: seeds(4),
        targets: vec![goal, guard],
        ..Targets::default()
    };
    ts.margin.share_pp = 5.0;
    // Burg hält in 3 von 4 Seeds (75 %): das goal liegt im Korridor 70–90 %, der guard (95–100 %) nicht.
    let ms: Vec<Option<Metrics>> = [false, false, false, true]
        .map(|fallen| Some(metrics_with(fallen, 1, &[])))
        .into();
    let report = Report {
        matrix: Matrix {
            seeds: ts.seeds.clone(),
            ..Matrix::default()
        },
        results: results(&ms),
    };
    let root = repo_root();
    let s = ts.summarize(&root, &report);
    let want = GoalView {
        reached: true,
        hit: 1,
        total: 1,
    };
    assert!(s.pass && s.goal == Some(want), "pass={} goal={:?}", s.pass, s.goal);
    assert!(
        s.quality.is_some_and(|q| q.passed == 0 && q.total == 1),
        "{:?}",
        s.quality
    );
    // Go ruft `s.Markdown(nil)`; bis RU70.17 liefert `Summary::insights` keine Auswertung, darum hier von Hand.
    let hints = default_hints(&root).expect("Richtwerte");
    let insights = build_insights(&root, &s, None, &hints, &Meta::default());
    let md = s.markdown_with(None, Some(&insights));
    assert!(
        md.contains("**Ziel erreicht**") && md.contains("Qualität: 0 von 1 Korridoren"),
        "Markdown nennt die zwei Urteile nicht:\n{md}"
    );
    (ts.targets[0].lower, ts.targets[0].upper) = (Some(95.0), Some(100.0));
    let s = ts.summarize(&root, &report);
    assert!(
        !s.pass && s.goal.is_some_and(|g| !g.reached),
        "verfehltes goal muss scheitern"
    );
}

/// B-648 (Knapp-Regel): „knapp“ gibt es nur bei Zielen, ein guard zeigt ok oder verletzt.
#[test]
fn knapp_nur_bei_zielen() {
    for (role, want) in [(ROLE_GOAL, STATUS_NARROW), (ROLE_GUARD, STATUS_OK)] {
        let mut g = share_target();
        g.role = role.to_owned();
        // 14 von 20 Seeds = 70 %: genau an der Untergrenze, im Rand von 5 pp
        let ms: Vec<Option<Metrics>> = (0..20).map(|i| Some(metrics_with(i >= 14, 1, &[]))).collect();
        let mut ts = Targets {
            seeds: seeds(20),
            targets: vec![g],
            ..Targets::default()
        };
        ts.margin.share_pp = 5.0;
        let report = Report {
            matrix: Matrix {
                seeds: ts.seeds.clone(),
                ..Matrix::default()
            },
            results: results(&ms),
        };
        let s = ts.summarize(&repo_root(), &report);
        assert_eq!(s.targets[0].verdict.status, want, "Rolle {role}");
    }
}

/// B-648/AC-02: Die Datei der Ziele nennt je Ziel eine Rolle; ohne Rolle oder mit unbekannter ist sie ein Prüffehler.
#[test]
fn rollen_in_der_datei() {
    let root = repo_root();
    default_targets(&root).expect("data/balance-targets.json");
    let raw = datafiles::read(&root, "balance-targets.json").expect("lesen");
    let mut ts = load_targets(&root, &raw).expect("laden");
    ts.targets[0].role = String::new();
    assert!(ts.check_roles().is_err(), "fehlende Rolle muss ein Fehler sein");
    ts.targets[0].role = "boss".to_owned();
    assert!(ts.check_roles().is_err(), "unbekannte Rolle muss ein Fehler sein");
}

/// B-648/AC-03: Die Leiter deckt Stufen, alle Minibosse und den Endboss der ersten Insel der Welt ab.
#[test]
fn leiter_deckt_alle_bosse_ab() {
    let root = repo_root();
    let have: Vec<String> = milestones(&root, "standard")
        .expect("Leiter")
        .into_iter()
        .map(|d| d.id)
        .collect();
    let world = load_world(&root, "standard").expect("Welt");
    let file = load_bosses(&root).expect("bosses.json");
    let depths = &world.islands[0].depths;
    let mut want: Vec<String> = (0..depths.len()).map(|i| format!("stage:{i}")).collect();
    want.extend(
        file.bosses
            .iter()
            .filter(|b| depths.contains(&b.depth))
            .map(|b| format!("boss:{}", b.id)),
    );
    want.extend(
        file.end_bosses
            .iter()
            .filter(|b| depths.contains(&b.depth))
            .map(|b| format!("endboss:{}", b.id)),
    );
    for id in want {
        assert!(have.contains(&id), "{id} fehlt in der Leiter");
    }
}

fn island_result(seed: &str, stage_days: &[i64], boss_days: &[(&str, i64)], end_day: Option<i64>) -> RunResult {
    let mut m = Metrics::default();
    m.island.stages = stage_days
        .iter()
        .map(|d| StageMetrics {
            arrival_day: Some(*d),
            ..StageMetrics::default()
        })
        .collect();
    let len = m.island.stages.len().max(5);
    m.island.stages.resize(len, StageMetrics::default());
    m.island.bosses = boss_days
        .iter()
        .map(|(boss, day)| BossMetrics {
            boss: (*boss).to_owned(),
            day: *day,
            tick: 0,
        })
        .collect();
    m.island.endboss_day = end_day;
    m.dawns = vec![DawnMetrics {
        day: 40,
        gold: vec![2],
        hub_level: 1,
        fighters: 0,
        ..DawnMetrics::default()
    }];
    RunResult {
        scenario: Scenario {
            seed: seed.to_owned(),
            ..scenario()
        },
        valid: true,
        metrics: Some(m),
        ..RunResult::default()
    }
}

fn scenario() -> Scenario {
    Scenario {
        players: 1,
        bot: "wirtschaft".to_owned(),
        days: 40,
        world: "standard".to_owned(),
        helps: "voll".to_owned(),
        ..Scenario::default()
    }
}

fn dev_seed() -> Vec<String> {
    vec!["k3c-dev".to_owned()]
}

/// B-648/AC-03, AC-05, AC-06: Leiter mit Quote und Median je Meilenstein, Blocker-Satz, Entsprechung zum Stufen-Test.
#[test]
fn fortschritt_leiter_und_blocker() {
    let mut sc = scenario();
    let report = Report {
        results: vec![
            island_result("k3c-dev", &[1, 6], &[("goblinLeader", 5)], None),
            island_result("1", &[1, 8], &[("goblinLeader", 7)], None),
        ],
        ..Report::default()
    };
    let p = progress_of(&repo_root(), &report, &sc, &dev_seed()).expect("kein Fortschritt");
    let goblin = p.ladder.iter().find(|r| r.id == "boss:goblinLeader").expect("Goblin");
    assert!(
        goblin.reached == 2 && goblin.runs == 2 && goblin.median_day == Some(5),
        "Goblin-Anführer: {goblin:?}"
    );
    assert!(
        p.blocker.contains("Stecken geblieben nach") && p.blocker.contains("Hub 1, 0 Kämpfer, Gold 2"),
        "Blocker = {}",
        p.blocker
    );
    assert!(
        p.test_match == "weicht vom Stufen-Test ab: Seeds nicht nur k3c-dev"
            || p.test_match == "entspricht dem Stufen-Test voll",
        "TestMatch = {}",
        p.test_match
    );
    sc.players = 2;
    let got = test_match(&sc, &dev_seed());
    assert!(got.contains("weicht vom Stufen-Test ab: 2 Spieler"), "{got}");
    sc.players = 1;
    assert_eq!(test_match(&sc, &dev_seed()), "entspricht dem Stufen-Test voll");
}

/// B-648/AC-01: Ein Lauf mit besiegtem Endboss hat keinen Blocker.
#[test]
fn kein_blocker_bei_sieg() {
    let defs = milestones(&repo_root(), "standard").expect("Leiter");
    let all: Vec<(&str, i64)> = defs
        .iter()
        .filter_map(|d| d.id.strip_prefix("boss:"))
        .map(|id| (id, 3))
        .collect();
    let res = island_result("k3c-dev", &[1, 4, 7, 10, 12], &all, Some(19));
    let m = res.metrics.as_ref().expect("Kennzahlen");
    assert_eq!(blocker(&[(&res, m)], &defs), "", "Blocker bei Sieg");
}

/// B-648/AC-05: Pfeile gegen den letzten Lauf gleicher Konfiguration; `result.json` hat die Felder für Agenten.
#[test]
fn vergleich_mit_letztem_lauf_und_result_json() {
    let (_tmp, root) = temp_root();
    let data = repo_root();
    // der Wächter erlaubt Laufausgaben nur unter reports/ (WZ63.7)
    let dir = root.join("reports").join("r");
    fs::create_dir_all(&dir).expect("Ordner");
    let make = |stem: &str, day: i64| {
        let res = island_result("k3c-dev", &[1, 6], &[("goblinLeader", day)], None);
        let report = Report {
            results: vec![res],
            ..Report::default()
        };
        let s = Summary {
            version: 1,
            seeds: 1,
            goal: Some(GoalView {
                total: 1,
                ..GoalView::default()
            }),
            progress: progress_of(&data, &report, &scenario(), &dev_seed()),
            ..Summary::default()
        };
        fs::write(dir.join(format!("{stem}.json")), s.json().expect("JSON")).expect("schreiben");
        s
    };
    make("balance-20261001-100000", 9);
    let mut cur = make("balance-20261002-100000", 5);
    let stem = dir.join("balance-20261002-100000");
    let (prev, name) = find_previous(&stem, &cur).expect("früherer Lauf");
    assert_eq!(name, "r/balance-20261001-100000");
    mark_arrows(cur.progress.as_mut(), prev.progress.as_ref());
    let ladder = &cur.progress.as_ref().expect("Fortschritt").ladder;
    let goblin = ladder.iter().find(|r| r.id == "boss:goblinLeader").expect("Goblin");
    assert_eq!(goblin.arrow, "▲", "Tag 5 statt 9");
    // wie ein von Go oder Rust geschriebener Bericht wieder lesbar
    assert!(read_summary(cur.json().expect("JSON").as_bytes()).is_ok());
    let hints = default_hints(&data).expect("Richtwerte");
    let meta = Meta {
        run: "r".to_owned(),
        ..Meta::default()
    };
    let mut insights = build_insights(&data, &cur, None, &hints, &meta);
    insights.previous = name;
    write_result(&root, &stem, &insights).expect("result.json");
    let raw = fs::read_to_string(dir.join("result.json")).expect("lesen");
    for key in [
        "\"goalReached\"",
        "\"milestones\"",
        "\"blocker\"",
        "\"previous\"",
        "\"testMatch\"",
    ] {
        assert!(raw.contains(key), "result.json ohne {key}:\n{raw}");
    }
}
