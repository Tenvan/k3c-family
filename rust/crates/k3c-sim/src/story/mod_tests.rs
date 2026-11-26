//! Go: `story_test.go` (B-965, GS1.2, AC-02 bis AC-05), Teil Lesen: Feld `story` laden, streng lesen, Standard-Welt, Grenzen,
//! nicht vererbt. Die Hilfen (`story_world_json`, `story_example`) nutzen die Tests von `check_tests.rs` und `points_tests.rs`.

use serde_json::json;

use super::{decode_island_story, decode_world_story};
use crate::island::WorldDef;
use crate::island::switch_testutil::{test_island, test_world};
use crate::island::testutil::{data_vor_wl2, parse_world_in, standard_world, test_data};

/// Go: `harborPlan`: Hafen in Tiefe 0 (jede Insel einer Etappe mit mehreren Inseln braucht einen).
pub const HARBOR_PLAN: &str = r#","plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}"#;

/// Go: `storyLex`.
pub const STORY_LEX: &str = r#""lexicon":["oru","kamae","no","sen","shiro","nu"]"#;

/// Go: `storyWorldJSON`: Welt „test“ mit Inseln a (Etappe 1), b und c (Etappe 2); `world_story` ist der Inhalt von `story` der
/// Welt (ohne Klammern, leer = kein Feld), `island_story` je Insel a, b, c der Inhalt von `story` (leer = kein Feld).
pub fn story_world_json(world_story: &str, island_story: &[&str]) -> String {
    let islands: Vec<String> = ["a", "b", "c"]
        .iter()
        .enumerate()
        .map(|(i, id)| {
            let mut f = HARBOR_PLAN.to_string();
            if *id != "a" {
                f += r#","etappe":2"#;
            }
            if let Some(s) = island_story.get(i).filter(|s| !s.is_empty()) {
                f += &format!(r#","story":{{{s}}}"#);
            }
            test_island(id, &f)
        })
        .collect();
    let raw = test_world(&islands.join(","));
    if world_story.is_empty() {
        return raw;
    }
    raw.replacen(r#""islands""#, &format!(r#""story":{{{world_story}}},"islands""#), 1)
}

/// Go: `storyExample`: die Welt aus B-965 › Beispiele mit eindeutig erkennbaren Zeilen je Wendepunkt.
pub fn story_example() -> WorldDef {
    let ws = format!(
        r#"{STORY_LEX},"start":["Oru kamae KRONE no sen. Shiro nu BRENNT."],"etappen":{{"1":["Sen no MEER, oru KRONE kamae."]}},"end":["Shiro no FRIEDE, kamae KRONE sen."],"bottles":[["Nu sen FLASCHE, oru MEER."]]"#
    );
    let raw = story_world_json(
        &ws,
        &[
            r#""arrival":["Oru no REIS, sen SCHIFF nu."],"boss":["Nu kamae MASKE, shiro TIEF."],"done":["MASKE nu, KRONE oru sen."]"#,
            r#""arrival":["Sen oru HAFEN, nu SEE."],"done":["Nu sen RUHE, oru BERG."]"#,
            r#""arrival":["Kamae no WIND, sen SEE."],"boss":["Shiro nu DRACHE, sen NACHT."]"#,
        ],
    );
    parse_world_in(&raw, "test").expect("Welt mit Geschichte")
}

#[test]
fn story_laedt_mit_und_ohne() {
    let w = story_example();
    assert!(
        w.story.lexicon.len() == 6
            && !w.story.etappen["1"][0].is_empty()
            && w.story.bottles.len() == 1
            && w.islands[0].story.boss.is_some(),
        "Geschichte nicht gelesen: {:?} / {:?}",
        w.story,
        w.islands[0].story
    );
    let plain = parse_world_in(&story_world_json("", &[]), "test").expect("Welt ohne story");
    assert!(
        plain.story.lexicon.is_empty() && plain.story.start.is_none() && plain.islands[0].story.arrival.is_none(),
        "Welt ohne story: {:?}",
        plain.story
    );
    assert!(super::story_start(&plain).is_empty(), "Zeilen ohne story");
}

#[test]
fn standard_welt_erzaehlt_eine_geschichte() {
    // GS1.5 (B-976): Die Standard-Welt erzählt eine Geschichte (Insel 1, ein Bogen); feste Welt vor WL2, siehe `data_vor_wl2`.
    let std = standard_world(&data_vor_wl2());
    assert!(
        std.story.start.as_ref().is_some_and(|s| !s.is_empty()) && !super::story_start(&std).is_empty(),
        "Standard-Welt ohne Zeilen: {:?}",
        std.story
    );
    assert!(std.story.end.as_ref().is_some_and(|s| !s.is_empty()));
}

#[test]
fn story_limits_aus_daten() {
    let l = test_data().story;
    assert_eq!((l.max_line_chars, l.max_lexicon), (64, 24), "data/story.json: {l:?}");
}

/// Go: `TestStoryNichtVererbt`: `extends` vererbt die Geschichte nicht.
#[test]
fn story_wird_nicht_vererbt() {
    let raw = story_world_json(STORY_LEX, &[]).replacen(r#""islands""#, r#""extends":"standard","islands""#, 1);
    let w = parse_world_in(&raw, "test").expect("Welt");
    assert!(
        w.story.start.is_none() && w.islands[0].story.arrival.is_none(),
        "geerbt: {:?}",
        w.story
    );
}

#[test]
fn decode_liest_streng() {
    let s = decode_world_story(&json!({"lexicon": ["oru"], "start": ["x"], "etappen": {"1": ["y"]}, "end": ["z"]}))
        .expect("Geschichte");
    assert_eq!(s.lexicon, ["oru"]);
    assert_eq!(s.start.as_deref(), Some(&["x".to_string()][..]));
    assert_eq!(s.etappen["1"], ["y"]);
    assert!(s.bottles.is_empty());
    let err = decode_world_story(&json!({"intro": ["x"]})).expect_err("unbekanntes Feld");
    assert_eq!(err, r#"story: unbekanntes Feld "intro""#);
    let err = decode_island_story(&json!({"ende": ["x"]})).expect_err("unbekanntes Feld");
    assert_eq!(err, r#"story: unbekanntes Feld "ende""#);
    let err = decode_island_story(&json!({"Arrival": ["x"]})).expect_err("Groß-/Kleinschreibung zählt");
    assert_eq!(err, r#"story: unbekanntes Feld "Arrival""#);
    assert!(decode_world_story(&json!({"start": "kein Array"})).is_err());
    assert!(decode_world_story(&json!(["x"])).is_err());
    assert_eq!(decode_island_story(&json!(null)), Ok(Default::default()), "null = leer");
    let island = decode_island_story(&json!({"arrival": [], "boss": ["b"]})).expect("Insel");
    assert!(island.arrival.as_ref().is_some_and(Vec::is_empty), "vorhanden und leer");
    assert!(island.done.is_none(), "fehlend");
}
