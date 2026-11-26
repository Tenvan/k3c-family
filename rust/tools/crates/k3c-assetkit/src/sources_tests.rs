use std::collections::BTreeMap;
use std::path::Path;

use axum::Router;
use axum::http::{HeaderMap, StatusCode, Uri, header};

use super::*;
use crate::net::test_client;
use crate::testutil::serve;

fn testdata(file: &str) -> Vec<u8> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("testdata").join(file);
    std::fs::read(path).unwrap_or_default()
}

fn answer(uri: &Uri, headers: &HeaderMap, fail_oga: bool) -> (StatusCode, Vec<u8>) {
    let query: Vec<(String, String)> = url::form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes())
        .into_owned()
        .collect();
    let has_token = query.iter().any(|(k, _)| k == "token");
    let authorized = headers.get(header::AUTHORIZATION).is_some_and(|v| v == "Token test");
    let path = uri.path();
    if path.starts_with("/apiv2/") {
        if !authorized || has_token {
            return (StatusCode::UNAUTHORIZED, testdata("freesound-401.json"));
        }
        return (StatusCode::OK, testdata("freesound.json"));
    }
    let two_d = query.iter().any(|(k, v)| k == "field_art_type_tid[]" && v == "9");
    let file = match path {
        "/w/api.php" => "commons.json",
        "/art-search-advanced" if fail_oga => return (StatusCode::SERVICE_UNAVAILABLE, Vec::new()),
        "/art-search-advanced" if two_d => "oga-2d.html",
        "/art-search-advanced" => "oga-sound.html",
        _ => "",
    };
    (StatusCode::OK, testdata(file))
}

/// Spielt aufgezeichnete Antworten ab (`testdata/`): OpenGameArt und Commons echt aufgezeichnet, freesound im Format der
/// API-Doku (ohne Token kam nur 401). Kein Netz im Test.
async fn recorded(fail_oga: bool) -> Searcher {
    let router =
        Router::new().fallback(move |uri: Uri, headers: HeaderMap| async move { answer(&uri, &headers, fail_oga) });
    let base = serve(router).await;
    Searcher {
        client: test_client(),
        freesound: base.clone(),
        commons: base.clone(),
        oga: base,
        token: "test".to_string(),
    }
}

fn by_source(hits: Vec<Hit>) -> BTreeMap<String, Vec<Hit>> {
    let mut out: BTreeMap<String, Vec<Hit>> = BTreeMap::new();
    for h in hits {
        out.entry(h.source.clone()).or_default().push(h);
    }
    out
}

/// B-298/AC-04: Vorschläge mit Link, Quelle und Lizenz aus drei Quellen, OpenGameArt ohne API.
#[tokio::test]
async fn recherche_sound() {
    let (hits, notes) = recorded(false).await.search("coin", KIND_SOUND).await;
    assert!(notes.is_empty(), "Hinweise: {notes:?}");
    let src = by_source(hits);
    let fs = &src["freesound"];
    assert!(
        fs.len() == 2
            && fs[0].license == "CC0 1.0"
            && fs[1].license == "CC BY-NC 3.0"
            && fs[0].artist == "Beetlemuse"
            && fs[0].preview.ends_with(".mp3"),
        "freesound: {fs:?}"
    );
    check_sound_oga(&src["OpenGameArt"]);
    let c = &src["Wikimedia Commons"];
    assert!(
        c.len() == 2 && c[1].license == "CC BY-SA 3.0" && c[1].artist == "devnewton" && !c[1].download.is_empty(),
        "Commons: {c:?}"
    );
}

fn check_sound_oga(oga: &[Hit]) {
    assert!(
        oga.len() == 3
            && oga[0].title == "Level up, power up, Coin get (13 Sounds)"
            && oga[0].link.ends_with("/content/level-up-power-up-coin-get-13-sounds")
            && oga[0].preview.ends_with(".mp3")
            && oga[0].license.is_empty(),
        "OpenGameArt: {oga:?}"
    );
}

#[tokio::test]
async fn recherche_grafik_ohne_freesound_und_mit_ausfall() {
    let mut s = recorded(true).await;
    s.token = String::new();
    let (hits, notes) = s.search("farm", KIND_GRAPHIC).await;
    let src = by_source(hits);
    assert!(
        !src.contains_key("freesound")
            && src["Wikimedia Commons"].len() == 2
            && notes.len() == 1
            && notes[0].contains("OpenGameArt: HTTP 503"),
        "Grafik mit OGA-Ausfall: {src:?} {notes:?}"
    );
    let (hits, _) = recorded(false).await.search("farm", KIND_GRAPHIC).await;
    let src = by_source(hits);
    let oga = &src["OpenGameArt"];
    assert!(
        oga.len() == 3 && oga[0].title == "Farm" && oga[0].preview.ends_with(".png"),
        "OGA 2D: {oga:?}"
    );
}

/// Ohne Token meldet freesound einen Hinweis, die anderen Quellen liefern weiter (Token nie im Repo).
#[tokio::test]
async fn freesound_ohne_token() {
    let mut s = recorded(false).await;
    s.token = String::new();
    let (hits, notes) = s.search("coin", KIND_SOUND).await;
    assert!(
        notes.len() == 1 && notes[0].contains("freesound: token fehlt") && hits.len() == 5,
        "ohne Token: {} Treffer, {notes:?}",
        hits.len()
    );
}
