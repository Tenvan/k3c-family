//! Port von `native/k3c-game/devtools/report.go`.
//!
//! Sitzungszusammenfassung (B-889): am Ende eines Spiels (Spielende, Verlassen, Programmende) eine JSON-Datei
//! `native-session-<Zeit>-t<Takte>.json` in `reports/` (lesbar über `report_read` der Workbench). Ein Schreibfehler steht
//! im Log. Dazu der kleine Zeit-Typ `Zeit` (UTC, ohne `chrono`), den nur Bericht und Takt-Zeiten nutzen, nie die Sim.

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize, Serializer};

use super::chronicle::Chronicle;
use super::chronicle_stage::round1;

/// Zeit ist ein Zeitpunkt in UTC, gefüllt aus `std::time::SystemTime` (Go: `time.Time`, nur `Format` und `Sub`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Zeit {
    pub year: i64,
    pub month: i64,
    pub day: i64,
    pub hour: i64,
    pub minute: i64,
    pub second: i64,
    /// Sekunden seit 1970-01-01T00:00:00 UTC
    pub unix_secs: i64,
    pub nanos: u32,
}

impl Zeit {
    /// now liest die Systemuhr (vor 1970 oder zu groß: Nullpunkt).
    pub fn now() -> Zeit {
        let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
        Zeit::from_unix(i64::try_from(d.as_secs()).unwrap_or_default(), d.subsec_nanos())
    }

    /// from_unix zerlegt Sekunden seit 1970 (UTC) in Datum und Uhrzeit.
    pub fn from_unix(unix_secs: i64, nanos: u32) -> Zeit {
        let days = unix_secs.div_euclid(86_400);
        let rest = unix_secs.rem_euclid(86_400);
        let (year, month, day) = civil_from_days(days);
        Zeit {
            year,
            month,
            day,
            hour: rest / 3600,
            minute: rest % 3600 / 60,
            second: rest % 60,
            unix_secs,
            nanos,
        }
    }

    /// format schreibt `2006-01-02T15:04:05`.
    pub fn format(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }

    /// since liefert die Sekunden von `earlier` bis self (Go: `Sub(...).Seconds()`).
    #[allow(clippy::cast_precision_loss)]
    pub fn since(&self, earlier: &Zeit) -> f64 {
        (self.unix_secs - earlier.unix_secs) as f64 + (f64::from(self.nanos) - f64::from(earlier.nanos)) / 1e9
    }
}

/// civil_from_days rechnet Tage seit 1970-01-01 in (Jahr, Monat, Tag) des gregorianischen Kalenders (Hinnant).
fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    (yoe + era * 400 + i64::from(month <= 2), month, day)
}

/// go_float schreibt ganze Zahlen ohne Nachkomma (`60`), wie Gos `encoding/json` bei `float64`.
fn go_float<S: Serializer>(v: &f64, s: S) -> Result<S::Ok, S::Error> {
    if v.fract() == 0.0 && v.abs() < 1e15 {
        #[allow(clippy::cast_possible_truncation)]
        let i = *v as i64;
        s.serialize_i64(i)
    } else {
        s.serialize_f64(*v)
    }
}

/// Report ist die Sitzungszusammenfassung (Feldnamen und Reihenfolge wie Go).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Report {
    /// immer "native-session"
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "seed")]
    pub seed: String,
    #[serde(rename = "start")]
    pub start: String,
    #[serde(rename = "end")]
    pub end: String,
    #[serde(rename = "dauerSekunden", serialize_with = "go_float")]
    pub seconds: f64,
    #[serde(rename = "takte")]
    pub ticks: u64,
    #[serde(rename = "ersterTag")]
    pub first_day: i64,
    #[serde(rename = "letzterTag")]
    pub last_day: i64,
    #[serde(rename = "tage")]
    pub days: i64,
    /// gewonnen, verloren, verlassen, beendet
    #[serde(rename = "endeUrsache")]
    pub cause: String,
}

impl Chronicle {
    /// report baut die Zusammenfassung der laufenden Sitzung.
    pub fn report(&self, cause: &str) -> Report {
        let end = (self.now)();
        let r = &self.run;
        Report {
            kind: "native-session".to_string(),
            seed: r.seed.clone(),
            start: r.start.format(),
            end: end.format(),
            seconds: round1(end.since(&r.start)),
            ticks: r.tick.saturating_sub(r.start_tick),
            first_day: r.first_day,
            last_day: r.day,
            days: r.day - r.first_day + 1,
            cause: cause.to_string(),
        }
    }

    /// finish_with schreibt die Zusammenfassung, einmal je Sitzung.
    pub(super) fn finish_with(&mut self, cause: &str) {
        if self.run.written || self.dir.is_empty() {
            return;
        }
        self.run.written = true;
        let r = self.report(cause);
        match write_report(Path::new(&self.dir), &r) {
            Err(err) => self.log.warn("💥 Sitzungsbericht nicht geschrieben", &[("err", err)]),
            Ok(path) => self.log.info(
                "💾 Sitzungsbericht",
                &[
                    ("datei", path),
                    ("seed", r.seed.clone()),
                    ("tage", r.days.to_string()),
                    ("ursache", r.cause.clone()),
                ],
            ),
        }
    }
}

/// write_report schreibt r als JSON nach dir und liefert den Pfad.
pub fn write_report(dir: &Path, r: &Report) -> Result<String, String> {
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let raw = serde_json::to_string_pretty(r).map_err(|e| e.to_string())?;
    let stamp = r.end.get(.."2006-01-02T15:04:05".len()).unwrap_or(&r.end);
    let name = sanitize(&format!("native-session-{stamp}-t{}.json", r.ticks));
    let path = dir.join(name);
    std::fs::write(&path, raw).map_err(|e| e.to_string())?;
    Ok(path.display().to_string())
}

/// sanitize ersetzt die Doppelpunkte der Uhrzeit (unter Windows nicht im Dateinamen erlaubt).
pub fn sanitize(name: &str) -> String {
    name.replace(':', "-")
}
