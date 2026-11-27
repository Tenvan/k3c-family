//! `sim_run` rechnet mit der Spiel-EXE des Checkouts (`k3c-game sim run`, Port von `tools_sim.go`, RU34.2); während der
//! Sim-Sperre lehnt es ab (B-788). Die Registrierung von `sim_run` und `replay_run` (Go `registerSim`) steht hier,
//! `replay_run` selbst in `tools_replay`.

use std::time::Duration;

use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::service::{RequestContext, RoleServer};
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};

use super::kit::Workspace;
use super::simtest_lock::lock_err;
use super::tools::Studio;
use super::tools_engine::{self, ENGINE_TIMEOUT};
use super::tools_replay::{self, ReplayIn};
use crate::gameexe::{self, Event, Options};

/// Begrenzt einen Lauf mit Fenster: man sieht zu und kann das Fenster schließen.
pub(super) const HEAD_TIMEOUT: Duration = Duration::from_secs(60 * 60);

// Die Signatur verlangt serde `skip_serializing_if`.
#[expect(clippy::trivially_copy_pass_by_ref)]
fn zero(v: &i64) -> bool {
    *v == 0
}

#[expect(clippy::trivially_copy_pass_by_ref)]
fn zero_f(v: &f64) -> bool {
    *v == 0.0
}

#[expect(clippy::trivially_copy_pass_by_ref)]
fn off(v: &bool) -> bool {
    !*v
}

/// Gedrückte Tasten eines Monarchen über einen Tick-Bereich (engine/enginetools › Segment).
#[derive(Debug, Default, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(inline)]
pub(super) struct Segment {
    #[serde(default, skip_serializing_if = "zero")]
    #[schemars(description = "Monarch ab 0, höchstens 3")]
    pub(super) player: i64,
    #[schemars(description = "erster Tick (einschließlich)")]
    pub(super) from_tick: i64,
    #[schemars(description = "letzter Tick (ausschließlich)")]
    pub(super) to_tick: i64,
    #[serde(default, skip_serializing_if = "zero_f")]
    #[schemars(description = "-1 (links) bis 1 (rechts)")]
    pub(super) move_x: f64,
    #[serde(default, skip_serializing_if = "off")]
    pub(super) sprint: bool,
    #[serde(default, skip_serializing_if = "off")]
    #[schemars(description = "Bezahl-Taste gehalten")]
    pub(super) pay: bool,
}

/// `null` ist wie bei Gos Slices die leere Liste.
fn null_segments<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<Segment>, D::Error> {
    Ok(Option::<Vec<Segment>>::deserialize(d)?.unwrap_or_default())
}

/// Argumente von `sim_run`; serialisiert die Eingabe-JSON der EXE (ohne `head` und `speed`, die sind Schalter).
#[derive(Debug, Default, Deserialize, Serialize, JsonSchema)]
pub(super) struct SimIn {
    #[schemars(description = "Seed als Text")]
    pub(super) seed: String,
    #[schemars(description = "Anzahl Ticks, 1 bis 100000 (30 pro Sekunde)")]
    pub(super) ticks: i64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    #[schemars(description = "forest (Standard), cave oder mine")]
    pub(super) biome: String,
    #[serde(default, deserialize_with = "null_segments", skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "gedrückte Tasten je Monarch und Tick-Bereich; ohne Angabe ein Monarch ohne Eingaben")]
    pub(super) inputs: Vec<Segment>,
    #[serde(default, skip_serializing)]
    #[schemars(description = "Spiel mit Fenster öffnen und den Lauf zeigen; ohne Angabe headless")]
    pub(super) head: bool,
    #[serde(default, skip_serializing)]
    #[schemars(description = "Zeitraffer 1 bis 64 (Standard 1 = Echtzeit), nur mit head")]
    pub(super) speed: i64,
}

/// Prüft `speed` wie `sim_run` und `replay_run`.
pub(super) fn check_speed(tool: &str, head: bool, speed: i64) -> Result<(), String> {
    if speed != 0 && !head {
        return Err(format!("{tool}: speed gilt nur mit head"));
    }
    if !(0..=64).contains(&speed) {
        return Err(format!("{tool}: speed muss 1 bis 64 sein"));
    }
    Ok(())
}

/// Das Tool `sim_run`. `call` bekommt Optionen und Zeitlimit ([`tools_engine::game_call_within`]); Tests ersetzen ihn.
pub(super) async fn sim_run<F>(
    unlocked: bool,
    ws: &Workspace,
    input: &SimIn,
    call: impl FnOnce(Options, Duration) -> F,
) -> Result<String, String>
where
    F: Future<Output = Result<Event, gameexe::Error>>,
{
    lock_err(unlocked, "sim_run")?;
    check_speed("sim_run", input.head, input.speed)?;
    let limit = if input.head { HEAD_TIMEOUT } else { ENGINE_TIMEOUT };
    let options = Options {
        root: ws.root.clone(),
        sub: "run".to_owned(),
        input: serde_json::to_vec(input).map_err(|e| e.to_string())?,
        window: input.head,
        speed: input.speed,
        ..Options::default()
    };
    call(options, limit).await.map(|ev| ev.text).map_err(|e| e.to_string())
}

/// Die Engine-Tools des Balancings (B-047): rechnen mit der Spiel-EXE des Checkouts, ohne laufenden Server.
#[tool_router(router = sim_router, vis = "pub(super)")]
impl Studio {
    #[tool(
        description = "Deterministischer Simulationslauf mit der Spiel-EXE des Checkouts (k3c-game sim run, kein Server nötig), höchstens 100000 Ticks (30/s): Tag, Welle, Gold, Truppen, Verluste, Ende. Optional Eingaben je Monarch und Tick-Bereich (höchstens 100 Segmente). Mit head: true öffnet sich das Spiel und zeigt den Lauf (speed 1 bis 64), sonst läuft er headless. Bis auf Widerruf gesperrt (Beschluss 🧑 2026-11-11, aufheben: Studio mit K3C_SIM_ENTSPERRT=1). Nutze es bei: kurzer deterministischer Regel-Check. Statt: go run oder ein Spiel von Hand.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn sim_run(
        &self,
        Parameters(input): Parameters<SimIn>,
        Extension(ws): Extension<Workspace>,
        context: RequestContext<RoleServer>,
    ) -> Result<String, String> {
        let canceled = context.ct.cancelled_owned();
        sim_run((self.sim.unlocked)(), &ws, &input, |o, limit| {
            tools_engine::game_call_within(o, limit, canceled)
        })
        .await
    }

    #[tool(
        description = "Spielt eine Replay-Datei des Balancing-Testers (task balance:run -- --replay-dir) mit der Spiel-EXE des Checkouts (k3c-game sim balance --play) ohne Bot ab: Endzustand-Hash, Burgfall-Tick, Vergleich mit der Aufnahme, Warnung bei anderem Datenstand. Nur Dateien im Repo. Mit head: true zeigt das Spiel die Wiedergabe im Fenster (speed 1 bis 64). Nutze es bei: Replay nachrechnen oder Abweichung finden. Statt: Balancing-Binary in der Shell.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn replay_run(
        &self,
        Parameters(input): Parameters<ReplayIn>,
        Extension(ws): Extension<Workspace>,
        context: RequestContext<RoleServer>,
    ) -> Result<String, String> {
        let canceled = context.ct.cancelled_owned();
        tools_replay::replay_run(&ws, &input, |o, limit| {
            tools_engine::game_call_within(o, limit, canceled)
        })
        .await
    }
}
