//! Benannte Baselines (Port von `engine/balance/baselines.go`, B-640, BS7.2): Ein durchgelaufener Bericht wird unter einem Namen
//! wie M1 gesichert, mit der Konfiguration des Laufs. Ablage `reports/baseline/<name>/` (`balance.json` = der Bericht,
//! `run.json` = `BaselineInfo`).
//!
//! Abweichung von Go: `saved` schreibt UTC (`2026-11-17T10:00:00Z`), Go die Ortszeit mit Versatz; Rust-std kennt keine
//! Zeitzone, und eine Zeit-Crate braucht eine Freigabe.

use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::evaluate::STATUS_INVALID;
use crate::fehler::Fehler;
use crate::summary::{Summary, read_summary};
use crate::targets::default_targets;

/// Der Ordner der benannten Baselines, relativ zum Checkout.
pub const BASELINE_DIR: &str = "reports/baseline";

/// Eine gesicherte Baseline: Name, Zeitpunkt, Herkunft und Konfiguration des Laufs (Go: `BaselineInfo`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct BaselineInfo {
    pub name: String,
    pub saved: String,
    /// Berichtsordner, aus dem gesichert wurde (`simtest-run-N`).
    pub run: String,
    /// Gespielte Seeds.
    pub seeds: i64,
    /// dev, all oder auswahl.
    pub seed_mode: String,
    pub bots: Vec<String>,
    pub worlds: Vec<String>,
    pub pass: bool,
    /// Eingefrorene Gruppen zum Zeitpunkt des Sicherns.
    pub frozen: Vec<String>,
    /// Der Auftrag des Laufs (`simrun.Spec` als JSON); damit setzt „Übernehmen“ im Editor die Konfiguration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<Value>,
}

fn fail<T>(s: String) -> Result<T, Fehler> {
    Err(Fehler::Daten(s))
}

/// Prüft einen Namen: Buchstaben, Ziffern, `_` und `-`, höchstens 40 Zeichen (Go: `ValidBaselineName`).
pub fn valid_baseline_name(name: &str) -> Result<(), Fehler> {
    let ok = (1..=40).contains(&name.len())
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-');
    if ok {
        Ok(())
    } else {
        fail(format!(
            "baseline {name:?}: Buchstaben, Ziffern, _ und -, höchstens 40 Zeichen"
        ))
    }
}

/// Der Bericht der Baseline, relativ zum Checkout (mit `/`; Go: `BaselineFile`).
pub fn baseline_file(name: &str) -> String {
    format!("{BASELINE_DIR}/{name}/balance.json")
}

fn base_dir(root: &Path) -> PathBuf {
    root.join("reports").join("baseline")
}

/// Die benannte Baseline liegt im Checkout (Go: `HasBaseline`).
pub fn has_baseline(root: &Path, name: &str) -> bool {
    base_dir(root).join(name).join("balance.json").exists()
}

/// Sichert den Bericht `report_path` (`balance-*.json`) unter `name`. Nur ein durchgelaufener Lauf (`pass`) wird gesichert,
/// außer `open` ist gesetzt (offener Meilenstein: der Lauf ist gültig, hat aber sein Ziel nicht erreicht); ein vergebener Name
/// wird nie überschrieben. `spec` ist der Auftrag des Laufs (Go: `SaveBaseline`).
pub fn save_baseline(
    root: &Path,
    name: &str,
    report_path: &Path,
    spec: Option<Value>,
    open: bool,
) -> Result<BaselineInfo, Fehler> {
    valid_baseline_name(name)?;
    let io = |e: std::io::Error| Fehler::Daten(e.to_string());
    let raw = std::fs::read_to_string(report_path).map_err(io)?;
    let file = report_path
        .file_name()
        .map_or(String::new(), |f| f.to_string_lossy().into_owned());
    let s = read_summary(&raw).map_err(|e| Fehler::Daten(format!("{file}: {e}")))?;
    if !s.pass && !open {
        return fail(format!(
            "baseline {name}: der Lauf hat sein Ziel nicht erreicht (Pass fehlt); gesichert werden nur durchgelaufene Läufe, bei einem offenen Meilenstein ohnePass setzen"
        ));
    }
    if s.seeds == 0 || !s.targets.iter().any(|e| e.verdict.status != STATUS_INVALID) {
        return fail(format!(
            "baseline {name}: der Bericht hat keine gültige Messung (Lauf abgestürzt?)"
        ));
    }
    let base = base_dir(root);
    let dir = base.join(name);
    let run = report_path
        .parent()
        .and_then(Path::file_name)
        .map_or(String::new(), |f| f.to_string_lossy().into_owned());
    let mut info = info_of(name, &run, &s);
    info.spec = spec;
    let meta = serde_json::to_string_pretty(&info).map_err(|e| Fehler::Daten(e.to_string()))?;
    std::fs::create_dir_all(&base).map_err(io)?;
    if let Err(e) = std::fs::create_dir(&dir) {
        // atomar: genau ein Aufruf legt den Namen an
        if e.kind() == ErrorKind::AlreadyExists {
            return fail(format!(
                "baseline {name} gibt es schon ({BASELINE_DIR}/{name}); anderer Name oder Ordner von Hand löschen"
            ));
        }
        return Err(io(e));
    }
    // run.json zuletzt: ein Ordner ohne run.json ist ein Fehlschlag und wird entfernt
    let written =
        std::fs::write(dir.join("balance.json"), &raw).and_then(|()| std::fs::write(dir.join("run.json"), meta));
    if let Err(e) = written {
        let _ = std::fs::remove_dir_all(&dir);
        return Err(io(e));
    }
    Ok(info)
}

fn info_of(name: &str, run: &str, s: &Summary) -> BaselineInfo {
    let mut info = BaselineInfo {
        name: name.into(),
        saved: rfc3339_utc(SystemTime::now()),
        run: run.into(),
        seeds: s.seeds,
        seed_mode: s.seed_mode.clone(),
        pass: s.pass,
        ..BaselineInfo::default()
    };
    for e in &s.targets {
        let t = &e.verdict.target;
        if !info.bots.contains(&t.bot) {
            info.bots.push(t.bot.clone());
        }
        if !t.world.is_empty() && !info.worlds.contains(&t.world) {
            info.worlds.push(t.world.clone());
        }
    }
    if let Ok(ts) = default_targets() {
        info.frozen = ts.frozen.clone();
    }
    info
}

/// Die Zeit als RFC 3339 in UTC, `2026-11-17T10:00:00Z` (Go: `time.Now().Format(time.RFC3339)` in Ortszeit).
pub fn rfc3339_utc(t: SystemTime) -> String {
    let [year, month, day, h, min, s] = utc_parts(t);
    format!("{year:04}-{month:02}-{day:02}T{h:02}:{min:02}:{s:02}Z")
}

/// Jahr, Monat, Tag, Stunde, Minute, Sekunde in UTC (ohne Zeitzone; Rust-std kennt keine, Abweichung zu Go: dort Ortszeit).
pub fn utc_parts(t: SystemTime) -> [i64; 6] {
    let secs = t.duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64);
    let (days, rest) = (secs.div_euclid(86400), secs.rem_euclid(86400));
    // Tage seit 1970-01-01 → Datum (Howard Hinnants `civil_from_days`)
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    [year, month, day, rest / 3600, rest % 3600 / 60, rest % 60]
}

/// Liest die gesicherten Baselines, nach Name sortiert; ein Ordner ohne lesbares `run.json` fehlt in der Liste (Go: `ListBaselines`).
pub fn list_baselines(root: &Path) -> Result<Vec<BaselineInfo>, Fehler> {
    let base = base_dir(root);
    let entries = match std::fs::read_dir(&base) {
        Ok(e) => e,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(Fehler::Daten(e.to_string())),
    };
    let mut out = Vec::new();
    for d in entries.flatten() {
        let dir_name = d.file_name().to_string_lossy().into_owned();
        let Ok(raw) = std::fs::read_to_string(base.join(&dir_name).join("run.json")) else {
            continue;
        };
        if let Ok(info) = serde_json::from_str::<BaselineInfo>(&raw)
            && info.name == dir_name
        {
            out.push(info);
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}
