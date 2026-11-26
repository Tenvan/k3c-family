//! „In Claude öffnen“ (`workkit/claude`): eine neue Code-Session in Claude Desktop mit vorbefülltem Prompt über den Deep
//! Link `claude://code/new` (<https://support.claude.com/en/articles/14729294-open-claude-desktop-with-a-link>).
//! Abgeschickt wird nichts: Claude Desktop füllt nur das Eingabefeld, Enter bleibt beim Nutzer.

use std::fmt::Write as _;
use std::process::{Command, Stdio};

/// Claude Desktop kürzt `q` bei rund 14.000 Zeichen; lieber ablehnen als still kürzen.
pub const MAX_PROMPT: usize = 14000;

/// Hält den Link unter der Windows-Grenze für eine Befehlszeile (32.767).
const MAX_LINK: usize = 32000;

/// Kodiert einen Wert wie Go `url.Values.Encode`, Leerzeichen als `%20`; ein `+` ist immer `%2B`.
fn escape(value: &str, out: &mut String) {
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || b"-_.~".contains(&byte) {
            out.push(char::from(byte));
        } else {
            // In einen String schreiben schlägt nicht fehl.
            let _ = write!(out, "%{byte:02X}");
        }
    }
}

/// Baut den Deep Link.
pub fn link(prompt: &str, folder: &str) -> Result<String, String> {
    if prompt.trim().is_empty() {
        return Err("prompt ist leer".to_string());
    }
    let count = prompt.chars().count();
    if count > MAX_PROMPT {
        return Err(format!(
            "prompt hat {count} Zeichen, Claude Desktop übernimmt höchstens {MAX_PROMPT}"
        ));
    }
    let mut link = String::from("claude://code/new?");
    // Schlüssel sortiert wie in Go.
    if !folder.is_empty() {
        link.push_str("folder=");
        escape(folder, &mut link);
        link.push('&');
    }
    link.push_str("q=");
    escape(prompt, &mut link);
    if link.len() > MAX_LINK {
        let length = link.len();
        return Err(format!("prompt ist kodiert zu lang für einen Link ({length} Zeichen)"));
    }
    Ok(link)
}

/// Öffnet eine neue Claude-Code-Session mit dem Prompt im Eingabefeld und `folder` als Ordner. Bewusst ohne
/// [`crate::proc`]: dessen Job-Objekt würde ein von rundll32 gestartetes Claude Desktop beim Beenden der App mitreißen.
/// Keine Shell dazwischen. Nur Windows (rundll32), die Apps laufen nur dort.
pub fn open(prompt: &str, folder: &str) -> Result<(), String> {
    let link = link(prompt, folder)?;
    let mut child = Command::new("rundll32")
        .args(["url.dll,FileProtocolHandler", link.as_str()])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("claude desktop nicht startbar: {e}"))?;
    std::thread::spawn(move || {
        let _ = child.wait();
    });
    Ok(())
}

#[cfg(test)]
#[path = "claude_tests.rs"]
mod claude_tests;
