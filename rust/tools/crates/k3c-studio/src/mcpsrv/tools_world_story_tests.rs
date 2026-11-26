//! Port von `tools_world_story_test.go` (B-975): `world_story_set`, `world_get`, `world_check` und `world_create` mit
//! der Geschichte, auf einer Kopie von `data/` mit der Spiel-EXE.

use std::path::PathBuf;

use serde_json::{Value, json};

use super::super::tools_world::tests::World;
use super::{STORY_MISSING, StoryIn};

fn story_args() -> Value {
    json!({
        "lexicon": ["oru", "kamae", "no", "sen", "shiro", "nu"],
        "start": ["Oru kamae KRONE no sen. Shiro nu BRENNT."],
        "etappen": {"1": ["Sen no MEER, oru KRONE kamae."]},
        "end": ["Shiro no FRIEDE, kamae KRONE sen."],
        "islands": {"island1": {
            "arrival": ["Oru no REIS, sen SCHIFF nu."],
            "done": ["MASKE nu, KRONE oru sen."],
        }},
    })
}

fn with_id(id: &str, mut args: Value) -> Value {
    args["id"] = json!(id);
    args
}

fn saga(w: &World) -> PathBuf {
    w.root.join("data").join("worlds").join("saga.json")
}

fn read(w: &World) -> String {
    std::fs::read_to_string(saga(w)).unwrap()
}

async fn check_hints(w: &mut World, hints: &[&str]) {
    let wants: Vec<&str> = ["Welt saga ist gültig"]
        .into_iter()
        .chain(hints.iter().copied())
        .collect();
    let text = w.ok("world_check", json!({"id": "saga"}), &wants).await;
    assert_eq!(text.matches("Hinweis:").count(), hints.len(), "{text}");
}

/// AC-01: schreibt Welt- und Insel-Geschichte, die übrigen Felder bleiben byte-gleich.
async fn check_set(w: &mut World, before: &str) {
    let wants = [
        "Geschichte der Welt saga gespeichert",
        "Gesetzt: lexicon (6 Wörter), start, etappen.1, end, island1.arrival, island1.done",
        "Fehlt noch: story.bottles, Insel island1: story.boss",
        "Checkout:",
    ];
    w.ok("world_story_set", with_id("saga", story_args()), &wants).await;
    let after = read(w);
    let world = "  \"story\": {\n    \"lexicon\": [\"oru\", \"kamae\", \"no\", \"sen\", \"shiro\", \"nu\"],\n    \
                 \"start\": [\"Oru kamae KRONE no sen. Shiro nu BRENNT.\"],\n    \
                 \"etappen\": { \"1\": [\"Sen no MEER, oru KRONE kamae.\"] },\n    \
                 \"end\": [\"Shiro no FRIEDE, kamae KRONE sen.\"]\n  },\n";
    let island =
        ", \"story\": { \"arrival\": [\"Oru no REIS, sen SCHIFF nu.\"], \"done\": [\"MASKE nu, KRONE oru sen.\"] }";
    // Die Kopie von `data/` kann CRLF haben (Checkout unter Windows).
    let world = if before.contains("\r\n") {
        world.replace('\n', "\r\n")
    } else {
        world.to_owned()
    };
    let got = after.replacen(&world, "", 1).replacen(island, "", 1);
    assert_eq!(got, before, "übrige Felder nicht byte-gleich:\n{after}");
    // Je Teil: ein Eintrag dazu, einer weg ([] entfernt), der Rest bleibt.
    let part = json!({"id": "saga", "bottles": [["Nu sen FLASCHE, oru MEER no."]],
        "islands": {"island1": {"boss": ["Nu kamae MASKE, shiro TIEF."], "arrival": []}}});
    let wants = ["Entfernt: island1.arrival", "Fehlt noch: Insel island1: story.arrival"];
    w.ok("world_story_set", part, &wants).await;
    let got = read(w);
    assert!(!got.contains("\"arrival\""), "{got}");
    assert!(
        got.contains("\"boss\": [\"Nu kamae MASKE, shiro TIEF.\"], \"done\""),
        "{got}"
    );
    assert!(got.contains("\"start\": [\"Oru kamae"), "{got}");
}

/// AC-02: `world_get` zeigt Wortschatz und Zeilen, fehlende als „fehlt“; `world_check` nennt fehlende als Hinweis.
async fn check_get_and_check(w: &mut World) {
    let wants = [
        "Geschichte:\n  lexicon (6): oru, kamae, no, sen, shiro, nu\n",
        "  start: Oru kamae KRONE no sen. Shiro nu BRENNT.\n",
        "  etappen.1: Sen no MEER, oru KRONE kamae.\n",
        "  bottles[1]: Nu sen FLASCHE, oru MEER no.\n",
        "  island1.arrival: fehlt\n",
        "  island1.boss: Nu kamae MASKE, shiro TIEF.\n",
    ];
    w.ok("world_get", json!({"id": "saga"}), &wants).await;
    check_hints(w, &["Hinweis: Geschichte fehlt (Insel island1: story.arrival)"]).await;
}

/// AC-01/AC-02: Verstöße sind Fehler mit Feld und Zeile, geschrieben wird nichts.
async fn check_rejects(w: &mut World) {
    let before = read(w);
    let cases = [
        (
            "story.start Zeile 1: Wort \"kame\" fehlt im lexicon",
            json!({"start": ["Oru kame KRONE no MEER."]}),
        ),
        (
            "Insel island1: story.done Zeile 1:",
            json!({"islands": {"island1": {"done": ["MASKE nu oru sen."]}}}),
        ),
        ("fehlt im lexicon", json!({"lexicon": ["oru"]})),
        (
            "die Insel \"island7\" gibt es nicht (bekannt: island1)",
            json!({"islands": {"island7": {"done": ["MASKE nu, KRONE oru sen."]}}}),
        ),
        ("unknown field", json!({"islands": {"island1": {"ankunft": []}}})),
    ];
    for (want, args) in cases {
        w.rejects("world_story_set", with_id("saga", args), want).await;
        assert_eq!(read(w), before, "Datei trotz Fehler geändert ({want})");
    }
    let want = "die Welt \"nirgendwo\" gibt es nicht (bekannt: ";
    w.rejects("world_story_set", with_id("nirgendwo", story_args()), want)
        .await;
    let (text, _) = w.call("world_story_set", with_id("nirgendwo", story_args())).await;
    assert!(text.contains("saga"), "{text}");
    let patch = json!({"id": "saga", "patch": {"story": {}}});
    w.rejects("world_set", patch, "world_story_set").await;
    // Von Hand verdorbene Datei: `world_check` meldet den Verstoß als Fehler.
    let broken = before.replacen("Oru kamae KRONE no sen.", "Oru kamae krone no sen.", 1);
    std::fs::write(saga(w), broken).unwrap();
    let want = "story.start Zeile 1: Wort \"krone\" fehlt im lexicon";
    w.rejects("world_check", json!({"id": "saga"}), want).await;
    std::fs::write(saga(w), before).unwrap();
}

/// AC-03: `world_create` mit Geschichte schreibt sie mit, die Antwort nennt den Hinweis dann nicht.
async fn check_create(w: &mut World) {
    let create = json!({"id": "edda", "name": "Edda", "story": story_args()});
    let text = w.ok("world_create", create, &["Welt edda angelegt"]).await;
    assert!(!text.contains(STORY_MISSING), "{text}");
    let wants = [
        "  island1.done: MASKE nu, KRONE oru sen.\n",
        "\"lexicon\": [\"oru\", \"kamae\"",
    ];
    w.ok("world_get", json!({"id": "edda"}), &wants).await;
    let mut bad = story_args();
    bad["end"] = json!(["Shiro no kamae sen."]);
    let create = json!({"id": "kaputt", "name": "Kaputt", "story": bad});
    w.rejects("world_create", create, "story.end Zeile 1").await;
    let (text, is_error) = w.call("world_get", json!({"id": "kaputt"})).await;
    assert!(is_error, "Welt trotz ungültiger Geschichte angelegt: {text}");
}

#[tokio::test(flavor = "multi_thread")]
async fn world_story_tools() {
    let Some(mut w) = World::start().await else { return };
    // AC-03: ohne Geschichte endet die Antwort mit dem Hinweis (danach folgt nur die Checkout-Zeile des Servers).
    let missing = format!("\n{STORY_MISSING}");
    let create = json!({"id": "saga", "name": "Saga"});
    w.ok("world_create", create, &["Welt saga angelegt", &missing]).await;
    let before = read(&w);
    let hints = [
        "Hinweis: Geschichte fehlt (story)",
        "Hinweis: Geschichte fehlt (Insel island1: story)",
    ];
    check_hints(&mut w, &hints).await;
    check_set(&mut w, &before).await;
    check_get_and_check(&mut w).await;
    check_rejects(&mut w).await;
    check_create(&mut w).await;
    w.stop().await;
}

/// `null` in `etappen` und `bottles` ist wie bei Go die leere Liste; unbekannte Unterfelder werden abgelehnt.
#[test]
fn story_argumente_wie_go() {
    let input: StoryIn = serde_json::from_value(json!({"id": "a", "etappen": {"1": null}, "bottles": [null]})).unwrap();
    let patch = input.story.patch();
    assert_eq!(patch.story.etappen.unwrap()["1"], Vec::<String>::new());
    assert_eq!(patch.story.bottles, Some(vec![Vec::new()]));
    assert!(patch.story.lexicon.is_none() && patch.islands.is_empty());
    let unknown = json!({"id": "a", "islands": {"island1": {"ankunft": []}}});
    assert!(serde_json::from_value::<StoryIn>(unknown).is_err());
}
