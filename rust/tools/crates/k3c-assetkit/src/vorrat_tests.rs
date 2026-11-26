use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicI64, Ordering};

use axum::Router;
use axum::http::{StatusCode, header};
use axum::routing::any;
use walkdir::WalkDir;

use super::*;
use crate::catalog::{catalog, search};
use crate::net::{self, test_download_client};
use crate::register::load;
use crate::testutil::{VorratRepo, day, ogg_data, serve, suggestion, vorrat_repo};
use crate::vorrat::{VORRAT_LOADED, VORRAT_OPEN, vorrat_states};
use crate::vorrat_adopt::{vorrat_adopt, vorrat_cleanup, vorrat_cleanup_preview};

async fn download(repo: &VorratRepo, id: &str, progress: Option<Progress>) -> Result<VorratEntry> {
    let s = suggestion(repo.root(), id);
    super::vorrat_download(repo.root(), &s, &repo.client, day(), progress).await
}

fn work_files(root: &Path) -> Vec<PathBuf> {
    WalkDir::new(root.join(".work"))
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(walkdir::DirEntry::into_path)
        .collect()
}

/// B-601/AC-02: Laden in den Vorrat mit Index (Quelle, Lizenz, Größe, Prüfsumme); erneutes Laden ist ein No-op.
#[tokio::test]
async fn vorrat_download() {
    let repo = vorrat_repo().await;
    let last = Arc::new(AtomicI64::new(0));
    let seen = last.clone();
    let progress: Progress = Arc::new(move |n| seen.store(n, Ordering::SeqCst));
    let entry = download(&repo, "v1", Some(progress)).await.expect("laden");
    let size = ogg_data().len() as i64;
    assert!(
        entry.file == "audio/opengameart/v1-coin.ogg"
            && entry.size == size
            && entry.sha256.len() == 64
            && entry.license == "CC0",
        "Eintrag {entry:?}"
    );
    assert_eq!(last.load(Ordering::SeqCst), size, "Fortschritt");
    let index = load_vorrat(repo.root()).expect("Index");
    let entries = index.entries();
    assert!(
        entries.len() == 1 && entries[0].source == "OpenGameArt" && entries[0].loaded == "2026-10-14T09:00:00Z",
        "Index {entries:?}"
    );
    download(&repo, "v1", None).await.expect("zweites Laden");
    assert_eq!(repo.hits.load(Ordering::SeqCst), 1, "Anfragen");
    let list = ["v1", "v3", "v4"].map(|id| suggestion(repo.root(), id));
    let states = vorrat_states(repo.root(), &list).expect("Status");
    assert!(
        states["v1"].status == VORRAT_LOADED
            && states["v3"].status == VORRAT_OPEN
            && states["v4"].status == VORRAT_BLOCKED,
        "Status {states:?}"
    );
}

/// Falsche Signatur, falsche Größe, HTTP-Fehler und verbotene Lizenz lassen nichts im Vorrat zurück.
#[tokio::test]
async fn vorrat_download_fehler() {
    let repo = vorrat_repo().await;
    for id in ["v3", "v4", "v5", "v6"] {
        assert!(download(&repo, id, None).await.is_err(), "{id}: Fehler erwartet");
    }
    assert_eq!(repo.hits.load(Ordering::SeqCst), 3, "gesperrte Lizenz darf nicht laden");
    let files = work_files(repo.root());
    assert!(files.is_empty(), "Reste im Vorrat: {files:?}");
    let mut bad = suggestion(repo.root(), "v1");
    // Der Testserver spricht selbst http (V7): http-Ablehnung an einem erlaubten Host.
    bad.file_url = "http://opengameart.org/coin.ogg".to_string();
    let err = check_vorrat_download(&bad).expect_err("http soll abgelehnt werden");
    assert!(err.to_string().contains("nur über https"), "{err}");
    (bad.file_url, bad.file_name) = ("https://x/a.gif".to_string(), "a.gif".to_string());
    assert!(check_vorrat_download(&bad).is_err(), "gif soll abgelehnt werden");
}

/// B-601/AC-02: Übernahme kopiert nach `public/`, legt Credit (und damit den Katalog-Eintrag) und die Zuordnung an;
/// Pakete verlangen die Auswahl von Hand; Aufräumen entfernt nur Unverwendetes.
#[tokio::test]
async fn vorrat_adopt_und_aufraeumen() {
    let repo = vorrat_repo().await;
    let root = repo.root();
    for id in ["v1", "v2"] {
        download(&repo, id, None).await.expect(id);
    }
    let err = vorrat_adopt(root, "v2", day()).expect_err("Paket");
    assert!(err.to_string().contains("von Hand"), "{err}");
    let rel = vorrat_adopt(root, "v1", day()).expect("Übernahme");
    assert_eq!(rel, "audio/kandidaten/muenze-v1.ogg");
    assert_eq!(vorrat_adopt(root, "v1", day()).expect("zweite Übernahme"), rel);
    let credits = fs::read_to_string(root.join("public").join("audio").join("README.md")).expect("Credits");
    let row = "| kandidaten/muenze-v1.ogg | Münze v1 | Anna | CC0 | https://opengameart.org/content/v1 |";
    assert!(credits.contains(row), "Credit:\n{credits}");
    let assets = catalog(root).expect("Katalog");
    let found = search(&assets, "muenze-v1", KIND_SOUND, 0);
    assert!(found.len() == 1 && found[0].license == "CC0", "Katalog {found:?}");
    let register = load(root).expect("load");
    let choices = register.find("sfx:coin").expect("Slot").choices();
    assert!(choices.len() == 1 && choices[0].file == rel, "Zuordnung {choices:?}");
    check_cleanup(root);
}

/// v1 ist übernommen und zugeordnet, v2 nur geladen; das Aufräumen entfernt nur v2.
fn check_cleanup(root: &Path) {
    let preview = vorrat_cleanup_preview(root).expect("Vorschau");
    assert!(preview.len() == 1 && preview[0].id == "v2", "Vorschau {preview:?}");
    let removed = vorrat_cleanup(root).expect("Aufräumen");
    assert_eq!(removed.len(), 1, "{removed:?}");
    let list = ["v1", "v2"].map(|id| suggestion(root, id));
    let states = vorrat_states(root, &list).expect("Status");
    assert!(
        states["v2"].status == VORRAT_OPEN && states["v1"].status == VORRAT_LOADED && states["v1"].used,
        "Status nach Aufräumen {states:?}"
    );
}

/// Fremde Hosts, Klartext und Umleitungen auf nicht erlaubte Ziele lädt der Vorrat nicht (SSRF, Review WZ29.6).
#[test]
fn vorrat_nur_erlaubte_hosts() {
    let parse = |raw: &str| Url::parse(raw).expect(raw);
    for raw in [
        "https://127.0.0.1/a.png",
        "http://opengameart.org/a.png",
        "https://evil.example/a.png",
        "https://opengameart.org.evil.example/a.png",
        "https://evilopengameart.org/a.png",
        "https://opengameart.org@evil.example/a.png",
    ] {
        assert!(vorrat_url_allowed(&parse(raw)).is_err(), "{raw} erlaubt");
    }
    for raw in [
        "https://opengameart.org/sites/default/files/a.png",
        "https://cdn.freesound.org/previews/a.mp3",
    ] {
        vorrat_url_allowed(&parse(raw)).expect(raw);
    }
    assert!(
        vorrat_redirect(&parse("https://192.168.1.10/x"), 0).is_err(),
        "Umleitung ins Heimnetz erlaubt"
    );
    let ok = parse("https://kenney.nl/x");
    assert!(vorrat_redirect(&ok, 4).is_ok() && vorrat_redirect(&ok, 5).is_err());
}

#[tokio::test]
async fn vorrat_download_umleitung_und_zu_gross_lassen_nichts_zurueck() {
    let repo = vorrat_repo().await;
    let away = || async { (StatusCode::FOUND, [(header::LOCATION, "https://192.168.0.5/x.ogg")]) };
    let big = || async { vec![b'x'; MAX_SUGGESTION_BYTES as usize + 1] };
    let base = serve(Router::new().route("/weg.ogg", any(away)).route("/big.ogg", any(big))).await;
    net::testing::allow(&base);
    for (file, want) in [("weg.ogg", "keine erlaubte Quelle"), ("big.ogg", "größer als 50 MB")] {
        let mut s = suggestion(repo.root(), "v1");
        s.file_url = format!("{base}/{file}");
        let err = super::vorrat_download(repo.root(), &s, &test_download_client(), day(), None)
            .await
            .expect_err(want);
        assert!(err.to_string().contains(want), "{err}");
        let files = work_files(repo.root());
        assert!(files.is_empty(), "Reste: {files:?}");
    }
}

/// Felder der Liste (ID, Quelle, Dateiname) werden nie zu Pfaden außerhalb des Vorrats.
#[test]
fn vorrat_ziel_bleibt_im_vorrat() {
    let s = Suggestion {
        id: "..\\..\\v:1".to_string(),
        source: "../..".to_string(),
        kind: KIND_SOUND.to_string(),
        file_name: "../../x/..\\evil.. .ogg".to_string(),
        ..Suggestion::default()
    };
    assert_eq!(target_rel(&s, ".ogg"), "audio/import/v-1-evil.ogg");
}

/// Byte-Form des Index wie Go `json.MarshalIndent` (HTML maskiert, `nil` als `null`).
#[tokio::test]
async fn vorrat_index_wie_go() {
    let repo = vorrat_repo().await;
    let index = repo.root().join(".work").join("vorrat").join("index.json");
    vorrat_cleanup(repo.root()).expect("Aufräumen ohne Index");
    let text = fs::read_to_string(&index).expect("Index");
    assert!(
        text.starts_with("{\n  \"_comment\": \"Vorrat der Vorschlagsliste (B-601): geladene Dateien mit Quelle, ")
            && text.contains("SHA-256 und Ladezeit. Nur lokal, nie in Git.\",\n  \"eintraege\": null\n}\n"),
        "{text}"
    );
    download(&repo, "v1", None).await.expect("laden");
    let entry = load_vorrat(repo.root()).expect("Index").entries()[0].clone();
    let want = format!(
        "  \"eintraege\": [\n    {{\n      \"id\": \"v1\",\n      \"quelle\": \"OpenGameArt\",\n      \
         \"lizenz\": \"CC0\",\n      \"groesse\": 64,\n      \"pruefsumme\": \"{}\",\n      \
         \"geladen\": \"2026-10-14T09:00:00Z\",\n      \"datei\": \"audio/opengameart/v1-coin.ogg\"\n    }}\n  ]\n}}\n",
        entry.sha256
    );
    let text = fs::read_to_string(&index).expect("Index");
    assert!(text.ends_with(&want), "{text}");
}

/// Die Downloads laufen auch auf einer Laufzeit mit mehreren Threads (V11).
#[tokio::test]
async fn download_futures_sind_send() {
    fn assert_send<T: Send>(_: T) {}
    let repo = vorrat_repo().await;
    let s = suggestion(repo.root(), "v1");
    assert_send(super::vorrat_download(repo.root(), &s, &repo.client, day(), None));
    assert_send(crate::import_download::download_hit(
        repo.root(),
        "farm",
        "x",
        &repo.client,
        day(),
    ));
}
