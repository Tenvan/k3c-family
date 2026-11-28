//! Tool `workbench_status` (Port von `tools_status.go`): kurz, eine Zeile je Punkt.

use std::collections::BTreeSet;
use std::path::Path;

use chrono::TimeDelta;
use k3c_kit::mcp::Workspace;

use super::tools::Workbench;
use crate::ports;

pub(super) async fn workbench_status(app: &Workbench, ws: &Workspace) -> Result<String, String> {
    let server = app
        .server
        .get()
        .and_then(std::sync::Weak::upgrade)
        .ok_or("server beendet")?;
    let stats = server.stats().await;
    let mut lines = vec![
        format!(
            "k3c-workbench · {} · läuft seit {}",
            server.url(),
            format_uptime(server.uptime())
        ),
        format!("Checkout: {}", ws.label()),
        format!(
            "Aufrufe {} · Fehler {} · Clients {} (max {}) · parallel {} (max {})",
            stats.total_calls, stats.errors, stats.clients, stats.peak_clients, stats.in_flight, stats.peak_in_flight
        ),
    ];
    if let Some(usage) = &app.usage {
        let session = usage.snapshot().session;
        lines.push(format!(
            "Sitzung: p95 {} · Ausreißer {}",
            format_ms(session.p95_ms),
            session.outliers
        ));
    }
    lines.push(app.service_summary(ws).await);
    lines.push(format!("Log-Quellen: {}", log_sources(&ws.root).join(", ")));
    lines.extend(app.env.checks.run_lines(ws));
    Ok(lines.join("\n"))
}

/// Alle `logs/*.jsonl` unter `root` und das eigene Log, sortiert (Go `logSources`).
pub(crate) fn log_sources(root: &Path) -> Vec<String> {
    let mut seen = BTreeSet::from([ports::LOG_NAME.to_owned()]);
    for entry in std::fs::read_dir(root.join("logs")).into_iter().flatten().flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if let Some(stem) = name.strip_suffix(".jsonl") {
            seen.insert(stem.to_owned());
        }
    }
    seen.into_iter().collect()
}

/// Laufzeit grob: „< 1 min“, „13 min“, „2 h 13 min“, „3 T 4 h“.
pub(super) fn format_uptime(d: TimeDelta) -> String {
    let minutes = d.num_minutes();
    match minutes {
        ..1 => "< 1 min".to_owned(),
        1..60 => format!("{minutes} min"),
        60..1440 => format!("{} h {} min", minutes / 60, minutes % 60),
        _ => format!("{} T {} h", minutes / 1440, minutes % 1440 / 60),
    }
}

/// Dauer deutsch: „468 ms“, „12,4 s“, „2 min 5 s“, „1 h 2 min“.
pub(super) fn format_ms(ms: f64) -> String {
    if ms < 1000.0 {
        format!("{ms:.0} ms")
    } else if ms < 60_000.0 {
        format!("{:.1} s", ms / 1000.0).replacen('.', ",", 1)
    } else if ms < 3_600_000.0 {
        let seconds = (ms / 1000.0) as u64;
        format!("{} min {} s", seconds / 60, seconds % 60)
    } else {
        let minutes = (ms / 60_000.0) as u64;
        format!("{} h {} min", minutes / 60, minutes % 60)
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeDelta;

    use super::{format_ms, format_uptime, log_sources};
    use crate::ports;
    use crate::testutil::tmp_dir;

    /// Aus `TestFormatUptime`.
    #[test]
    fn laufzeit_wie_go() {
        for (d, want) in [
            (TimeDelta::seconds(30), "< 1 min"),
            (TimeDelta::minutes(13), "13 min"),
            (TimeDelta::minutes(133), "2 h 13 min"),
            (TimeDelta::hours(76), "3 T 4 h"),
        ] {
            assert_eq!(format_uptime(d), want);
        }
    }

    #[test]
    fn dauer_wie_go() {
        for (ms, want) in [
            (0.0, "0 ms"),
            (468.2, "468 ms"),
            (12_400.0, "12,4 s"),
            (125_000.0, "2 min 5 s"),
            (3_720_000.0, "1 h 2 min"),
        ] {
            assert_eq!(format_ms(ms), want);
        }
    }

    #[test]
    fn log_quellen_sortiert_mit_eigenem_log() {
        let root = tmp_dir();
        let logs = root.path().join("logs");
        std::fs::create_dir_all(&logs).unwrap();
        for file in ["zz.jsonl", "aa.jsonl", "notiz.txt"] {
            std::fs::write(logs.join(file), "").unwrap();
        }
        assert_eq!(log_sources(root.path()), ["aa", ports::LOG_NAME, "zz"]);
    }
}
