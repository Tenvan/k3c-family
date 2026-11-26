use std::collections::BTreeMap;

use super::super::world_tests::{checkout, failing, fixture, read_world};
use super::super::{Detail, Editor, Island};
use super::{IslandStory, Story, StoryPatch};

const STORY_BLOCK: &str = r#"  "story": {
    "lexicon": ["oru", "kamae", "no", "sen", "shiro", "nu"],
    "start": ["Oru kamae KRONE no sen. Shiro nu BRENNT."],
    "etappen": { "1": ["Sen no MEER, oru KRONE kamae."] },
    "end": ["Shiro no FRIEDE, kamae KRONE sen."],
    "bottles": [ ["Nu sen FLASCHE, oru MEER no."] ]
  },
"#;

const ISLAND_STORY_TEXT: &str = r#", "story": { "arrival": ["Oru no REIS, sen SCHIFF nu."], "boss": ["Nu kamae MASKE, shiro TIEF."], "done": ["MASKE nu, KRONE oru sen."] }"#;

const END_LINE: &str = "Shiro no FRIEDE, kamae KRONE sen.";
const DONE_LINE: &str = "MASKE nu, KRONE oru sen.";

fn lines(items: &[&str]) -> Option<Vec<String>> {
    Some(items.iter().map(|s| (*s).to_owned()).collect())
}

fn etappen(key: &str, items: &[&str]) -> Option<BTreeMap<String, Vec<String>>> {
    Some([(key.to_owned(), lines(items).unwrap_or_default())].into())
}

fn island(id: &str, story: IslandStory) -> BTreeMap<String, IslandStory> {
    [(id.to_owned(), story)].into()
}

fn world_patch(story: Story) -> StoryPatch {
    StoryPatch {
        story,
        ..StoryPatch::default()
    }
}

/// Gültige Geschichte der Welt mit einer Insel island1 (Muster aus docs/rules/welten.md § 2.2).
fn story_fixture() -> StoryPatch {
    StoryPatch {
        story: Story {
            lexicon: lines(&["oru", "kamae", "no", "sen", "shiro", "nu"]),
            start: lines(&["Oru kamae KRONE no sen. Shiro nu BRENNT."]),
            etappen: etappen("1", &["Sen no MEER, oru KRONE kamae."]),
            end: lines(&[END_LINE]),
            bottles: Some(vec![vec!["Nu sen FLASCHE, oru MEER no.".to_owned()]]),
        },
        islands: island(
            "island1",
            IslandStory {
                arrival: lines(&["Oru no REIS, sen SCHIFF nu."]),
                boss: lines(&["Nu kamae MASKE, shiro TIEF."]),
                done: lines(&[DONE_LINE]),
            },
        ),
    }
}

// B-975/AC-01: set_story schreibt Welt- und Insel-Geschichte, die übrigen Bytes bleiben; ein zweiter Lauf ändert nichts,
// Entfernen stellt die Datei byte-gleich wieder her.
#[test]
fn set_story_schreibt_nur_story() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    e.create("saga", "Saga", "").unwrap();
    let before = read_world(e, "saga");
    let res = e.set_story("saga", &story_fixture()).unwrap();
    let after = read_world(e, "saga");
    let stripped = after.replacen(STORY_BLOCK, "", 1).replacen(ISLAND_STORY_TEXT, "", 1);
    assert_eq!(stripped, before, "übrige Felder nicht byte-gleich:\n{after}");
    assert!(after.find(r#""story""#) < after.find(r#""islands""#), "{after}");
    assert_eq!(
        res.set.join(", "),
        "lexicon (6 Wörter), start, etappen.1, end, bottles (1 Einträge), island1.arrival, island1.boss, island1.done"
    );
    assert!(res.removed.is_empty() && res.missing.is_empty(), "{res:?}");
    e.set_story("saga", &story_fixture()).unwrap();
    assert_eq!(read_world(e, "saga"), after);
    let d = e.load("saga");
    assert_eq!(d.story.and_then(|s| s.lexicon).map(|l| l.len()), Some(6));
    let done = d.islands[0].story.as_ref().and_then(|s| s.done.clone());
    assert_eq!(done, lines(&[DONE_LINE]));
    story_cleared(e, &before);
}

/// Leere Einträge entfernen die Geschichte, die Datei ist wieder byte-gleich `before`.
fn story_cleared(e: &Editor, before: &str) {
    let none = StoryPatch {
        story: Story {
            lexicon: lines(&[]),
            start: lines(&[]),
            etappen: etappen("1", &[]),
            end: lines(&[]),
            bottles: Some(Vec::new()),
        },
        islands: island(
            "island1",
            IslandStory {
                arrival: lines(&[]),
                boss: lines(&[]),
                done: lines(&[]),
            },
        ),
    };
    let res = e.set_story("saga", &none).unwrap();
    assert_eq!(read_world(e, "saga"), before);
    assert_eq!(res.removed.len(), 8, "{res:?}");
    assert_eq!(res.missing.join("; "), "story; Insel island1: story");
}

/// Fälle: Name, Welt, erwarteter Teil des Fehlers, Patch.
fn rejected_cases() -> Vec<(&'static str, &'static str, &'static str, StoryPatch)> {
    let start = Story {
        start: lines(&["Oru kame KRONE no MEER."]),
        ..Story::default()
    };
    let boss = IslandStory {
        boss: lines(&["Nu kamae MASKE, shiro TIEF.", "Oru KRONE sen."]),
        ..IslandStory::default()
    };
    let words = Story {
        lexicon: lines(&["oru", "kamae"]),
        ..Story::default()
    };
    let stage = Story {
        etappen: etappen("2", &["Sen no MEER, oru KRONE kamae."]),
        ..Story::default()
    };
    let done = IslandStory {
        done: lines(&[DONE_LINE]),
        ..IslandStory::default()
    };
    let islands = |id: &str, s| StoryPatch {
        islands: island(id, s),
        ..StoryPatch::default()
    };
    vec![
        (
            "Wort fehlt im Wortschatz",
            "saga",
            r#"story.start Zeile 1: Wort "kame" fehlt im lexicon"#,
            world_patch(start),
        ),
        (
            "zu wenig echte Wörter",
            "saga",
            "Insel island1: story.boss Zeile 2",
            islands("island1", boss),
        ),
        ("Wortschatz verkleinert", "saga", "fehlt im lexicon", world_patch(words)),
        (
            "Etappe ohne Insel",
            "saga",
            "Etappe 2 hat keine Insel",
            world_patch(stage),
        ),
        (
            "unbekannte Insel",
            "saga",
            r#"die Insel "island9" gibt es nicht (bekannt: island1)"#,
            islands("island9", done),
        ),
        (
            "unbekannte Welt",
            "nirgendwo",
            r#"die Welt "nirgendwo" gibt es nicht (bekannt: "#,
            story_fixture(),
        ),
        ("leerer Aufruf", "saga", "kein Eintrag genannt", StoryPatch::default()),
    ]
}

// B-975/AC-01 und Fehlerfälle: Ungültiges wird abgelehnt, die Datei bleibt.
#[test]
fn set_story_lehnt_ab() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    e.create("saga", "Saga", "").unwrap();
    e.set_story("saga", &story_fixture()).unwrap();
    let before = read_world(e, "saga");
    for (name, world, want, patch) in rejected_cases() {
        let error = e.set_story(world, &patch).unwrap_err();
        assert!(error.contains(want), "{name}: {error:?}, erwartet {want:?}");
        assert_eq!(read_world(e, "saga"), before, "{name}: Datei trotz Fehler geändert");
    }
}

// Review B-975: Die übrigen Werkzeuge lassen `story` stehen; world_set nimmt `story` nicht an.
#[test]
fn andere_aenderungen_behalten_story() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    e.create("saga", "Saga", "").unwrap();
    e.set_story("saga", &story_fixture()).unwrap();
    let patch = r#"{"notes":"Sagen","islands":[{"id":"island1","name":"Fjord","depthScaling":{"hp":2}}]}"#;
    e.set("saga", patch).unwrap();
    let harbor = r#"[{"slot":-7,"kind":"harbor"}]"#;
    e.set_chunks("saga", "island1", 0, Some(harbor), Some(24)).unwrap();
    let view = e.set_etappe_view("saga", "island1", Some(1));
    assert!(view.error.is_empty(), "{view:?}");
    let text = read_world(e, "saga");
    for want in [
        STORY_BLOCK,
        ISLAND_STORY_TEXT,
        r#""notes": "Sagen""#,
        r#""name": "Fjord", "etappe": 1"#,
        r#""chunkCounts": { "0": 24 }"#,
    ] {
        assert!(text.contains(want), "fehlt {want:?}:\n{text}");
    }
    for patch in [
        r#"{"story":{"lexicon":["oru"]}}"#,
        r#"{"islands":[{"id":"island1","story":{"done":["MASKE nu, KRONE oru sen."]}}]}"#,
    ] {
        let error = e.set("saga", patch).unwrap_err();
        assert!(error.contains("world_story_set"), "{error}");
    }
    assert_eq!(read_world(e, "saga"), text);
}

// B-975/AC-03: Anlegen mit Geschichte schreibt sie mit; eine ungültige legt keine Welt an. CRLF bleibt.
#[test]
fn anlegen_mit_story_und_crlf() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    let mut bad = story_fixture();
    bad.story.start = lines(&["Oru kamae KRONE."]);
    let error = e.create_with_story("saga", "Saga", "", Some(&bad)).unwrap_err();
    assert!(error.contains("story.start Zeile 1"), "{error}");
    assert!(!e.load("saga").error.is_empty(), "Welt trotz ungültiger Geschichte");
    e.create_with_story("saga", "Saga", "", Some(&story_fixture())).unwrap();
    let text = read_world(e, "saga");
    assert!(text.contains(STORY_BLOCK) && text.contains(ISLAND_STORY_TEXT), "{text}");
    assert!(e.load("saga").story_gaps().is_empty());
    e.create("crlf", "CRLF", "").unwrap();
    let crlf = read_world(e, "crlf").replace('\n', "\r\n");
    std::fs::write(e.file("crlf"), crlf).unwrap();
    e.set_story("crlf", &story_fixture()).unwrap();
    let got = read_world(e, "crlf");
    assert!(
        !got.replace("\r\n", "").contains('\n') && got.contains("\"story\": {\r\n"),
        "Zeilenenden gemischt:\n{got:?}"
    );
}

// B-975/R3: fehlende Einträge je Welt und Insel.
#[test]
fn story_luecken() {
    let x = || lines(&["x"]);
    let a = Island {
        id: "a".to_owned(),
        story: Some(IslandStory {
            arrival: x(),
            done: x(),
            boss: None,
        }),
        ..Island::default()
    };
    let b = Island {
        id: "b".to_owned(),
        ..Island::default()
    };
    let d = Detail {
        story: Some(Story {
            lexicon: lines(&["oru"]),
            start: x(),
            ..Story::default()
        }),
        islands: vec![a, b],
        ..Detail::default()
    };
    assert_eq!(
        d.story_gaps().join("; "),
        "story.end; story.bottles; Insel a: story.boss; Insel b: story"
    );
}

// Review B-975: Nennt der Aufruf nur Inseln, bleibt ein von Hand geschriebenes `story` der Welt byte-gleich; ein Eintrag
// der Welt schreibt es danach im Stil der Datei neu.
#[test]
fn nur_inseln_lassen_welt_story_byte_gleich() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    e.create("saga", "Saga", "").unwrap();
    let hand = "\"story\": {\"lexicon\":[\"oru\",\"kamae\",\"no\",\"sen\",\"shiro\",\"nu\"]},\n  \"islands\"";
    let text = read_world(e, "saga").replacen("\"islands\"", hand, 1);
    std::fs::write(e.file("saga"), &text).unwrap();
    let islands = StoryPatch {
        islands: story_fixture().islands,
        ..StoryPatch::default()
    };
    e.set_story("saga", &islands).unwrap();
    let want = text.replacen(" }\n  ]", &format!("{ISLAND_STORY_TEXT} }}\n  ]"), 1);
    assert_eq!(read_world(e, "saga"), want);
    let end = world_patch(Story {
        end: lines(&[END_LINE]),
        ..Story::default()
    });
    e.set_story("saga", &end).unwrap();
    let got = read_world(e, "saga");
    let styled =
        "\"story\": {\n    \"lexicon\": [\"oru\", \"kamae\", \"no\", \"sen\", \"shiro\", \"nu\"],\n    \"end\"";
    assert!(got.contains(styled), "{got}");
}

// Format ohne Spiel-EXE: Etappen nach Nummer, `<>&` unmaskiert, übrige Bytes unverändert.
#[test]
fn story_format_ohne_engine() {
    let f = checkout(|_| failing("keine Engine"));
    let raw = std::fs::read(f.editor.file("standard")).unwrap();
    let mut map = etappen("10", &["b"]).unwrap_or_default();
    map.insert("2".to_owned(), vec!["<a> & b".to_owned()]);
    let patch = world_patch(Story {
        etappen: Some(map),
        ..Story::default()
    });
    let (out, _) = super::apply_story(&raw, &patch).unwrap();
    let block = "\"story\": {\n    \"etappen\": { \"2\": [\"<a> & b\"], \"10\": [\"b\"] }\n  },\n  ";
    let want = String::from_utf8_lossy(&raw).replacen("\"islands\"", &format!("{block}\"islands\""), 1);
    assert_eq!(String::from_utf8_lossy(&out), want);
}
