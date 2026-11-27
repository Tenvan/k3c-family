//! Abschnitte eines Dokuments (Go `waves_spec.go`). Stand RU69.9: nur `section_lines`; der Rest folgt in RU69.11.

/// Zeilen unter der Überschrift `head` bis zur nächsten `## `.
pub fn section_lines(text: &str, head: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut inside = false;
    for line in text.replace("\r\n", "\n").split('\n') {
        if line.starts_with("## ") {
            inside = line.trim() == head;
        } else if inside {
            out.push(line.to_owned());
        }
    }
    out
}
