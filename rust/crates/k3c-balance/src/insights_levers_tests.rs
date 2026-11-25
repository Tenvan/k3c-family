//! Tests zu `insights_levers.rs` mit von Hand gebauten Zielen (die Pläne liefert erst RU31.3).

use crate::hints::Lever;
use crate::insights::{Insights, LIGHT_RED, TargetInsight};
use crate::insights_levers::{lever_groups, recommendations};
use crate::insights_next::WHERE_CHAT;
use crate::insights_plain::LeverPlan;

fn rot(name: &str, distance: f64, levers: &[(&str, &str)]) -> TargetInsight {
    TargetInsight {
        name: name.into(),
        grade: "normal".into(),
        light: LIGHT_RED.into(),
        distance: Some(distance),
        levers: levers
            .iter()
            .map(|(p, d)| Lever {
                path: (*p).into(),
                direction: (*d).into(),
                ..Lever::default()
            })
            .collect(),
        plans: levers
            .iter()
            .map(|(p, _)| LeverPlan {
                label: format!("Wert {p}"),
                known: *p != "b.json:y",
                start: "5".into(),
                current: "4".into(),
                min: "3".into(),
                max: "9".into(),
                short: "kurz".into(),
                ..LeverPlan::default()
            })
            .collect(),
        ..TargetInsight::default()
    }
}

/// Derselbe Wert bei mehreren Verletzungen steht einmal, die meisten Verletzungen zuerst; Gegenrichtung warnt; ohne
/// seriösen Startwert und in Gegenrichtung entfällt der Wert aus dem Empfehlungsblock (Go: `leverGroups`, `recommendations`).
#[test]
fn hebel_gruppen_und_empfehlungen() {
    let inn = Insights {
        targets: vec![
            rot("A", -50.0, &[("a.json:x", "+"), ("b.json:y", "-")]),
            rot("B", -90.0, &[("a.json:x", "+")]),
            rot("C", -10.0, &[("a.json:x", "-")]),
        ],
        ..Insights::default()
    };
    let g = lever_groups(&inn);
    assert_eq!(g.len(), 3, "{g:?}");
    assert_eq!((g[0].path.as_str(), g[0].direction.as_str()), ("a.json:x", "+"));
    assert_eq!(g[0].targets, ["B", "A"], "am weitesten daneben zuerst");
    assert!(g[0].conflict.starts_with("Achtung"), "{:?}", g[0]);
    assert_eq!(
        g[0].text,
        format!(
            "(erhöhen) Wert a.json:x `a.json:x` – soll 2 Verletzung(en) verbessern: B; A. kurz {}",
            g[0].conflict
        )
    );
    let c = recommendations(&g).expect("Empfehlungen");
    assert_eq!(c.r#where, WHERE_CHAT);
    for want in [
        "a.json:x=5  # Wert a.json:x; aktuell 4, Bereich 3 … 9; verbessert: B; A",
        "# a.json:x: Gegenrichtung",
        "# b.json:y: Startwert nicht seriös",
    ] {
        assert!(c.text.contains(want), "{want:?} fehlt in {}", c.text);
    }
    assert!(recommendations(&[]).is_none());
}
