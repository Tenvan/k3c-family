//! Gemeinsame Anbindung der Render-CLI für `review` und `paperassets` (B-706): Skript-Wahl, Zeitlimit, Ablehnung
//! (Exit 1) und der Hinweis, wenn bun fehlt. Dazu der Prozesslauf, den auch `git.rs` nutzt: Argumente einzeln, ohne
//! Shell, über `k3c_kit::proc`.

use std::ffi::{OsStr, OsString};
use std::io;
use std::path::{Path, PathBuf};
use std::process::{ExitStatus, Stdio};
use std::time::Duration;

use k3c_kit::proc::Cmd;
use serde::Deserialize;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};

use super::cli::Error;
use super::types::Fehler;

/// Begrenzt jeden Aufruf: eine hängende CLI hielte sonst eine Asset-Sperre oder einen Review-Lauf für immer.
pub const ZEITLIMIT: Duration = Duration::from_secs(120);

/// Die Render-CLI des Checkouts `root`: `K3C_PAPER_CLI`, sonst `tools/paper/bin/paper.ts`, solange die Datei fehlt die
/// Attrappe `tools/paper/fixtures/cli-stub.ts`.
pub fn skript(root: &Path) -> PathBuf {
    if let Some(s) = std::env::var_os("K3C_PAPER_CLI").filter(|s| !s.is_empty()) {
        return PathBuf::from(s);
    }
    let paper = root.join("tools").join("paper");
    let echt = paper.join("bin").join("paper.ts");
    if echt.metadata().is_ok() {
        return echt;
    }
    paper.join("fixtures").join("cli-stub.ts")
}

/// Ein Programmlauf: `args[0]` ist das Programm.
pub(super) struct Lauf<'a> {
    pub args: Vec<OsString>,
    pub dir: &'a Path,
    pub env: &'a [(&'a str, &'a str)],
    /// Leer: stdin hängt am Null-Gerät.
    pub stdin: Vec<u8>,
    pub zeit: Duration,
}

pub(super) struct Ausgabe {
    pub status: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

pub(super) enum LaufFehler {
    Zeit,
    System(io::Error),
}

/// Führt den Lauf zu Ende. Eigener Thread mit eigener Laufzeit, damit synchrone Aufrufer (auch aus `spawn_blocking`)
/// ihn nutzen können.
pub(super) fn ausfuehren(lauf: Lauf<'_>) -> Result<Ausgabe, LaufFehler> {
    std::thread::scope(|scope| {
        let thread = scope.spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(LaufFehler::System)?
                .block_on(starte(lauf))
        });
        thread
            .join()
            .unwrap_or_else(|_| Err(LaufFehler::System(io::Error::other("Prozesslauf abgebrochen"))))
    })
}

async fn lies(pipe: Option<impl AsyncRead + Unpin>) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();
    if let Some(mut pipe) = pipe {
        pipe.read_to_end(&mut out).await?;
    }
    Ok(out)
}

async fn starte(lauf: Lauf<'_>) -> Result<Ausgabe, LaufFehler> {
    let mut cmd = Cmd::new(&lauf.args);
    let command = cmd.command_mut();
    command
        .current_dir(lauf.dir)
        .envs(lauf.env.iter().copied())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if !lauf.stdin.is_empty() {
        command.stdin(Stdio::piped());
    }
    let mut proc = cmd.start().map_err(LaufFehler::System)?;
    let kill = proc.kill_handle();
    let child = proc.child_mut();
    let (stdin, stdout, stderr) = (child.stdin.take(), child.stdout.take(), child.stderr.take());
    let eingabe = lauf.stdin;
    let alles = async {
        let schreiben = async {
            // Schließt der Prozess stdin früh, steht der Grund in Exit-Code und stderr.
            if let Some(mut pipe) = stdin {
                let _ = pipe.write_all(&eingabe).await;
            }
        };
        tokio::join!(proc.wait(), lies(stdout), lies(stderr), schreiben)
    };
    let Ok((status, stdout, stderr, ())) = tokio::time::timeout(lauf.zeit, alles).await else {
        // Beendet den ganzen Baum; scheitert das, räumt das Fallenlassen den Job ab.
        let _ = kill.kill();
        return Err(LaufFehler::Zeit);
    };
    Ok(Ausgabe {
        status: status.map_err(LaufFehler::System)?,
        stdout: stdout.map_err(LaufFehler::System)?,
        stderr: stderr.map_err(LaufFehler::System)?,
    })
}

/// Go `time.Duration.String` für ganze Sekunden.
fn go_dauer(zeit: Duration) -> String {
    let s = zeit.as_secs();
    match (s / 3600, s / 60 % 60, s % 60) {
        (0, 0, s) if zeit.subsec_millis() == 0 => format!("{s}s"),
        (0, 0, _) => format!("{}ms", zeit.as_millis()),
        (0, m, s) => format!("{m}m{s}s"),
        (h, m, s) => format!("{h}h{m}m{s}s"),
    }
}

/// Führt `bun <skript> <args…>` im Ordner `root` aus (höchstens `zeit` lang): Exit 0 → stdout, Exit 1 mit `{fehler}` →
/// [`Error::Abgelehnt`], sonst ein Fehler mit stderr.
pub fn aufruf<S: AsRef<OsStr>>(root: &Path, skript: &Path, zeit: Duration, args: &[S]) -> Result<Vec<u8>, Error> {
    let befehl = args
        .first()
        .map_or_else(String::new, |a| a.as_ref().to_string_lossy().into_owned());
    let mut alle: Vec<OsString> = vec!["bun".into(), skript.into()];
    alle.extend(args.iter().map(|a| a.as_ref().to_owned()));
    let lauf = Lauf {
        args: alle,
        dir: root,
        env: &[],
        stdin: Vec::new(),
        zeit,
    };
    match ausfuehren(lauf) {
        Ok(a) => auswerten(
            &befehl,
            a.status.code().unwrap_or(-1),
            a.stdout,
            &String::from_utf8_lossy(&a.stderr),
            &a.status.to_string(),
        ),
        Err(LaufFehler::Zeit) => Err(Error::Sonst(format!(
            "Render-CLI {befehl}: Zeitlimit {} überschritten",
            go_dauer(zeit)
        ))),
        // `NotFound` kommt auch von einem fehlenden Arbeitsordner.
        Err(LaufFehler::System(e)) if e.kind() == io::ErrorKind::NotFound && root.is_dir() => Err(Error::Sonst(
            format!("Render-CLI {befehl}: bun fehlt im PATH (Installation: requirements.md)"),
        )),
        Err(LaufFehler::System(e)) => auswerten(&befehl, -1, Vec::new(), "", &e.to_string()),
    }
}

#[derive(Deserialize)]
struct Ablehnung {
    #[serde(default)]
    fehler: Vec<Fehler>,
}

/// Deutet einen beendeten Befehl wie [`aufruf`] (auch je Schritt eines Sammelaufrufs `batch`, B-856): Exit 0 → stdout,
/// Exit 1 mit `{fehler}` → abgelehnt, sonst ein Fehler mit stderr (leer: `ersatz`).
pub fn auswerten(befehl: &str, code: i32, stdout: Vec<u8>, stderr: &str, ersatz: &str) -> Result<Vec<u8>, Error> {
    if code == 0 {
        return Ok(stdout);
    }
    // Exit 1 ohne Antwort ist ein Absturz von bun (z. B. fehlende node_modules), keine Ablehnung.
    if code == 1
        && let Ok(ab) = serde_json::from_slice::<Ablehnung>(&stdout)
        && !ab.fehler.is_empty()
    {
        return Err(Error::Abgelehnt(ab.fehler));
    }
    let text = match stderr.trim() {
        "" => ersatz,
        text => text,
    };
    Err(Error::Sonst(format!("Render-CLI {befehl} (Exit {code}): {text}")))
}
