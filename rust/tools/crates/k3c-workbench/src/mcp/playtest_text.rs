//! Antworttexte der Tools `playtest_*` (Port der Formatierung in `tools_playtest.go`).

use std::fmt::Write;
use std::path::{Path, PathBuf};

use crate::playtest::{Entry, Finding, Session, Store, TicketResult, Timeline};

/// Zeilen der Zeitleiste je Aufruf von `playtest_get`.
const PAGE_LIMIT: i64 = 60;

/// Pfad mit einheitlichen Trennern wie Go `filepath.Join`.
pub(super) fn clean(path: &Path) -> String {
    path.components().collect::<PathBuf>().display().to_string()
}

fn yes_no(value: bool) -> &'static str {
    if value { "ja" } else { "nein" }
}

fn minutes(ms: i64) -> String {
    if ms <= 0 {
        return "Dauer ?".to_owned();
    }
    format!("{} min", (ms / 60_000).max(1))
}

fn clock(ms: i64) -> String {
    let seconds = ms.max(0) / 1000;
    format!("{:02}:{:02}", seconds / 60, seconds % 60)
}

/// Antwort von `playtest_list`.
pub(super) fn list(store: &Store) -> Result<String, String> {
    let sessions = store.list().map_err(|e| e.to_string())?;
    if sessions.is_empty() {
        return Ok("keine Spieltests in playtests/".to_owned());
    }
    let mut out = format!("{} Spieltests (neueste zuerst):\n", sessions.len());
    for s in &sessions {
        let findings = store.findings(&s.id).map_or(0, |f| f.len());
        // Schreiben in eine Zeichenkette scheitert nicht.
        let _ = writeln!(
            out,
            "- {} · {} · {} · {} Marker · {findings} Befunde · Sprache {}, Video {}",
            s.id,
            s.stage,
            minutes(s.duration_ms),
            s.markers,
            yes_no(s.has_voice),
            yes_no(s.has_video)
        );
    }
    Ok(out)
}

/// Höchstens `limit` Zeichen.
fn cut(text: String, limit: usize) -> String {
    if text.chars().count() > limit {
        return text.chars().take(limit).collect::<String>() + " …";
    }
    text
}

fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn check_mark(finding: &Finding) -> &'static str {
    if finding.selected { " [angehakt]" } else { "" }
}

fn ticket_note(finding: &Finding) -> String {
    if !finding.ticket.is_empty() {
        format!(" → {}", finding.ticket)
    } else if !finding.duplicate_of.is_empty() {
        format!(" (Duplikat von {})", finding.duplicate_of)
    } else if !finding.error.is_empty() {
        format!(" (Fehler: {})", finding.error)
    } else {
        String::new()
    }
}

/// Kopf der ersten Seite: Stand, Hinweise, Fragebogen, Befunde.
fn head(out: &mut String, store: &Store, session: &Session, timeline: &Timeline) {
    let _ = writeln!(
        out,
        "Session {} · {} · {} · {} Marker",
        session.id,
        session.stage,
        minutes(session.duration_ms),
        session.markers
    );
    if !timeline.has_transcript {
        out.push_str(
            "Hinweis: noch kein Transkript (Ralf muss auf der Seite Spieltest Auswerten drücken); nur Marker.\n",
        );
    }
    if !session.has_video {
        out.push_str("Hinweis: video.mkv fehlt, keine neuen Bilder möglich.\n");
    }
    if let Ok(raw) = std::fs::read(store.root.join(&session.id).join("survey.json")) {
        let text = cut(one_line(&String::from_utf8_lossy(&raw)), 1500);
        let _ = writeln!(out, "Fragebogen: {text}");
    } else {
        out.push_str("Fragebogen: –\n");
    }
    let findings = store.findings(&session.id).unwrap_or_default();
    let _ = writeln!(out, "Befunde: {}", findings.len());
    for f in &findings {
        let _ = writeln!(
            out,
            "- {}{}{} · ts {} · „{}“",
            f.id,
            check_mark(f),
            ticket_note(f),
            f.ts,
            cut(one_line(&f.quote), 100)
        );
    }
}

fn timeline_line(dir: &Path, entry: &Entry) -> String {
    let head = format!("[{} ts={}]", clock(entry.video_ms), entry.ms);
    if entry.kind == "sprache" {
        return format!("{head} Sprache: {}", entry.text);
    }
    let mut line = format!("{head} Marker #{} {}", entry.nr, entry.text);
    if let Some(m) = &entry.marker {
        let world = crate::planning::store::quoted(&m.world);
        let _ = write!(
            line,
            " · Welt {world} Phase {} Tag {} · schlechtester Frame {:.1} ms",
            m.phase, m.day, m.worst_frame_ms
        );
        if let Some(frame) = m.frame_ms {
            let _ = write!(line, " · Ruckler {frame:.1} ms");
        }
    }
    if entry.skip {
        line.push_str(" · Bild nicht ausgewählt (nicht ansehen)");
    } else if !entry.frame.is_empty() {
        if !entry.desc.is_empty() {
            let _ = write!(line, " · Bildbeschreibung: {}", one_line(&entry.desc));
        }
        let _ = write!(line, " · Bild {}", clean(&dir.join(&entry.frame)));
    }
    line
}

/// Antwort von `playtest_get`: eine Seite der Zeitleiste, die erste mit Kopf.
pub(super) fn get(store: &Store, id: &str, limit: i64, cursor: i64) -> Result<String, String> {
    let session = store.get(id).map_err(|e| e.to_string())?;
    let timeline = store.timeline(id).map_err(|e| e.to_string())?;
    let limit = if limit <= 0 { PAGE_LIMIT } else { limit.min(500) };
    let cursor = usize::try_from(cursor).unwrap_or(0);
    let mut out = String::new();
    if cursor == 0 {
        head(&mut out, store, &session, &timeline);
    }
    let total = timeline.entries.len();
    let end = cursor.saturating_add(usize::try_from(limit).unwrap_or(0)).min(total);
    let first = cursor.saturating_add(1).min(end);
    let _ = writeln!(
        out,
        "Zeitleiste {first}–{end} von {total} (ts = Unix-ms, mm:ss = Videozeit):"
    );
    let dir = store.root.join(&session.id);
    for entry in timeline.entries.iter().take(end).skip(cursor) {
        let _ = writeln!(out, "{}", timeline_line(&dir, entry));
    }
    if end < total {
        let _ = writeln!(out, "weiter mit cursor={end}");
    }
    Ok(out)
}

/// Zusammenfassung der Ticket-Anlage.
pub(super) fn ticket_report(result: &TicketResult) -> String {
    let mut out = format!(
        "Tickets: {} angelegt, {} Hinweise in vorhandenen Tickets, {} fehlgeschlagen, {} übersprungen (nicht angehakt \
         oder schon erledigt)\n",
        result.created.len(),
        result.noted.len(),
        result.failed.len(),
        result.skipped
    );
    let groups = [&result.created, &result.noted, &result.failed, &result.similar];
    for line in groups.into_iter().flatten() {
        let _ = writeln!(out, "- {line}");
    }
    out.trim_end_matches('\n').to_owned()
}
