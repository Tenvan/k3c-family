//! Das Repo und der Aufruf von git (Port von `repo.go`).

use std::collections::HashMap;
use std::future::Future;
use std::io;
use std::path::PathBuf;
use std::pin::Pin;
use std::process::{ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use k3c_kit::proc::Cmd;
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::sync::OnceCell;

use super::merged::Verdict;

/// Ziel-Branch aller PRs; lässt sich nicht löschen und nicht pushen.
pub const BASE: &str = "main";
/// Release-Branch; ebenso geschützt.
pub const MAIN: &str = "main";
/// [`BASE`] auf origin.
pub(super) const BASE_REMOTE: &str = "origin/main";
const TIMEOUT: Duration = Duration::from_secs(60);

/// Ergebnis eines laufenden [`Runner`]-Aufrufs.
pub type RunFuture = Pin<Box<dyn Future<Output = Result<Vec<u8>, String>> + Send>>;
/// Führt git mit den Argumenten (ohne „git“) im Repo aus und liefert stdout; ein Fehler trägt die letzte Zeile von
/// stderr.
pub type Runner = Arc<dyn Fn(Vec<String>) -> RunFuture + Send + Sync>;
/// Änderung an der Umgebung von git: `Some` setzt, `None` entfernt.
pub type EnvChange = (String, Option<String>);

/// Das Repo unter einer Wurzel. Klone teilen sich Sperre und Gedächtnis.
#[derive(Clone)]
pub struct Repo(pub(super) Arc<Inner>);

pub(super) struct Inner {
    pub(super) root: PathBuf,
    run: Runner,
    /// Eine schreibende Aktion zur Zeit.
    pub(super) write: tokio::sync::Mutex<()>,
    /// Merge-Zustand je „branch-sha main-sha“ (`merged.rs`).
    pub(super) memo: Mutex<HashMap<String, Verdict>>,
    /// Gemeinsamer Git-Ordner für den Stempel (`stamp.rs`).
    pub(super) dir: OnceCell<Result<PathBuf, String>>,
}

impl Repo {
    /// Repo für `root` mit dem echten git.
    pub fn new(root: PathBuf) -> Repo {
        let run = exec_runner(root.clone(), Vec::new());
        Repo::with_runner(root, run)
    }

    /// Repo mit eigenem Runner (Tests).
    pub fn with_runner(root: PathBuf, run: Runner) -> Repo {
        Repo(Arc::new(Inner {
            root,
            run,
            write: tokio::sync::Mutex::new(()),
            memo: Mutex::new(HashMap::new()),
            dir: OnceCell::new(),
        }))
    }

    /// git mit Frist; stdout ohne Zeilenende am Schluss.
    pub(super) async fn git(&self, args: &[&str]) -> Result<String, String> {
        let owned: Vec<String> = args.iter().map(|a| (*a).to_owned()).collect();
        // Beim Fallenlassen endet der Prozessbaum.
        let out = tokio::time::timeout(TIMEOUT, (self.0.run)(owned))
            .await
            .map_err(|_| format!("{}: keine Antwort (Netz?)", subcommand(args.first().copied())))??;
        Ok(String::from_utf8_lossy(&out).trim_end_matches(['\r', '\n']).to_owned())
    }
}

/// Der echte Aufruf von git in `root`; `env` kommt zusätzlich zur geerbten Umgebung (Tests schotten damit ab).
pub fn exec_runner(root: PathBuf, env: Vec<EnvChange>) -> Runner {
    let env = Arc::new(env);
    Arc::new(move |args: Vec<String>| {
        let (root, env) = (root.clone(), Arc::clone(&env));
        Box::pin(async move { exec(root, args, &env).await })
    })
}

async fn exec(root: PathBuf, args: Vec<String>, env: &[EnvChange]) -> Result<Vec<u8>, String> {
    let mut line: Vec<String> = ["git", "-c", "core.quotePath=false"].map(str::to_owned).into();
    line.extend(args.iter().cloned());
    let mut cmd = Cmd::new(&line);
    let command = cmd.command_mut();
    command
        .current_dir(root)
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (name, value) in env {
        match value {
            Some(value) => command.env(name, value),
            None => command.env_remove(name),
        };
    }
    let sub = subcommand(args.first().map(String::as_str));
    match output(cmd).await {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Err("git nicht gefunden".to_owned()),
        Err(e) => Err(format!("{sub}: {e}")),
        Ok((status, out, _)) if status.success() => Ok(out),
        Ok((status, _, err)) => Err(exit_error(&sub, status, &err)),
    }
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

fn subcommand(first: Option<&str>) -> String {
    first.map_or_else(|| "git".to_owned(), |a| format!("git {a}"))
}

/// Satz für die Oberfläche; ohne Fehlerausgabe der Exit-Code (`merged.rs` liest „exit status 1“ als Nein).
fn exit_error(sub: &str, status: ExitStatus, stderr: &[u8]) -> String {
    if stderr.is_empty() {
        return format!("{sub}: exit status {}", status.code().unwrap_or(-1));
    }
    format!("{sub}: {}", last_line(&String::from_utf8_lossy(stderr)))
}

/// Die letzte nicht leere Zeile; bei git steht dort der eigentliche Grund (vor ihr oft „hint:“-Zeilen).
pub(super) fn last_line(s: &str) -> String {
    s.trim()
        .split('\n')
        .rev()
        .map(str::trim)
        .find(|l| !l.is_empty() && !l.starts_with("hint:"))
        .unwrap_or_else(|| s.trim())
        .to_owned()
}

/// Zeilen einer Ausgabe ohne leere.
pub(super) fn lines(s: &str) -> impl Iterator<Item = &str> {
    s.split('\n')
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
}

/// Prüft einen Branch-Namen aus der Oberfläche: keine Leerzeichen und Steuerzeichen, kein führendes „-“, kein „..“,
/// nur Zeichen, die in diesem Repo vorkommen. Strenger als `git check-ref-format`, damit nichts als Option gilt.
pub fn valid_branch(name: &str) -> bool {
    if name.is_empty()
        || name.len() > 200
        || name.starts_with('-')
        || name.contains("..")
        || name.ends_with('/')
        || name.ends_with(".lock")
        || name.contains("//")
    {
        return false;
    }
    name.chars().all(|c| c.is_ascii_alphanumeric() || "/-_.".contains(c))
}

/// Branches, die die Seite weder löscht noch pusht.
pub(super) fn protected(name: &str) -> bool {
    name == BASE || name == MAIN
}
