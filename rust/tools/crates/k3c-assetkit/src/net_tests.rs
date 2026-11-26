use axum::Router;
use axum::routing::get;
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
