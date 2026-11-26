use std::fs;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use axum::Router;
use axum::http::{Method as HttpMethod, StatusCode as HttpStatus, Uri, header};
use axum::response::IntoResponse;

use super::*;
use crate::credits::LICENSE_CONFIRMED;
use crate::net::{self, test_download_client};
use crate::register::{KIND_GRAPHIC, KIND_SOUND};
use crate::research::{Hit, LICENSE_CHECK, LICENSE_KNOWN, add_hits};
use crate::research_add::plan_research_import;
use crate::suggestions::load_suggestions;
use crate::testutil::{day, repo, serve, write_file};

/// Mini-Repo (Slots farm, sfx:coin) ohne Vorschlagsliste; der Testserver liefert nur HEAD-Größen.
async fn add_repo() -> (tempfile::TempDir, String, SizeFunc) {
    let dir = repo();
    let router = Router::new().fallback(|method: HttpMethod, uri: Uri| async move {
        if method != HttpMethod::HEAD || uri.path().ends_with("missing.png") {
            return HttpStatus::NOT_FOUND.into_response();
        }
        ([(header::CONTENT_LENGTH, "1234")], ()).into_response()
    });
    let base = serve(router).await;
    net::testing::allow(&base);
    (dir, base, head_size(test_download_client()))
}

fn cand(base: &str, file: &str, slot: &str, kind: &str, license: &str) -> Suggestion {
    Suggestion {
        slot: slot.to_string(),
        kind: kind.to_string(),
        title: format!("Titel {file}"),
        source: "Kenney".to_string(),
        page: "https://kenney.nl/assets/x".to_string(),
        file_url: format!("{base}/{file}"),
        license: license.to_string(),
        ..Suggestion::default()
    }
}

/// B-673/AC-05: ungültige Einträge (Lizenz, Quelle, Slot, Endung, Art, doppelte URL) stehen mit Grund in `skipped`,
/// gültige werden mit neuer ID und Größe per HEAD in einem Schritt geschrieben.
#[tokio::test]
async fn add_suggestions() {
    let (dir, base, size) = add_repo().await;
    let root = dir.path();
    let mut foreign = cand(&base, "q.png", "farm", KIND_GRAPHIC, "CC0");
    foreign.source = "Wikimedia Commons".to_string();
    let input = [
        cand(&base, "ok.png", "farm", KIND_GRAPHIC, "CC0"),
        cand(&base, "ok.png", "farm", KIND_GRAPHIC, "CC0"),
        foreign,
        cand(&base, "nc.png", "farm", KIND_GRAPHIC, "CC-BY-NC 4.0"),
        cand(&base, "slot.png", "weg", KIND_GRAPHIC, "CC0"),
        cand(&base, "x.exe", "farm", KIND_GRAPHIC, "CC0"),
        cand(&base, "art.png", "farm", KIND_SOUND, "CC0"),
        cand(&base, "missing.png", "farm", KIND_GRAPHIC, "CC0"),
    ];
    let result = super::add_suggestions(root, &input, Some(&size))
        .await
        .expect("anhängen");
    assert!(
        result.added.len() == 1
            && result.added[0].id == "v001"
            && result.added[0].size == 1234
            && result.added[0].status == "offen"
            && result.bytes == 1234,
        "übernommen: {result:?}"
    );
    assert_eq!(result.skipped.len(), 7, "übersprungen: {:?}", result.skipped);
    want_reasons(
        &result.skipped,
        &[
            "schon in der Liste",
            "Quelle nicht erlaubt",
            "nicht erlaubt (Q70)",
            "nicht im Register",
            "endung",
            "passt nicht",
            "Größe nicht ermittelbar: HEAD: HTTP 404",
        ],
    );
    let list = load_suggestions(root).expect("Liste");
    assert_eq!(list.list.len(), 1, "geschrieben: {:?}", list.list);
    second_run(root, &base, &size).await;
}

/// Ein zweiter Lauf vergibt die nächste ID und lehnt die Datei-URL als doppelt ab.
async fn second_run(root: &Path, base: &str, size: &SizeFunc) {
    let input = [
        cand(base, "ok.png", "farm", KIND_GRAPHIC, "CC0"),
        cand(base, "two.png", "sfx:coin", KIND_SOUND, "CC0"),
    ];
    // `two.png` an einem Ton-Slot geht durch: die Prüfung kennt nur die Endungen des Vorrats.
    let again = super::add_suggestions(root, &input, Some(size))
        .await
        .expect("zweiter Lauf");
    assert!(
        again.added.len() == 1 && again.added[0].id == "v002" && again.skipped.len() == 1,
        "zweiter Lauf: {again:?}"
    );
    let left: Vec<String> = fs::read_dir(root.join("data"))
        .expect("data")
        .map(|e| e.expect("Eintrag").file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with(".assets-"))
        .collect();
    assert!(left.is_empty(), "Zwischendateien bleiben: {left:?}");
}

/// Jeder Teiltext kommt in einem der Gründe vor.
fn want_reasons(skipped: &[SkippedSuggestion], want: &[&str]) {
    let text: String = skipped.iter().map(|s| format!("{}|", s.reason)).collect();
    for w in want {
        assert!(text.contains(w), "Grund {w:?} fehlt in {text}");
    }
}

/// Ändert sich die Liste während der Größenabfrage, bricht das Anhängen ab und schreibt nichts (B-673, Fehlerfälle).
#[tokio::test]
async fn add_suggestions_changed_meanwhile() {
    let (dir, base, _) = add_repo().await;
    let path = dir.path().join(SUGGESTIONS_PATH);
    let target = path.clone();
    let size: SizeFunc = Arc::new(move |_| {
        write_file(&target, r#"{"_comment":"extern","vorschlaege":[]}"#);
        Box::pin(async { Ok(10) })
    });
    let input = [cand(&base, "a.png", "farm", KIND_GRAPHIC, "CC0")];
    let err = super::add_suggestions(dir.path(), &input, Some(&size))
        .await
        .expect_err("geändert");
    assert_eq!(err.to_string(), ERR_SUGGESTIONS_CHANGED);
    let text = fs::read_to_string(path).expect("lesen");
    assert!(text.contains("extern"), "Datei überschrieben: {text}");
}

/// Legt Recherche-Treffer zu farm an (Datei-URLs auf dem Testserver).
fn seed_hits(root: &Path, base: &str) {
    let hit = |title: &str, source: &str, file: &str, license: &str, status: &str| Hit {
        title: title.to_string(),
        link: format!("https://example.org/{title}"),
        source: source.to_string(),
        license: license.to_string(),
        artist: "Anna".to_string(),
        license_status: status.to_string(),
        download: if file.is_empty() {
            String::new()
        } else {
            format!("{base}/{file}")
        },
        ..Hit::default()
    };
    for h in [
        hit("gut", "Kenney", "gut.png", "CC0", LICENSE_KNOWN),
        hit("bestaetigt", "OpenGameArt", "b.png", "CC BY 4.0", LICENSE_CONFIRMED),
        hit("ohne", "Kenney", "", "CC0", LICENSE_KNOWN),
        hit("wiki", "Wikimedia Commons", "w.png", "CC0", LICENSE_KNOWN),
        hit("pruefen", "OpenGameArt", "p.png", "", LICENSE_CHECK),
        hit("nc", "OpenGameArt", "n.png", "CC-BY-NC 3.0", LICENSE_KNOWN),
    ] {
        add_hits(root, "farm", &[h], day()).expect("add_hits");
    }
}

/// B-673/AC-04: Aus der Recherche nur Treffer mit Datei-URL, erlaubter Quelle und Lizenz erkannt/bestätigt; der Plan
/// schreibt nichts, die Übernahme schreibt ohne Duplikate, nur die bestätigten URLs.
#[tokio::test]
async fn import_research() {
    let (dir, base, size) = add_repo().await;
    let root = dir.path();
    seed_hits(root, &base);
    let plan = plan_research_import(root, "", Some(&size)).await.expect("Plan");
    assert!(
        plan.added.len() == 2 && plan.skipped.len() == 4 && plan.bytes == 2468,
        "Plan: {plan:?}"
    );
    assert!(!root.join(SUGGESTIONS_PATH).exists(), "Plan hat geschrieben");
    let filtered = plan_research_import(root, "Kenney", Some(&size)).await.expect("Filter");
    assert!(
        filtered.added.len() == 1 && filtered.added[0].license == "CC0",
        "Filter Quelle: {filtered:?}"
    );
    let only = [format!("{base}/gut.png")];
    let result = crate::research_add::import_research(root, "", Some(&only), Some(&size))
        .await
        .expect("Übernahme");
    assert_eq!(result.added.len(), 1, "Übernahme: {result:?}");
    let again = crate::research_add::import_research(root, "", None, Some(&size))
        .await
        .expect("zweite Übernahme");
    assert!(
        again.added.len() == 1 && again.added[0].license == "CC-BY 4.0" && again.added[0].id == "v002",
        "zweite Übernahme ohne Duplikat: {again:?}"
    );
    want_reasons(
        &again.skipped,
        &[
            "keine direkte Datei-URL",
            "Quelle nicht erlaubt",
            "Lizenz nicht geklärt",
            "nicht erlaubt (Q70)",
            "schon in der Liste",
        ],
    );
}

/// Nur Rust: HEAD folgt keiner Weiterleitung auf ein fremdes Ziel und fragt keine fremde Quelle.
#[tokio::test]
async fn head_size_prueft_ziel_und_weiterleitung() {
    let followed = Arc::new(AtomicUsize::new(0));
    let location = Arc::new(Mutex::new("https://192.168.0.5/x.png".to_string()));
    let (count, target) = (followed.clone(), location.clone());
    let router = Router::new().fallback(move || async move {
        count.fetch_add(1, Ordering::SeqCst);
        let to = target.lock().expect("Ziel").clone();
        (HttpStatus::FOUND, [(header::LOCATION, to)])
    });
    let base = serve(router).await;
    net::testing::allow(&base);
    let size = head_size(test_download_client());
    let err = size(format!("{base}/a.png")).await.expect_err("Weiterleitung");
    assert!(err.to_string().contains("keine erlaubte Quelle"), "{err}");
    assert_eq!(followed.load(Ordering::SeqCst), 1);
    *location.lock().expect("Ziel") = "http://opengameart.org/x.png".to_string();
    let err = size(format!("{base}/a.png")).await.expect_err("Klartext");
    assert!(err.to_string().contains("nur über https"), "{err}");
    let err = size("https://example.org/a.png".to_string()).await.expect_err("fremd");
    assert!(err.to_string().contains("keine erlaubte Quelle"), "{err}");
}
