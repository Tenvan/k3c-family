use axum::Router;
use axum::extract::Path as UrlPath;
use axum::http::{Method as HttpMethod, StatusCode, header};
use axum::routing::{any, get};
use reqwest::Method;
use url::Url;

use super::*;
use crate::testutil::serve;

#[test]
fn query_escape_wie_go() {
    assert_eq!(query_escape("coin sound"), "coin+sound");
    assert_eq!(query_escape("a~b*c-d_e.f"), "a~b%2Ac-d_e.f");
    assert_eq!(query_escape("Münze"), "M%C3%BCnze");
    assert_eq!(query_escape("a+b&c=d/e?f:g[]"), "a%2Bb%26c%3Dd%2Fe%3Ff%3Ag%5B%5D");
    assert_eq!(
        encode_query(&[("keys", "x y"), ("field[]", "13"), ("field[]", "12")]),
        "field%5B%5D=13&field%5B%5D=12&keys=x+y"
    );
}

#[test]
fn adresse_mit_host_wie_go() {
    for ok in [
        "https://opengameart.org/x.png??",
        "HTTPS://example.org",
        "//example.org/x",
    ] {
        assert!(parse_with_host(ok).is_some(), "{ok}");
    }
    for bad in [
        "",
        "x.png",
        "https:example.org",
        "https:/example.org",
        "https:///x",
        "https://",
        " https://a",
        "https://a\n",
    ] {
        assert!(parse_with_host(bad).is_none(), "{bad:?}");
    }
}

#[tokio::test]
async fn read_limited_an_der_grenze() {
    let base = serve(Router::new().route("/", get(|| async { "12345" }))).await;
    for (limit, want) in [(4, "1234"), (5, "12345"), (6, "12345"), (0, "")] {
        let res = test_client().get(&base).send().await.expect("Anfrage");
        assert_eq!(
            read_limited(res, limit).await.expect("lesen"),
            want.as_bytes(),
            "{limit}"
        );
    }
}

#[test]
fn naht_ohne_eintrag_false() {
    // Port 1 vergibt kein Testserver.
    let url = Url::parse("http://127.0.0.1:1/x").expect("URL");
    assert!(!is_test_origin(&url));
    testing::allow("http://127.0.0.1:1");
    assert!(is_test_origin(&url));
    assert!(!is_test_origin(&Url::parse("http://127.0.0.1:2/x").expect("URL")));
}

/// `/n` leitet relativ auf `/n-1`, `/0` antwortet mit der Methode.
async fn chain() -> String {
    let step = |UrlPath(n): UrlPath<u32>, method: HttpMethod| async move {
        let next = match n {
            0 => return (StatusCode::OK, [(header::LOCATION, String::new())], method.to_string()),
            n => (n - 1).to_string(),
        };
        let status = if n % 2 == 0 {
            StatusCode::FOUND
        } else {
            StatusCode::TEMPORARY_REDIRECT
        };
        (status, [(header::LOCATION, next)], String::new())
    };
    serve(Router::new().route("/{n}", any(step))).await
}

async fn follow(method: Method, path: &str) -> Result<Response> {
    let url = Url::parse(&format!("{}/{path}", chain().await)).expect("URL");
    send_following(&test_download_client(), method, &url, 5, |_, _| Ok(())).await
}

#[tokio::test]
async fn send_following_relativer_location_und_fuenf_spruenge() {
    let response = follow(Method::GET, "5").await.expect("fünf Sprünge");
    assert_eq!(response.url().path(), "/0");
    assert_eq!(response.text().await.expect("Text"), "GET");
    let err = follow(Method::GET, "6").await.expect_err("sechster Sprung");
    assert!(err.to_string().contains("zu viele Weiterleitungen"), "{err}");
}

#[tokio::test]
async fn send_following_head_bleibt_head() {
    let response = follow(Method::HEAD, "2").await.expect("HEAD");
    assert_eq!(response.url().path(), "/0");
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn send_following_pruef_fehler_bricht_ab() {
    let url = Url::parse(&format!("{}/3", chain().await)).expect("URL");
    let check = |next: &Url, hops: usize| match hops {
        1 => Ok(()),
        _ => Err(Error::Msg(format!("halt bei {} nach {hops}", next.path()))),
    };
    let err = send_following(&test_download_client(), Method::GET, &url, 5, check)
        .await
        .expect_err("Abbruch");
    assert_eq!(err.to_string(), "halt bei /1 nach 2");
}

#[tokio::test]
async fn send_following_verwirft_selbst_folgenden_client() {
    let url = Url::parse(&format!("{}/1", chain().await)).expect("URL");
    let following = reqwest::Client::builder().no_proxy().build().expect("Client");
    let err = send_following(&DownloadClient(following), Method::GET, &url, 5, |_, _| Ok(()))
        .await
        .expect_err("abgelehnt");
    assert!(err.to_string().contains("folgt selbst"), "{err}");
}

#[tokio::test]
async fn send_following_mit_zugangsdaten_und_fragment() {
    let base = chain().await.replace("http://", "http://u:p@");
    let url = Url::parse(&format!("{base}/0#teil")).expect("URL");
    let response = send_following(&test_download_client(), Method::GET, &url, 5, |_, _| Ok(()))
        .await
        .expect("kein Sprung");
    assert_eq!(response.status(), StatusCode::OK);
}

/// Pfad wie Go `url.URL.Path`: dekodiert, ohne Abfrage, Fragment und Auflösen von `..`; `None`, wo Go ablehnt.
#[test]
fn url_path_wie_go() {
    for (raw, want) in [
        ("https://x.org/a/my%20file.png?q=1#f", Some("/a/my file.png")),
        ("https://x.org/a/..", Some("/a/..")),
        ("https://x.org", Some("")),
        ("mailto:a@b", Some("")),
        ("a/b.png", Some("a/b.png")),
        ("", Some("")),
        ("127.0.0.1:80/a.png", None),
        ("https://x.org/a%zz.png", None),
    ] {
        assert_eq!(super::url_path(raw).as_deref(), want, "{raw:?}");
    }
}

// Sprint-Review RU67: Was `url` still entfernt, Go aber ablehnt.
#[test]
fn plain_authority_wie_go() {
    for (raw, want) in [
        ("https://upload.wikimedia.org/a.png", true),
        ("https://upload.wikimedia.org:443/a@b.png", true),
        ("a.png", true),
        ("https://@upload.wikimedia.org/a.png", false),
        ("https://:@upload.wikimedia.org/a.png", false),
        ("//@upload.wikimedia.org/a.png", false),
        ("https://upload.wikimedia.org:0443/a.png", false),
    ] {
        assert_eq!(plain_authority(raw), want, "{raw}");
    }
}
