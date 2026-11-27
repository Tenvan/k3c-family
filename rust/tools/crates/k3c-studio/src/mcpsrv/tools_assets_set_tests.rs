//! Port von `tools_assets_set_test.go`. Anders als Go läuft auch der Material-Test immer (V8), und auf den Render-Job
//! wartet der Test direkt statt über `assets_render` (RU70.13).

use std::fs;
use std::time::Duration;

use regex::Regex;
use serde_json::json;

use super::super::test_helpers::{connect, enthaelt, hash_baum, lies, paper_repo};
use super::{ASSET_JOBS, set_abbruch};
use crate::paperassets::test_helpers::schreibe;
use crate::paperassets::{Asset, Error, FeldAenderung, SetErgebnis};
use crate::review::test_helpers::repo_root;

const MONARCH: &str = "public/grafik/figuren/monarch.paper.json";

fn passt(muster: &str, text: &str) -> bool {
    Regex::new(muster).expect("Muster").is_match(text)
}

#[tokio::test(flavor = "multi_thread")]
async fn assets_set_material() {
    let Some(w) = paper_repo() else { return };
    let mut cs = connect(&w.pfad).await;
    let args = json!({"id": "monarch", "patch": {"umhang": {"material": "purpur"}}});
    let text = cs.ok("assets_set", args).await;
    let kopf = "monarch geändert: 1 Feld · umhang.material koenigsrot → purpur\n";
    assert!(text.starts_with(kopf), "{text}");
    let render = r"\nRender: \d+ PNGs? \(idle, lauf, reiten\) · Pixel-Anteil idle \d+,\d\d %, lauf \d+,\d\d %, reiten \d+,\d\d %";
    assert!(passt(render, &text), "{text}");
    enthaelt(&text, &["\nCheckout: Repo-Wurzel "]);
    assert!(lies(&w.pfad, MONARCH).contains("\"material\": \"purpur\""));
}

/// MC1/AC-02 · 005/AC-06: ein abgelehnter Patch ändert weder Spec noch Ausgabe.
#[tokio::test(flavor = "multi_thread")]
async fn assets_set_abgelehnt() {
    let Some(w) = paper_repo() else { return };
    let mut cs = connect(&w.pfad).await;
    let vorher = hash_baum(&w.pfad, &["public/grafik/figuren"]);
    let args = json!({"id": "monarch", "patch": {"umhang": {"material": "scharlach"}}});
    let text = cs.abgelehnt("assets_set", args).await;
    let want = "abgelehnt, nichts geändert: 1 Fehler\numhang.material · material-unbekannt · Material „scharlach“ fehlt in der Stil-Datei\n\
                Erlaubt: Papier-Materialien und Einzeltöne aus assets_style_get";
    assert!(text.starts_with(want) && text.contains("\nCheckout: "), "{text}");
    assert_eq!(hash_baum(&w.pfad, &["public/grafik/figuren"]), vorher);
}

#[tokio::test(flavor = "multi_thread")]
async fn assets_style() {
    let Some(w) = paper_repo() else { return };
    let mut cs = connect(&w.pfad).await;
    let text = cs.ok("assets_style_get", json!({})).await;
    assert!(
        passt(r"^public/grafik/stil\.paper\.json · stilHash [0-9a-f]{64}\n", &text),
        "{text}"
    );
    assert!(passt(r"Materialien \(\d+\): haut, haar", &text), "{text}");
    enthaelt(
        &text,
        &[
            "Einzeltöne (5): huf, auge",
            "papier: lage 0.5/0.9/0.45/0.55 #1a0f14 · auflage 0.3/0.5/0.3/0.35 · struktur 1.3/2/0.18 · falz 0.25 · licht rechts-oben",
        ],
    );
    let patch = json!({"patch": {"materialien": {"stein": {"basis": "#a8a294"}}}});
    let text = cs.ok("assets_style_set", patch).await;
    let kopf = "Stil geändert: materialien.stein.basis #9a9386 → #a8a294\n";
    assert!(text.starts_with(kopf), "{text}");
    let muster = Regex::new(
        r#"\nBetroffen: 1 Asset \(mauer\) · Render als Job (j-\d+) gestartet\nWeiter: assets_render \{job: "j-\d+", wait: true\}"#,
    )
    .expect("Muster");
    let job = muster.captures(&text).unwrap_or_else(|| panic!("{text}"))[1].to_owned();
    // Der Job rendert im Test-Repo; vor dem Aufräumen auf sein Ende warten.
    let stand = tokio::task::spawn_blocking(move || ASSET_JOBS.wait(&job, Some(Duration::from_secs(60))))
        .await
        .expect("Task")
        .expect("Job");
    assert!(stand.fertig && stand.erledigt == 1 && stand.n == 1, "{stand:?}");
    let stil = "public/grafik/stil.paper.json";
    let vorher = lies(&w.pfad, stil);
    let text = cs
        .abgelehnt("assets_style_set", json!({"patch": {"farbe": "#fff"}}))
        .await;
    assert!(
        text.starts_with("abgelehnt, nichts geändert: 1 Fehler\nstil · schema · "),
        "{text}"
    );
    assert_eq!(lies(&w.pfad, stil), vorher);
    let text = cs.ok("assets_style_set", json!({"patch": {}})).await;
    assert!(
        text.starts_with("Stil unverändert: der Patch ändert kein Feld"),
        "{text}"
    );
}

/// B-710: teile als Objekt nach ID ändert nur das Teil; B-706: `assets_set` auf ein Reittier nennt die Reiter.
#[tokio::test(flavor = "multi_thread")]
async fn assets_set_teil_patch_und_reiter() {
    let Some(w) = paper_repo() else { return };
    let mut cs = connect(&w.pfad).await;
    let args = json!({"id": "monarch", "patch": {"teile": {"kopf": {"pivot": [1, -91]}}}});
    cs.ok("assets_set", args).await;
    let spec = lies(&w.pfad, MONARCH);
    assert!(
        spec.contains("\"pivot\": [1, -91]") && spec.contains("\"id\": \"rumpf\""),
        "{spec}"
    );
    let args = json!({"id": "monarch", "patch": {"teile": {"nase": {}}}});
    let text = cs.abgelehnt("assets_set", args).await;
    enthaelt(&text, &["teile.nase · teil-unbekannt · Teil „nase“ fehlt; vorhanden: "]);
    let args = json!({"id": "pferd", "patch": {"version": 2}});
    let text = cs.ok("assets_set", args).await;
    enthaelt(
        &text,
        &["\nReiter veraltet: monarch · Weiter: assets_render {id} je Reiter"],
    );
}

/// B-870/AC-01: die Fehlerantwort bei Abbruch nennt Grund und bekannte Felder; ohne Abbruch bleibt die Ablehnung.
#[test]
fn set_abbruch_text() {
    let r = SetErgebnis {
        asset: Asset {
            id: "monarch".to_owned(),
            ..Asset::default()
        },
        abbruch: "Render-CLI batch: Zeitlimit 3m0s überschritten".to_owned(),
        felder: vec![FeldAenderung {
            pfad: "umhang.material".to_owned(),
            alt: json!("koenigsrot"),
            neu: json!("purpur"),
        }],
        ..SetErgebnis::default()
    };
    let want = "monarch: Render abgebrochen, nichts geändert · Render-CLI batch: Zeitlimit 3m0s überschritten\n\
                Bekannt vor dem Abbruch: 1 Feld · umhang.material koenigsrot → purpur";
    let fixture = fs::read_to_string(repo_root().join("tools/paper/fixtures/mcp/assets_set-abbruch.txt"));
    let fixture = fixture.expect("Fixture").replace("\r\n", "\n");
    assert!(fixture.starts_with(&format!("{want}\nCheckout: ")), "{fixture}");
    assert_eq!(set_abbruch(&r, &Error::Sonst("x".to_owned())), want);
    let ohne = set_abbruch(&SetErgebnis::default(), &Error::Sonst("anderer".to_owned()));
    assert_eq!(ohne, "anderer");
}

#[tokio::test(flavor = "multi_thread")]
async fn assets_delete() {
    let Some(w) = paper_repo() else { return };
    let root = &w.pfad;
    schreibe(root, "public/grafik/gebaeude/mauer/stand.png", "x");
    let index = r#"[{"id": "mauer", "rig": "gebaeude", "manifest": "mauer/manifest.json"}]"#;
    schreibe(root, "public/grafik/gebaeude/index.paper.json", index);
    let mut cs = connect(root).await;
    let text = cs.ok("assets_delete", json!({"id": "mauer"})).await;
    let hinweis = "nicht gelöscht: assets_delete braucht confirm=true (mauer, 1 PNG)";
    assert!(text.starts_with(hinweis), "{text}");
    let spec = root.join("public/grafik/gebaeude/mauer.paper.json");
    assert!(spec.exists(), "Spec ohne confirm gelöscht");
    let text = cs.ok("assets_delete", json!({"id": "mauer", "confirm": true})).await;
    let want = "mauer gelöscht: public/grafik/gebaeude/mauer.paper.json und public/grafik/gebaeude/mauer/ (1 PNG); Gruppen-Index gebaeude nachgezogen";
    assert!(text.starts_with(want), "{text}");
    assert!(!spec.exists() && !root.join("public/grafik/gebaeude/mauer").exists());
}
