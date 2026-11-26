use std::fs;
use std::path::Path;

use axum::Router;
use axum::http::{StatusCode, header};
use axum::routing::{any, get};
use url::Url;

use super::*;
use crate::catalog::catalog;
use crate::credits::{IMPORT_SECTION, LICENSE_CONFIRMED, confirm_license};
use crate::import_download::{self, download_allowed};
use crate::net::{self, test_download_client};
use crate::research::add_hits;
use crate::testutil::{day, import_repo, png_bytes, serve, src_file};

fn credits_of(root: &Path) -> String {
    fs::read_to_string(root.join("public").join("grafik").join("CREDITS.md")).expect("Credits")
}

/// B-298/AC-05: Ablage, Credit-Eintrag und „Lizenz prüfen“; Bestätigen setzt den Status in Treffer und Credit.
#[test]
fn import_manuell() {
    let (dir, itch, _) = import_repo();
    let root = dir.path();
    let (_src, src) = src_file("download.PNG", &png_bytes(16, 16));
    let rel = import_file(root, "farm", &itch, &src, day()).expect("Import");
    assert_eq!(rel, "grafik/kandidaten/farm-haeuschen.png");
    let credits = credits_of(root);
    let row = "| kandidaten/farm-haeuschen.png | ../../Farm Häuschen | Anna | unbekannt | https://x.itch.io/farm | \
               Lizenz prüfen |";
    assert!(
        credits.contains(IMPORT_SECTION) && credits.contains(row),
        "Credit:\n{credits}"
    );
    let register = load(root).expect("load");
    let choices = register.find("farm").expect("farm").choices();
    assert!(choices.len() == 1 && choices[0].file == rel, "Zuordnung: {choices:?}");
    confirm_license(root, "farm", &itch, day()).expect("bestätigen");
    let credits = credits_of(root);
    let hit = &hits_of(root, "farm").expect("hits")[0];
    assert!(
        credits.contains("| bestätigt 🧑 2026-10-14 |") && hit.license_status == LICENSE_CONFIRMED && hit.file == rel,
        "nach Bestätigen: {hit:?}\n{credits}"
    );
    for asset in catalog(root).expect("Katalog") {
        if asset.file == rel {
            assert_eq!(asset.artist, "Anna", "Katalog nutzt die Credit-Zeile der Datei nicht");
        }
    }
}

/// Vorhandene Datei bleibt, Pfadanteile im Titel landen nie außerhalb von `kandidaten/`.
#[test]
fn import_zweimal_und_pfad() {
    let (dir, itch, _) = import_repo();
    let root = dir.path();
    let (_src, src) = src_file("a.png", &png_bytes(16, 16));
    let first = import_file(root, "farm", &itch, &src, day()).expect("erster Import");
    let second = import_file(root, "farm", &itch, &src, day()).expect("zweiter Import");
    assert_ne!(first, second);
    assert_eq!(second, "grafik/kandidaten/farm-haeuschen-2.png");
    assert!(
        !root.join("public").join("farm-haeuschen.png").exists(),
        "Datei außerhalb von kandidaten/"
    );
}

#[test]
fn import_abgelehnt() {
    let (dir, itch, pixabay) = import_repo();
    let png = png_bytes(16, 16);
    let mut big = b"\x89PNG\r\n\x1a\n".to_vec();
    big.resize(big.len() + MAX_IMPORT_BYTES, 0);
    let svg = b"<svg><script>alert(1)</script></svg>".to_vec();
    let cases: [(&str, &str, &str, &[u8]); 6] = [
        ("nicht übernehmbar", &pixabay, "a.png", &png),
        ("erlaubt sind .png", &itch, "a.jpg", &png),
        ("a.svg: erlaubt sind .png", &itch, "a.svg", &svg), // B-514: kein SVG importierbar
        ("inhalt ist kein png", &itch, "a.png", b"geheim"),
        ("größer als 20 MB", &itch, "big.png", &big),
        ("nicht gefunden", "nichts", "a.png", &png),
    ];
    for (want, hit, name, data) in cases {
        let (_src, src) = src_file(name, data);
        let err = import_file(dir.path(), "farm", hit, &src, day()).expect_err(want);
        assert!(err.to_string().contains(want), "{want}: {err}");
    }
    assert!(!dir.path().join("public").join("grafik").join("kandidaten").exists());
}

fn hit_with_download(root: &Path, title: &str, license: &str, source: &str, download: String) -> String {
    let link = format!("https://commons.wikimedia.org/{title}");
    let hit = Hit {
        title: title.to_string(),
        link: link.clone(),
        source: source.to_string(),
        license: license.to_string(),
        download,
        ..Hit::default()
    };
    add_hits(root, "farm", &[hit], day()).expect("add_hits");
    let hits = hits_of(root, "farm").expect("hits");
    hits.into_iter().find(|h| h.link == link).expect("Treffer").id
}

async fn png_server() -> String {
    serve(Router::new().fallback(get(|| async { png_bytes(16, 16) }))).await
}

fn no_leftovers(root: &Path) {
    let count = |dir: &Path| fs::read_dir(dir).map_or(0, Iterator::count);
    let kandidaten = root.join("public").join("grafik").join("kandidaten");
    assert_eq!(count(&kandidaten), 0, "Datei entstanden");
    assert_eq!(count(&root.join(".work").join("tmp")), 0, "Temp-Datei geblieben");
}

/// Direkte Übernahme nur bei https und erkannter Lizenz.
#[tokio::test]
async fn download_hit() {
    let (dir, itch, _) = import_repo();
    let root = dir.path();
    let base = png_server().await;
    let client = test_download_client();
    // Wie Go (TLS-Testserver): https auf einem fremden Host, noch nicht freigegeben.
    let https = base.replace("http:", "https:");
    let intern = hit_with_download(root, "Intern", "CC0 1.0", "Agent", format!("{https}/intern.png"));
    let err = import_download::download_hit(root, "farm", &intern, &client, day())
        .await
        .expect_err("fremder Host");
    assert!(
        err.to_string().contains("download nur von upload.wikimedia.org"),
        "{err}"
    );
    net::testing::allow(&base);
    let url = format!("{base}/coin.png");
    let coin = hit_with_download(root, "Coin video game.png", "CC BY-SA 3.0", "Wikimedia Commons", url);
    let rel = import_download::download_hit(root, "farm", &coin, &client, day())
        .await
        .expect("Download");
    assert_eq!(rel, "grafik/kandidaten/coin-video-game-png.png");
    let credits = credits_of(root);
    let want =
        "| CC BY-SA 3.0 | https://commons.wikimedia.org/Coin video game.png | Lizenz erkannt (Wikimedia Commons) |";
    assert!(credits.contains(want), "Credit:\n{credits}");
    let err = import_download::download_hit(root, "farm", &itch, &client, day())
        .await
        .expect_err("ohne Download");
    assert!(err.to_string().contains("manuell"), "{err}");
    assert_eq!(fs::read_dir(root.join(".work").join("tmp")).expect("tmp").count(), 0);
}

/// B-678 AC-03: Ein Treffer-Download von einem erlaubten Host folgt Weiterleitungen.
#[tokio::test]
async fn download_hit_folgt_weiterleitung() {
    let (dir, _, _) = import_repo();
    let redirect = || async { (StatusCode::FOUND, [(header::LOCATION, "/neu.png")]) };
    let router = Router::new()
        .route("/alt.png", get(redirect))
        .route("/neu.png", get(|| async { png_bytes(16, 16) }));
    let base = serve(router).await;
    net::testing::allow(&base);
    let url = format!("{base}/alt.png");
    let id = hit_with_download(dir.path(), "Weiter.png", "CC0 1.0", "Wikimedia Commons", url);
    import_download::download_hit(dir.path(), "farm", &id, &test_download_client(), day())
        .await
        .expect("Weiterleitung");
}

async fn redirect_server(target: &'static str) -> String {
    let redirect = move || async move { (StatusCode::FOUND, [(header::LOCATION, target)]) };
    let base = serve(Router::new().fallback(any(redirect))).await;
    net::testing::allow(&base);
    base
}

/// B-688 AC-01: Weiterleitung auf einen nicht erlaubten Host oder eine lokale Adresse bricht ab, es entsteht keine Datei.
#[tokio::test]
async fn download_hit_weiterleitung_auf_fremden_host_abgelehnt() {
    for target in [
        "https://localhost:1/x.png",
        "https://192.168.0.5/x.png",
        "http://127.0.0.1/x.png",
        "//169.254.169.254/x.png",
        "https://upload.wikimedia.org.evil.example/x.png",
        "https://user:pw@upload.wikimedia.org/x.png",
        "https://@upload.wikimedia.org/x.png",
        "https://upload.wikimedia.org:0443/x.png",
        "http://upload.wikimedia.org/x.png",
        "file:///c:/windows/win.ini",
    ] {
        let (dir, _, _) = import_repo();
        let url = format!("{}/alt.png", redirect_server(target).await);
        let id = hit_with_download(dir.path(), "Weg.png", "CC0 1.0", "Wikimedia Commons", url);
        let err = import_download::download_hit(dir.path(), "farm", &id, &test_download_client(), day())
            .await
            .expect_err(target);
        assert!(err.to_string().contains("abgelehnt"), "{target}: {err}");
        no_leftovers(dir.path());
    }
}

/// Wie Go (`len(via) >= 5`): vier Sprünge gehen, der fünfte nicht.
#[tokio::test]
async fn download_hit_hoechstens_vier_spruenge() {
    let (dir, _, _) = import_repo();
    let base = redirect_server("/wieder.png").await;
    let id = hit_with_download(
        dir.path(),
        "Kreis",
        "CC0 1.0",
        "Wikimedia Commons",
        format!("{base}/a.png"),
    );
    let err = import_download::download_hit(dir.path(), "farm", &id, &test_download_client(), day())
        .await
        .expect_err("Kreis");
    assert!(err.to_string().contains("zu viele Weiterleitungen"), "{err}");
    no_leftovers(dir.path());
}

#[tokio::test]
async fn download_hit_zu_gross_und_http_fehler_lassen_nichts_zurueck() {
    let (dir, _, _) = import_repo();
    let big = || async {
        let mut data = b"\x89PNG\r\n\x1a\n".to_vec();
        data.resize(MAX_IMPORT_BYTES + 100, 0);
        data
    };
    let base = serve(Router::new().route("/big.png", get(big))).await;
    net::testing::allow(&base);
    for (title, path, want) in [
        ("Groß", "big.png", "größer als 20 MB"),
        ("Weg", "weg.png", "download: HTTP 404"),
    ] {
        let url = format!("{base}/{path}");
        let id = hit_with_download(dir.path(), title, "CC0 1.0", "Wikimedia Commons", url);
        let err = import_download::download_hit(dir.path(), "farm", &id, &test_download_client(), day())
            .await
            .expect_err(want);
        assert!(err.to_string().contains(want), "{err}");
        no_leftovers(dir.path());
    }
}

/// Endungen aus Netzdaten: `\`, `:` und Pfadanteile gelangen nie in den Namen der Temp-Datei.
#[tokio::test]
async fn download_hit_boese_endung() {
    let (dir, _, _) = import_repo();
    let base = png_server().await;
    net::testing::allow(&base);
    for path in [
        "a.%5C..%5C..%5Cx",
        "a.png%3Astrom",
        "a.png%2F..%2F..%2Fx.",
        "a.png.",
        "con",
    ] {
        let url = format!("{base}/{path}");
        let id = hit_with_download(dir.path(), path, "CC0 1.0", "Wikimedia Commons", url);
        let err = import_download::download_hit(dir.path(), "farm", &id, &test_download_client(), day())
            .await
            .expect_err(path);
        assert!(err.to_string().contains("erlaubt sind .png"), "{path}: {err}");
        no_leftovers(dir.path());
    }
}

/// B-694: Fremder Port und Zugangsdaten in der URL gelten auch auf einem erlaubten Host nicht.
#[test]
fn download_allowed_port_und_userinfo() {
    for (raw, want) in [
        ("https://upload.wikimedia.org/a.png", true),
        ("https://upload.wikimedia.org:443/a.png", true),
        ("https://upload.wikimedia.org:8443/a.png", false),
        ("https://user:pw@upload.wikimedia.org/a.png", false),
        ("https://localhost/a.png", false),
    ] {
        let url = Url::parse(raw).expect("URL");
        assert_eq!(download_allowed(&url), want, "{raw}");
    }
}

/// Dateinamen aus Titeln: nur a–z, 0–9 und `-`; Gerätenamen von Windows bleiben Dateien im Zielordner.
#[test]
fn slug_und_geraetenamen() {
    assert_eq!(slug("../../Farm Häuschen"), "farm-haeuschen");
    assert_eq!(slug("C:\\x\\..\\y. "), "c-x-y");
    assert_eq!(slug("İa ẞ"), "ia-ss");
    assert_eq!(slug("..."), "import");
    assert_eq!(slug(&"a ".repeat(40)), "a-".repeat(19) + "a");
    let dir = crate::testutil::tempdir();
    for title in ["CON", "nul", "com1", "aux"] {
        let rel = store_import(dir.path(), "sound", title, ".ogg", b"OggS").expect(title);
        let file = dir.path().join("public").join(&rel);
        assert_eq!(fs::read(&file).expect(title), b"OggS", "{rel}");
        assert!(rel.starts_with("audio/kandidaten/"), "{rel}");
    }
}

/// Veralteter Stand: Datei und Credit bleiben (wie Go), die Zuordnung entsteht nicht.
#[test]
fn import_stand_veraltet() {
    let (dir, itch, _) = import_repo();
    let (_src, src) = src_file("a.png", &png_bytes(16, 16));
    let result = import_file_on(dir.path(), "farm", &itch, &src, Some("alt"), day());
    assert!(matches!(result, Err(Error::Stale)), "{result:?}");
    let register = load(dir.path()).expect("load");
    assert!(register.find("farm").expect("farm").choices().is_empty());
}
