//! `sim_test` wartet wie `check_run` auf das Ende des Laufs (Port von `simtest_wait.go`, B-644): `start` mit `wait` und
//! `wait {id}`. Mit `progressToken` meldet der Aufruf alle `progress_every` ein Lebenszeichen und wartet bis zum Ende;
//! ohne höchstens `check_wait` und nennt danach die Lauf-ID. Der Lauf selbst überlebt den Aufruf wie immer.

use std::path::Path;
use std::time::Duration;

use chrono::Local;
use k3c_kit::mcp::Progress;
use tokio::time::{Instant, interval_at, sleep_until};

use super::simtest::{MAX_STATUS_LINES, unknown_run};
use super::simtest_seam::Sim;
use crate::simrun::info::go_seconds;
use crate::simrun::{DONE, RUNNING, WAITING};

/// Abstand, in dem der wartende Aufruf den Stand des Laufs liest.
const SIM_POLL: Duration = Duration::from_millis(250);

/// Der Lauf rechnet nicht mehr und wartet nicht auf einen Platz.
fn ended(state: &str) -> bool {
    state != RUNNING && state != WAITING
}

/// Wartet auf das Ende des Laufs `id` und liefert den Status. Wird das Future fallen gelassen, endet nur das Warten.
pub(super) async fn await_sim(sim: &Sim, root: &Path, id: &str, progress: &Progress) -> Result<String, String> {
    // Mit Lebenszeichen wartet der Aufruf bis zum Ende (der Lauf endet spätestens mit seinem Zeitlimit).
    let deadline = (!progress.is_active()).then(|| Instant::now() + sim.check_wait);
    let mut poll = interval_at(Instant::now() + SIM_POLL, SIM_POLL);
    let mut beat = interval_at(Instant::now() + sim.progress_every, sim.progress_every);
    loop {
        let info = sim.registry.get(root, id).map_err(|_| unknown_run(id))?;
        // `finish` setzt „fertig“, bevor `simtest.md` steht: sonst fehlte dem Status gelegentlich die Zeile „Bericht:“.
        // „fertig“ gibt es nur über `finish`, das den Bericht immer nachträgt.
        if ended(&info.state) && !(info.state == DONE && info.report.is_empty()) {
            return Ok(info.status(Local::now(), MAX_STATUS_LINES));
        }
        let elapsed = (Local::now() - info.started).to_std().unwrap_or_default();
        // volle Sekunden wie Go `Truncate(time.Second)`
        let age = go_seconds(
            i64::try_from(elapsed.as_secs())
                .unwrap_or(i64::MAX)
                .saturating_mul(1000),
        );
        tokio::select! {
            _ = poll.tick() => {}
            _ = beat.tick() => {
                if progress.is_active() {
                    progress.report(&format!("{} {} seit {age}", info.id, info.state), elapsed);
                }
            }
            () = sleep_until(deadline.unwrap_or_else(Instant::now)), if deadline.is_some() => {
                return Ok(format!(
                    "{} läuft noch seit {age} · weiter warten mit sim_test {{action: wait, id: \"{}\"}}\n{}",
                    info.id,
                    info.id,
                    info.status(Local::now(), MAX_STATUS_LINES)
                ));
            }
        }
    }
}
