//! Port von `tools_assets_preview_test.go`. Statt `K3C_PAPER_CLI` liegt für die echte CLI ein Verweis im Test-Repo.

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use regex::Regex;
use serde_json::{Value, json};

use super::super::test_helpers::{connect, connect_mit, enthaelt, fixture_review, paper_repo};
use super::{box_reihenfolge, data_url, png_groesse};
use crate::paperassets::test_helpers::{schreibe, ts_text};
use crate::review::Teile;
use crate::review::test_helpers::repo_root;

fn passt(muster: &str, text: &str) -> bool {
    Regex::new(muster).expect("Muster").is_match(text)
}

fn bild_und_text(res: &Value) -> (Vec<u8>, String) {
    assert!(res["isError"] != json!(true), "{res}");
    assert_eq!(res["content"][0]["type"], "image", "{res}");
    assert_eq!(res["content"][0]["mimeType"], "image/png", "{res}");
    let data = res["content"][0]["data"].as_str().expect("data");
    let text = res["content"][1]["text"].as_str().expect("text");
    (STANDARD.decode(data).expect("Base64"), text.to_owned())
}

/// MC1/AC-03: `assets_preview` liefert ein Bild `image/png` plus Text mit Teil-Boxen.
#[tokio::test(flavor = "multi_thread")]
async fn assets_preview_bild() {
    let Some(w) = paper_repo() else { return };
    let mut cs = connect(&w.pfad).await;
    for (anim, zeige, fixture) in [
        ("", "idle", "monarch-idle-0.png"),
        ("reiten", "reiten", "monarch-galopp-0.png"),
    ] {
        let mut args = json!({"id": "monarch"});
        if !anim.is_empty() {
            args["animation"] = json!(anim);
        }
        let (bild, text) = bild_und_text(&cs.result("assets_preview", args).await);
        let want = std::fs::read(repo_root().join("tools/paper/fixtures/render/monarch").join(fixture));
        assert_eq!(bild, want.expect("Fixture"), "{anim}");
        let kopf =
            format!(r"^monarch · {zeige} · Frame 0 · Basis – · \d+×\d+ px\nTeile \(x,y,b,h px\): \S+ \d+,\d+,\d+,\d+");
        assert!(passt(&kopf, &text), "{text}");
        enthaelt(&text, &["rumpf 53,58,47,78"]);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn assets_preview_fehler() {
    let Some(w) = paper_repo() else { return };
    let mut cs = connect(&w.pfad).await;
    for (args, want) in [
        (json!({"id": "monarch", "animation": "flug"}), "idle, lauf, reiten"),
        (json!({"id": "x"}), "Asset „x“ fehlt"),
        (
            json!({"id": "monarch", "basis": "main"}),
            "Basis „main“ ungültig: HEAD, HEAD~1, Commit-SHA oder origin/main",
        ),
    ] {
        let text = cs.abgelehnt("assets_preview", args).await;
        enthaelt(&text, &[want]);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn assets_preview_basis() {
    let Some(w) = paper_repo() else { return };
    let mut cs = connect_mit(&w.pfad, |s| s.review = fixture_review()).await;
    let args = json!({"id": "monarch", "basis": "HEAD~1"});
    let (bild, text) = bild_und_text(&cs.result("assets_preview", args).await);
    assert!(!bild.is_empty(), "kein Bild");
    enthaelt(&text, &["monarch · idle · Frame 0 · Basis HEAD~1 · "]);
}

/// B-709: `assets_preview` reicht die Boxen `reittier:*` der echten CLI durch (nach den Teilen der Spec, sortiert).
#[tokio::test(flavor = "multi_thread")]
async fn assets_preview_reittier_boxen() {
    let Some(w) = paper_repo() else { return };
    let verweis = format!(
        "import {{ run }} from {};\n\
         const r = run(process.argv.slice(2));\n\
         if (r.stdout) process.stdout.write(r.stdout + '\\n');\n\
         if (r.stderr) process.stderr.write(r.stderr + '\\n');\n\
         process.exit(r.code);\n",
        ts_text(&repo_root().join("tools/paper/bin/paper.ts"))
    );
    schreibe(&w.pfad, "tools/paper/bin/paper.ts", verweis);
    let args = json!({"id": "monarch", "animation": "reiten"});
    let mut cs = connect(&w.pfad).await;
    let (_, text) = bild_und_text(&cs.result("assets_preview", args).await);
    assert!(passt(r"· reittier:[a-z-]+ \d+,\d+,\d+,\d+", &text), "{text}");
}

#[test]
fn boxen_reihenfolge() {
    let boxen: Teile = [
        ("kopf", vec![1, 2, 3, 4]),
        ("reittier:rumpf", vec![5, 6, 7, 8]),
        ("reittier:bein-v", vec![9, 9, 9, 9]),
        ("rumpf", vec![0, 0, 1, 1]),
        ("fremd", vec![1, 1, 1, 1]),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_owned(), v))
    .collect();
    let got = box_reihenfolge(r#"{"teile":[{"id":"rumpf"},{"id":"kopf"}]}"#, &boxen);
    assert_eq!(got.join(","), "rumpf,kopf,reittier:bein-v,reittier:rumpf");
}

#[test]
fn data_url_und_groesse() {
    let png = std::fs::read(repo_root().join("tools/paper/fixtures/render/monarch/monarch-idle-0.png"));
    let png = png.expect("Fixture");
    let url = format!("data:image/png;base64,{}", STANDARD.encode(&png));
    assert_eq!(data_url(&url), Ok(png.clone()));
    assert_eq!(data_url("kein Bild"), Err("Bild ist keine Data-URL".to_owned()));
    assert!(png_groesse(&png).is_some_and(|(w, h)| w > 0 && h > 0));
    assert_eq!(png_groesse(b"kein PNG"), None);
}
