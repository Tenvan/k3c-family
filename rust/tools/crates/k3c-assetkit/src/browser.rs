//! Öffnet eine Adresse im Standard-Browser: der Web-Start von Workbench und Studio (`-open`, B-614).

use std::process::{Command, Stdio};

use crate::error::{Error, Result};

/// Startet den Standard-Browser mit `url` und wartet nicht auf ihn. Nur http(s), damit nie eine Datei oder ein fremdes
/// Protokoll gestartet wird.
pub fn open(url: &str) -> Result<()> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(Error::Msg(format!("nur http(s)-Adressen: {url}")));
    }
    let mut command = if cfg!(windows) {
        let mut c = Command::new("rundll32");
        c.arg("url.dll,FileProtocolHandler");
        c
    } else if cfg!(target_os = "macos") {
        Command::new("open")
    } else {
        Command::new("xdg-open")
    };
    // Wie Go `exec.Command` ohne eigene Ströme: alle drei auf das Nullgerät.
    let mut child = command
        .arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    std::thread::spawn(move || {
        let _ = child.wait();
    });
    Ok(())
}

#[cfg(test)]
#[path = "browser_tests.rs"]
mod tests;
