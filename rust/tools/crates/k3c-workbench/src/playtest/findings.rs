//! Befunde (Port von `findings.go`, B-1224): Claude liest Transkript, Marker und Bilder per MCP und schlägt Befunde vor
//! (`findings.json`). Den Haken (`selected`), die Ticket-ID und den Fehler setzt nie der Vorschlag, sondern nur die
//! Oberfläche ([`Store::update_finding`]) bzw. die Ticket-Anlage.

use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};

use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{Error, FILE_FINDINGS, Store, go_json, lock, null_default};
use crate::planning::store::quoted;

/// Ein Befund.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Finding {
    #[serde(deserialize_with = "null_default")]
    pub id: String,
    /// Titel des Tickets als Aussage.
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub title: String,
    /// Zitat aus dem Transkript (oder Beobachtung im Bild).
    #[serde(deserialize_with = "null_default")]
    pub quote: String,
    /// Zeitpunkt, Unix-ms.
    #[serde(deserialize_with = "null_default")]
    pub ts: i64,
    /// Bild relativ zur Session, z. B. `frames/marker-2.jpg`.
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub frame: String,
    /// Spielkontext (Welt, Insel, Stufe, Tageszeit, Spieler).
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub context: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub domain: String,
    #[serde(
        rename = "type",
        skip_serializing_if = "String::is_empty",
        deserialize_with = "null_default"
    )]
    pub kind: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub prio: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub project: String,
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub sprint: String,
    /// Offenes Ticket B-123, sonst leer.
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub duplicate_of: String,
    /// Haken von Ralf.
    #[serde(deserialize_with = "null_default")]
    pub selected: bool,
    /// ID nach der Anlage.
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub ticket: String,
    /// Grund, wenn die Anlage scheiterte.
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub error: String,
}

/// Ein Vorschlag von Claude (ohne Haken, Ticket und Fehler).
#[derive(Clone, Debug, Default, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FindingIn {
    #[schemars(description = "Kennung des Befunds, a-z A-Z 0-9 - _ (höchstens 32), z. B. f1")]
    pub id: String,
    #[serde(default)]
    #[schemars(description = "Titel des Tickets als Aussage")]
    pub title: String,
    #[schemars(description = "Zitat aus dem Transkript")]
    pub quote: String,
    #[schemars(description = "Zeitpunkt in Unix-ms (aus playtest_get)")]
    pub ts: i64,
    #[serde(default)]
    #[schemars(description = "Bild relativ zur Session, z. B. frames/marker-2.jpg")]
    pub frame: String,
    #[serde(default)]
    #[schemars(description = "Spielkontext: Welt, Insel, Stufe, Tageszeit, Spieler")]
    pub context: String,
    #[serde(default)]
    #[schemars(description = "Vorschlag Domäne des Tickets (REG, SIM, SRV, CLI, PLAT, INF, DEV, PAP)")]
    pub domain: String,
    #[serde(default, rename = "type")]
    #[schemars(description = "Vorschlag Typ: Idee, Problem, Schuld oder Frage")]
    pub kind: String,
    #[serde(default)]
    #[schemars(description = "Vorschlag Prio: hoch, mittel, niedrig oder ?")]
    pub prio: String,
    #[serde(default)]
    #[schemars(description = "Vorschlag Projekt-Kürzel")]
    pub project: String,
    #[serde(default)]
    #[schemars(description = "Vorschlag Sprint-Kürzel")]
    pub sprint: String,
    #[serde(default)]
    #[schemars(
        description = "offenes Ticket B-123, das denselben Befund beschreibt (geprüft mit plan_list), sonst leer"
    )]
    pub duplicate_of: String,
}

/// Was Ralf auf der Seite ändert: Haken und Vorschlag für Domäne, Typ, Prio, Projekt, Sprint.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct FindingPatch {
    pub selected: Option<bool>,
    pub domain: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub prio: Option<String>,
    pub project: Option<String>,
    pub sprint: Option<String>,
}

static FINDING_ID: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^[A-Za-z0-9_-]{1,32}$").ok());
static TICKET_ID: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^B-[0-9]+$").ok());

/// Lesen und Schreiben von `findings.json` laufen nie nebeneinander.
pub(super) static FINDINGS: Mutex<()> = Mutex::new(());

const TYPES: [&str; 5] = ["", "Idee", "Problem", "Schuld", "Frage"];
const PRIOS: [&str; 5] = ["", "hoch", "mittel", "niedrig", "?"];

fn matches(re: &LazyLock<Option<Regex>>, text: &str) -> bool {
    re.as_ref().is_some_and(|re| re.is_match(text))
}

impl FindingIn {
    fn validate(&self) -> Result<(), Error> {
        let id = &self.id;
        let reason = if !matches(&FINDING_ID, id) {
            format!(": id {} (a-z, A-Z, 0-9, -, _; höchstens 32 Zeichen)", quoted(id))
        } else if self.quote.trim().is_empty() {
            format!(" {id}: quote fehlt")
        } else if !TYPES.contains(&self.kind.as_str()) {
            format!(" {id}: type {} (Idee, Problem, Schuld, Frage)", quoted(&self.kind))
        } else if !PRIOS.contains(&self.prio.as_str()) {
            format!(" {id}: prio {} (hoch, mittel, niedrig, ?)", quoted(&self.prio))
        } else if !self.duplicate_of.is_empty() && !matches(&TICKET_ID, &self.duplicate_of) {
            let shown = quoted(&self.duplicate_of);
            format!(" {id}: duplicate_of {shown} ist keine Ticket-ID (B-123)")
        } else if self.frame.contains(['\\', ':']) || self.frame.contains("..") {
            format!(" {id}: frame muss ein Pfad in der Session sein")
        } else {
            return Ok(());
        };
        Err(Error::BadFinding(reason))
    }

    /// Neuer Befund ohne Haken.
    fn finding(&self) -> Finding {
        let v = self.clone();
        Finding {
            id: v.id,
            title: v.title,
            quote: v.quote,
            ts: v.ts,
            frame: v.frame,
            context: v.context,
            domain: v.domain,
            kind: v.kind,
            prio: v.prio,
            project: v.project,
            sprint: v.sprint,
            duplicate_of: v.duplicate_of,
            ..Finding::default()
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
struct File<T> {
    findings: T,
}

/// Liest `findings.json`; fehlt die Datei, ist die Liste leer.
pub(super) fn read_findings(dir: &Path) -> Result<Vec<Finding>, Error> {
    let raw = match std::fs::read(dir.join(FILE_FINDINGS)) {
        Ok(raw) => raw,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(Error::Other(e.to_string())),
    };
    serde_json::from_slice::<File<Option<Vec<Finding>>>>(&raw)
        .map(|f| f.findings.unwrap_or_default())
        .map_err(|e| Error::Other(format!("findings.json nicht lesbar: {e}")))
}

/// Schreibt `findings.json` atomar.
pub(super) fn write_findings(dir: &Path, list: &[Finding]) -> Result<(), Error> {
    let failed = |e: &dyn std::fmt::Display| Error::Other(e.to_string());
    let raw = go_json(&File { findings: list }).map_err(|e| failed(&e))?;
    let tmp = dir.join(format!("{FILE_FINDINGS}.tmp"));
    std::fs::write(&tmp, raw).map_err(|e| failed(&e))?;
    std::fs::rename(&tmp, dir.join(FILE_FINDINGS)).map_err(|e| failed(&e))
}

impl Store {
    /// Prüft die Kennung und liefert den Ordner einer vorhandenen Session.
    pub(super) fn session_dir(&self, id: &str) -> Result<PathBuf, Error> {
        Ok(self.root.join(self.get(id)?.id))
    }

    /// Die Befunde einer Session.
    pub fn findings(&self, id: &str) -> Result<Vec<Finding>, Error> {
        read_findings(&self.session_dir(id)?)
    }

    /// Schreibt oder aktualisiert Befunde nach `id`. Haken, Ticket und Fehler eines vorhandenen Befunds bleiben; ein
    /// neuer beginnt ohne Haken. Erst werden alle geprüft, bei einem Fehler wird nichts geschrieben. Ergebnis: neue und
    /// aktualisierte Befunde.
    pub fn set_findings(&self, id: &str, input: &[FindingIn]) -> Result<(usize, usize), Error> {
        let dir = self.session_dir(id)?;
        if input.is_empty() {
            return Err(Error::BadFinding(": keine Befunde angegeben".to_owned()));
        }
        let mut seen = std::collections::BTreeSet::new();
        for finding in input {
            finding.validate()?;
            if !seen.insert(&finding.id) {
                return Err(Error::BadFinding(format!(": id {} doppelt", finding.id)));
            }
        }
        let _writing = lock(&FINDINGS);
        let mut list = read_findings(&dir)?;
        let (mut added, mut updated) = (0, 0);
        for proposal in input {
            let mut new = proposal.finding();
            match list.iter_mut().rev().find(|f| f.id == proposal.id) {
                Some(old) => {
                    new.selected = old.selected;
                    new.ticket = std::mem::take(&mut old.ticket);
                    new.error = std::mem::take(&mut old.error);
                    *old = new;
                    updated += 1;
                }
                None => {
                    list.push(new);
                    added += 1;
                }
            }
        }
        write_findings(&dir, &list)?;
        Ok((added, updated))
    }

    /// Wendet die Änderung von der Seite auf einen Befund an; ein Befund mit Ticket bleibt unverändert.
    pub fn update_finding(&self, id: &str, finding: &str, patch: &FindingPatch) -> Result<Finding, Error> {
        let dir = self.session_dir(id)?;
        let _writing = lock(&FINDINGS);
        let mut list = read_findings(&dir)?;
        let Some(found) = list.iter_mut().find(|f| f.id == finding) else {
            let text = format!("{}: Befund {finding} gibt es nicht", Error::NotFound);
            return Err(Error::Other(text));
        };
        if !found.ticket.is_empty() {
            let reason = format!(" {finding}: Ticket {} ist schon angelegt", found.ticket);
            return Err(Error::BadFinding(reason));
        }
        if let Some(selected) = patch.selected {
            found.selected = selected;
        }
        for (target, value) in [
            (&mut found.domain, &patch.domain),
            (&mut found.kind, &patch.kind),
            (&mut found.prio, &patch.prio),
            (&mut found.project, &patch.project),
            (&mut found.sprint, &patch.sprint),
        ] {
            if let Some(value) = value {
                value.trim().clone_into(target);
            }
        }
        if !TYPES.contains(&found.kind.as_str()) || !PRIOS.contains(&found.prio.as_str()) {
            return Err(Error::BadFinding(format!(" {finding}: Typ oder Prio ungültig")));
        }
        let changed = found.clone();
        write_findings(&dir, &list)?;
        Ok(changed)
    }
}

#[cfg(test)]
#[path = "findings_tests.rs"]
mod tests;
