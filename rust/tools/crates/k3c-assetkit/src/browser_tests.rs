use super::*;

/// Nur die Ablehnung: kein Test startet einen Browser.
#[test]
fn nur_http_adressen() {
    for url in [
        "",
        "file:///C:/x",
        "C:\\x.exe",
        "javascript:alert(1)",
        "HTTP://a",
        " http://a",
    ] {
        let err = open(url).expect_err(url);
        assert_eq!(err.to_string(), format!("nur http(s)-Adressen: {url}"));
    }
}
