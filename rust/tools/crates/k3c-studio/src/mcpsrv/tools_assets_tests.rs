//! Port von `tools_assets_test.go` (paper-mcp.md › 1 und 2); dazu die Prompts und die Argument-Prüfung.

use serde_json::{Map, Value, json};

use super::super::checkout_policy::{takes_checkout, write_tool};
use super::super::test_helpers::{connect, enthaelt, hash_baum, lies, paper_repo};
use super::super::tools_assets_prompts::prompt_text;
use crate::paperassets::test_helpers::schreibe;

/// Die Asset-Tools dieser Session; `assets_render` und die übrigen bringt RU70.13.
const TOOLS: [&str; 8] = [
    "assets_guide",
    "assets_list",
    "assets_get",
    "assets_create",
    "assets_set",
    "assets_style_get",
    "assets_style_set",
    "assets_delete",
];

/// MC1/AC-01 · paper-mcp.md › 1: jedes Asset-Tool nimmt `checkout`; schreibende nennen den Checkout.
#[tokio::test(flavor = "multi_thread")]
async fn assets_tools_nehmen_checkout() {
    let Some(w) = paper_repo() else { return };
    let mut cs = connect(&w.pfad).await;
    let tools = cs.tools().await;
    for name in TOOLS {
        let tool = tools
            .iter()
            .find(|t| t["name"] == name)
            .unwrap_or_else(|| panic!("{name}"));
        assert!(
            tool["inputSchema"]["properties"].get("checkout").is_some(),
            "{name}: kein checkout im Schema"
        );
        assert!(takes_checkout(name), "{name}");
    }
    for name in ["assets_create", "assets_set", "assets_style_set", "assets_delete"] {
        assert!(write_tool(name), "{name} fehlt in write_tool");
    }
    let text = cs.ok("assets_delete", json!({"id": "monarch"})).await;
    enthaelt(&text, &["\nCheckout: "]);
}

#[tokio::test(flavor = "multi_thread")]
async fn assets_guide() {
    let Some(w) = paper_repo() else { return };
    let mut cs = connect(&w.pfad).await;
    let text = cs.ok("assets_guide", json!({"rig": "figur"})).await;
    enthaelt(
        &text,
        &[
            "Papierschnitt · Rig figur · Standard docs/standards/paper-spec.md",
            "Papier-Materialien (licht/basis/schatten): haut, haar, gold, koenigsblau",
            "Einzeltöne (ohne Ton): huf, auge, weiss, linie, edelstein",
            "Pflicht-Teile figur: rumpf, kopf, oberarm-v",
            "Pivots: Hüfte (0,-50)",
            "Proportion: Kopfhöhe / Rahmenhöhe = 0.286 ± 0.15",
            "Lagen: flaeche {form, material, ton?, auflage?}",
            "Ablauf: assets_get → assets_set (Merge-Patch) → assets_preview → assets_review",
            // B-709/AC-02: Prozent gerundet.
            "(0.5/0.9, Unschärfe 0.45, 55 %)",
            "0.3/0.5/0.3/35 %",
            "\nVorlage figur: {",
        ],
    );
    let alle = cs.ok("assets_guide", json!({})).await;
    for rig in ["figur", "vierbeiner", "gebaeude", "requisite", "hintergrund"] {
        enthaelt(&alle, &[&format!("Pflicht-Teile {rig}: "), &format!("Pivots {rig}: ")]);
    }
    let text = cs.abgelehnt("assets_guide", json!({"rig": "drache"})).await;
    enthaelt(&text, &["figur, vierbeiner"]);
    cs.abgelehnt("assets_guide", json!({"rigg": "figur"})).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn assets_list_paper() {
    let Some(w) = paper_repo() else { return };
    let mut cs = connect(&w.pfad).await;
    let text = cs.ok("assets_list", json!({})).await;
    assert!(
        text.starts_with("3 Assets · 0 ungültig · 0 geändert gegen HEAD\n"),
        "{text}"
    );
    enthaelt(
        &text,
        &["monarch · figur · public/grafik/figuren/monarch.paper.json · gültig\n"],
    );
    schreibe(&w.pfad, "public/grafik/figuren/monarch/idle.png", "neu");
    let text = cs.ok("assets_list", json!({"geaendert": true})).await;
    assert!(
        text.starts_with("1 Asset · 0 ungültig · 1 geändert gegen HEAD\n"),
        "{text}"
    );
    enthaelt(
        &text,
        &["monarch · figur · public/grafik/figuren/monarch.paper.json · gültig · geändert"],
    );
    let text = cs.ok("assets_list", json!({"rig": "gebaeude"})).await;
    assert!(text.contains("mauer · gebaeude") && !text.contains("monarch"), "{text}");
    let text = cs.ok("assets_list", json!({"gruppe": "reittiere"})).await;
    assert!(text.contains("pferd · vierbeiner") && !text.contains("mauer"), "{text}");
    let text = cs.ok("assets_list", json!({"gruppe": "keine"})).await;
    assert_eq!(text.lines().next(), Some("keine Assets für diesen Filter"), "{text}");
}

#[tokio::test(flavor = "multi_thread")]
async fn assets_get_felder() {
    let Some(w) = paper_repo() else { return };
    let mut cs = connect(&w.pfad).await;
    let args = json!({"id": "monarch", "felder": ["umhang", "ausgabe"]});
    let text = cs.ok("assets_get", args).await;
    let kopf = "public/grafik/figuren/monarch.paper.json · Felder umhang, ausgabe\n{\n  \"umhang\": {";
    assert!(text.starts_with(kopf), "{text}");
    assert!(text.contains("\"ausgabe\": {") && !text.contains("\"teile\""), "{text}");
    let text = cs.ok("assets_get", json!({"id": "monarch", "felder": null})).await;
    assert!(
        text.starts_with("public/grafik/figuren/monarch.paper.json\n{"),
        "{text}"
    );
    assert!(!text.contains("$schema") && text.contains("\"teile\""), "{text}");
    let text = cs.abgelehnt("assets_get", json!({"id": "x"})).await;
    assert_eq!(text, "Asset „x“ fehlt");
    let args = json!({"id": "monarch", "felder": ["farbe"]});
    enthaelt(&cs.abgelehnt("assets_get", args).await, &["umhang"]);
}

#[tokio::test(flavor = "multi_thread")]
async fn assets_create() {
    let Some(w) = paper_repo() else { return };
    let mut cs = connect(&w.pfad).await;
    let args = json!({"id": "fass", "rig": "requisite", "beschreibung": "Ein Fass", "gruppe": "requisiten"});
    let text = cs.ok("assets_create", args.clone()).await;
    enthaelt(
        &text,
        &[
            "fass angelegt: public/grafik/requisiten/fass.paper.json (Rig requisite, Vorlage mit Teil korpus)\n",
            "validate: ok\n",
            "Render: 1 PNG · ",
            "Nächster Schritt: assets_set {id: \"fass\", patch: …}, dann assets_preview",
            "\nCheckout: ",
        ],
    );
    let vorher = lies(&w.pfad, "public/grafik/requisiten/fass.paper.json");
    enthaelt(&cs.abgelehnt("assets_create", args).await, &["fass gibt es schon"]);
    assert_eq!(lies(&w.pfad, "public/grafik/requisiten/fass.paper.json"), vorher);
    let args = json!({"id": "Fass!", "rig": "requisite", "beschreibung": "x", "gruppe": "g"});
    cs.abgelehnt("assets_create", args).await;
}

/// Wie Go: unbekannte oder fehlende Argumente ändern nichts.
#[tokio::test(flavor = "multi_thread")]
async fn falsche_argumente_aendern_nichts() {
    let Some(w) = paper_repo() else { return };
    let mut cs = connect(&w.pfad).await;
    let vorher = hash_baum(&w.pfad, &["public/grafik"]);
    for (tool, args) in [
        ("assets_delete", json!({"id": "mauer", "confirm": true, "alle": true})),
        ("assets_set", json!({"id": "monarch"})),
        ("assets_set", json!({"id": "monarch", "patch": null})),
        ("assets_style_get", json!({"id": "monarch"})),
        ("assets_style_set", json!({"patch": [1]})),
    ] {
        cs.abgelehnt(tool, args).await;
    }
    assert_eq!(hash_baum(&w.pfad, &["public/grafik"]), vorher);
}

#[test]
fn prompt_fuellt_platzhalter() {
    let dir = crate::test_helpers::tmp_dir();
    let vorlagen = r#"[{"id": "a", "titel": "T", "ort": "asset", "text": "{asset}: {zustand}/{asset} {fremd}"}]"#;
    schreibe(dir.path(), "data/asset-prompts.json", vorlagen);
    let args = |v: Value| v.as_object().cloned().unwrap_or_else(Map::new);
    let r = prompt_text(dir.path(), "a", &args(json!({"asset": "{basis}"}))).expect("Prompt");
    let r = serde_json::to_value(r).expect("JSON");
    assert_eq!(r["description"], "T");
    assert_eq!(r["messages"][0]["role"], "user");
    assert_eq!(r["messages"][0]["content"]["text"], "{basis}: /{basis} {fremd}");
    for (id, a, want) in [
        ("a", json!({"asset": ""}), "Argument „asset“ fehlt"),
        ("a", json!({"asset": 1}), "Argument „asset“ ist kein Text"),
        ("b", json!({}), "Vorlage „b“ fehlt in data/asset-prompts.json"),
    ] {
        assert_eq!(prompt_text(dir.path(), id, &args(a)).map(|_| ()), Err(want.to_owned()));
    }
}

/// Ein Server ohne Vorlagen-Datei meldet keine Prompts.
#[tokio::test(flavor = "multi_thread")]
async fn ohne_vorlagen_keine_prompts() {
    let dir = crate::test_helpers::tmp_dir();
    let mut cs = connect(dir.path()).await;
    let list = cs.request("prompts/list").await;
    assert_eq!(list["prompts"], json!([]), "{list}");
}
