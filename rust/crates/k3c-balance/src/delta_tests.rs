//! Tests zu `delta.rs` (Port von `engine/balance/delta_test.go`, B-640/AC-03). Die Go-Tests gehen über `BuildInsights`
//! (RU31.1); hier rechnet `build_delta` zuerst mit eigenen Zielen über den Trait `DeltaTarget`, gleiche Fälle: Die Ziele sind je
//! zur Hälfte Gruppe basis und holz, im Lauf kippt ein erfülltes Ziel der Gruppe basis auf verletzt. Danach die Tests über
//! `build_insights` wie in Go (Hinweise der Hebel nur für freie Gruppen; die Hinweise selbst liefert RU31.4).

use k3c_data::balance_hints::GROUPS;

use crate::delta::{
    DeltaTarget, GroupDelta, TREND_SAME, TREND_WORSE, TrendFacts, build_delta, free_levers, worse_light,
};
use crate::evaluate::{STATUS_OK, STATUS_VIOLATED};
use crate::groups::{GROUP_BASIS, GROUP_HOLZ, GROUP_SCHIFF};
use crate::hints::Lever;
use crate::insights::{Insights, LIGHT_GREEN, LIGHT_GREY, LIGHT_RED, LIGHT_YELLOW, Meta, build_insights};
use crate::insights_tests::{fixture, test_hints};
use crate::summary::Summary;
use crate::targets::KIND_SHARE;

struct T {
    name: &'static str,
    group: &'static str,
    light: &'static str,
    status: &'static str,
    trend: Option<(f64, bool, &'static str)>,
}

impl DeltaTarget for T {
    fn name(&self) -> &str {
        self.name
    }
    fn group(&self) -> &str {
        self.group
    }
    fn light(&self) -> &str {
        self.light
    }
    fn status(&self) -> &str {
        self.status
    }
    fn trend(&self) -> Option<TrendFacts<'_>> {
        self.trend.map(|(delta, better, before_status)| TrendFacts {
            delta,
            better,
            before_status,
        })
    }
}

/// Drei Ziele basis und drei holz; das erste Ziel basis kippt von ok auf verletzt, alle anderen bleiben gleich.
fn targets() -> Vec<T> {
    let same = |name, group| T {
        name,
        group,
        light: LIGHT_GREEN,
        status: STATUS_OK,
        trend: Some((0.0, false, STATUS_OK)),
    };
    vec![
        T {
            name: "gekippt",
            group: GROUP_BASIS,
            light: LIGHT_RED,
            status: STATUS_VIOLATED,
            trend: Some((-40.0, false, STATUS_OK)),
        },
        same("b2", GROUP_BASIS),
        same("b3", GROUP_BASIS),
        same("h1", GROUP_HOLZ),
        same("h2", GROUP_HOLZ),
        same("h3", GROUP_HOLZ),
    ]
}

fn group<'a>(d: &'a [crate::delta::GroupDelta], g: &str) -> Option<&'a crate::delta::GroupDelta> {
    d.iter().find(|x| x.group == g)
}

#[test]
fn delta_freie_gruppen_immer_mit_trend() {
    let d = build_delta(&targets(), &[]);
    let (basis, holz) = (
        group(&d, GROUP_BASIS).expect("basis"),
        group(&d, GROUP_HOLZ).expect("holz"),
    );
    assert!(
        basis.light == LIGHT_RED
            && basis.trend == TREND_WORSE
            && basis.tipped == ["gekippt"]
            && basis.warning.is_empty()
            && (basis.worse, basis.better, basis.targets) == (1, 0, 3),
        "basis: {basis:?}"
    );
    assert!(
        holz.trend == TREND_SAME && holz.tipped.is_empty() && holz.light == LIGHT_GREEN,
        "holz unverändert: {holz:?}"
    );
    assert!(
        group(&d, GROUP_SCHIFF).is_none(),
        "schiff hat keine Ziele und fehlt im Delta"
    );
}

#[test]
fn delta_eingefroren_nur_beim_kippen() {
    let frozen = vec![GROUP_BASIS.to_string(), GROUP_HOLZ.to_string()];
    let d = build_delta(&targets(), &frozen);
    assert_eq!(d.len(), 1, "nur die gekippte eingefrorene Gruppe erscheint: {d:?}");
    let g = &d[0];
    assert!(
        g.group == GROUP_BASIS
            && g.frozen
            && g.warning.contains("eingefrorene Gruppe basis kippt")
            && g.warning.contains("gekippt"),
        "Warnung: {g:?}"
    );
    let unchanged: Vec<T> = targets()
        .into_iter()
        .map(|mut t| {
            t.status = STATUS_OK;
            t.light = LIGHT_GREEN;
            t
        })
        .collect();
    assert!(
        build_delta(&unchanged, &frozen).is_empty(),
        "unveränderter Lauf: eingefrorene Gruppen schweigen"
    );
}

#[test]
fn delta_ohne_ziele_leer_und_in_reihenfolge_der_gruppen() {
    assert!(build_delta::<T>(&[], &[]).is_empty());
    let d = build_delta(&targets(), &[]);
    let order: Vec<&str> = d.iter().map(|g| g.group.as_str()).collect();
    let want: Vec<&str> = GROUPS.iter().copied().filter(|g| order.contains(g)).collect();
    assert_eq!(order, want, "Reihenfolge der Gruppen");
}

#[test]
fn schlechteste_ampel() {
    assert_eq!(worse_light(LIGHT_GREEN, LIGHT_GREY), LIGHT_GREY);
    assert_eq!(worse_light(LIGHT_GREY, LIGHT_YELLOW), LIGHT_YELLOW);
    assert_eq!(worse_light(LIGHT_RED, LIGHT_YELLOW), LIGHT_RED);
    assert_eq!(worse_light(LIGHT_YELLOW, LIGHT_GREEN), LIGHT_YELLOW);
}

/// Hebel eingefrorener Gruppen fallen weg (Go: `TestDeltaHinweiseNurFuerFreieGruppen`, der Teil `freeLevers`).
#[test]
fn hebel_nur_fuer_freie_gruppen() {
    let lever = |path: &str, group: &str| Lever {
        path: path.into(),
        group: group.into(),
        ..Lever::default()
    };
    let all = [lever("a", GROUP_BASIS), lever("b", GROUP_HOLZ)];
    let free = free_levers(&all, &[GROUP_BASIS.to_string()]);
    assert_eq!(free.len(), 1);
    assert_eq!(free[0].path, "b");
    assert_eq!(free_levers(&all, &[]).len(), 2);
}

/// Baseline und Lauf auf Basis von run7 (Go: `deltaPair`): erste Hälfte der Ziele Gruppe basis, zweite holz; im Lauf kippt ein
/// bisher erfülltes Ziel der Gruppe basis auf verletzt. Zurück: Baseline, Lauf, Name des gekippten Ziels.
fn delta_pair() -> (Summary, Summary, String) {
    let mut base = fixture();
    let half = base.targets.len() / 2;
    for (i, e) in base.targets.iter_mut().enumerate() {
        e.verdict.target.group = if i >= half { GROUP_HOLZ } else { GROUP_BASIS }.into();
    }
    let mut cur = base.clone();
    for e in cur.targets.iter_mut().take(half) {
        let t = &e.verdict.target;
        if e.verdict.status == STATUS_OK && t.kind == KIND_SHARE && t.lower.is_some() {
            e.verdict.value = t.lower.unwrap_or_default() - 40.0;
            e.verdict.status = STATUS_VIOLATED.into();
            e.pass = false;
            let name = e.verdict.target.name.clone();
            return (base, cur, name);
        }
    }
    panic!("kein erfülltes Ziel mit Untergrenze in der ersten Hälfte der Fixture");
}

fn group_of<'a>(inn: &'a Insights, group: &str) -> Option<&'a GroupDelta> {
    inn.delta.iter().find(|d| d.group == group)
}

/// B-640/AC-03 (Go: `TestDeltaFreieGruppenImmerMitTrend`).
#[test]
fn insights_delta_freie_gruppen_immer_mit_trend() {
    let (base, cur, name) = delta_pair();
    let m = Meta {
        run: "r".into(),
        baseline: "M1".into(),
        ..Meta::default()
    };
    let inn = build_insights(&cur, Some(&base), &test_hints(), &m);
    let (basis, holz) = (group_of(&inn, GROUP_BASIS), group_of(&inn, GROUP_HOLZ));
    let (Some(basis), Some(holz)) = (basis, holz) else {
        panic!("freie Gruppen fehlen: {:?}", inn.delta);
    };
    assert!(
        basis.light == LIGHT_RED
            && basis.trend == TREND_WORSE
            && basis.tipped.contains(&name)
            && basis.warning.is_empty()
    );
    assert!(
        holz.trend == TREND_SAME && holz.tipped.is_empty(),
        "holz unverändert: {holz:?}"
    );
    assert!(
        group_of(&inn, GROUP_SCHIFF).is_none(),
        "schiff hat keine Ziele in der Fixture"
    );
}

/// B-640/AC-03 (Go: `TestDeltaEingefrorenNurBeimKippen`).
#[test]
fn insights_delta_eingefroren_nur_beim_kippen() {
    let (base, cur, name) = delta_pair();
    let m = Meta {
        run: "r".into(),
        baseline: "M1".into(),
        frozen: vec![GROUP_BASIS.into(), GROUP_HOLZ.into()],
        ..Meta::default()
    };
    let inn = build_insights(&cur, Some(&base), &test_hints(), &m);
    assert_eq!(
        inn.delta.len(),
        1,
        "nur die gekippte eingefrorene Gruppe erscheint: {:?}",
        inn.delta
    );
    let d = &inn.delta[0];
    assert!(d.group == GROUP_BASIS && d.frozen, "{d:?}");
    assert!(
        d.warning.contains("eingefrorene Gruppe basis kippt") && d.warning.contains(&name),
        "Warnung: {d:?}"
    );
    let same = build_insights(&base, Some(&base), &test_hints(), &m);
    assert!(
        same.delta.is_empty(),
        "unveränderter Lauf: eingefrorene Gruppen schweigen: {:?}",
        same.delta
    );
}

/// B-640/AC-03 (Go: `TestDeltaHinweiseNurFuerFreieGruppen`): Die Hebel der Ziele; die Hinweise (`hints`) liefert RU31.4.
#[test]
fn delta_hebel_nur_fuer_freie_gruppen() {
    let count = |inn: &Insights| -> usize {
        inn.targets
            .iter()
            .flat_map(|t| &t.levers)
            .filter(|l| l.group == GROUP_BASIS)
            .count()
    };
    let cur = fixture();
    let free = build_insights(&cur, None, &test_hints(), &Meta::default());
    let m = Meta {
        frozen: vec![GROUP_BASIS.into()],
        ..Meta::default()
    };
    let frozen = build_insights(&cur, None, &test_hints(), &m);
    assert!(
        count(&free) > 0,
        "ohne Einfrieren nennt die Fixture Hebel der Gruppe basis"
    );
    assert_eq!(count(&frozen), 0, "eingefrorene Gruppe basis nennt noch Hebel");
}

/// B-640/AC-03 (Go: `TestDeltaOhneBaselineLeer`): ohne Baseline kein Delta, aber nie `null`.
#[test]
fn delta_ohne_baseline_leer() {
    let inn = build_insights(&fixture(), None, &test_hints(), &Meta::default());
    assert!(inn.delta.is_empty());
    assert!(serde_json::to_string(&inn).expect("JSON").contains(r#""delta":[]"#));
}
