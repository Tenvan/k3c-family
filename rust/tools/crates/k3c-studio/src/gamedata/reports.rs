//! `reports_list` und `report_read` (Port von `reports.go`).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::Deserialize;

use super::{json_files, local_minute, read_json};

/// Tasten der Standard-Belegung je Index, wie früher auf der Gamepad-Testseite (B-822).
const BUTTON_NAMES: [&str; 17] = [
    "A", "B", "X", "Y", "LB", "RB", "LT", "RT", "View", "Menu", "LS", "RS", "↑", "↓", "←", "→", "Xbox",
];

/// Ein Name ohne Ordner und ohne führenden Punkt, mit Endung `.json`.
const FILE_NAME: &str = r"\A[A-Za-z0-9_-][A-Za-z0-9_.-]*\.json\z";

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Report {
    created_at: String,
    received_at: String,
    user_agent: String,
    pads: BTreeMap<String, Pad>,
    perf: Vec<Perf>,
    fullscreen_attempts: Vec<Attempt>,
    back_navigations: i64,
    key_events: Vec<String>,
    max_simultaneous_pads: i64,
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Pad {
    id: String,
    mapping: String,
    buttons_seen: Vec<i64>,
}

#[derive(Clone, Copy, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Perf {
    sprites: i64,
    avg_fps: f64,
    min_fps: f64,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Attempt {
    via: String,
    ok: bool,
    error: String,
}

/// Name und Inhalt; `None` = nicht lesbar.
struct Named {
    name: String,
    report: Option<Report>,
}

impl Named {
    fn sort_key(&self) -> &str {
        match &self.report {
            Some(r) if !r.received_at.is_empty() => &r.received_at,
            _ => &self.name,
        }
    }

    fn line(&self) -> String {
        let Some(r) = &self.report else {
            return format!("{} · nicht lesbar", self.name);
        };
        let mut line = format!(
            "{} · {} · {} · {} Controller",
            self.name,
            r.when(),
            r.device(),
            r.pads.len()
        );
        if let Some(best) = r.biggest_stage() {
            line.push_str(&format!(" · {:.0} FPS bei {} Sprites", best.avg_fps, best.sprites));
        }
        line
    }
}

/// `reports_list`: neueste zuerst, eine Zeile je Bericht, unlesbare am Ende.
pub fn reports_list(root: &Path) -> Result<String, String> {
    let mut all: Vec<Named> = json_files(&root.join("reports"))
        .into_iter()
        .map(|(name, path)| Named {
            name,
            report: read_json(&path).ok(),
        })
        .collect();
    if all.is_empty() {
        return Ok("keine Berichte".to_owned());
    }
    all.sort_by(|a, b| {
        let readable = b.report.is_some().cmp(&a.report.is_some());
        readable.then_with(|| b.sort_key().cmp(a.sort_key()))
    });
    Ok(all.iter().map(Named::line).collect::<Vec<_>>().join("\n"))
}

/// `report_read`: ein Bericht in wenigen Zeilen.
pub fn report_read(root: &Path, name: &str) -> Result<String, String> {
    let path = inside(&root.join("reports"), name)?;
    let r: Report = read_json(&path).map_err(|e| format!("{name} nicht lesbar: {e}"))?;
    let mut lines = vec![format!("{name} · {} · {}", r.when(), r.device())];
    for (key, pad) in &r.pads {
        lines.push(format!(
            "Controller {key}: {} ({}) · Tasten {}",
            pad.id,
            pad.mapping,
            buttons(&pad.buttons_seen)
        ));
    }
    lines.push(format!("FPS min/Ø: {}", r.fps()));
    lines.push(format!("Vollbild: {}", r.fullscreen()));
    lines.push(format!(
        "Gleichzeitig max. {} Controller · Zurück-Navigationen {} · Tasten-Ereignisse {}",
        r.max_simultaneous_pads,
        r.back_navigations,
        r.key_events.len()
    ));
    Ok(lines.join("\n"))
}

/// Text in Anführungszeichen, annähernd wie Go `%q`.
fn quoted(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' | '\\' => out.extend(['\\', c]),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_ascii_control() => out.push_str(&format!("\\x{:02x}", u32::from(c))),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", u32::from(c))),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Prüft einen Dateinamen und liefert den Pfad nur, wenn er genau eine Datei in `dir` benennt.
fn inside(dir: &Path, name: &str) -> Result<PathBuf, String> {
    let pattern = Regex::new(FILE_NAME).map_err(|e| e.to_string())?;
    if !pattern.is_match(name) || name.contains("..") {
        return Err(format!(
            "name {} abgelehnt: nur ein Dateiname aus reports/ mit Endung .json",
            quoted(name)
        ));
    }
    let path = dir.join(name);
    if path.strip_prefix(dir).ok() != Some(Path::new(name)) {
        return Err(format!("name {} liegt nicht in reports/", quoted(name)));
    }
    Ok(path)
}

impl Report {
    fn when(&self) -> String {
        [&self.received_at, &self.created_at]
            .into_iter()
            .find_map(|s| local_minute(s))
            .unwrap_or_else(|| "ohne Datum".to_owned())
    }

    /// Inhalt der ersten Klammer im User-Agent, z. B. „Xbox; Xbox One“.
    fn device(&self) -> &str {
        self.user_agent
            .split_once('(')
            .and_then(|(_, rest)| rest.split_once(')'))
            .map_or("unbekanntes Gerät", |(device, _)| device)
    }

    fn biggest_stage(&self) -> Option<Perf> {
        let best = self.perf.iter().fold(
            Perf::default(),
            |best, p| {
                if p.sprites > best.sprites { *p } else { best }
            },
        );
        (!self.perf.is_empty()).then_some(best)
    }

    fn fps(&self) -> String {
        if self.perf.is_empty() {
            return "keine Messung".to_owned();
        }
        let parts: Vec<String> = self
            .perf
            .iter()
            .map(|p| format!("{} Sprites {:.0}/{:.0}", p.sprites, p.min_fps, p.avg_fps))
            .collect();
        parts.join(" · ")
    }

    fn fullscreen(&self) -> String {
        if self.fullscreen_attempts.is_empty() {
            return "kein Versuch".to_owned();
        }
        let parts: Vec<String> = self
            .fullscreen_attempts
            .iter()
            .map(|a| {
                if a.ok {
                    format!("{} → ok", a.via)
                } else {
                    format!("{} → Fehler: {}", a.via, a.error)
                }
            })
            .collect();
        parts.join(" · ")
    }
}

fn buttons(seen: &[i64]) -> String {
    if seen.is_empty() {
        return "keine".to_owned();
    }
    let names: Vec<String> = seen
        .iter()
        .map(|&b| {
            let known = usize::try_from(b).ok().and_then(|i| BUTTON_NAMES.get(i));
            known.map_or_else(|| format!("#{b}"), |name| (*name).to_owned())
        })
        .collect();
    names.join(" ")
}
