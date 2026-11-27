//! `sim_test` (Port von `simtest.go`, B-348): ein Tool für jeden Testlauf. `start` kehrt sofort mit einer Lauf-ID
//! zurück, der Lauf arbeitet im Hintergrund des Studios im Checkout der Session; `status` zeigt ihn in höchstens 10
//! Zeilen, `stop` bricht ab, `list` zeigt die Läufe des Checkouts. Register, Warteschlange und Offline-Runner stehen in
//! `crate::simrun`; `sim_test` reiht nicht ein, sondern lehnt bei belegten Plätzen ab (wie vor WZ28.4).

use std::path::Path;

use chrono::Local;
use k3c_kit::mcp::Progress;
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::service::{RequestContext, RoleServer};
use rmcp::{tool, tool_router};

use super::kit::Workspace;
use super::simtest_baseline;
use super::simtest_lock::lock_err;
use super::simtest_seam::Sim;
use super::simtest_spec::SimTestIn;
use super::simtest_wait::await_sim;
use super::tools::Studio;
use super::tools_world::blocking;
use crate::balance::baselines::has_baseline;
use crate::kitx::go_quoted;
use crate::simrun::{MAX_PER_ROOT, RUNNING, WAITING};

/// `status` und `list` zeigen höchstens 10 Zeilen (B-348).
pub(super) const MAX_STATUS_LINES: usize = 10;

pub(super) fn unknown_run(id: &str) -> String {
    format!(
        "lauf {} unbekannt in diesem Checkout; sim_test list zeigt die Läufe",
        go_quoted(id)
    )
}

/// Das Tool `sim_test`. Der Abbruch des Aufrufs beendet nur das Warten, nie den Lauf.
pub(super) async fn sim_test(
    sim: &Sim,
    ws: &Workspace,
    input: SimTestIn,
    progress: Progress,
) -> Result<String, String> {
    let (sim, root) = (sim.clone(), ws.root.clone());
    match input.action.as_str() {
        "start" => {
            let wait = input.wait;
            let (starter, place) = (sim.clone(), ws.short().to_owned());
            let dir = root.clone();
            let (id, label) = blocking(move || start(&starter, &dir, &place, &input)).await?;
            if wait {
                return await_sim(&sim, &root, &id, &progress).await;
            }
            Ok(format!(
                "▶ {id} {label} gestartet; Fortschritt mit sim_test status {id}"
            ))
        }
        "wait" => await_sim(&sim, &root, &input.id, &progress).await,
        "status" => blocking(move || status(&sim, &root, &input.id)).await,
        "stop" => blocking(move || stop(&sim, &root, &input.id)).await,
        "list" => blocking(move || Ok(list(&sim, &root))).await,
        "baseline" => blocking(move || simtest_baseline::baseline(&sim, &root, &input)).await,
        other => Err(format!(
            "action {} unbekannt: start, status, wait, stop, list oder baseline",
            go_quoted(other)
        )),
    }
}

/// Prüft und startet einen Lauf; liefert Lauf-ID und Label.
fn start(sim: &Sim, root: &Path, place: &str, input: &SimTestIn) -> Result<(String, String), String> {
    lock_err((sim.unlocked)(), "sim_test start")?;
    let spec = input.spec(root)?;
    let baseline = &spec.offline.baseline;
    if !baseline.is_empty() && !has_baseline(root, baseline) {
        return Err(format!(
            "sim_test start abgelehnt: baseline {} gibt es nicht; sim_test {{action: baseline}} listet die gesicherten",
            go_quoted(baseline)
        ));
    }
    let label = spec.label();
    let runner = sim.registry.offline(spec.offline.clone());
    let run = sim
        .registry
        .start(root, place, &label, Some(spec.offline), runner, false)
        .map_err(|_| format!("schon {MAX_PER_ROOT} Läufe in diesem Checkout; mit sim_test stop einen beenden"))?;
    Ok((run.id().to_owned(), label))
}

fn status(sim: &Sim, root: &Path, id: &str) -> Result<String, String> {
    let info = sim.registry.get(root, id).map_err(|_| unknown_run(id))?;
    Ok(info.status(Local::now(), MAX_STATUS_LINES))
}

fn stop(sim: &Sim, root: &Path, id: &str) -> Result<String, String> {
    let before = sim
        .registry
        .stop(root, id)
        .map_err(|_| format!("lauf {} unbekannt in diesem Checkout", go_quoted(id)))?;
    Ok(match before {
        WAITING => format!("■ {id} entfernt, er wartete noch auf einen Platz"),
        RUNNING => {
            format!("■ {id} abgebrochen; der Bericht folgt, sobald der Runner aufgeräumt hat (sim_test status {id})")
        }
        other => format!("{id} ist schon {other}"),
    })
}

pub(super) fn list(sim: &Sim, root: &Path) -> String {
    let out: Vec<String> = sim
        .registry
        .list(root)
        .iter()
        .take(MAX_STATUS_LINES)
        .map(|info| info.head(Local::now()))
        .collect();
    if out.is_empty() {
        return "keine Testläufe in diesem Checkout; starten mit sim_test {action: start, mode: offline}".to_owned();
    }
    out.join("\n")
}

#[tool_router(router = simtest_router, vis = "pub(super)")]
impl Studio {
    #[tool(
        description = "Jeder Testlauf (Balancing, Performance, Stabilität) über ein Tool: action start (sofort zurück mit ID), status (≤ 10 Zeilen), stop, list. Nur mode offline (die Spiel-EXE des Checkouts rechnet, k3c-game sim balance; der Spielserver ist entfernt, B-887), focus balance. head: true zeigt den Lauf im Spiel mit Fenster (speed 1 bis 64, nur mit head, ein Lauf zugleich), sonst headless. stop beendet die EXE samt Kindprozessen. Bericht unter reports/simtest-<id>/. Ohne seeds gilt seedMode aus data/balance-targets.json (dev: nur k3c-dev); bots (z. B. coop2) und seedList. world (z. B. standard) und days (1–60): der Lauf spielt die ganze erste Insel. wait: true (bei start) oder action wait {id} wartet wie check_run auf das Ende (mit progressToken bis zum Ende, sonst höchstens 4 min). BAS-Standardlauf: {action: start, bots: coop2}; Meilenstein-Lauf: {action: start, bots: coop2, world: standard, days: 40}. Delta-Runde (B-640): action baseline {id, saveBaseline: M1} sichert einen durchgelaufenen Lauf als Baseline, start mit baseline: M1 vergleicht je Werte-Gruppe. start ist bis auf Widerruf gesperrt (Beschluss 🧑 2026-11-11, aufheben: Studio mit K3C_SIM_ENTSPERRT=1). Nutze es bei: jedem Balancing-, Performance- oder Stabilitätslauf. Statt: task balance oder Binaries in der Shell.",
        annotations(read_only_hint = false, idempotent_hint = false, open_world_hint = false)
    )]
    async fn sim_test(
        &self,
        Parameters(input): Parameters<SimTestIn>,
        Extension(ws): Extension<Workspace>,
        Extension(progress): Extension<Progress>,
        context: RequestContext<RoleServer>,
    ) -> Result<String, String> {
        tokio::select! {
            text = sim_test(&self.sim, &ws, input, progress) => text,
            () = context.ct.cancelled() => Err("context canceled".to_owned()),
        }
    }
}

#[cfg(test)]
#[path = "simtest_baseline_tests.rs"]
mod baseline_tests;
#[cfg(test)]
#[path = "simtest_checkout_tests.rs"]
mod checkout_tests;
#[cfg(test)]
#[path = "simtest_lock_tests.rs"]
mod lock_tests;
#[cfg(test)]
#[path = "simtest_tests.rs"]
mod tests;
#[cfg(test)]
#[path = "simtest_wait_tests.rs"]
mod wait_tests;
