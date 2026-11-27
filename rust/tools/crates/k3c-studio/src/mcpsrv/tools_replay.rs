//! `replay_run` (Port von `tools_replay.go`, B-159): spielt eine Replay-Datei ohne Bot ab, mit der Spiel-EXE des
//! Checkouts (`k3c-game sim balance --play`, RU40.3); mit `head` zeigt das Spiel die Wiedergabe.

use std::path::{Path, PathBuf};
use std::time::Duration;

use k3c_kit::mcp::inside_root;
use schemars::JsonSchema;
use serde::Deserialize;

use super::kit::Workspace;
use super::tools_sim::{HEAD_TIMEOUT, check_speed};
use super::tools_world::blocking;
use crate::gameexe::{self, Event, Options};
use crate::kitx::go_quoted;

#[cfg(test)]
#[path = "tools_replay_tests.rs"]
mod tests;

/// Begrenzt eine Wiedergabe (höchstens 1 000 000 Ticks).
const REPLAY_TIMEOUT: Duration = Duration::from_secs(10 * 60);

/// Begrenzt die gelesene Datei (eine Aufnahme über 60 Tage hat wenige MB).
const MAX_REPLAY_BYTES: u64 = 64 << 20;

/// Argumente von `replay_run`.
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub(super) struct ReplayIn {
    #[schemars(
        description = "Replay-Datei relativ zur Repo-Wurzel, z. B. replays/12-p2-saver-d0.replay.json (aus task balance:run -- --replay-dir)"
    )]
    pub(super) path: String,
    #[serde(default)]
    #[schemars(description = "Spiel mit Fenster öffnen und die Wiedergabe zeigen; ohne Angabe headless")]
    pub(super) head: bool,
    #[serde(default)]
    #[schemars(description = "Zeitraffer 1 bis 64 (Standard 1 = Echtzeit), nur mit head")]
    pub(super) speed: i64,
}

/// Die Felder einer Replay-Datei, die `replay_run` nennt (engine/balance › Replay); fehlende bleiben leer.
#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct ReplayHead {
    seed: Option<String>,
    players: Option<i64>,
    bot: Option<String>,
    depth: Option<i64>,
    days: Option<i64>,
    end_hash: Option<String>,
    castle_fall_tick: Option<i64>,
}

/// Das Ergebnis von `k3c-game sim balance --play` (engine/balance › Playback).
#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Playback {
    ticks: Option<i64>,
    end_hash: Option<String>,
    castle_fall_tick: Option<i64>,
    warnings: Option<Vec<String>>,
}

/// Prüft den Pfad und liest den Kopf der Aufnahme (blockiert).
fn load(root: &Path, rel: &str) -> Result<(PathBuf, ReplayHead), String> {
    let path = inside_root(root, rel)?;
    if std::fs::metadata(&path).is_ok_and(|m| m.len() > MAX_REPLAY_BYTES) {
        return Err(format!(
            "{rel} ist größer als {} MB, keine Replay-Datei",
            MAX_REPLAY_BYTES >> 20
        ));
    }
    let bytes = std::fs::read(&path).map_err(|e| format!("{rel} nicht lesbar: {e}"))?;
    let head = serde_json::from_slice(&bytes).map_err(|e| format!("{rel}: kein Replay: {e}"))?;
    Ok((path, head))
}

/// Das Tool `replay_run`. `call` bekommt Optionen und Zeitlimit; Tests ersetzen ihn.
pub(super) async fn replay_run<F>(
    ws: &Workspace,
    input: &ReplayIn,
    call: impl FnOnce(Options, Duration) -> F,
) -> Result<String, String>
where
    F: Future<Output = Result<Event, gameexe::Error>>,
{
    check_speed("replay_run", input.head, input.speed)?;
    let (root, rel) = (ws.root.clone(), input.path.clone());
    let (path, head) = blocking(move || load(&root, &rel)).await?;
    let limit = if input.head { HEAD_TIMEOUT } else { REPLAY_TIMEOUT };
    let options = Options {
        root: ws.root.clone(),
        sub: "balance".to_owned(),
        input: b"{}".to_vec(),
        window: input.head,
        speed: input.speed,
        args: vec!["--play".to_owned(), path.to_string_lossy().into_owned()],
        ..Options::default()
    };
    let event = call(options, limit).await.map_err(|e| e.to_string())?;
    let report = event.report.as_ref().map_or("", |r| r.get());
    let playback: Playback =
        serde_json::from_str(report).map_err(|e| format!("k3c-game sim balance --play: Ergebnis nicht lesbar: {e}"))?;
    Ok(text(&input.path, &head, playback))
}

fn text(path: &str, recorded: &ReplayHead, played: Playback) -> String {
    let seed = recorded.seed.as_deref().unwrap_or_default();
    let recorded_hash = recorded.end_hash.as_deref().unwrap_or_default();
    let played_hash = played.end_hash.unwrap_or_default();
    let mut lines = vec![
        format!(
            "replay_run {path} · Seed {} · {} Spieler · Bot {} · Tiefe {} · {} Tage",
            go_quoted(seed),
            recorded.players.unwrap_or_default(),
            recorded.bot.as_deref().unwrap_or_default(),
            recorded.depth.unwrap_or_default(),
            recorded.days.unwrap_or_default()
        ),
        format!(
            "Ticks {} · Endzustand-Hash {played_hash} · Burgfall-Tick {}",
            played.ticks.unwrap_or_default(),
            tick_text(played.castle_fall_tick)
        ),
    ];
    if played_hash == recorded_hash && played.castle_fall_tick == recorded.castle_fall_tick {
        lines.push("Aufnahme: gleicher Endzustand und Burgfall-Tick".to_owned());
    } else {
        lines.push(format!(
            "Aufnahme weicht ab: Endzustand-Hash {recorded_hash} · Burgfall-Tick {}",
            tick_text(recorded.castle_fall_tick)
        ));
    }
    for warning in played.warnings.unwrap_or_default() {
        lines.push(format!("Warnung: {warning}"));
    }
    lines.join("\n")
}

/// Ein Tick oder „keiner“.
fn tick_text(tick: Option<i64>) -> String {
    tick.map_or_else(|| "keiner".to_owned(), |t| t.to_string())
}
