//! Go `delta_test.go` (B-640/AC-03). Go prüft über `BuildInsights`; die Auswertung folgt erst mit `insights`, darum
//! prüfen diese Tests [`build_delta`] und [`free_levers`] auf denselben Daten (Ziele der Fixture, Vergleich wie
//! `targetInsight`). „Ohne Baseline kein Delta“ (`TestDeltaOhneBaselineLeer`) liegt in `BuildInsights` und folgt dort.

use super::super::evaluate::{STATUS_NARROW, STATUS_OK, STATUS_VIOLATED};
use super::super::groups::{GROUP_BASIS, GROUP_HOLZ, GROUP_SCHIFF};
use super::super::hints::{Lever, default_hints};
use super::super::insights::{LIGHT_GREEN, LIGHT_GREY, LIGHT_RED, LIGHT_YELLOW, TargetInsight, Trend};
use super::super::summary::{Entry, Summary};
use super::super::targets::KIND_SHARE;
use super::super::test_helpers::{fixture, repo_root};
use super::{GroupDelta, TREND_BETTER, TREND_SAME, TREND_WORSE, build_delta, free_levers};

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

fn light_of(status: &str) -> &'static str {
    match status {
        STATUS_OK => LIGHT_GREEN,
        STATUS_NARROW => LIGHT_YELLOW,
        STATUS_VIOLATED => LIGHT_RED,
        _ => LIGHT_GREY,
    }
}

/// Die Felder, die das Delta liest; ein geänderter Wert gilt hier als schlechter (so baut ihn [`delta_pair`]).
fn insight(e: &Entry, before: &Entry) -> TargetInsight {
    TargetInsight {
        name: e.verdict.target.name.clone(),
        group: e.verdict.target.group.clone(),
        status: e.verdict.status.clone(),
        light: light_of(&e.verdict.status).to_owned(),
        trend: Some(Trend {
            before: before.verdict.value,
            delta: e.verdict.value - before.verdict.value,
            better: e.verdict.value == before.verdict.value,
            before_status: before.verdict.status.clone(),
        }),
        ..TargetInsight::default()
    }
}

fn insights(cur: &Summary, base: &Summary) -> Vec<TargetInsight> {
    cur.targets
        .iter()
        .zip(&base.targets)
        .map(|(e, b)| insight(e, b))
        .collect()
}

fn group_of<'a>(delta: &'a [GroupDelta], group: &str) -> Option<&'a GroupDelta> {
    delta.iter().find(|d| d.group == group)
}

fn frozen_groups() -> Vec<String> {
    vec![GROUP_BASIS.to_owned(), GROUP_HOLZ.to_owned()]
}

#[test]
fn freie_gruppen_immer_mit_trend() {
    let (base, cur, name) = delta_pair();
    let delta = build_delta(&insights(&cur, &base), &[]);
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
    let delta = build_delta(&insights(&cur, &base), &frozen_groups());
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
    let same = build_delta(&insights(&base, &base), &frozen_groups());
    assert!(
        same.is_empty(),
        "unveränderter Lauf: eingefrorene Gruppen schweigen: {same:?}"
    );
}

#[test]
fn hebel_nur_fuer_freie_gruppen() {
    let hints = default_hints(&repo_root()).expect("Richtwerte");
    let levers: Vec<Lever> = fixture()
        .targets
        .iter()
        .filter(|e| e.verdict.status == STATUS_VIOLATED || e.verdict.status == STATUS_NARROW)
        .flat_map(|e| hints.levers_for(e))
        .collect();
    let basis = |list: &[Lever]| list.iter().filter(|l| l.group == GROUP_BASIS).count();
    assert!(
        basis(&free_levers(&levers, &[])) > 0,
        "ohne Einfrieren nennt die Fixture Hebel der Gruppe basis"
    );
    let rest = free_levers(&levers, &[GROUP_BASIS.to_owned()]);
    assert_eq!(basis(&rest), 0, "eingefrorene Gruppe basis nennt noch Hebel");
    assert_eq!(rest.len(), levers.len() - basis(&levers), "andere Gruppen bleiben");
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
