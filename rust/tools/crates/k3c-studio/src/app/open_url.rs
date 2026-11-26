//! Welche Adressen `open_url` öffnet: nur `http://` und `https://` (V5), dazu die Prüfungen, die Wails
//! `BrowserOpenURL` vor dem Öffnen macht (Go-Orakel, `ValidateAndSanitizeURL`).

use url::Url;

/// Zeichen, die Wails in der normalisierten Adresse ablehnt.
const SHELL_CHARS: &str = ";|`$\\<>*{}[]()~! \t\n\r";

fn unicode_danger(c: char) -> bool {
    matches!(
        c,
        '\u{0}'..='\u{1f}' | '\u{7f}' | '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200f}' | '\u{2028}'..='\u{202f}'
            | '\u{205f}' | '\u{2060}' | '\u{3000}' | '\u{feff}'
    )
}

/// Liefert die normalisierte Adresse oder den Grund der Ablehnung.
pub fn allowed_url(raw: &str) -> Result<String, String> {
    // Vor dem Parsen: der Parser entfernt Zeilenumbrüche und Tabulatoren stillschweigend.
    if raw.chars().any(char::is_control) {
        return Err("Adresse abgelehnt: Steuerzeichen".to_owned());
    }
    let parsed = Url::parse(raw).map_err(|e| format!("Adresse abgelehnt: {e}"))?;
    if !matches!(parsed.scheme(), "http" | "https") {
        return Err("Adresse abgelehnt: nur http und https".to_owned());
    }
    if parsed.host_str().is_none_or(str::is_empty) {
        return Err("Adresse abgelehnt: ohne Rechnernamen".to_owned());
    }
    let url = String::from(parsed);
    if url.chars().any(|c| SHELL_CHARS.contains(c) || unicode_danger(c)) {
        return Err("Adresse abgelehnt: unzulässige Zeichen".to_owned());
    }
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::allowed_url;

    #[test]
    fn laesst_web_adressen_durch() {
        for ok in [
            "https://freesound.org/people/x/sounds/1/",
            "http://127.0.0.1:8193/spiel?seed=1&a=%20",
        ] {
            assert_eq!(allowed_url(ok).as_deref(), Ok(ok), "{ok}");
        }
    }

    #[test]
    fn lehnt_alles_andere_ab() {
        for bad in [
            "",
            "freesound.org",
            "file:///C:/Windows/System32/calc.exe",
            r"C:\Windows\System32\calc.exe",
            r"\\server\freigabe\x.exe",
            "javascript:alert(1)",
            "data:text/html,x",
            "mailto:a@example.org",
            "ms-settings:display",
            "https://",
            "https://example.org/a;calc",
            "https://example.org/$(calc)",
            "https://example.org/a|b",
            "https://example.org/\ncalc",
            "https://example.org/a`b?x=~",
        ] {
            assert!(allowed_url(bad).is_err(), "{bad:?} nicht abgelehnt");
        }
    }
}
