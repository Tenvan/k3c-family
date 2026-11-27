//! `services.json` lesen und prüfen (Port von `config.go`). Die Datei wird nur gelesen.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::Deserialize;

use super::filewatch::Watch;

/// Gesund, solange der Prozess läuft (ohne Port, etwa das Spiel).
pub const HEALTH_PROCESS: &str = "process";

/// Ein Eintrag aus `services.json`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
pub struct Service {
    pub name: String,
    /// Ein Satz: wofür der Dienst da ist (Oberfläche).
    pub description: String,
    /// Einordnung für die Oberfläche: Spiel/Tool, Backend/Client.
    pub tags: Vec<String>,
    /// Programm und Argumente, ohne Shell gestartet.
    pub command: Vec<String>,
    /// Relativ zur Repo-Wurzel.
    pub cwd: String,
    /// 0 nur bei health process.
    pub port: i64,
    /// http, tcp oder process.
    pub health: String,
    pub log: String,
    pub auto_restart: bool,
    pub env: BTreeMap<String, String>,
    /// Umgebungsvariable, in der jeder Dienst den Port dieses Dienstes bekommt ([`shift`]).
    pub port_env: String,
    /// Bei Änderungen dieser Dateien startet die Workbench den Dienst neu; `None` = nie.
    pub watch: Option<Watch>,
    /// Nur einzeln starten, nie über „alle starten“.
    pub manual: bool,
    /// Datei relativ zur Repo-Wurzel, ohne die der Start mit `needs_hint` abbricht.
    pub needs: String,
    pub needs_hint: String,
    /// Tasks, vor deren Start die Workbench den laufenden Dienst beendet.
    pub stop_before: Vec<String>,
}

impl Service {
    /// Adresse der Prüfung, wie sie die Karten zeigen; leer bei health process.
    pub fn health_url(&self) -> String {
        match self.health.as_str() {
            HEALTH_PROCESS => String::new(),
            "tcp" => format!("tcp://127.0.0.1:{}", self.port),
            _ => format!("http://127.0.0.1:{}/", self.port),
        }
    }

    /// Port als Netzwerk-Port; die Prüfung der Konfiguration lässt nur 0 bis 65535 zu.
    pub(crate) fn net_port(&self) -> u16 {
        u16::try_from(self.port).unwrap_or_default()
    }
}

/// Liest und prüft die Konfiguration; unbekannte Felder sind ein Fehler.
pub fn load(path: &Path) -> Result<Vec<Service>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("open {}: {e}", path.display()))?;
    let list: Vec<Service> = serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    validate(path, &list)?;
    Ok(list)
}

fn validate(path: &Path, list: &[Service]) -> Result<(), String> {
    let (mut names, mut ports) = (BTreeSet::new(), BTreeSet::new());
    for (i, s) in list.iter().enumerate() {
        check(s, &names, &ports).map_err(|e| format!("{}: Dienst {} ({:?}): {e}", path.display(), i + 1, s.name))?;
        names.insert(s.name.as_str());
        if s.port != 0 {
            ports.insert(s.port);
        }
    }
    Ok(())
}

fn check(s: &Service, names: &BTreeSet<&str>, ports: &BTreeSet<i64>) -> Result<(), String> {
    check_core(s)?;
    check_optional(s)?;
    if names.contains(s.name.as_str()) {
        return Err("name doppelt".to_owned());
    }
    if ports.contains(&s.port) {
        return Err(format!("port {} doppelt", s.port));
    }
    Ok(())
}

/// 1 bis `max` Zeichen aus A–Z, a–z, 0–9 und `extra`.
fn made_of(text: &str, extra: &str, max: usize) -> bool {
    !text.is_empty() && text.len() <= max && text.chars().all(|c| c.is_ascii_alphanumeric() || extra.contains(c))
}

/// Dienst- bzw. Log-Name: Konsolen-Quelle, MCP-Argument und Dateiname unter `logs/`.
fn is_name(text: &str) -> bool {
    made_of(text, "_.-", 40)
}

fn is_env_name(text: &str) -> bool {
    text.starts_with(|c: char| c.is_ascii_uppercase())
        && text.len() <= 64
        && text
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
}

/// Name, Befehl und Gesundheitsprüfung.
fn check_core(s: &Service) -> Result<(), String> {
    if !is_name(&s.name) {
        return Err("name fehlt oder enthält andere Zeichen als A–Z, a–z, 0–9, _ . - (höchstens 40)".to_owned());
    }
    if s.command.first().is_none_or(String::is_empty) {
        return Err("command fehlt".to_owned());
    }
    if !["http", "tcp", HEALTH_PROCESS].contains(&s.health.as_str()) {
        return Err(format!("health {:?}: erlaubt sind http, tcp und process", s.health));
    }
    check_process(s)
}

/// Prozess-Dienste: kein Port, `needs` relativ zur Wurzel, gültige Task-Namen.
fn check_process(s: &Service) -> Result<(), String> {
    if s.health == HEALTH_PROCESS && s.port != 0 {
        return Err(format!("health process läuft ohne port (port {})", s.port));
    }
    if !s.needs.is_empty() && !rel_path(&s.needs) {
        return Err(format!(
            "needs {:?} muss ein Pfad relativ zur Repo-Wurzel sein (mit /, ohne ..)",
            s.needs
        ));
    }
    match s.stop_before.iter().find(|t| !made_of(t, "_.:-", 80)) {
        Some(t) => Err(format!("stopBefore {t:?} ist kein Task-Name")),
        None => Ok(()),
    }
}

/// Port, Log, `portEnv` und `watch`.
fn check_optional(s: &Service) -> Result<(), String> {
    if s.health != HEALTH_PROCESS && !(1..=65535).contains(&s.port) {
        return Err(format!("port {} ungültig", s.port));
    }
    if !s.log.is_empty() && !is_name(&s.log) {
        return Err(format!("log {:?} ist kein gültiger Name", s.log));
    }
    if !s.port_env.is_empty() && !is_env_name(&s.port_env) {
        return Err(format!(
            "portEnv {:?} ist kein Variablenname wie K3C_HTTP_PORT",
            s.port_env
        ));
    }
    check_watch(s.watch.as_ref())
}

/// Ob der bereinigte Pfad (Go `path.Clean`) die Wurzel verlässt.
fn leaves_root(p: &str) -> bool {
    let mut depth = 0_usize;
    for part in p.split('/') {
        match part {
            "" | "." => {}
            ".." if depth == 0 => return true,
            ".." => depth -= 1,
            _ => depth += 1,
        }
    }
    false
}

/// Pfad unterhalb der Repo-Wurzel, mit `/` und ohne `..`.
fn rel_path(p: &str) -> bool {
    !p.is_empty() && !p.starts_with('/') && !p.contains(['\\', ':']) && !leaves_root(p)
}

/// `watch`: nur Pfade unterhalb der Repo-Wurzel, Endungen mit Punkt.
fn check_watch(w: Option<&Watch>) -> Result<(), String> {
    let Some(w) = w else { return Ok(()) };
    if w.paths.is_empty() || w.paths.len() > 16 {
        return Err("watch.paths: 1 bis 16 Pfade erwartet".to_owned());
    }
    for p in &w.paths {
        if p.is_empty() || p.starts_with('/') || p.contains('\\') || leaves_root(p) {
            return Err(format!(
                "watch.paths: {p:?} muss ein Pfad relativ zur Repo-Wurzel sein (mit /, ohne ..)"
            ));
        }
    }
    let bad = |e: &&String| e.len() < 2 || !e.starts_with('.') || e[1..].contains(['.', '/', '\\', '*', '?']);
    match w.ext.iter().find(bad) {
        Some(e) => Err(format!("watch.ext: {e:?} ist keine Endung wie \".go\"")),
        None => Ok(()),
    }
}

/// Verschiebt alle Ports um `off` (Dienste eines Worktrees) und gibt jedem Dienst die Ports aller Dienste mit
/// `portEnv` mit. `off` 0 ändert nur die Umgebung.
pub fn shift(list: &[Service], off: i64) -> Vec<Service> {
    let ports: BTreeMap<String, String> = list
        .iter()
        .filter(|s| !s.port_env.is_empty())
        .map(|s| (s.port_env.clone(), (s.port + off).to_string()))
        .collect();
    list.iter()
        .map(|s| {
            let mut out = s.clone();
            // Prozess-Dienste haben keinen Port.
            if out.port != 0 {
                out.port += off;
            }
            out.env.extend(ports.clone());
            out
        })
        .collect()
}
