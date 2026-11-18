//! Go: `story_test.go` (B-965, GS1.2, AC-04, AC-05), Teil Wendepunkte: Beispiel-Welt, Etappe ohne Angabe, Determinismus.

use super::mod_tests::story_example;
use super::{story_arrival, story_boss, story_defeated, story_start, story_victory};
use crate::island::testutil::{standard_world, test_data};
use crate::island::{IslandDef, WorldDef};
use crate::story::WorldStory;

fn strings(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

fn first(v: &Option<Vec<String>>) -> String {
    v.as_ref().and_then(|l| l.first()).cloned().unwrap_or_default()
}

fn completed(ids: &[&str]) -> Vec<String> {
    strings(ids)
}

#[test]
fn story_wendepunkte_beispiel() {
    let w = story_example();
    let (s, a, b, c) = (&w.story, &w.islands[0].story, &w.islands[1].story, &w.islands[2].story);
    let etappe1 = s.etappen["1"][0].clone();
    assert_eq!(story_start(&w), [first(&s.start), first(&a.arrival)], "Start");
    assert_eq!(
        story_arrival(&w, "c"),
        c.arrival.clone().unwrap_or_default(),
        "Ankunft c"
    );
    assert!(story_arrival(&w, "zz").is_empty(), "Ankunft unbekannt");
    assert_eq!(story_boss(&w, "a"), a.boss.clone().unwrap_or_default(), "Boss a");
    assert!(story_boss(&w, "b").is_empty(), "Boss b ohne Eintrag");
    assert_eq!(
        story_defeated(&w, "a", &[]),
        [first(&a.done), etappe1],
        "Sieg a: Etappe 1 abgeschlossen"
    );
    assert!(
        story_defeated(&w, "c", &completed(&["a"])).is_empty(),
        "Sieg c, b offen: c ohne done"
    );
    assert_eq!(
        story_defeated(&w, "b", &completed(&["a", "c"])),
        [first(&b.done), first(&s.end)],
        "Sieg b nach c: letzte Insel"
    );
    assert_eq!(
        story_defeated(&w, "c", &completed(&["a", "b"])),
        [first(&s.end)],
        "Sieg c nach b: letzte Insel"
    );
    assert_eq!(
        story_defeated(&w, "b", &completed(&["a"])),
        b.done.clone().unwrap_or_default(),
        "Sieg b, c offen"
    );
    assert!(story_victory(&w, "a", &[]).is_empty(), "Variante auf früherer Insel");
    assert_eq!(
        story_victory(&w, "b", &completed(&["a", "c"])),
        [first(&b.done), first(&s.end)],
        "Variante auf letzter Insel"
    );
    assert!(story_defeated(&w, "zz", &[]).is_empty(), "Insel unbekannt");
}

/// Ohne `etappe` ist jede Insel eine eigene Etappe: auf jede Abschlusszeile folgt die Etappenzeile der Welt.
#[test]
fn story_etappe_ohne_angabe() {
    let mut std: WorldDef = standard_world(&test_data());
    if std.islands.len() < 2 {
        return; // Standard-Welt hat nur eine Insel (Go: Skip)
    }
    std.story = WorldStory {
        etappen: [("1".to_string(), strings(&["x"]))].into(),
        end: Some(strings(&["e"])),
        ..Default::default()
    };
    let mut islands: Vec<IslandDef> = std.islands.clone();
    islands[0].story.done = Some(strings(&["d"]));
    std.islands = islands;
    assert_eq!(story_defeated(&std, &std.islands[0].id.clone(), &[]), ["d", "x"]);
}

/// AC-05: Zwei Aufrufe liefern dasselbe, die Funktionen ändern die Welt nicht, der Aufrufer darf das Ergebnis ändern.
#[test]
fn story_deterministisch_und_ohne_zustand() {
    let w = story_example();
    let before = w.clone();
    let mut first = story_defeated(&w, "a", &[]);
    first[0] = "verändert".into();
    let second = story_defeated(&w, "a", &[]);
    let third = story_defeated(&w, "a", &[]);
    assert!(
        second == third && second[0] != "verändert",
        "nicht deterministisch oder Alias: {second:?} {third:?}"
    );
    let _ = story_start(&w);
    let _ = story_boss(&w, "a");
    let _ = story_victory(&w, "b", &completed(&["a", "c"]));
    assert_eq!(w, before, "Welt verändert");
}
