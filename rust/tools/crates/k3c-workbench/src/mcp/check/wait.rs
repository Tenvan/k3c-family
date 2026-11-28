//! Warten auf einen Lauf (Port von `check_wait.go`, B-520): Trägt der Aufruf ein `progressToken`, meldet `check_run`
//! alle `progress_every` Fortschritt (zählt beim Client als Aktivität gegen dessen Leerlauf-Frist) und wartet bis zum
//! Ende; ohne Token wartet es höchstens `wait`, knapp unter der Leerlauf-Frist von HTTP-MCP-Clients (5 min), und nennt
//! danach die Lauf-ID. `check_run {run}` wartet genauso. Die Frist je Aufruf hebt `timeout` in `.mcp.json`.

use std::time::Duration;

use k3c_kit::mcp::Progress;
use serde_json::json;

use super::super::status::format_ms;
use super::Checks;
use super::job::Job;

pub(super) const DEFAULT_CHECK_WAIT: Duration = Duration::from_secs(4 * 60);
pub(super) const DEFAULT_PROGRESS_EVERY: Duration = Duration::from_secs(20);

fn elapsed_text(elapsed: Duration) -> String {
    format_ms(elapsed.as_millis() as f64)
}

impl Checks {
    /// Wartet auf das Ende des Laufs: mit Fortschritts-Meldungen bis zum Ende, ohne höchstens `hooks.wait`.
    pub(super) async fn await_job(&self, job: &Job, progress: &Progress) -> Result<String, String> {
        let active = progress.is_active();
        let wait = if active {
            // Der Lauf endet spätestens mit seinem Zeitlimit.
            job.target.timeout + Duration::from_secs(60)
        } else {
            self.hooks.wait
        };
        let attrs = [
            ("ns", json!("check")),
            ("id", json!(job.id)),
            ("target", json!(job.target.name)),
            ("progress", json!(active)),
            ("wait_s", json!(wait.as_secs())),
        ];
        self.log.info("⏳ check_run wartet auf Lauf", &attrs);
        let every = self.hooks.progress_every;
        let mut tick = tokio::time::interval_at(tokio::time::Instant::now() + every, every);
        let deadline = tokio::time::sleep(wait);
        tokio::pin!(deadline);
        loop {
            tokio::select! {
                () = job.done() => return job.answer(),
                _ = tick.tick(), if active => {
                    let elapsed = job.start.elapsed();
                    let message = format!("{} läuft seit {}", job.target.name, elapsed_text(elapsed));
                    progress.report(&message, elapsed);
                }
                () = &mut deadline => {
                    return Ok(format!(
                        "{name} läuft noch seit {} · Lauf-ID {id} · weiter warten mit check_run {{run: {id:?}}}, \
                         Ausgabe mit console_tail check:{name}",
                        elapsed_text(job.start.elapsed()),
                        name = job.target.name,
                        id = job.id,
                    ));
                }
            }
        }
    }
}
