//! Die Läufe von `sim balance --window` in Takte zerlegt (RU40.2): `ScenarioRun` spielt ein Szenario Takt für Takt wie
//! `k3c_balance::run::run_one`, `ReplayRun` eine Aufnahme wie `k3c_balance::replay::play`. Beide rechnen mit denselben
//! öffentlichen Bausteinen von `k3c-balance` (Bot, Collector, `step_island`), darum ist das Ergebnis dasselbe wie headless
//! (Test `balance_window_tests`). Ein öffentlicher Schritt-Einstieg in `k3c-balance` würde die Doppelung beenden (Folge-Ticket).

use std::panic::{AssertUnwindSafe, catch_unwind};

use k3c_balance::bot_abstieg::deepest_of;
use k3c_balance::bots::{Bot, bot_by_name};
use k3c_balance::crown_bot::retrieve_crown;
use k3c_balance::fehler::Fehler;
use k3c_balance::helps::after_step;
use k3c_balance::island_world::{Shared, shared};
use k3c_balance::metrics::Collector;
use k3c_balance::parallel::panic_text;
use k3c_balance::replay::{Playback, Replay, data_hash, end_hash};
use k3c_balance::run::{MAX_TICKS_PER_DAY, RunResult, Scenario, TICK_HZ, new_island};
use k3c_sim::island::{Island, step_island};
use k3c_sim::state::PlayerCommand;

/// Der Lauf, der gerade im Fenster rechnet.
pub enum Active {
    Scenario(Box<ScenarioRun>),
    Replay(Box<ReplayRun>),
}

/// Das Ergebnis eines fertigen Laufs.
pub enum Done {
    Scenario(Box<RunResult>),
    Replay(Box<Result<Playback, Fehler>>),
}

impl Active {
    /// Die Insel zum Zeichnen; `None`, wenn der Lauf schon vor dem ersten Takt scheiterte.
    pub fn island(&self) -> Option<&Island> {
        match self {
            Active::Scenario(s) => s.live.as_ref().map(|l| &l.isl),
            Active::Replay(r) => r.live.as_ref().map(|l| &l.isl),
        }
    }

    /// Rechnet einen Takt; nach dem Ende ohne Wirkung.
    pub fn step(&mut self) {
        match self {
            Active::Scenario(s) => s.step(),
            Active::Replay(r) => r.step(),
        }
    }

    pub fn finished(&self) -> bool {
        match self {
            Active::Scenario(s) => s.live.is_none(),
            Active::Replay(r) => r.live.is_none(),
        }
    }

    pub fn into_done(self) -> Done {
        match self {
            Active::Scenario(s) => Done::Scenario(Box::new(s.res)),
            Active::Replay(r) => Done::Replay(Box::new(r.finish())),
        }
    }
}

struct Live {
    isl: Island,
    bot: Bot,
    c: Collector,
    cmds: Vec<PlayerCommand>,
    rp: Replay,
    shared: &'static Shared,
    tick: i64,
}

/// Ein Szenario in Takten (Go: `runOne`).
pub struct ScenarioRun {
    sc: Scenario,
    res: RunResult,
    live: Option<Live>,
}

fn empty_result(sc: &Scenario) -> RunResult {
    RunResult {
        scenario: sc.clone(),
        valid: false,
        error: String::new(),
        ticks: 0,
        metrics: None,
        replay: None,
    }
}

impl ScenarioRun {
    /// Legt Insel, Bot und Messung an; scheitert das, ist der Lauf sofort fertig und trägt den Fehler.
    pub fn start(sc: &Scenario, threshold: i64) -> ScenarioRun {
        let mut run = ScenarioRun {
            sc: sc.clone(),
            res: empty_result(sc),
            live: None,
        };
        match Self::build(sc, threshold) {
            Ok(live) => run.live = Some(live),
            Err(e) => run.res.error = e,
        }
        run
    }

    fn build(sc: &Scenario, threshold: i64) -> Result<Live, String> {
        let shared = shared().map_err(|e| e.to_string())?;
        let isl = new_island(sc).map_err(|e| e.to_string())?;
        let mut bot = bot_by_name(&sc.bot).ok_or_else(|| format!("unbekannter Bot {:?}", sc.bot))?;
        if !sc.world.is_empty() {
            let deepest = deepest_of(&isl);
            let descent = Bot::Descent {
                inner: Box::new(bot),
                deepest,
            };
            bot = Bot::BossFight {
                inner: Box::new(descent),
                deepest,
            };
        }
        Ok(Live {
            c: Collector::new(&isl, threshold),
            cmds: vec![PlayerCommand::default(); sc.players.max(0) as usize],
            rp: Replay::new(sc),
            isl,
            bot,
            shared,
            tick: 0,
        })
    }

    /// Ein Takt; eine Panik wird wie in `run_one` zu einem ungültigen Lauf `abgebrochen: <Meldung>`.
    fn step(&mut self) {
        let Some(live) = self.live.as_mut() else { return };
        let (sc, res) = (&self.sc, &mut self.res);
        match catch_unwind(AssertUnwindSafe(|| live.step(sc, res))) {
            Ok(false) => {}
            Ok(true) => self.live = None,
            Err(p) => {
                res.valid = false;
                res.metrics = None;
                res.error = format!("abgebrochen: {}", panic_text(p.as_ref()));
                self.live = None;
            }
        }
    }
}

impl Live {
    /// Ein Takt von `play` (Go: der Rumpf der Schleife in `runOne`); `true`: der Lauf ist zu Ende.
    fn step(&mut self, sc: &Scenario, res: &mut RunResult) -> bool {
        self.tick += 1;
        for w in &self.isl.stages {
            for p in &w.players {
                if let Some(slot) = self.cmds.get_mut(p.index as usize) {
                    *slot = retrieve_crown(w, p, self.bot.decide(&self.isl, w, p));
                }
            }
        }
        self.rp.record(&self.cmds);
        step_island(
            &mut self.isl,
            &self.cmds,
            &self.shared.data.difficulty,
            1.0 / TICK_HZ as f64,
        );
        after_step(&mut self.isl, &sc.world, &sc.helps, self.tick, TICK_HZ);
        self.c.observe(&self.isl, self.tick);
        res.ticks = self.tick;
        let lost = self.isl.over;
        if lost || self.isl.stages[0].cycle.day > sc.days {
            self.finish(res, lost);
            return true;
        }
        if self.tick >= sc.days * MAX_TICKS_PER_DAY {
            res.error = format!("Tag {} nach {} Ticks nicht beendet", sc.days, res.ticks);
            return true;
        }
        false
    }

    fn finish(&mut self, res: &mut RunResult, lost: bool) {
        let mut metrics = self.c.result(&self.isl);
        if lost {
            metrics.lost = "crown".into();
        }
        self.rp.end_hash = end_hash(&self.isl);
        self.rp.castle_fall_tick = metrics.castle_fall_tick;
        res.valid = true;
        res.metrics = Some(metrics);
        res.replay = Some(std::mem::take(&mut self.rp));
    }
}

struct ReplayLive {
    isl: Island,
    c: Collector,
    cmds: Vec<PlayerCommand>,
    pos: Vec<usize>,
    left: Vec<i64>,
    shared: &'static Shared,
    tick: i64,
}

/// Eine Aufnahme in Takten (Go: `Play`).
pub struct ReplayRun {
    r: Replay,
    pb: Playback,
    err: Option<Fehler>,
    live: Option<ReplayLive>,
}

impl ReplayRun {
    pub fn start(r: &Replay) -> ReplayRun {
        let mut run = ReplayRun {
            r: r.clone(),
            pb: Playback::default(),
            err: None,
            live: None,
        };
        let current = data_hash();
        if r.data_hash != current {
            run.pb.warnings.push(format!(
                "Datenstand weicht ab: Datei {}, aktuell {current}",
                r.data_hash
            ));
        }
        match Self::build(r) {
            Ok(live) => run.live = Some(live),
            Err(e) => run.err = Some(e),
        }
        run
    }

    fn build(r: &Replay) -> Result<ReplayLive, Fehler> {
        let isl = new_island(&r.scenario)?;
        let n = r.scenario.players.max(0) as usize;
        if r.inputs.len() != n {
            return Err(Fehler::Daten(format!(
                "replay: {} Eingabe-Listen für {n} Spieler",
                r.inputs.len()
            )));
        }
        Ok(ReplayLive {
            c: Collector::new(&isl, 0),
            cmds: vec![PlayerCommand::default(); n],
            pos: vec![0; n],
            left: vec![0; n],
            shared: shared()?,
            isl,
            tick: 0,
        })
    }

    fn step(&mut self) {
        let Some(live) = self.live.as_mut() else { return };
        match live.step(&self.r) {
            Ok(false) => {}
            Ok(true) => {
                self.pb.ticks = self.r.ticks;
                self.pb.end_hash = end_hash(&live.isl);
                self.pb.castle_fall_tick = live.c.result(&live.isl).castle_fall_tick;
                self.live = None;
            }
            Err(e) => {
                self.err = Some(e);
                self.live = None;
            }
        }
    }

    fn finish(self) -> Result<Playback, Fehler> {
        self.err.map_or(Ok(self.pb), Err)
    }
}

impl ReplayLive {
    /// Ein Takt von `play`; `true`: alle Ticks der Aufnahme sind gespielt (eine Aufnahme ohne Ticks ist sofort fertig).
    fn step(&mut self, r: &Replay) -> Result<bool, Fehler> {
        if self.tick >= r.ticks {
            return Ok(true);
        }
        self.tick += 1;
        for (i, spans) in r.inputs.iter().enumerate() {
            if self.left[i] == 0 {
                let s = spans.get(self.pos[i]).ok_or_else(|| {
                    Fehler::Daten(format!(
                        "replay: Spieler {i} hat zu wenige Eingaben für Tick {}",
                        self.tick
                    ))
                })?;
                (self.cmds[i], self.left[i]) = (s.command.clone(), s.n);
                self.pos[i] += 1;
            }
            self.left[i] -= 1;
        }
        step_island(
            &mut self.isl,
            &self.cmds,
            &self.shared.data.difficulty,
            1.0 / TICK_HZ as f64,
        );
        after_step(&mut self.isl, &r.scenario.world, &r.scenario.helps, self.tick, TICK_HZ);
        self.c.observe(&self.isl, self.tick);
        Ok(self.tick >= r.ticks)
    }
}
