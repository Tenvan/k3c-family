//! Prüfung der Task-Version vor einem Task-Ziel mit Testmuster (Port von `check_taskversion.go`).

use std::path::Path;
use std::process::Stdio;
use std::sync::LazyLock;
use std::time::Duration;

use k3c_kit::proc::Cmd;
use regex::Regex;
use tokio::io::AsyncReadExt;
use tokio::sync::OnceCell;

use super::targets::CheckTarget;
use crate::services::executable;

/// Kleinste Go-Task-Version, mit der `check_run` Testmuster an Task-Ziele gibt: Task quotet `CLI_ARGS` nach `--` und
/// setzt `GO_TEST_FLAGS` als Text ein (B-866), belegt mit 3.54 (B-868). `requirements.md` nennt dieselbe.
pub(super) const MIN_TASK_VERSION: [u64; 3] = [3, 54, 0];

/// Frist für `task --version`.
const VERSION_TIMEOUT: Duration = Duration::from_secs(10);

static VERSION_RULE: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"v?(\d+)\.(\d+)(?:\.(\d+))?").ok());

/// Merkt das Ergebnis von `task --version` (auch den Fehlerfall) einmal je Workbench-Lauf (B-871). Wird der erste
/// Aufruf abgebrochen (Future fallen gelassen), merkt es nichts; der nächste Lauf fragt neu.
pub(super) type VersionMemo = OnceCell<Result<String, String>>;

async fn output_of(args: &[&str]) -> Result<String, String> {
    let mut line = vec![executable(args[0], Path::new(".")).into_os_string()];
    line.extend(args.iter().skip(1).map(Into::into));
    let mut cmd = Cmd::new(&line);
    cmd.command_mut().stdout(Stdio::piped());
    let mut proc = cmd.start().map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    if let Some(mut stdout) = proc.child_mut().stdout.take() {
        stdout.read_to_end(&mut out).await.map_err(|e| e.to_string())?;
    }
    let status = proc.wait().await.map_err(|e| e.to_string())?;
    if !status.success() {
        return Err(status.to_string());
    }
    Ok(String::from_utf8_lossy(&out).into_owned())
}

/// `task --version` (z. B. „3.54.0“ oder „Task version: v3.54.0 (h1:…)“).
pub(super) async fn read_task_version() -> Result<String, String> {
    tokio::time::timeout(VERSION_TIMEOUT, output_of(&["task", "--version"]))
        .await
        .unwrap_or_else(|_| Err("Zeitlimit".to_owned()))
}

/// Die erste Version `x.y[.z]` im Text.
pub(super) fn parse_version(s: &str) -> Option<[u64; 3]> {
    let found = VERSION_RULE.as_ref()?.captures(s)?;
    let part = |i: usize| found.get(i).map_or(0, |m| m.as_str().parse().unwrap_or(u64::MAX));
    Some([part(1), part(2), part(3)])
}

pub(super) fn version_text(v: [u64; 3]) -> String {
    format!("{}.{}.{}", v[0], v[1], v[2])
}

/// Ob dieser Aufruf die Task-Version braucht: nur Task-Ziele mit Testmuster.
pub(super) fn needs_version(target: &CheckTarget, pattern: &str) -> bool {
    !pattern.is_empty() && target.command.first() == Some(&"task")
}

/// Zu alt: Fehler mit der Mindestversion. Nicht lesbar: Warnung (Text für die Antwort), kein Abbruch.
pub(super) fn check_task_version(
    target: &CheckTarget,
    read: &Result<String, String>,
) -> Result<Option<String>, String> {
    let min = version_text(MIN_TASK_VERSION);
    let Some(found) = read.as_ref().ok().and_then(|out| parse_version(out)) else {
        return Ok(Some(format!(
            "⚠ Task-Version nicht lesbar (task --version); Testmuster brauchen mindestens {min} (requirements.md)"
        )));
    };
    if found < MIN_TASK_VERSION {
        return Err(format!(
            "{} mit Testmuster braucht Go Task ab {min} (gefunden {}), sonst quotet Task das Muster nicht; Task \
             aktualisieren (requirements.md, danach die Workbench neu starten: die Version wird je Lauf gemerkt) oder \
             ohne Muster laufen lassen",
            target.name,
            version_text(found)
        ));
    }
    Ok(None)
}
