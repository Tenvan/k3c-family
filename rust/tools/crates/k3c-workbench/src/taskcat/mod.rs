//! Task-Katalog der Taskfile-Hierarchie (Port von `internal/taskcat`): alle Tasks aus `Taskfile.yml` samt eingebundener
//! Taskfiles, gruppiert nach dem Namensraum vor dem Doppelpunkt.
//!
//! Quelle ist `task --list-all --json`, kein eigener YAML-Parser: go-task löst Includes, Aliase und `internal: true`
//! selbst auf.

mod group;

#[cfg(test)]
mod tests;

use std::io;
use std::path::{Component, Path};
use std::process::{ExitStatus, Stdio};
use std::time::Duration;

use k3c_kit::proc::Cmd;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncReadExt};

pub use group::{Namespace, filter, group};

use crate::services::executable;

/// Namensraum der Tasks ohne Doppelpunkt im Namen (Aggregate aus dem Root-Taskfile).
pub const WORKSPACE: &str = "Workspace";
/// Der Aufruf in Fehlertexten, so wie er sich in einer Shell nachstellen lässt.
const LIST_COMMAND: &str = "task --list-all --json --no-status";

/// Ein Eintrag des Katalogs (`TaskInfo` in `types.ts`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Task {
    /// Vollständiger Name, wie `task <name>` ihn erwartet.
    pub name: String,
    /// Teil vor dem ersten Doppelpunkt, sonst [`WORKSPACE`].
    pub namespace: String,
    /// Rest nach dem ersten Doppelpunkt, sonst der ganze Name.
    pub leaf: String,
    pub desc: String,
    pub summary: String,
    pub aliases: Vec<String>,
    /// Definierendes Taskfile, relativ zur Repo-Wurzel mit `/`.
    pub file: String,
    pub line: i64,
}

/// JSON-Form von `task --list-all --json` (go-task 3.53); unbekannte Felder werden ignoriert, `null` zählt als leer.
#[derive(Deserialize)]
struct ListOutput {
    tasks: Option<Vec<Listed>>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Listed {
    name: Option<String>,
    desc: Option<String>,
    summary: Option<String>,
    aliases: Option<Vec<String>>,
    location: Option<Location>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Location {
    line: i64,
    taskfile: String,
}

/// Ruft `task --list-all --json --no-status` in `root` auf; nach `limit` endet der Prozess.
///
/// `--no-status` ist nötig: ohne das Flag wertet go-task für jeden Task die `sources`-Globs aus (gemessen 47,9 s statt
/// 0,19 s).
pub async fn load(root: &Path, limit: Duration) -> Result<Vec<Task>, String> {
    let program = executable("task", root).into_os_string();
    let mut args = vec![program];
    args.extend(["--list-all", "--json", "--no-status"].map(Into::into));
    let mut cmd = Cmd::new(&args);
    cmd.command_mut()
        .current_dir(root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    match tokio::time::timeout(limit, output(cmd)).await {
        // Beim Fallenlassen endet der Prozessbaum.
        Err(_) => Err(format!("{LIST_COMMAND}: Frist abgelaufen: context deadline exceeded")),
        Ok(Err(e)) if e.kind() == io::ErrorKind::NotFound => Err(format!("\"task\" nicht im PATH: {e}")),
        Ok(Err(e)) => Err(format!("{LIST_COMMAND}: {e}")),
        Ok(Ok((status, out, _))) if status.success() => parse(&out, root),
        Ok(Ok((status, _, err))) => Err(failed(status, &err)),
    }
}

/// Ende mit Fehler: die Fehlerausgabe von task, ohne sie der Exit-Code.
fn failed(status: ExitStatus, stderr: &[u8]) -> String {
    // Die Fehlerausgabe kommt unter Windows nicht zwingend als UTF-8.
    let message = String::from_utf8_lossy(stderr).trim().to_owned();
    if message.is_empty() {
        let code = status.code().unwrap_or(-1);
        return format!("{LIST_COMMAND}: exit status {code}");
    }
    format!("{LIST_COMMAND}: {message}")
}

async fn drain(source: Option<impl AsyncRead + Unpin>) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    if let Some(mut source) = source {
        source.read_to_end(&mut bytes).await?;
    }
    Ok(bytes)
}

async fn output(cmd: Cmd) -> io::Result<(ExitStatus, Vec<u8>, Vec<u8>)> {
    let mut proc = cmd.start()?;
    let (out, err) = (proc.child_mut().stdout.take(), proc.child_mut().stderr.take());
    let (status, out, err) = tokio::join!(proc.wait(), drain(out), drain(err));
    Ok((status?, out?, err?))
}

/// Wandelt die JSON-Ausgabe in Tasks. Taskfile-Pfade werden relativ zu `root` mit `/` geschrieben; ein Pfad außerhalb
/// von `root` bleibt absolut.
pub fn parse(data: &[u8], root: &Path) -> Result<Vec<Task>, String> {
    let out: ListOutput =
        serde_json::from_slice(data).map_err(|e| format!("{LIST_COMMAND}: Ausgabe nicht lesbar: {e}"))?;
    let tasks = out.tasks.unwrap_or_default().into_iter().map(|t| {
        let name = t.name.unwrap_or_default();
        let (namespace, leaf) = split(&name);
        let location = t.location.unwrap_or_default();
        Task {
            namespace,
            leaf,
            desc: t.desc.unwrap_or_default(),
            summary: t.summary.unwrap_or_default(),
            aliases: t.aliases.unwrap_or_default(),
            file: relative(root, &location.taskfile),
            line: location.line,
            name,
        }
    });
    Ok(tasks.collect())
}

/// Trennt am ersten Doppelpunkt: `backend:test:vitest` gehört zu `backend` mit dem Blatt `test:vitest`.
fn split(name: &str) -> (String, String) {
    match name.split_once(':') {
        Some((namespace, leaf)) if !namespace.is_empty() => (namespace.to_owned(), leaf.to_owned()),
        _ => (WORKSPACE.to_owned(), name.to_owned()),
    }
}

fn slashed(path: &str) -> String {
    if cfg!(windows) {
        path.replace('\\', "/")
    } else {
        path.to_owned()
    }
}

/// Wie Go `filepath.Rel` für Pfade unter `root`; unter Windows ohne Groß- und Kleinschreibung.
fn relative(root: &Path, path: &str) -> String {
    let same = |a: Component, b: Component| {
        let (a, b) = (a.as_os_str().to_string_lossy(), b.as_os_str().to_string_lossy());
        if cfg!(windows) {
            a.to_lowercase() == b.to_lowercase()
        } else {
            a == b
        }
    };
    if root.as_os_str().is_empty() || path.is_empty() {
        return slashed(path);
    }
    let mut rest = Path::new(path).components();
    for part in root.components() {
        if !rest.next().is_some_and(|other| same(part, other)) {
            return slashed(path);
        }
    }
    slashed(&rest.as_path().to_string_lossy())
}
