//! Ein Log-Eintrag und das Lesen einer Zeile (Port von `entry.go`).

use chrono::{DateTime, FixedOffset, Timelike};
use serde::{Serialize, Serializer};
use serde_json::{Map, Value};

/// Die bekannten Level in aufsteigender Reihenfolge.
pub const LEVELS: [&str; 4] = ["DEBUG", "INFO", "WARN", "ERROR"];

/// Größte Ganzzahl, die ein `float64` genau hält; Go liest jede Zahl als solches.
const EXACT_INT: u64 = 1 << 53;

/// Ein Log-Eintrag (`LogEntry` in `types.ts`); `data` hält alle Felder außer time, level, msg, ns und checkout.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Entry {
    #[serde(serialize_with = "go_time")]
    pub time: DateTime<FixedOffset>,
    pub level: String,
    pub ns: String,
    /// Checkout des Aufrufs (Feld `checkout` der Zeile, z. B. „Repo-Wurzel“ oder der Worktree-Name), leer ohne.
    pub checkout: String,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Map<String, Value>>,
}

/// Zeit wie Go `time.Time` in JSON: RFC 3339 mit Nanosekunden ohne Nullen am Ende, bei Offset 0 `Z`.
pub(super) fn go_time<S: Serializer>(t: &DateTime<FixedOffset>, out: S) -> Result<S::Ok, S::Error> {
    let nanos = t.nanosecond() % 1_000_000_000;
    let frac = if nanos == 0 {
        String::new()
    } else {
        format!(".{}", format!("{nanos:09}").trim_end_matches('0'))
    };
    let offset = if t.offset().local_minus_utc() == 0 {
        "Z".to_owned()
    } else {
        t.format("%:z").to_string()
    };
    out.serialize_str(&format!("{}{frac}{offset}", t.format("%Y-%m-%dT%H:%M:%S")))
}

/// Ordnet ein Level ein; ein unbekanntes zählt als niedrigstes (`-1`), damit ein Filter es nie durchlässt.
pub fn level_rank(level: &str) -> i32 {
    (0..)
        .zip(LEVELS)
        .find(|(_, known)| known.eq_ignore_ascii_case(level))
        .map_or(-1, |(rank, _)| rank)
}

/// RFC 3339 so streng wie Go `time.Parse`: großes `T` und `Z` (chrono ließe auch Leerzeichen und Kleinbuchstaben zu).
pub fn parse_rfc3339(text: &str) -> Option<DateTime<FixedOffset>> {
    if text.as_bytes().get(10) != Some(&b'T') || text.ends_with('z') {
        return None;
    }
    DateTime::parse_from_rfc3339(text).ok()
}

/// Wie Go nach `Unmarshal` in `any` und erneutem `Marshal`: Schlüssel sortiert, ganze Zahlen ohne Nachkommastellen.
fn normalize(value: Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(sorted(map)),
        Value::Array(list) => Value::Array(list.into_iter().map(normalize).collect()),
        Value::Number(n) => Value::Number(normalize_number(n)),
        other => other,
    }
}

fn sorted(map: Map<String, Value>) -> Map<String, Value> {
    let mut pairs: Vec<(String, Value)> = map.into_iter().collect();
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    pairs.into_iter().map(|(k, v)| (k, normalize(v))).collect()
}

fn normalize_number(n: serde_json::Number) -> serde_json::Number {
    let exact = n.as_i64().is_some_and(|i| i.unsigned_abs() <= EXACT_INT);
    match n.as_f64() {
        // Der Bereich ist geprüft, die Umwandlung verliert nichts.
        #[allow(clippy::cast_possible_truncation)]
        Some(f) if !exact && f.fract() == 0.0 && f.abs() < 9.2e18 => serde_json::Number::from(f as i64),
        _ => n,
    }
}

/// Liest eine Zeile; `None` heißt: kein gültiges JSON oder keine Zeit.
pub(super) fn parse(line: &[u8]) -> Option<Entry> {
    // Wie Go: ungültiges UTF-8 wird ersetzt, nicht abgelehnt.
    let mut raw: Map<String, Value> = serde_json::from_str(&String::from_utf8_lossy(line)).ok()?;
    let time = parse_rfc3339(raw.get("time")?.as_str()?)?;
    let text = |key: &str| raw.get(key).and_then(Value::as_str).unwrap_or_default().to_owned();
    let (level, msg) = (text("level").to_uppercase(), text("msg"));
    let (ns, checkout) = (text("ns"), text("checkout"));
    for key in ["time", "level", "msg", "ns", "checkout"] {
        raw.remove(key);
    }
    Some(Entry {
        time,
        level,
        ns,
        checkout,
        msg,
        data: (!raw.is_empty()).then(|| sorted(raw)),
    })
}
