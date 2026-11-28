//! Kindprozess mit gelesener Ausgabe wie Go `cmd.Output` samt `WaitDelay` (`workkit/proc`), für `gh` und den Commit.
//! Wird die Zukunft fallen gelassen (Frist des Aufrufers), endet der Prozessbaum.

use std::io;
use std::process::{ExitStatus, Stdio};
use std::time::Duration;

use k3c_kit::proc::Cmd;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};
use tokio::process::ChildStdin;

/// So lange wartet Go nach dem Ende auf Pipes, die ein Enkelprozess (z. B. aus einem Hook) offen hält.
const WAIT_DELAY: Duration = Duration::from_secs(5);

/// Ende, stdout und stderr.
pub(crate) type Output = (ExitStatus, Vec<u8>, Vec<u8>);

async fn drain(source: Option<impl AsyncRead + Unpin>) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    if let Some(mut source) = source {
        source.read_to_end(&mut bytes).await?;
    }
    Ok(bytes)
}

/// Schreibt `input` und schließt stdin. Liest der Prozess nicht alles, zählt wie bei Go nur sein Ende.
async fn feed(stdin: Option<ChildStdin>, input: Vec<u8>) {
    if let Some(mut stdin) = stdin
        && stdin.write_all(&input).await.is_ok()
    {
        let _ = stdin.shutdown().await;
    }
}

/// Startet `cmd`, gibt ihm `input` auf stdin (sonst Null-Gerät) und liest beide Ausgaben.
pub(crate) async fn output(mut cmd: Cmd, input: Option<Vec<u8>>) -> io::Result<Output> {
    let command = cmd.command_mut();
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    if input.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut proc = cmd.start()?;
    let stdin = proc.child_mut().stdin.take();
    let (out, err) = (proc.child_mut().stdout.take(), proc.child_mut().stderr.take());
    let pipes = async { tokio::join!(feed(stdin, input.unwrap_or_default()), drain(out), drain(err)) };
    let wait = proc.wait();
    tokio::pin!(pipes, wait);
    let (status, read) = tokio::select! {
        status = &mut wait => (status?, tokio::time::timeout(WAIT_DELAY, &mut pipes).await.ok()),
        read = &mut pipes => (wait.await?, Some(read)),
    };
    match read {
        Some((_, out, err)) => Ok((status, out?, err?)),
        None if status.success() => Err(io::Error::other("exec: WaitDelay expired before I/O complete")),
        None => Ok((status, Vec::new(), Vec::new())),
    }
}
