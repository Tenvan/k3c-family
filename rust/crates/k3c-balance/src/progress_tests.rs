//! Tests zu `progress.rs` (Port von `engine/balance/progress_test.go`, B-648). Nur der Test zum Vergleich mit dem
//! letzten Lauf (`findPrevious`, `BuildInsights`) wartet auf RU31.

use std::collections::BTreeMap;

use k3c_data::balance_targets::{FILE, Target, Targets};

use crate::evaluate::{STATUS_NARROW, STATUS_OK};
use crate::evaluate_tests::{metrics_with, report, seeds, share_target, targets};
use crate::island_world::{load_world, shared};
use crate::metrics::{DawnMetrics, Metrics};
use crate::metrics_island::{BossMetrics, StageMetrics};
use crate::progress::{QualityView, blocker, median_day, milestone_defs, progress_of, test_match};
use crate::run::{Report, RunResult, Scenario};
use crate::summary::Summarize;
use crate::targets::{default_targets, load_targets};

/// B-648/AC-01: Das Urteil hängt nur an den Zielen mit Rolle goal; verfehlte guard-Korridore ändern es nie.
#[test]
fn urteil_haengt_nur_an_goal() {
    let goal = Target {
        role: "goal".into(),
        ..share_target()
    };
    let guard = Target {
        name: "g".into(),
        role: "guard".into(),
        lower: Some(95.0),
        upper: Some(100.0),
        ..share_target()
    };
    let mut ts = Targets {
        seeds: seeds(4),
        targets: vec![goal, guard],
        ..Targets::default()
    };
    ts.margin.share_pp = 5.0;
    // Burg hält in 3 von 4 Seeds (75 %): das goal liegt im Korridor 70–90 %, der guard (95–100 %) nicht.
    let ms = [false, false, false, true].map(|f| Some(metrics_with(f, 1, &[])));
    let mut rep = report(ms.to_vec());
    rep.matrix.seeds = ts.seeds.clone();
    let s = ts.summarize(&rep);
    let g = s.goal.as_ref().expect("Urteil über die Ziele");
    assert!(
        s.pass && g.reached && g.hit == 1 && g.total == 1,
        "Ziel erreicht erwartet: {s:?}"
    );
    assert_eq!(s.quality, Some(QualityView { passed: 0, total: 1 }), "Qualität 0 von 1");
    let md = s.markdown_with(None, None);
    assert!(
        md.contains("**Ziel erreicht**"),
        "Markdown nennt das Urteil nicht:\n{md}"
    );
    // Die Zeile „Qualität: 0 von 1 Korridoren“ schreibt die Auswertung (`Insights`, RU31).
    ts.targets[0].lower = Some(95.0);
    ts.targets[0].upper = Some(100.0);
    let s = ts.summarize(&rep);
    assert!(
        !s.pass && !s.goal.expect("Urteil").reached,
        "verfehltes goal muss den Lauf scheitern lassen"
    );
}

/// B-648 (Knapp-Regel): „knapp“ gibt es nur bei Zielen, ein guard zeigt ok oder verletzt.
#[test]
fn knapp_nur_bei_zielen() {
    for (role, want) in [("goal", STATUS_NARROW), ("guard", STATUS_OK)] {
        let g = Target {
            role: role.into(),
            ..share_target()
        };
        // 14 von 20 Seeds = 70 %: genau an der Untergrenze, im Rand von 5 pp
        let ms = (0..20).map(|i| Some(metrics_with(i >= 14, 1, &[]))).collect();
        let mut ts = targets(20, g);
        ts.margin.share_pp = 5.0;
        let mut rep = report(ms);
        rep.matrix.seeds = ts.seeds.clone();
        let got = &ts.summarize(&rep).targets[0].verdict.status;
        assert_eq!(got, want, "Rolle {role}");
    }
}

/// B-648/AC-02: Die Datei der Ziele nennt je Ziel eine Rolle; ohne Rolle oder mit unbekannter ist sie ein Prüffehler.
#[test]
fn rollen_in_der_datei() {
    default_targets().expect("data/balance-targets.json");
    let raw = shared().expect("Daten").source.read(FILE).expect("Datei der Ziele");
    let ts = load_targets(&raw).expect("Ziele");
    let role = format!("\"role\": \"{}\"", ts.targets[0].role);
    assert!(raw.contains(&role), "Rolle des ersten Ziels nicht in der Datei: {role}");
    // Die Rolle des ersten Ziels fehlt bzw. ist unbekannt: ein Ladefehler (Go: `checkRoles`).
    assert!(
        load_targets(&raw.replacen(&role, "\"role\": \"\"", 1)).is_err(),
        "fehlende Rolle muss ein Fehler sein"
    );
    assert!(
        load_targets(&raw.replacen(&role, "\"role\": \"boss\"", 1)).is_err(),
        "unbekannte Rolle muss ein Fehler sein"
    );
}

/// B-648/AC-03: Die Leiter deckt Stufen, alle Minibosse und den Endboss der ersten Insel der Welt ab.
#[test]
fn leiter_deckt_alle_bosse_ab() {
    let defs = milestone_defs("standard").expect("Leiter");
    let have = |id: &str| defs.iter().any(|d| d.id == id);
    let world = load_world("standard").expect("Welt");
    let isl = &world.islands[0];
    let bosses = &shared().expect("Daten").data.bosses;
    for i in 0..isl.depths.len() {
        assert!(have(&format!("stage:{i}")), "Stufe {i} fehlt in der Leiter");
    }
    for b in &bosses.bosses {
        if isl.depths.contains(&b.depth) {
            assert!(have(&format!("boss:{}", b.id)), "Miniboss {} fehlt in der Leiter", b.id);
        }
    }
    // Nur der Endboss der Insel steht in der Leiter (B-981): das Feld `endBoss`, sonst der der tiefsten Stufe.
    let deepest = isl.depths.iter().max().copied();
    let mut ends = 0;
    for b in &bosses.end_bosses {
        let want = b.id == isl.end_boss || isl.end_boss.is_empty() && Some(b.depth) == deepest;
        let got = have(&format!("endboss:{}", b.id));
        assert_eq!(got, want, "Endboss {}: in der Leiter {got}, erwartet {want}", b.id);
        ends += usize::from(want);
    }
    assert_eq!(ends, 1, "Endbosse der Insel in der Leiter, erwartet 1");
}

fn island_result(seed: &str, stage_days: &[i64], boss_day: &BTreeMap<&str, i64>, end_day: Option<i64>) -> RunResult {
    let mut m = Metrics::default();
    for &d in stage_days {
        m.stages.push(StageMetrics {
            arrival_day: Some(d),
            ..StageMetrics::default()
        });
    }
    while m.stages.len() < 5 {
        m.stages.push(StageMetrics::default());
    }
    for (id, &d) in boss_day {
        m.bosses.push(BossMetrics {
            boss: (*id).to_string(),
            day: d,
            ..BossMetrics::default()
        });
    }
    m.endboss_day = end_day;
    m.dawns = vec![DawnMetrics {
        day: 40,
        gold: vec![2],
        hub_level: 1,
        fighters: 0,
        ..DawnMetrics::default()
    }];
    RunResult {
        scenario: Scenario {
            seed: seed.into(),
            players: 1,
            bot: "wirtschaft".into(),
            days: 40,
            world: "standard".into(),
            helps: "voll".into(),
            ..Scenario::default()
        },
        valid: true,
        error: String::new(),
        ticks: 0,
        metrics: Some(m),
        replay: None,
    }
}

fn goblin(day: i64) -> BTreeMap<&'static str, i64> {
    BTreeMap::from([("goblinLeader", day)])
}

/// B-648/AC-03, AC-05, AC-06: Leiter mit Quote und Median je Meilenstein, Blocker-Satz, Entsprechung zum Stufen-Test.
#[test]
fn fortschritt_leiter_und_blocker() {
    let mut sc = Scenario {
        players: 1,
        bot: "wirtschaft".into(),
        days: 40,
        world: "standard".into(),
        helps: "voll".into(),
        ..Scenario::default()
    };
    let r = Report {
        matrix: Default::default(),
        results: vec![
            island_result("k3c-dev", &[1, 6], &goblin(5), None),
            island_result("1", &[1, 8], &goblin(7), None),
        ],
    };
    let seeds = vec!["k3c-dev".to_string()];
    let p = progress_of(&r, &sc, &seeds).expect("kein Fortschritt");
    let row = p
        .ladder
        .iter()
        .find(|row| row.id == "boss:goblinLeader")
        .expect("Goblin-Anführer");
    assert!(
        row.reached == 2 && row.runs == 2 && row.median_day == Some(5),
        "Goblin-Anführer: {row:?}"
    );
    assert!(
        p.blocker.contains("Stecken geblieben nach") && p.blocker.contains("Hub 1, 0 Kämpfer, Gold 2"),
        "Blocker = {:?}",
        p.blocker
    );
    assert!(
        p.test_match == "weicht vom Stufen-Test ab: Seeds nicht nur k3c-dev"
            || p.test_match == "entspricht dem Stufen-Test voll",
        "TestMatch = {:?}",
        p.test_match
    );
    sc.players = 2;
    let got = test_match(&sc, &seeds);
    assert!(
        got.contains("weicht vom Stufen-Test ab: 2 Spieler"),
        "Abweichung = {got:?}"
    );
    sc.players = 1;
    assert_eq!(test_match(&sc, &seeds), "entspricht dem Stufen-Test voll");
}

/// B-648/AC-01: Ein Lauf mit besiegtem Endboss hat keinen Blocker.
#[test]
fn kein_blocker_bei_sieg() {
    let defs = milestone_defs("standard").expect("Leiter");
    let all: BTreeMap<&str, i64> = defs
        .iter()
        .filter_map(|d| d.id.strip_prefix("boss:"))
        .map(|id| (id, 3))
        .collect();
    let res = island_result("k3c-dev", &[1, 4, 7, 10, 12], &all, Some(19));
    let b = blocker(&[&res], &defs);
    assert!(b.is_empty(), "Blocker bei Sieg: {b:?}");
}

/// B-648/AC-05: Pfeile gegen den letzten Lauf gleicher Konfiguration; result.json hat die Felder für Agenten.
#[test]
#[ignore = "wartet auf RU31"]
fn vergleich_mit_letztem_lauf_und_result_json() {
    // Go: `findPrevious`, `markArrows`, `BuildInsights`, `writeResult` (insights.go, RU31); `Summary::json` liefert RU28.3.
}

/// Unterer Median: bei gerader Anzahl der kleinere der beiden mittleren Tage; ohne Tage keiner.
#[test]
fn median_ist_der_untere() {
    assert_eq!(median_day(&[]), None);
    assert_eq!(median_day(&[7]), Some(7));
    assert_eq!(median_day(&[9, 5]), Some(5));
    assert_eq!(median_day(&[9, 1, 5]), Some(5));
    assert_eq!(median_day(&[8, 2, 6, 4]), Some(4));
}
