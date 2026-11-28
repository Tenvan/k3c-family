//! Die Commit-Seite der Workbench (Port von `internal/gitcommit`): Stand von Index und Arbeitsbaum, Staging und der
//! Commit mit einer Nachricht im Format des Repos (`typ(domäne): Betreff`, `docs/arbeitsweise.md` › Commit-Titel).
//! Verändert wird nur der Index und die Historie, nie der Arbeitsbaum: kein Discard, kein Checkout, kein Stash, kein
//! Push. git läuft ohne Shell mit festen Argumenten; Pfade und Nachricht gehen über stdin.

mod paths;
mod status;

#[cfg(test)]
mod gitcommit_tests;

use std::fmt;
use std::io;
use std::path::PathBuf;

use k3c_kit::proc::Cmd;
use serde::{Deserialize, Serialize};

use crate::capture;
use crate::gitops::EnvChange;

pub use paths::check_paths;
pub use status::{File, State, parse_status};

/// Die erlaubten Commit-Typen.
pub const TYPES: [&str; 10] = [
    "feat", "fix", "docs", "refactor", "test", "perf", "chore", "ci", "build", "style",
];

/// Domänen-Kürzel aus der Arbeitsweise; der Scope ist frei, diese schlägt eine Oberfläche vor.
pub const DOMAINS: [&str; 6] = ["reg", "sim", "srv", "cli", "plat", "inf"];

/// Höchste Länge des Betreffs (die erste Zeile soll in die Log-Ansicht passen).
pub const MAX_SUBJECT: usize = 72;

/// Eingabe des Formulars.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Message {
    #[serde(rename = "type")]
    pub kind: String,
    pub scope: String,
    pub subject: String,
    pub body: String,
}

/// Nennt das Feld, das die Prüfung verletzt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldError {
    pub field: &'static str,
    pub msg: String,
}

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.msg)
    }
}

fn scope_ok(scope: &str) -> bool {
    let plain = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit();
    scope.starts_with(plain) && scope.chars().all(|c| plain(c) || c == '-')
}

impl Message {
    /// Trimmt die Eingabe und prüft sie in Formularreihenfolge; gemeldet wird der erste Verstoß.
    pub fn normalize(&self) -> Result<Message, FieldError> {
        let m = Message {
            kind: self.kind.trim().to_owned(),
            scope: self.scope.trim().to_owned(),
            subject: self.subject.trim().to_owned(),
            body: self.body.trim().to_owned(),
        };
        let fail = |field, msg: String| Err(FieldError { field, msg });
        if !TYPES.contains(&m.kind.as_str()) {
            return fail("type", format!("unbekannter Typ, erlaubt: {}", TYPES.join(", ")));
        }
        if !m.scope.is_empty() && !scope_ok(&m.scope) {
            return fail("scope", "nur Kleinbuchstaben, Ziffern und Bindestrich".to_owned());
        }
        if m.subject.is_empty() {
            return fail("subject", "darf nicht leer sein".to_owned());
        }
        if m.subject.contains(['\r', '\n']) {
            return fail("subject", "genau eine Zeile".to_owned());
        }
        if m.subject.chars().count() > MAX_SUBJECT {
            return fail("subject", format!("höchstens {MAX_SUBJECT} Zeichen"));
        }
        Ok(m)
    }

    /// Die Nachricht: Kopfzeile, bei Text eine Leerzeile und der Rumpf.
    pub fn format(&self) -> String {
        let mut head = self.kind.clone();
        if !self.scope.is_empty() {
            head.push_str(&format!("({})", self.scope));
        }
        head.push_str(&format!(": {}", self.subject));
        if self.body.is_empty() {
            return format!("{head}\n");
        }
        format!("{head}\n\n{}\n", self.body)
    }
}

/// git in einem Checkout.
pub struct Git {
    root: PathBuf,
    env: Vec<EnvChange>,
}

impl Git {
    /// git in `root` mit der geerbten Umgebung.
    pub fn new(root: PathBuf) -> Git {
        Git::with_env(root, Vec::new())
    }

    /// Wie [`Git::new`] mit Änderungen an der Umgebung (Tests schotten damit ab).
    pub fn with_env(root: PathBuf, env: Vec<EnvChange>) -> Git {
        Git { root, env }
    }

    /// Ein Aufruf von git; ein Fehler trägt die ganze Ausgabe von stderr (auch die der Hooks).
    pub async fn run(&self, stdin: Option<Vec<u8>>, args: &[&str]) -> Result<Vec<u8>, String> {
        let mut line = vec!["git"];
        line.extend_from_slice(args);
        let mut cmd = Cmd::new(&line);
        let command = cmd.command_mut();
        command.current_dir(&self.root);
        for (name, value) in &self.env {
            match value {
                Some(value) => command.env(name, value),
                None => command.env_remove(name),
            };
        }
        let sub = args.first().copied().unwrap_or_default();
        match capture::output(cmd, stdin).await {
            Err(e) if e.kind() == io::ErrorKind::NotFound => Err(format!("git {sub}: git nicht gefunden")),
            Err(e) => Err(format!("git {sub}: {e}")),
            Ok((status, out, _)) if status.success() => Ok(out),
            Ok((status, _, err)) => match String::from_utf8_lossy(&err).trim() {
                "" => Err(format!("git {sub}: exit status {}", status.code().unwrap_or(-1))),
                message => Err(format!("git {sub}: {message}")),
            },
        }
    }

    /// Branch, Index, Arbeitsbaum und die letzten Commits.
    pub async fn read(&self) -> Result<State, String> {
        let args = ["status", "--porcelain=v1", "-z", "--untracked-files=all", "--branch"];
        let mut state = parse_status(&self.run(None, &args).await?)?;
        // Vor dem ersten Commit gibt es kein HEAD; die Liste bleibt dann leer.
        if let Ok(log) = self.run(None, &["log", "--oneline", "-n", "8"]).await {
            let log = String::from_utf8_lossy(&log);
            state.recent = log
                .trim()
                .split('\n')
                .filter(|l| !l.is_empty())
                .map(str::to_owned)
                .collect();
        }
        Ok(state)
    }

    /// Ein Befehl über die genannten Pfade: NUL-getrennt über stdin, ohne Glob-Auswertung und ohne Längengrenze.
    async fn with_paths(&self, command: &[&str], paths: &[String]) -> Result<(), String> {
        let mut args = vec!["--literal-pathspecs"];
        args.extend_from_slice(command);
        args.extend(["--pathspec-from-file=-", "--pathspec-file-nul"]);
        let input = paths.iter().flat_map(|p| p.bytes().chain([0])).collect();
        self.run(Some(input), &args).await?;
        Ok(())
    }

    /// Nimmt Dateien in den Index auf.
    pub async fn stage(&self, paths: &[String]) -> Result<(), String> {
        check_paths(paths)?;
        self.with_paths(&["add"], paths).await
    }

    /// Nimmt Dateien aus dem Index. Vor dem ersten Commit fehlt HEAD, dort hilft nur `rm --cached`.
    pub async fn unstage(&self, paths: &[String]) -> Result<(), String> {
        check_paths(paths)?;
        let Err(error) = self.with_paths(&["restore", "--staged"], paths).await else {
            return Ok(());
        };
        if self.run(None, &["rev-parse", "--verify", "-q", "HEAD"]).await.is_ok() {
            return Err(error);
        }
        self.with_paths(&["rm", "--cached", "-r"], paths).await
    }

    /// Committet den Index und liefert den kurzen Hash. Ein leerer Index wird abgelehnt, bevor git es tut. Hooks laufen
    /// (kein `--no-verify`); scheitert einer, steht seine Ausgabe im Fehler.
    pub async fn commit(&self, message: &Message) -> Result<String, String> {
        let message = message.normalize().map_err(|e| e.to_string())?;
        let staged = self.run(None, &["diff", "--cached", "--name-only", "-z"]).await?;
        if staged.iter().all(|b| *b == 0) {
            return Err("nichts gestaged".to_owned());
        }
        self.run(Some(message.format().into_bytes()), &["commit", "-F", "-"])
            .await?;
        let hash = self.run(None, &["rev-parse", "--short", "HEAD"]).await?;
        Ok(String::from_utf8_lossy(&hash).trim().to_owned())
    }
}
