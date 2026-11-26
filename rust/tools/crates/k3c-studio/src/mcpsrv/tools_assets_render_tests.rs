//! Port von `tools_assets_render_test.go`; wie in Rust üblich läuft es immer (V8). Dazu die Texte gegen
//! `tools/paper/fixtures/mcp/`, das Warten und die aus RU70.12 verschobenen Prüfungen (Checkout, `assets_style_set`).

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use k3c_kit::mcp::Progress;
use regex::Regex;
use serde_json::json;

use super::super::checkout_policy::{takes_checkout, write_tool};
use super::super::test_helpers::{connect, enthaelt, paper_repo};
use super::{job_text, render_key, warte};
use crate::paperassets as pa;
use crate::review::test_helpers::repo_root;

fn passt(muster: &str, text: &str) -> bool {
    Regex::new(muster).expect("Muster").is_match(text)
}

fn gruppe<'a>(muster: &str, text: &'a str) -> &'a str {
    let found = Regex::new(muster).expect("Muster").captures(text);
    found
        .unwrap_or_else(|| panic!("{muster}:\n{text}"))
        .get(1)
        .expect("Gruppe")
        .as_str()
}

/// MC1/AC-05: `{alle}` antwortet sofort mit dem Job, `{job, wait}` liefert das Ergebnis.
#[tokio::test(flavor = "multi_thread")]
async fn assets_render() {
    let Some(w) = paper_repo() else { return };
    let mut cs = connect(&w.pfad).await;
    let text = cs.ok("assets_render", json!({"id": "monarch"})).await;
    let eins = r"^monarch gerendert: \d+ PNGs?, manifest\.json · renderKey \S+\n";
    assert!(passt(eins, &text), "{text}");
    enthaelt(&text, &["\nCheckout: "]);
    let text = cs.ok("assets_render", json!({"alle": true})).await;
    let job = gruppe(
        r#"^Job (j-\d+) gestartet: 3 Assets \(alle\)\nWeiter: assets_render \{job: "j-\d+"\} oder \{job: "j-\d+", wait: true\}"#,
        &text,
    )
    .to_owned();
    let (text, _) = cs.call("assets_render", json!({"job": job})).await;
    let stand = format!(r"^Job {job} (läuft \d/3 \(\w+\)|fertig: )");
    assert!(passt(&stand, &text), "{text}");
    let text = cs.ok("assets_render", json!({"job": job, "wait": true})).await;
    let fertig = format!(r"^Job {job} fertig: 3/3 Assets, \d+ PNGs, 0 Fehler, \d+,\d s\n");
    assert!(passt(&fertig, &text), "{text}");
    enthaelt(&text, &["\nmonarch · ", "\npferd · ", "\nmauer · "]);
}

#[tokio::test(flavor = "multi_thread")]
async fn assets_render_fehler() {
    let Some(w) = paper_repo() else { return };
    let mut cs = connect(&w.pfad).await;
    for args in [
        json!({}),
        json!({"id": "monarch", "alle": true}),
        json!({"id": "monarch", "wait": true}),
    ] {
        let text = cs.abgelehnt("assets_render", args).await;
        enthaelt(&text, &["{id}, {alle: true} oder {job"]);
    }
    for args in [json!({"job": "j-999"}), json!({"job": "j-999", "wait": true})] {
        let text = cs.abgelehnt("assets_render", args).await;
        assert!(text.starts_with("Job „j-999“ unbekannt"), "{text}");
    }
    let text = cs.abgelehnt("assets_render", json!({"id": "x"})).await;
    assert!(text.starts_with("Asset „x“ fehlt"), "{text}");
}

/// RU70.12: `assets_render` nimmt `checkout` und nennt ihn als schreibendes Tool.
#[tokio::test(flavor = "multi_thread")]
async fn assets_render_nimmt_checkout() {
    assert!(takes_checkout("assets_render") && write_tool("assets_render"));
    for tool in ["assets_preview", "assets_review", "assets_hits"] {
        assert!(takes_checkout(tool) && !write_tool(tool), "{tool}");
    }
    let Some(w) = paper_repo() else { return };
    let tools = connect(&w.pfad).await.tools().await;
    for name in [
        "assets_render",
        "assets_preview",
        "assets_review",
        "assets_research",
        "assets_hits",
        "assets_hit_add",
        "assets_hit_drop",
        "assets_import",
    ] {
        let tool = tools
            .iter()
            .find(|t| t["name"] == name)
            .unwrap_or_else(|| panic!("{name}"));
        assert!(
            tool["inputSchema"]["properties"].get("checkout").is_some(),
            "{name}: kein checkout im Schema"
        );
    }
}

/// RU70.12: der Job aus `assets_style_set` ist über `assets_render {job, wait}` zu sehen.
#[tokio::test(flavor = "multi_thread")]
async fn assets_style_job_ueber_render() {
    let Some(w) = paper_repo() else { return };
    let mut cs = connect(&w.pfad).await;
    let patch = json!({"patch": {"materialien": {"stein": {"basis": "#a8a294"}}}});
    let text = cs.ok("assets_style_set", patch).await;
    let job = gruppe(r"Render als Job (j-\d+) gestartet", &text).to_owned();
    let text = cs.ok("assets_render", json!({"job": job, "wait": true})).await;
    let fertig = format!(r"^Job {job} fertig: 1/1 Assets, \d+ PNGs, 0 Fehler, \d+,\d s\nmauer · ");
    assert!(passt(&fertig, &text), "{text}");
}

fn fixture(name: &str) -> String {
    let pfad = repo_root().join("tools/paper/fixtures/mcp").join(name);
    let text = std::fs::read_to_string(pfad).expect("Fixture");
    text.replace("\r\n", "\n").trim_end().to_owned()
}

/// Die Texte zu laufendem und fertigem Job wie die Vertrags-Fixtures.
#[test]
fn job_text_wie_vertrag() {
    let ergebnis = |id: &str, pngs: usize| pa::AssetErgebnis {
        id: id.to_owned(),
        pngs,
        manifest: format!("tools/paper/fixtures/render/{id}/manifest.json"),
        ..pa::AssetErgebnis::default()
    };
    let mut st = pa::Stand {
        id: "j-8".to_owned(),
        n: 4,
        erledigt: 2,
        aktuell: "pferd".to_owned(),
        ..pa::Stand::default()
    };
    assert_eq!(job_text(&st), fixture("assets_render-laeuft.txt"));
    st.erledigt = 4;
    st.fertig = true;
    st.dauer = Duration::from_millis(3200);
    st.ergebnisse = vec![
        ergebnis("monarch", 3),
        ergebnis("pferd", 3),
        ergebnis("mauer", 3 - 1),
        pa::AssetErgebnis {
            id: "banner".to_owned(),
            uebersprungen: "material-unbekannt".to_owned(),
            ..pa::AssetErgebnis::default()
        },
    ];
    // Die Fixture zählt 9 PNGs bei 3 + 3 + 2 in den Zeilen; hier zählt die Summe der Ergebnisse.
    let want = fixture("assets_render-wait.txt").replacen("9 PNGs", "8 PNGs", 1);
    assert_eq!(job_text(&st), want);
    st.ergebnisse = vec![pa::AssetErgebnis {
        id: "fass".to_owned(),
        fehler: "bun fehlt".to_owned(),
        ..pa::AssetErgebnis::default()
    }];
    assert!(job_text(&st).ends_with("0 PNGs, 1 Fehler, 3,2 s\nfass · Fehler: bun fehlt"));
}

/// Ohne `progressToken` endet das Warten an der Grenze; mit ihm kommen Lebenszeichen bis zum Ende.
#[tokio::test(flavor = "multi_thread")]
async fn warten_mit_und_ohne_fortschritt() {
    let jobs: &'static pa::Jobs = Box::leak(Box::new({
        let mut jobs = pa::Jobs::new();
        jobs.render = Arc::new(|_, a| {
            thread::sleep(Duration::from_millis(600));
            pa::AssetErgebnis {
                id: a.id.clone(),
                ..pa::AssetErgebnis::default()
            }
        });
        jobs
    }));
    let dir = crate::test_helpers::tmp_dir();
    let asset = pa::Asset {
        id: "monarch".to_owned(),
        ..pa::Asset::default()
    };
    let cli = pa::Cli::new(dir.path());
    let job = jobs.start(&cli, std::slice::from_ref(&asset)).id;
    let kurz = Duration::from_millis(50);
    let text = warte(jobs, job.clone(), Progress::none(), kurz, kurz)
        .await
        .expect("Text");
    assert_eq!(text, format!("Job {job} läuft 0/1 (monarch)"));

    let meldungen = Arc::new(Mutex::new(Vec::new()));
    let sammle = Arc::clone(&meldungen);
    let progress = Progress::from_fn(Arc::new(move |msg: &str, _| {
        sammle.lock().expect("Meldungen").push(msg.to_owned());
    }));
    let text = warte(jobs, job.clone(), progress, kurz, kurz).await.expect("Text");
    assert!(text.starts_with(&format!("Job {job} fertig: 1/1 Assets, ")), "{text}");
    let meldungen = meldungen.lock().expect("Meldungen");
    assert_eq!(
        meldungen.first(),
        Some(&format!("Job {job} 0/1 (monarch)")),
        "{meldungen:?}"
    );
}

#[test]
fn render_key_oder_strich() {
    let dir = crate::test_helpers::tmp_dir();
    std::fs::write(dir.path().join("manifest.json"), r#"{"renderKey": "abc"}"#).expect("schreiben");
    assert_eq!(render_key(dir.path(), "manifest.json"), "abc");
    assert_eq!(render_key(dir.path(), "fehlt.json"), "–");
    std::fs::write(dir.path().join("leer.json"), "{}").expect("schreiben");
    assert_eq!(render_key(dir.path(), "leer.json"), "–");
}
