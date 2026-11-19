//! Tests von `stressreport.rs`: Ursachen des Endes, Rundung wie Go und die Feldliste des Stressberichts.

use super::*;
use crate::game::TPS;
use crate::view::frame::Stage;
use crate::view::stress::GROWTH_LINEAR;
use k3c_sim::state::{Enemy, World};
use serde_json::Value;

/// Sekunde 0 der festen Uhr.
const T0: i64 = 1_794_000_000;

/// Frame mit einer Stufe mit `enemies` Gegnern.
fn stress_frame(tick: u64, enemies: usize) -> Frame {
    let w = World {
        enemies: vec![Enemy::default(); enemies],
        ..World::default()
    };
    Frame {
        tick,
        stages: vec![Stage {
            world: Some(std::rc::Rc::new(w)),
            ..Stage::default()
        }],
        ..Frame::default()
    }
}

fn cfg() -> StressTest {
    StressTest {
        players: 2,
        depth: 1,
        seed: 42,
        enemies: 10,
        growth: GROWTH_LINEAR.to_string(),
        factor: 5.0,
        waves: 2,
        enemy: "goblin".to_string(),
        ..StressTest::default()
    }
}

/// Die Ursachen `gewonnen`, `verloren` und `minuten` (Wanduhr) beenden den Test, vorher nicht.
#[test]
fn over_reports_won_lost_and_minutes() {
    let c = StressTest {
        minutes: 2,
        ..StressTest::default()
    };
    let r = StressRun::new(c, Zeit::from_unix(T0, 0));
    let mut f = stress_frame(5, 1);
    assert_eq!(r.over(&f, &Zeit::from_unix(T0 + 119, 0)), "");
    assert_eq!(r.over(&f, &Zeit::from_unix(T0 + 120, 0)), "minuten");
    f.outcome = Outcome::Won;
    assert_eq!(r.over(&f, &Zeit::from_unix(T0, 0)), "gewonnen");
    f.outcome = Outcome::Lost;
    assert_eq!(r.over(&f, &Zeit::from_unix(T0, 0)), "verloren");
}

/// `round2` rundet wie Go (`int64(v*100+0.5)/100`).
#[test]
fn round2_like_go() {
    assert!((round2(1.234) - 1.23).abs() < 1e-9 && (round2(1.235) - 1.24).abs() < 1e-9);
    assert!(round2(0.004).abs() < 1e-9 && (round2(2.0) - 2.0).abs() < 1e-9);
    assert_eq!(TPS, 30);
}

/// Die Namen der Schlüssel in der Reihenfolge der Datei auf der Einrückung `indent` (Leerzeichen).
fn keys_at(text: &str, indent: usize) -> Vec<String> {
    let pad = " ".repeat(indent);
    text.lines()
        .filter(|l| l.starts_with(&format!("{pad}\"")))
        .filter_map(|l| l.trim_start().strip_prefix('"')?.split('"').next().map(str::to_string))
        .collect()
}

/// Feldtest: Der Stressbericht trägt dieselben Felder in derselben Reihenfolge wie der des Go-Spiels (`StressReport` in
/// `stressreport.go`, Go-Tags 1:1); verschachtelt zählen nur die Namen.
#[test]
fn report_has_the_fields_of_the_go_report() {
    let r = StressRun::new(cfg(), Zeit::from_unix(T0, 0));
    let rep = r.report(&stress_frame(3, 1), &Zeit::from_unix(T0 + 10, 0), "wellen");
    let text = serde_json::to_string_pretty(&rep).unwrap();
    let top = "kind einstellungen start end dauerSekunden endeUrsache wellen takte takteJeSekunde taktdauerMs fpsMittel \
               fpsMin hoechstwerte amEnde goHeapMib goSysMib";
    assert_eq!(keys_at(&text, 2).join(" "), top);
    let v: Value = serde_json::from_str(&text).unwrap();
    let names = |k: &str| {
        let mut n: Vec<String> = v[k]
            .as_object()
            .map(|o| o.keys().cloned().collect())
            .unwrap_or_default();
        n.sort();
        n.join(" ")
    };
    let sorted = |s: &str| {
        let mut n: Vec<&str> = s.split(' ').collect();
        n.sort_unstable();
        n.join(" ")
    };
    assert_eq!(
        names("einstellungen"),
        sorted("id players depth seed enemies growth factor build troops timescale waves minutes enemy troop pure")
    );
    assert_eq!(names("taktdauerMs"), "max p50 p95 p99");
    assert_eq!(
        names("hoechstwerte"),
        sorted("gegner truppen geschosse muenzen wartend")
    );
    assert_eq!(names("amEnde"), names("hoechstwerte"));
    assert_eq!(rep.kind, "native-stress");
}
