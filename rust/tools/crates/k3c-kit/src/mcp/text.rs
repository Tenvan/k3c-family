//! Text-Hilfen für Aufruf-Log und Ergebnisse (Port von `mcpkit/text.go`). Längen zählen Zeichen, nicht Bytes.

use rmcp::ErrorData;
use rmcp::model::{CallToolResult, ContentBlock};

use super::{ARGS_RUNES, Outcome, TEXT_RUNES};

fn cut(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_owned();
    }
    let mut out: String = s.chars().take(max - 1).collect();
    out.push('…');
    out
}

/// Kürzt rohe Tool-Argumente für das Log.
pub fn clip_args(raw: &[u8]) -> String {
    cut(&String::from_utf8_lossy(raw), ARGS_RUNES)
}

/// Kürzt auf [`TEXT_RUNES`] Zeichen.
pub fn clip(s: &str) -> String {
    cut(s, TEXT_RUNES)
}

/// Kurzer, im Ring eindeutiger Name je Aufruf (Go `Ref`): Multiplikation mit einer ungeraden Zahl ist modulo 2^28
/// umkehrbar, zwei IDs unter 2^28 bekommen also nie denselben Namen.
pub fn ref_name(id: i64) -> String {
    format!("{:07x}", (id as u64).wrapping_mul(2_654_435_761) & 0x0fff_ffff)
}

/// Schreibt einen leeren Wert als „–“.
pub fn or_dash(s: &str) -> &str {
    if s.is_empty() { "–" } else { s }
}

/// Ergebnis aus einem Text, mit `is_error` als Fehler-Ergebnis.
pub fn text_result(text: impl Into<String>, is_error: bool) -> CallToolResult {
    let mut result = CallToolResult::success(vec![ContentBlock::text(text)]);
    // Go lässt isError bei Erfolg weg (`omitempty`).
    result.is_error = is_error.then_some(true);
    result
}

/// Hängt die Text-Inhalte eines Ergebnisses aneinander.
pub fn result_text(result: &CallToolResult) -> String {
    let parts: Vec<&str> = result
        .content
        .iter()
        .filter_map(|c| c.as_text())
        .map(|t| t.text.as_str())
        .collect();
    parts.join("\n")
}

/// Liest Erfolg, Fehlertext und erste Antwortzeile aus einem Ergebnis.
pub fn outcome_of(result: &Result<CallToolResult, ErrorData>) -> Outcome {
    let r = match result {
        Ok(r) => r,
        Err(e) => {
            return Outcome {
                err: e.message.to_string(),
                ..Outcome::default()
            };
        }
    };
    let text = result_text(r);
    let summary = text.split('\n').next().unwrap_or_default().to_owned();
    if r.is_error == Some(true) {
        return Outcome {
            ok: false,
            err: text,
            summary,
        };
    }
    Outcome {
        ok: true,
        err: String::new(),
        summary,
    }
}

#[cfg(test)]
#[path = "text_tests.rs"]
mod tests;
