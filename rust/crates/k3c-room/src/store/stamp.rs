//! Zeitstempel in UTC mit Handrechnung (kein `chrono`): Tage seit 1970 in Jahr, Monat, Tag nach Howard Hinnant.

/// Layout der Sicherungsnamen (Go `stampLayout`): sortiert als Text wie als Zeit, mit Nanosekunden.
pub const STAMP_LAYOUT: &str = "20060102T150405.000000000Z";

/// Ein Zeitpunkt in UTC: Sekunden seit 1970 und Nanosekunden (0 bis 999 999 999).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Stamp {
    pub secs: i64,
    pub nanos: u32,
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (yoe + era * 400 + i64::from(m <= 2), m, d)
}

fn days_in_month(y: i64, m: i64) -> i64 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 => 29,
        _ => 28,
    }
}

impl Stamp {
    pub fn new(secs: i64, nanos: u32) -> Stamp {
        Stamp { secs, nanos }
    }

    /// Baut einen Zeitpunkt aus Datum und Uhrzeit (UTC); die Werte müssen gültig sein.
    pub fn from_civil(y: i64, mo: i64, d: i64, h: i64, mi: i64, s: i64, nanos: u32) -> Stamp {
        let days = days_from_civil(y, mo, d);
        Stamp {
            secs: days * 86_400 + h * 3600 + mi * 60 + s,
            nanos,
        }
    }

    /// Jahr, Monat, Tag, Stunde, Minute, Sekunde (UTC).
    pub fn civil(&self) -> (i64, i64, i64, i64, i64, i64) {
        let days = self.secs.div_euclid(86_400);
        let rem = self.secs.rem_euclid(86_400);
        let (y, m, d) = civil_from_days(days);
        (y, m, d, rem / 3600, rem % 3600 / 60, rem % 60)
    }

    pub fn add_secs(self, n: i64) -> Stamp {
        Stamp {
            secs: self.secs + n,
            nanos: self.nanos,
        }
    }

    /// Unterschied `self - other` in Nanosekunden (Go `Sub`).
    pub fn since_nanos(self, other: Stamp) -> i128 {
        i128::from(self.secs - other.secs) * 1_000_000_000 + i128::from(self.nanos) - i128::from(other.nanos)
    }

    /// Millisekunden seit 1970 (Go `UnixMilli`).
    pub fn unix_millis(&self) -> i64 {
        self.secs * 1000 + i64::from(self.nanos / 1_000_000)
    }

    /// `20060102T150405.000000000Z` (Sicherungsnamen).
    pub fn format_backup(&self) -> String {
        let (y, mo, d, h, mi, s) = self.civil();
        format!("{y:04}{mo:02}{d:02}T{h:02}{mi:02}{s:02}.{:09}Z", self.nanos)
    }

    /// `2006-01-02T15:04:05.000Z` (Berichte; Millisekunden abgeschnitten wie in Go).
    pub fn format_report(&self) -> String {
        let (y, mo, d, h, mi, s) = self.civil();
        format!(
            "{y:04}-{mo:02}-{d:02}T{h:02}:{mi:02}:{s:02}.{:03}Z",
            self.nanos / 1_000_000
        )
    }

    /// Liest `format_backup` zurück; ein ungültiger Text ergibt `None` (Go: Nullzeit).
    pub fn parse_backup(text: &str) -> Option<Stamp> {
        let b = text.as_bytes();
        let ok = b.len() == STAMP_LAYOUT.len()
            && b[8] == b'T'
            && b[15] == b'.'
            && b[25] == b'Z'
            && b.iter()
                .enumerate()
                .all(|(i, c)| matches!(i, 8 | 15 | 25) || c.is_ascii_digit());
        if !ok {
            return None;
        }
        let n = |from: usize, to: usize| text[from..to].parse::<i64>().ok();
        let (y, mo, d) = (n(0, 4)?, n(4, 6)?, n(6, 8)?);
        let (h, mi, s) = (n(9, 11)?, n(11, 13)?, n(13, 15)?);
        let nanos = u32::try_from(n(16, 25)?).ok()?;
        let valid = (1..=12).contains(&mo) && d >= 1 && d <= days_in_month(y, mo) && h < 24 && mi < 60 && s < 60;
        valid.then(|| Stamp::from_civil(y, mo, d, h, mi, s, nanos))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formate_mit_schaltjahr_und_null_nanosekunden() {
        let faelle = [
            (
                Stamp::from_civil(2024, 2, 29, 23, 59, 59, 0),
                "20240229T235959.000000000Z",
                "2024-02-29T23:59:59.000Z",
            ),
            (
                Stamp::from_civil(2026, 9, 30, 12, 0, 1, 123_456_789),
                "20260930T120001.123456789Z",
                "2026-09-30T12:00:01.123Z",
            ),
            (
                Stamp::from_civil(1970, 1, 1, 0, 0, 0, 0),
                "19700101T000000.000000000Z",
                "1970-01-01T00:00:00.000Z",
            ),
            (
                Stamp::from_civil(2000, 2, 29, 0, 0, 0, 5_000_000),
                "20000229T000000.005000000Z",
                "2000-02-29T00:00:00.005Z",
            ),
            (
                Stamp::from_civil(1969, 12, 31, 23, 59, 59, 0),
                "19691231T235959.000000000Z",
                "1969-12-31T23:59:59.000Z",
            ),
        ];
        for (stamp, backup, report) in faelle {
            assert_eq!(stamp.format_backup(), backup);
            assert_eq!(stamp.format_report(), report);
            assert_eq!(Stamp::parse_backup(backup), Some(stamp));
        }
        assert_eq!(Stamp::new(0, 0).format_backup(), "19700101T000000.000000000Z");
        assert_eq!(Stamp::new(1_782_820_800, 0).format_report(), "2026-06-30T12:00:00.000Z");
    }

    #[test]
    fn parse_lehnt_ungueltiges_ab() {
        for text in [
            "",
            "x",
            "20230229T000000.000000000Z",
            "20241301T000000.000000000Z",
            "20240101T250000.000000000Z",
            "20240101T000000.000000000",
            "2024010AT000000.000000000Z",
        ] {
            assert_eq!(Stamp::parse_backup(text), None, "{text}");
        }
    }

    #[test]
    fn tage_und_unterschied() {
        assert_eq!(days_from_civil(2000, 3, 1) - days_from_civil(2000, 2, 28), 2);
        assert_eq!(days_from_civil(1900, 3, 1) - days_from_civil(1900, 2, 28), 1);
        let a = Stamp::new(100, 500);
        assert_eq!(a.since_nanos(Stamp::new(99, 600)), 999_999_900);
        assert_eq!(a.unix_millis(), 100_000);
    }
}
