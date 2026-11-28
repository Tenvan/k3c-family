//! Welche Adressen `open_url` öffnet: nur `http://` und `https://`, dazu die Zeichen, die Wails `BrowserOpenURL` vor
//! dem Öffnen ablehnt (Go-Orakel, `ValidateAndSanitizeURL`). Anders als Wails wird nichts umgeschrieben.

/// Zeichen, die Wails in der Adresse ablehnt.
const SHELL_CHARS: &str = ";|`$\\<>*{}[]()~! ";

fn unicode_danger(c: char) -> bool {
    matches!(
        c,
        '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200f}' | '\u{2028}'..='\u{202f}' | '\u{205f}' | '\u{2060}'
            | '\u{3000}' | '\u{feff}'
    )
}

/// Rechnername ohne Anmeldedaten und Port.
fn host(rest: &str) -> &str {
    let authority = rest.split(['/', '?', '#']).next().unwrap_or_default();
    let host_port = authority.rsplit('@').next().unwrap_or_default();
    host_port.split(':').next().unwrap_or_default()
}

/// Liefert die Adresse oder den Grund der Ablehnung.
pub fn allowed_url(raw: &str) -> Result<&str, String> {
    let refused = |why: &str| Err(format!("Adresse abgelehnt: {why}"));
    if raw.chars().any(char::is_control) {
        return refused("Steuerzeichen");
    }
    let Some((scheme, rest)) = raw.split_once("://") else {
        return refused("nur http und https");
    };
    if !scheme.eq_ignore_ascii_case("http") && !scheme.eq_ignore_ascii_case("https") {
        return refused("nur http und https");
    }
    if host(rest).is_empty() {
        return refused("ohne Rechnernamen");
    }
    if raw.chars().any(|c| SHELL_CHARS.contains(c) || unicode_danger(c)) {
        return refused("unzulässige Zeichen");
    }
    Ok(raw)
}

#[cfg(test)]
mod tests {
    use super::allowed_url;

    #[test]
    fn laesst_web_adressen_durch() {
        for ok in [
            "https://github.com/tenvan/k3c/pull/912",
            "http://127.0.0.1:8191/spiel?seed=1&a=%20",
            "HTTPS://example.org",
        ] {
            assert_eq!(allowed_url(ok), Ok(ok), "{ok}");
        }
    }

    #[test]
    fn lehnt_alles_andere_ab() {
        for bad in [
            "",
            "github.com",
            "file:///C:/Windows/System32/calc.exe",
            "file://server/x.exe",
            r"C:\Windows\System32\calc.exe",
            r"\\server\freigabe\x.exe",
            "javascript:alert(1)",
            "data:text/html,x",
            "mailto:a@example.org",
            "ms-settings:display",
            "https://",
            "https://?x=1",
            "https://user@:80/",
            "https://example.org/a;calc",
            "https://example.org/$(calc)",
            "https://example.org/a|b",
            "https://example.org/a b",
            "https://example.org/\ncalc",
            "https://example.org/a`b?x=~",
        ] {
            assert!(allowed_url(bad).is_err(), "{bad:?} nicht abgelehnt");
        }
    }
}
