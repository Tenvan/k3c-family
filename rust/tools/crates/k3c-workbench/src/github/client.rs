//! Aufruf von `gh` und der Zwischenspeicher (Port von `client.go`).

use std::future::Future;
use std::io;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use chrono::Local;
use k3c_kit::proc::Cmd;
use tokio::sync::Mutex;
use tokio::time::{Instant, timeout_at};

use super::{Data, parse};
use crate::capture;

/// Frist des Zwischenspeichers; GitHub soll nicht bei jedem Neuladen der Planung gefragt werden.
pub const TTL: Duration = Duration::from_secs(60);

/// Frist eines `gh`-Aufrufs.
pub(super) const TIMEOUT: Duration = Duration::from_secs(15);

/// Feste Argumente: nichts davon stammt aus Dateien oder Eingaben.
pub(super) const PR_ARGS: [&str; 11] = [
    "gh",
    "pr",
    "list",
    "--state",
    "all",
    "--base",
    "main",
    "--limit",
    "50",
    "--json",
    "number,title,state,isDraft,headRefName,mergeable,mergeStateStatus,statusCheckRollup,url",
];
pub(super) const RUN_ARGS: [&str; 9] = [
    "gh",
    "run",
    "list",
    "--branch",
    "main",
    "--limit",
    "1",
    "--json",
    "status,conclusion,displayTitle,url,createdAt",
];

/// Woran ein Aufruf von `gh` scheitert.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunError {
    /// Das Programm fehlt.
    NotFound,
    /// Frist abgelaufen.
    Timeout,
    /// Ende mit Fehler; Text von stderr.
    Exit(String),
    Other(String),
}

/// Ergebnis eines [`Runner`]-Aufrufs.
pub type RunFuture = Pin<Box<dyn Future<Output = Result<Vec<u8>, RunError>> + Send>>;
/// Führt die Argumente (mit „gh“ vorn) aus und liefert stdout; austauschbar, damit Tests nie `gh` und das Netz brauchen.
pub type Runner = Arc<dyn Fn(Vec<String>) -> RunFuture + Send + Sync>;

/// Der echte Aufruf in `root`; `program` ersetzt das erste Argument (Tests: ein Programm, das es nicht gibt).
pub fn exec_runner(root: PathBuf, program: &str) -> Runner {
    let program = program.to_owned();
    Arc::new(move |mut args: Vec<String>| {
        let root = root.clone();
        if let Some(first) = args.first_mut() {
            first.clone_from(&program);
        }
        Box::pin(async move {
            let mut cmd = Cmd::new(&args);
            cmd.command_mut().current_dir(root);
            match capture::output(cmd, None).await {
                Err(e) if e.kind() == io::ErrorKind::NotFound => Err(RunError::NotFound),
                Err(e) => Err(RunError::Other(e.to_string())),
                Ok((status, out, _)) if status.success() => Ok(out),
                Ok((_, _, err)) => Err(RunError::Exit(String::from_utf8_lossy(&err).into_owned())),
            }
        })
    })
}

/// Macht aus einem Fehler von `gh` einen Satz für Oberfläche und Tool.
pub(super) fn hint(error: &RunError) -> String {
    match error {
        RunError::NotFound => "gh nicht gefunden: GitHub CLI installieren und `gh auth login` ausführen".to_owned(),
        RunError::Timeout => "gh antwortet nicht (Netz?)".to_owned(),
        RunError::Exit(stderr) if stderr.contains("auth login") => {
            "gh nicht angemeldet: `gh auth login` ausführen".to_owned()
        }
        RunError::Exit(stderr) => format!("gh: {}", stderr.split('\n').next().unwrap_or_default().trim()),
        RunError::Other(message) => format!("gh: {message}"),
    }
}

#[derive(Default)]
pub(super) struct Cache {
    last: Data,
    /// `None`: noch nie geholt oder nach einer Aktion verworfen.
    pub(super) at: Option<Instant>,
}

/// Holt den Stand mit `gh` und hält ihn [`TTL`] lang.
pub struct Client {
    run: Runner,
    pub(super) cache: Mutex<Cache>,
}

impl Client {
    /// Client für das Repo unter `root` mit dem echten `gh`.
    pub fn new(root: PathBuf) -> Client {
        Client::with_runner(exec_runner(root, "gh"))
    }

    /// Client mit eigenem Aufruf (Tests).
    pub fn with_runner(run: Runner) -> Client {
        Client {
            run,
            cache: Mutex::default(),
        }
    }

    /// Ein Aufruf bis zum Zeitpunkt `deadline`.
    pub(super) async fn run<S: AsRef<str>>(&self, deadline: Instant, args: &[S]) -> Result<Vec<u8>, RunError> {
        let args = args.iter().map(|a| a.as_ref().to_owned()).collect();
        // Beim Fallenlassen endet der Prozessbaum.
        timeout_at(deadline, (self.run)(args))
            .await
            .unwrap_or(Err(RunError::Timeout))
    }

    /// Der Stand; `force` umgeht den Zwischenspeicher. Scheitert `gh`, bleibt der letzte Stand mit Hinweis.
    pub async fn status(&self, force: bool) -> Data {
        let mut cache = self.cache.lock().await;
        if !force && cache.at.is_some_and(|at| at.elapsed() < TTL) {
            return cache.last.clone();
        }
        match self.fetch().await {
            Ok(data) => {
                cache.last = data.clone();
                cache.at = Some(Instant::now());
                data
            }
            Err(message) if cache.at.is_none() => Data {
                error: message,
                ..Data::default()
            },
            Err(message) => {
                let mut stale = cache.last.clone();
                stale.error = format!("{message} (Stand von {})", cache.last.fetched);
                stale
            }
        }
    }

    async fn fetch(&self) -> Result<Data, String> {
        let deadline = Instant::now() + TIMEOUT;
        let prs = self.run(deadline, &PR_ARGS).await.map_err(|e| hint(&e))?;
        // Ohne main-Lauf geht es auch.
        let runs = self.run(deadline, &RUN_ARGS).await.unwrap_or_default();
        parse(&prs, &runs, Local::now().fixed_offset())
    }
}
