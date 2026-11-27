//! Go `insights_test.go` (B-632). Fixture: echter Bericht simtest-run-7 (13 Ziele, 4 verletzt, 2 knapp, mit
//! Kennzahlen).

use super::evaluate::{STATUS_INVALID, STATUS_OK};
use super::hints::{Lever, default_hints};
use super::insights::{
    Insights, LIGHT_GREEN, LIGHT_GREY, LIGHT_RED, LIGHT_YELLOW, Meta, TargetInsight, build_insights,
};
use super::insights_next::{WHERE_CHAT, WHERE_SHELL, WHERE_WORKBENCH};
use super::summary::Summary;
use super::test_helpers::{fixture, repo_root};
use super::tuneparam::parse_tune_param;

fn build(s: &Summary, base: Option<&Summary>, meta: &Meta) -> Insights {
    let root = repo_root();
    let hints = default_hints(&root).expect("Richtwerte");
    build_insights(&root, s, base, &hints, meta)
}

fn run7() -> Insights {
    let meta = Meta {
        run: "simtest-run-7".to_owned(),
        ..Meta::default()
    };
    build(&fixture(), None, &meta)
}

fn find<'a>(i: &'a Insights, measure: &str) -> &'a TargetInsight {
    i.targets
        .iter()
        .find(|t| t.measure == measure && t.grade == "normal")
        .expect("Ziel fehlt")
}

/// AC-01: Ampel je Ziel mit Abstand in % der Korridorbreite und Hebeln in die nötige Richtung.
#[test]
fn ampeln() {
    let i = run7();
    let counts = [LIGHT_GREEN, LIGHT_YELLOW, LIGHT_RED, LIGHT_GREY].map(|l| i.counts[l]);
    assert_eq!(counts, [7, 2, 4, 0]);
    let hub = find(&i, "hubLevel2BeforeDay5");
    assert!(
        hub.light == LIGHT_RED && hub.distance == Some(-240.0) && hub.levers[0].direction == "-",
        "Hub-Stufe 2: {hub:?}"
    );
    let castle = find(&i, "castleHeld");
    assert!(
        castle.light == LIGHT_YELLOW && castle.distance == Some(33.3),
        "Burg: {castle:?}"
    );
}

/// AC-01: Kurzauswertung, Stärken, Schwächen und Tuning-Hinweise, verletzte Ziele zuerst.
#[test]
fn texte() {
    let i = run7();
    assert!(
        (3..=6).contains(&i.summary.len()) && i.summary[0].contains("4 von 13 Zielen verletzt"),
        "Kurzauswertung: {:?}",
        i.summary
    );
    assert!(
        !i.strengths.is_empty() && i.weaknesses.first().is_some_and(|w| w.starts_with("Hub-Stufe 2")),
        "Stärken {:?}, Schwächen {:?}",
        i.strengths,
        i.weaknesses
    );
    assert!(
        i.hints.len() == 6 && i.hints[0].light == LIGHT_RED && i.hints[0].text.contains("Zu niedrig"),
        "Hinweise: {:?}",
        i.hints
    );
}

/// AC-01: Ampel je Kennzahl und Vorschlag für den nächsten Lauf mit Ziel je Befehl.
#[test]
fn kennzahlen_und_naechster_lauf() {
    let i = run7();
    let first = &i.figures.lights[0];
    assert!(
        first.key == "firstWallTick" && first.light == LIGHT_GREEN,
        "Kennzahl-Ampeln: {:?}",
        i.figures.lights
    );
    assert_eq!(i.next.seeds, ["1", "2", "3"]);
    assert!(i.next.commands.len() >= 4, "nächster Lauf: {:?}", i.next);
    for c in &i.next.commands {
        assert!(
            [WHERE_CHAT, WHERE_SHELL, WHERE_WORKBENCH].contains(&c.place.as_str()),
            "Befehl ohne Ziel: {c:?}"
        );
    }
}

/// AC-01: Trend zur Baseline und Vergleich; ohne Baseline kein Trend.
#[test]
fn trend_zur_baseline() {
    let cur = fixture();
    let mut base = fixture();
    for e in &mut base.targets {
        if e.verdict.target.measure == "goldAtDawnPerPlayer" {
            e.verdict.value = 22.0;
            e.verdict.status = STATUS_OK.to_owned();
            e.pass = true;
        }
    }
    let meta = Meta {
        baseline: "b.json".to_owned(),
        ..Meta::default()
    };
    let i = build(&cur, Some(&base), &meta);
    let gold = find(&i, "goldAtDawnPerPlayer");
    let trend = gold.trend.as_ref().expect("Trend");
    assert!(
        trend.delta == -5.0 && !trend.better && trend.before_status == STATUS_OK,
        "{trend:?}"
    );
    assert!(
        i.baseline.present && i.baseline.changes.len() == 1 && i.baseline.changes[0].tipped,
        "Vergleich: {:?}",
        i.baseline
    );
    let alone = build(&cur, None, &Meta::default());
    assert!(
        alone.targets[0].trend.is_none() && !alone.baseline.present,
        "ohne Baseline darf es keinen Trend geben"
    );
}

/// AC-06: Bericht ohne Kennzahlen: graue Kennzahl-Ampeln mit Hinweis; ungültiges Ziel: grau.
#[test]
fn alter_bericht() {
    let mut s = fixture();
    s.kennzahlen = None;
    s.targets[0].verdict.status = STATUS_INVALID.to_owned();
    s.targets[0].verdict.invalid_seeds = vec!["4".to_owned(), "9".to_owned()];
    let i = build(&s, None, &Meta::default());
    assert!(
        i.figures.note.contains("ohne Kennzahlen") && i.figures.nights.is_empty(),
        "Hinweis: {}",
        i.figures.note
    );
    assert!(
        i.figures
            .lights
            .iter()
            .all(|l| l.light == LIGHT_GREY && l.value.is_none()),
        "Kennzahl nicht grau: {:?}",
        i.figures.lights
    );
    let t = &i.targets[0];
    assert!(
        t.light == LIGHT_GREY && t.distance.is_none() && i.counts[LIGHT_GREY] == 1,
        "ungültiges Ziel: {t:?}"
    );
    assert!(
        i.hints.iter().any(|h| h.light == LIGHT_GREY && h.text.contains("4, 9")),
        "Hinweis zu ungültigen Seeds fehlt: {:?}",
        i.hints
    );
}

/// AC-01: Ein bestandener Lauf ohne Gelb schlägt die neue Baseline vor und hat keine roten Hinweise.
#[test]
fn alles_gruen() {
    let mut s = fixture();
    s.pass = true;
    s.targets.retain(|e| e.verdict.status == STATUS_OK);
    let i = build(&s, None, &Meta::default());
    assert!(
        i.hints.is_empty() && i.next.seeds.is_empty() && i.next.steps.join(" ").contains("Baseline neu schreiben"),
        "alles grün: {:?} {:?}",
        i.hints,
        i.next
    );
    // wie Go: keine Seeds ist `null`, die übrigen Listen sind `[]`
    let json = serde_json::to_string(&i).expect("JSON");
    for want in [
        r#""seeds":null"#,
        r#""hints":[]"#,
        r#""leverGroups":[]"#,
        r#""weaknesses":["#,
    ] {
        assert!(json.contains(want), "{want} fehlt in {json}");
    }
}

/// AC-03: Der Markdown-Bericht hat oben Kurzauswertung, Ampeln, Stärken und Schwächen, Hinweise, nächsten Lauf und
/// Kennzahlen je Nacht.
#[test]
fn markdown_mit_auswertung() {
    let md = fixture().markdown(&repo_root(), None);
    for want in [
        "## Kurzauswertung",
        "## Ampeln",
        "🔴 verletzt | Hub-Stufe 2",
        "## Stärken",
        "## Schwächen",
        "## Tuning-Hinweise",
        "`hub.json:levels.1.steps.0.cost.stone`",
        "## Nächster Lauf",
        "(in den Chat an Claude)",
        "## Kennzahlen",
        "| Grad | Nacht | Läufe |",
        "| normal | 1 | 40 | 90 |",
        "## Ziele im Detail",
    ] {
        assert!(md.contains(want), "{want} fehlt");
    }
    assert!(
        md.find("## Kurzauswertung") < md.find("| Ziel | Szenario |"),
        "Auswertung muss vor der Tabelle je Ziel stehen"
    );
}

/// AC-01: Der Bereich fürs Abstimmen geht vom Wert in `data/` höchstens drei Schritte in Richtung des Hebels.
#[test]
fn hebel_bereich() {
    let root = repo_root();
    let path = "buildings.json:wall.levels.0.hp";
    let p = parse_tune_param(&root, &format!("{path}=0..1")).expect("Parameter");
    let lever = |path: &str, direction: &str, step: f64| Lever {
        path: path.to_owned(),
        direction: direction.to_owned(),
        step,
        label: "x".to_owned(),
        ..Lever::default()
    };
    let want = format!("{path}={}..{}", p.format(p.base), p.format(p.base * 1.6));
    assert_eq!(lever(path, "+", 0.2).tune_param(&root), Some(want));
    assert_eq!(
        lever("buildings.json:fehlt.hp", "-", 0.1).tune_param(&root),
        None,
        "unbekannter Pfad darf keinen Bereich liefern"
    );
}

/// BS3.4: Ein Ziel ohne Eintrag in `balance-hints.json` darf keine Hebel als `null` melden.
#[test]
fn hebel_nie_null() {
    let mut s = fixture();
    for e in &mut s.targets {
        "ohneHinweise".clone_into(&mut e.verdict.target.measure);
    }
    let json = serde_json::to_string(&build(&s, None, &Meta::default())).expect("JSON");
    assert!(
        !json.contains(r#""levers":null"#),
        "levers:null im Bericht der Oberfläche"
    );
}
