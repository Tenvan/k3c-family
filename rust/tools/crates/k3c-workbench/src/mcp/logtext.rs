//! Texte der Log-Tools (Port der Hilfen aus `tools_logs.go`): Zeile je Eintrag, Fußzeile, Zeitspanne und `since`.

use chrono::{DateTime, Datelike, FixedOffset, Local, TimeDelta};
use k3c_kit::mcp::clip;

use super::services::format_bytes;
use crate::logs::{Entry, ScanResult, parse_rfc3339};

/// Eintrag als Zeile: Uhrzeit, Level, Bereich, Checkout („-“ ohne), Meldung und gekürzte Daten.
pub(super) fn entry_line(entry: &Entry) -> String {
    let checkout = if entry.checkout.is_empty() {
        "-"
    } else {
        &entry.checkout
    };
    let mut line = format!(
        "{} {} {} {checkout} {}",
        entry.time.with_timezone(&Local).format("%H:%M:%S"),
        entry.level,
        entry.ns,
        entry.msg
    );
    if let Some(data) = &entry.data {
        let json = serde_json::to_string(data).unwrap_or_default();
        line.push(' ');
        line.push_str(&clip(&html_escaped(&json)));
    }
    line
}

/// Go `json.Marshal` maskiert diese Zeichen; sie stehen in JSON nur innerhalb von Texten.
fn html_escaped(json: &str) -> String {
    json.replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029")
}

pub(super) fn scan_footer(head: &str, res: &ScanResult) -> String {
    let mut foot = format!("{head} · {} gelesen", format_bytes(res.bytes_read));
    if res.budget_hit {
        foot.push_str(" · Budget erreicht, ältere Treffer möglich");
    }
    if res.skipped > 0 {
        foot.push_str(&format!(" · {} unlesbare Zeilen übersprungen", res.skipped));
    }
    foot
}

/// Zeitspanne kurz: „13:02–13:40“, über Tage mit Datum.
pub(super) fn window(first: DateTime<FixedOffset>, last: DateTime<FixedOffset>) -> String {
    let (a, b) = (first.with_timezone(&Local), last.with_timezone(&Local));
    if a == b {
        return a.format("%H:%M").to_string();
    }
    let format = if (a.year(), a.ordinal()) == (b.year(), b.ordinal()) {
        "%H:%M"
    } else {
        "%d.%m. %H:%M"
    };
    format!("{}–{}", a.format(format), b.format(format))
}

/// Einheiten von Go `time.ParseDuration` in Nanosekunden.
fn unit_nanos(unit: &str) -> Option<f64> {
    Some(match unit {
        "ns" => 1.0,
        "us" | "µs" | "μs" => 1e3,
        "ms" => 1e6,
        "s" => 1e9,
        "m" => 60e9,
        "h" => 3600e9,
        _ => return None,
    })
}

/// Go `time.ParseDuration` in Nanosekunden: `[+-]?(Zahl Einheit)+` oder `0`; `None` bei Fehler oder Überlauf.
fn parse_duration(raw: &str) -> Option<i64> {
    let (sign, mut rest) = match raw.strip_prefix('-') {
        Some(rest) => (-1.0, rest),
        None => (1.0, raw.strip_prefix('+').unwrap_or(raw)),
    };
    if rest == "0" {
        return Some(0);
    }
    if rest.is_empty() {
        return None;
    }
    let numeric = |c: char| c.is_ascii_digit() || c == '.';
    let mut total = 0.0_f64;
    while !rest.is_empty() {
        let number_end = rest.find(|c| !numeric(c)).unwrap_or(rest.len());
        let (number, after) = rest.split_at(number_end);
        let unit_end = after.find(numeric).unwrap_or(after.len());
        let (unit, next) = after.split_at(unit_end);
        total += number.parse::<f64>().ok()? * unit_nanos(unit)?;
        rest = next;
    }
    // Der Bereich ist geprüft.
    #[allow(clippy::cast_possible_truncation)]
    let nanos = (sign * total) as i64;
    (total < 9.2e18).then_some(nanos)
}

/// Dauer (`30m`, `24h`, `7d`) oder Zeitpunkt (RFC 3339); leer heißt kein Filter.
pub(super) fn parse_since(raw: &str, now: DateTime<FixedOffset>) -> Result<Option<DateTime<FixedOffset>>, String> {
    if raw.is_empty() {
        return Ok(None);
    }
    let days = raw
        .strip_suffix('d')
        .and_then(|days| parse_duration(&format!("{days}h")))
        .and_then(|nanos| nanos.checked_mul(24));
    if let Some(nanos) = days.or_else(|| parse_duration(raw)) {
        return Ok(now.checked_sub_signed(TimeDelta::nanoseconds(nanos)));
    }
    parse_rfc3339(raw)
        .map(Some)
        .ok_or_else(|| format!("since {raw:?}: erwartet eine Dauer wie 30m, 24h, 7d oder einen Zeitpunkt RFC 3339"))
}

/// Jetzt mit dem Offset der Ortszeit.
pub(super) fn now() -> DateTime<FixedOffset> {
    Local::now().fixed_offset()
}
