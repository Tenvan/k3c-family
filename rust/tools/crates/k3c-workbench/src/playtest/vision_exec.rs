//! Der Prozess der Bildbeschreibung: nur stdout, mit Abbruch. Wie [`super::exec`] läuft er über `k3c_kit::proc`.

use std::path::Path;
use std::process::Stdio;
use std::sync::{Arc, mpsc};
use std::time::Duration;

use k3c_kit::proc::Cmd;
use tokio::io::AsyncReadExt;
use tokio::runtime::Handle;

use super::Error;
use crate::services::{Cancel, exit_of};

/// So lange darf nach dem Ende noch Ausgabe kommen, die ein Enkel offen hält.
const WAIT_DELAY: Duration = Duration::from_secs(5);

/// Aufruf mit stdout; `Cancel` beendet den Prozess (Tests setzen eine Attrappe).
pub type VisionRun = Arc<dyn Fn(&[String], &Cancel) -> Result<String, String> + Send + Sync>;

/// Startet `args` (im Ordner `dir`) und liefert stdout; stderr (Ladeprotokoll) bleibt ungelesen. Blockiert.
pub(super) fn run_capture(
    runtime: &Handle,
    args: &[String],
    dir: Option<&Path>,
    cancel: &Cancel,
) -> Result<String, String> {
    if cancel.is_cancelled() {
        return Err(Error::Canceled.to_string());
    }
    let mut cmd = Cmd::new(args);
    cmd.command_mut().stdout(Stdio::piped());
    if let Some(dir) = dir {
        cmd.command_mut().current_dir(dir);
    }
    // `tokio` braucht für den Start seine Laufzeit.
    let _guard = runtime.enter();
    let mut child = cmd.start().map_err(|e| e.to_string())?;
    let stdout = child.child_mut().stdout.take();
    let kill = child.kill_handle();
    let cancel = cancel.clone();
    let (send, receive) = mpsc::channel();
    runtime.spawn(async move {
        let mut reader = tokio::spawn(async move {
            let mut buf = Vec::new();
            if let Some(mut stdout) = stdout {
                // Ein Lesefehler lässt, was schon da ist.
                let _ = stdout.read_to_end(&mut buf).await;
            }
            buf
        });
        let status = {
            let wait = child.wait();
            tokio::pin!(wait);
            tokio::select! {
                status = &mut wait => status,
                () = cancel.cancelled() => {
                    // Ein Fehler heißt: der Prozess ist schon weg.
                    let _ = kill.kill();
                    wait.await
                }
            }
        };
        let bytes = match tokio::time::timeout(WAIT_DELAY, &mut reader).await {
            Ok(read) => read.unwrap_or_default(),
            Err(_) => {
                reader.abort();
                Vec::new()
            }
        };
        let _ = send.send((bytes, exit_of(status).map_err(|e| e.message)));
    });
    let (bytes, status) = receive.recv().map_err(|e| e.to_string())?;
    status?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

/// Echte Prozesse in der Laufzeit `runtime`. Der Aufruf blockiert: nie von einem ihrer Worker rufen.
pub fn exec_vision(runtime: Handle) -> VisionRun {
    Arc::new(move |args, cancel| run_capture(&runtime, args, None, cancel))
}
