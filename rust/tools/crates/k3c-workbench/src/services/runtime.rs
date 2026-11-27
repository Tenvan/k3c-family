//! Echte Prozesse und Health-Prüfung (Port von `runtime.go`): Start über `k3c_kit::proc` (ohne Konsolenfenster, im
//! Job-Objekt), Ausgabe zeilenweise.

use std::ffi::OsStr;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{ExitStatus, Stdio};
use std::time::Duration;

use k3c_kit::console::LineWriter;
use k3c_kit::proc::Cmd;
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::task::JoinHandle;

use super::config::Service;
use super::options::{Exit, OutFn, Spawned, WaitError};
use super::process::program;

/// Begrenzt eine einzelne Health-Prüfung.
const CHECK_TIMEOUT: Duration = Duration::from_secs(2);
/// So lange darf nach dem Ende noch Ausgabe kommen (Go `WaitDelay`).
const WAIT_DELAY: Duration = Duration::from_secs(5);

/// Sucht einen Programmnamen ohne Pfad in `PATH` mit den Endungen aus `PATHEXT` wie Go `exec.LookPath`; `Command`
/// ergänzt sonst nur `.exe`. Ohne Fund bleibt der Name (der Start meldet den Fehler).
fn look_path(name: &Path, path: &OsStr, extensions: &str) -> PathBuf {
    if name.components().count() != 1 || name.extension().is_some() {
        return name.to_path_buf();
    }
    // Wie Go (`ErrDot`): leere und relative Einträge machten das Arbeitsverzeichnis zum Suchort.
    for dir in std::env::split_paths(path).filter(|d| d.is_absolute()) {
        for extension in extensions.split(';').filter(|e| !e.is_empty()) {
            let mut candidate = dir.join(name).into_os_string();
            candidate.push(extension.to_lowercase());
            let candidate = PathBuf::from(candidate);
            if candidate.is_file() {
                return candidate;
            }
        }
    }
    name.to_path_buf()
}

/// Pfad eines Programms: relative Pfade unter `root`, Namen über `PATH` (auch für Tasks).
pub(crate) fn executable(name: &str, root: &Path) -> PathBuf {
    let program = program(name, root);
    if !cfg!(windows) {
        return program;
    }
    let path = std::env::var_os("PATH").unwrap_or_default();
    let extensions = std::env::var("PATHEXT").unwrap_or_else(|_| ".com;.exe;.bat;.cmd".to_owned());
    // npm heißt unter Windows npm.cmd (`k3c_kit::proc::executable`); die Suche findet es ebenso.
    look_path(&program, &path, &extensions)
}

/// Liest einen Ausgabestrom bis zum Ende und gibt ihn zeilenweise weiter.
fn pump<R>(source: Option<R>, mut lines: LineWriter) -> Option<JoinHandle<()>>
where
    R: AsyncRead + Unpin + Send + 'static,
{
    let mut source = source?;
    Some(tokio::spawn(async move {
        let mut buf = vec![0; 8192];
        loop {
            match source.read(&mut buf).await {
                Ok(0) | Err(_) => return,
                // Der Writer nimmt alles an und scheitert nie.
                Ok(n) => drop(lines.write_all(&buf[..n])),
            }
        }
    }))
}

/// Ergebnis wie Go `exec.ExitError`.
pub(crate) fn exit_of(status: std::io::Result<ExitStatus>) -> Exit {
    match status {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => {
            // Windows-Codes sind vorzeichenlos.
            let code = status.code().map_or(-1, |c| {
                if cfg!(windows) {
                    i64::from(c.cast_unsigned())
                } else {
                    i64::from(c)
                }
            });
            let message = if code >= 1 << 16 {
                format!("exit status {code:#x}")
            } else {
                format!("exit status {code}")
            };
            Err(WaitError { code, message })
        }
        Err(e) => Err(WaitError {
            code: -1,
            message: e.to_string(),
        }),
    }
}

fn line_writer(out: &OutFn, stream: &'static str) -> LineWriter {
    let out = OutFn::clone(out);
    LineWriter::new(move |text| out(stream, &text))
}

/// Startet einen Dienst. Braucht eine tokio-Laufzeit.
pub fn proc_starter(svc: &Service, root: &Path, out: OutFn) -> Result<Spawned, String> {
    let first = svc.command.first().ok_or_else(|| "command fehlt".to_owned())?;
    let mut args = vec![executable(first, root).into_os_string()];
    args.extend(svc.command.iter().skip(1).map(Into::into));
    let mut cmd = Cmd::new(&args);
    cmd.command_mut().current_dir(root.join(&svc.cwd)).envs(&svc.env);
    spawn(cmd, &out)
}

/// Startet `cmd` mit zeilenweiser Ausgabe an `out`. Braucht eine tokio-Laufzeit.
pub(crate) fn spawn(mut cmd: Cmd, out: &OutFn) -> Result<Spawned, String> {
    cmd.command_mut().stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut proc = cmd.start().map_err(|e| e.to_string())?;
    let (stdout, stderr) = (line_writer(out, "stdout"), line_writer(out, "stderr"));
    let readers = [
        pump(proc.child_mut().stdout.take(), stdout.clone()),
        pump(proc.child_mut().stderr.take(), stderr.clone()),
    ];
    let (pid, handle) = (proc.id(), proc.kill_handle());
    let wait = Box::pin(async move {
        // `wait` räumt den Job ab: übrig gebliebene Enkel enden, ihre Pipes schließen.
        let status = proc.wait().await;
        for mut reader in readers.into_iter().flatten() {
            if tokio::time::timeout(WAIT_DELAY, &mut reader).await.is_err() {
                reader.abort();
            }
        }
        stdout.flush_line();
        stderr.flush_line();
        exit_of(status)
    });
    Ok(Spawned {
        pid,
        kill: Box::new(move || handle.kill().map_err(|e| e.to_string())),
        wait,
    })
}

/// `GET /`; der Statuscode der Antwort. Einer Weiterleitung folgt nichts: ein 3xx zählt schon als gesund.
fn http_status(addr: SocketAddr) -> std::io::Result<u16> {
    let mut stream = TcpStream::connect_timeout(&addr, CHECK_TIMEOUT)?;
    stream.set_read_timeout(Some(CHECK_TIMEOUT))?;
    stream.set_write_timeout(Some(CHECK_TIMEOUT))?;
    stream.write_all(format!("GET / HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n").as_bytes())?;
    let mut head = Vec::new();
    let mut buf = [0; 256];
    while !head.contains(&b'\n') && head.len() < 4096 {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            break;
        }
        head.extend_from_slice(&buf[..n]);
    }
    let line = String::from_utf8_lossy(&head);
    let mut words = line.split_whitespace();
    match (words.next(), words.next().and_then(|code| code.parse().ok())) {
        (Some(version), Some(code)) if version.starts_with("HTTP/") => Ok(code),
        _ => Err(std::io::Error::other("keine HTTP-Antwort")),
    }
}

/// http: `GET /` mit 2xx oder 3xx; tcp: der Port nimmt Verbindungen an.
pub fn health_check(svc: &Service) -> Result<(), String> {
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, svc.net_port()));
    if svc.health == "tcp" {
        return TcpStream::connect_timeout(&addr, CHECK_TIMEOUT)
            .map(drop)
            .map_err(|e| e.to_string());
    }
    let code = http_status(addr).map_err(|e| e.to_string())?;
    if code >= 400 {
        return Err(format!("antwort {code}"));
    }
    Ok(())
}

#[cfg(test)]
#[path = "runtime_tests.rs"]
mod tests;
