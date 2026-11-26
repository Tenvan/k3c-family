//! Go `delta_test.go` (B-640/AC-03): Delta-Bericht je Werte-Gruppe, eingefrorene nur beim Kippen, Hinweise nur für
//! freie Gruppen.

use super::super::evaluate::{STATUS_NARROW, STATUS_OK, STATUS_VIOLATED};
use super::super::groups::{GROUP_BASIS, GROUP_HOLZ, GROUP_SCHIFF};
use super::super::hints::default_hints;
use super::super::insights::{Insights, LIGHT_GREEN, LIGHT_RED, Meta, TargetInsight, Trend, build_insights};
use super::super::summary::Summary;
use super::super::targets::KIND_SHARE;
use super::super::test_helpers::{fixture, repo_root};
use super::{GroupDelta, TREND_BETTER, TREND_SAME, TREND_WORSE, build_delta};

/// Baseline und Lauf auf Basis der Fixture: erste Hälfte der Ziele Gruppe basis, zweite holz; im Lauf kippt ein bisher
/// erfülltes Ziel der Gruppe basis auf verletzt. Zurück: Baseline, Lauf, Name des gekippten Ziels.
fn delta_pair() -> (Summary, Summary, String) {
    let mut base = fixture();
    let half = base.targets.len() / 2;
    for (i, e) in base.targets.iter_mut().enumerate() {
        let group = if i < half { GROUP_BASIS } else { GROUP_HOLZ };
        e.verdict.target.group = group.to_owned();
    }
    let mut cur = base.clone();
    let tipped = cur.targets.iter_mut().take(half).find_map(|e| {
        let t = &e.verdict.target;
        let lower = t
            .lower
            .filter(|_| e.verdict.status == STATUS_OK && t.kind == KIND_SHARE)?;
        e.verdict.value = lower - 40.0;
        e.verdict.status = STATUS_VIOLATED.to_owned();
        e.pass = false;
        Some(e.verdict.target.name.clone())
    });
    let name = tipped.expect("kein erfülltes Ziel mit Untergrenze in der ersten Hälfte der Fixture");
    (base, cur, name)
}

fn insights(cur: &Summary, base: Option<&Summary>, frozen: &[&str]) -> Insights {
    let root = repo_root();
    let hints = default_hints(&root).expect("Richtwerte");
    let meta = Meta {
        run: "r".to_owned(),
        baseline: "M1".to_owned(),
        frozen: frozen.iter().map(|g| (*g).to_owned()).collect(),
        ..Meta::default()
    };
    build_insights(&root, cur, base, &hints, &meta)
}

fn group_of<'a>(delta: &'a [GroupDelta], group: &str) -> Option<&'a GroupDelta> {
    delta.iter().find(|d| d.group == group)
}

#[test]
fn freie_gruppen_immer_mit_trend() {
    let (base, cur, name) = delta_pair();
    let delta = insights(&cur, Some(&base), &[]).delta;
    let basis = group_of(&delta, GROUP_BASIS).expect("basis fehlt");
    let holz = group_of(&delta, GROUP_HOLZ).expect("holz fehlt");
    assert_eq!(
        (basis.light.as_str(), basis.trend.as_str()),
        (LIGHT_RED, TREND_WORSE),
        "{basis:?}"
    );
    assert!(basis.tipped.contains(&name) && basis.warning.is_empty(), "{basis:?}");
    assert_eq!(holz.trend, TREND_SAME, "holz unverändert: {holz:?}");
    assert!(holz.tipped.is_empty(), "{holz:?}");
    assert!(
        group_of(&delta, GROUP_SCHIFF).is_none(),
        "schiff hat keine Ziele in der Fixture und fehlt im Delta"
    );
}

#[test]
fn eingefroren_nur_beim_kippen() {
    let (base, cur, name) = delta_pair();
    let frozen = [GROUP_BASIS, GROUP_HOLZ];
    let delta = insights(&cur, Some(&base), &frozen).delta;
    assert_eq!(
        delta.len(),
        1,
        "nur die gekippte eingefrorene Gruppe erscheint: {delta:?}"
    );
    let d = &delta[0];
    assert!(d.group == GROUP_BASIS && d.frozen, "{d:?}");
    assert!(
        d.warning.contains("eingefrorene Gruppe basis kippt") && d.warning.contains(&name),
        "{d:?}"
    );
    let same = insights(&base, Some(&base), &frozen).delta;
    assert!(
        same.is_empty(),
        "unveränderter Lauf: eingefrorene Gruppen schweigen: {same:?}"
    );
}

#[test]
fn hebel_nur_fuer_freie_gruppen() {
    let cur = fixture();
    let free = insights(&cur, None, &[]);
    let frozen = insights(&cur, None, &[GROUP_BASIS]);
    let count = |i: &Insights| {
        i.targets
            .iter()
            .flat_map(|t| &t.levers)
            .filter(|l| l.group == GROUP_BASIS)
            .count()
    };
    assert!(
        count(&free) > 0,
        "ohne Einfrieren nennt die Fixture Hebel der Gruppe basis"
    );
    assert_eq!(count(&frozen), 0, "eingefrorene Gruppe basis nennt noch Hebel");
    for h in &frozen.hints {
        assert!(
            h.levers.iter().all(|l| l.group != GROUP_BASIS),
            "Hinweis {} nennt einen Hebel einer eingefrorenen Gruppe",
            h.target
        );
    }
}

#[test]
fn ohne_baseline_leer() {
    let got = insights(&fixture(), None, &[]);
    assert!(got.delta.is_empty(), "ohne Baseline kein Delta: {:?}", got.delta);
    let json = serde_json::to_string(&got).expect("JSON");
    assert!(json.contains(r#""delta":[]"#), "nie null");
}

/// Trend nach Mehrheit; ohne Vergleich oder bei Gleichstand „gleich“; ohne Baseline-Werte kippt nichts.
#[test]
fn trend_nach_mehrheit() {
    let target = |trend: Option<(f64, bool)>| TargetInsight {
        group: GROUP_BASIS.to_owned(),
        status: STATUS_OK.to_owned(),
        light: LIGHT_GREEN.to_owned(),
        trend: trend.map(|(delta, better)| Trend {
            delta,
            better,
            before_status: STATUS_NARROW.to_owned(),
            ..Trend::default()
        }),
        ..TargetInsight::default()
    };
    let trend_of = |list: &[TargetInsight]| {
        let d = build_delta(list, &[]).remove(0);
        assert!(d.tipped.is_empty(), "ok und knapp zählen beide als erfüllt: {d:?}");
        (d.trend, d.better, d.worse)
    };
    let (better, worse, flat) = (Some((1.0, true)), Some((-1.0, false)), Some((0.0, true)));
    assert_eq!(
        trend_of(&[target(better), target(flat)]),
        (TREND_BETTER.to_owned(), 1, 0)
    );
    assert_eq!(
        trend_of(&[target(better), target(worse)]),
        (TREND_SAME.to_owned(), 1, 1)
    );
    assert_eq!(trend_of(&[target(None)]), (TREND_SAME.to_owned(), 0, 0));
}
