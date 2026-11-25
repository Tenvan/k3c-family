//! Matrix, Einzellauf und Bericht (Port von `engine/balance/run.go`, B-099, BAL1): Der Balancing-Tester spielt eine
//! Szenario-Matrix headless mit Bots und liefert Kennzahlen je Lauf als JSON. Die Simulation (`k3c-sim`) wird nur benutzt;
//! gleiche Matrix und gleiche Daten ergeben byte-gleiche Ausgabe (keine Wanduhr, keine Map-Iteration in der Ausgabe).

use std::fmt;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;

use k3c_sim::island::{Island, add_island_player, options::set_grade, step_island};
use k3c_sim::state::PlayerCommand;
use serde::{Deserialize, Serialize};

use crate::bot_abstieg::deepest_of;
use crate::bots::{Bot, MIX_SEP, bot_by_name, bot_names, check_players};
use crate::crown_bot::retrieve_crown;
use crate::fehler::Fehler;
use crate::helps::{after_step, apply_helps, check_helps, grade_of};
use crate::parallel::{panic_text, run_all, tick_progress};
use crate::pending::{Collector, Metrics, Replay, create_island, end_hash, shared};

/// Takte je Sekunde (Go: `enginetools.TickHz`); der Crate hängt nicht von `k3c-game` ab.
pub const TICK_HZ: i64 = 30;

/// Begrenzt einen Lauf: eine Stunde Spielzeit je Tag (ein Tag dauert heute 16 min, data/biomes).
pub const MAX_TICKS_PER_DAY: i64 = 3600 * TICK_HZ;

/// Wird nach jedem fertigen Lauf aufgerufen (Go: `Matrix.Tick`); eine Panik darin beendet den Lauf (`parallel.rs`, B-924).
#[derive(Clone)]
pub struct Tick(pub Arc<dyn Fn() + Send + Sync>);

impl fmt::Debug for Tick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Tick(..)")
    }
}

/// Beschreibt alle Läufe: jedes Paar aus Seed × Spieleranzahl × Bot × Tiefe läuft `days` Tage.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Matrix {
    pub seeds: Vec<String>,
    pub players: Vec<i64>,
    pub bots: Vec<String>,
    /// Tiefe = Biom (0 Wald, 1 Höhle, …).
    pub depths: Vec<i64>,
    pub days: i64,
    /// Schwelle für `Metrics::gold_threshold_tick` (Gold aller Monarchen zusammen); 0 = nicht messen.
    pub gold_threshold: i64,
    /// Schwierigkeitsgrad aller Läufe, leer = normal (Grad-Kurven, BAL3.3).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub grade: String,
    /// ID der Welt (z. B. standard); gesetzt baut der Lauf alle Tiefen der ersten Insel (BS11, B-642), `depths` gilt dann nicht.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub world: String,
    /// Dev-Hilfen wie im Stufen-Test (hilfen, voll, boss; `helps.rs`), nur mit Welt; leer = keine.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub helps: String,
    /// Steht nicht im Bericht (Go: `json:"-"`).
    #[serde(skip)]
    pub tick: Option<Tick>,
}

/// Ein einzelner Lauf.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Scenario {
    pub seed: String,
    pub players: i64,
    pub bot: String,
    pub depth: i64,
    pub days: i64,
    /// Leer = normal; steht so auch im Replay.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub grade: String,
    /// Leer = eine Stufe der Tiefe `depth`; sonst alle Tiefen der ersten Insel der Welt (BS11).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub world: String,
    /// Dev-Hilfen (`helps.rs`), leer = keine.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub helps: String,
}

/// Ein Lauf mit Kennzahlen (Go: `Result`); ein ungültiger Lauf hat `error` statt `metrics`.
#[derive(Debug, Clone, Serialize)]
pub struct RunResult {
    #[serde(flatten)]
    pub scenario: Scenario,
    pub valid: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub error: String,
    pub ticks: i64,
    pub metrics: Option<Metrics>,
    /// Die Aufnahme des Laufs (nur gültige Läufe); sie steht nicht im Bericht, der Befehl schreibt sie auf Wunsch als eigene
    /// Datei (BAL1.2).
    #[serde(skip)]
    pub replay: Option<Replay>,
}

/// Die Ausgabe: die Matrix und alle Läufe in der Reihenfolge der Matrix.
#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub matrix: Matrix,
    pub results: Vec<RunResult>,
}

impl Matrix {
    /// Prüft die Matrix und zählt die Läufe auf (Seeds außen, Tiefe innen; Go: `Scenarios`).
    pub fn scenarios(&self) -> Result<Vec<Scenario>, Fehler> {
        let bad = |s: String| Err(Fehler::Matrix(s));
        if self.seeds.is_empty() || self.players.is_empty() || self.bots.is_empty() || self.depths.is_empty() {
            return bad("matrix: Seeds, Spieler, Bots und Tiefen brauchen je mindestens einen Wert".into());
        }
        if self.days < 1 {
            return bad(format!("matrix: days muss mindestens 1 sein, war {}", self.days));
        }
        if let Some(n) = self.players.iter().find(|n| !(1..=4).contains(*n)) {
            return bad(format!("matrix: Spieleranzahl muss 1 bis 4 sein, war {n}"));
        }
        self.check_bots()?;
        check_helps(&self.helps, &self.world).map_err(|e| Fehler::Matrix(format!("matrix: {e}")))?;
        let mut out = Vec::new();
        for seed in &self.seeds {
            for &n in &self.players {
                for b in &self.bots {
                    for &d in &self.depths {
                        out.push(Scenario {
                            seed: seed.clone(),
                            players: n,
                            bot: b.clone(),
                            depth: d,
                            days: self.days,
                            grade: self.grade.clone(),
                            world: self.world.clone(),
                            helps: self.helps.clone(),
                        });
                    }
                }
            }
        }
        Ok(out)
    }

    /// Jeder Bot ist bekannt und hat in jeder Spieleranzahl genug Monarchen (Go: `checkBots`).
    pub fn check_bots(&self) -> Result<(), Fehler> {
        for b in &self.bots {
            if bot_by_name(b).is_none() {
                return Err(Fehler::Matrix(format!(
                    "matrix: unbekannter Bot {b:?} (bekannt: [{}], je Spieler mit {MIX_SEP})",
                    bot_names().join(" ")
                )));
            }
            for &n in &self.players {
                check_players(b, n).map_err(|e| Fehler::Matrix(format!("matrix: {e}")))?;
            }
        }
        Ok(())
    }
}

impl Report {
    /// Schreibt den Bericht eingerückt mit Zeilenende; Feldreihenfolge aus den Structs (Go: `JSON`).
    pub fn json(&self) -> Result<String, Fehler> {
        let mut s = serde_json::to_string_pretty(self).map_err(|e| Fehler::Daten(format!("balance: {e}")))?;
        s.push('\n');
        Ok(s)
    }
}

/// Spielt alle Läufe der Matrix (parallel, `parallel.rs`). Ein abgebrochener Lauf erscheint als ungültig, der Rest läuft
/// weiter; ein Absturz im `tick` beendet die Matrix mit `Fehler::Abbruch` (Go: `Run`).
pub fn run(m: &Matrix) -> Result<Report, Fehler> {
    let scenarios = m.scenarios()?;
    let progress = tick_progress(m.tick.clone(), None);
    let results = run_all(&scenarios, m.gold_threshold, progress).map_err(Fehler::Abbruch)?;
    Ok(Report {
        matrix: m.clone(),
        results,
    })
}

/// Rechnet einen Lauf bis zum Morgen nach dem letzten Tag (`Cycle.day > days`); eine Panik wird zu einem ungültigen Lauf
/// `abgebrochen: <Meldung>` (Go: `runOne`).
pub fn run_one(sc: &Scenario, threshold: i64) -> RunResult {
    let mut res = RunResult {
        scenario: sc.clone(),
        valid: false,
        error: String::new(),
        ticks: 0,
        metrics: None,
        replay: None,
    };
    if let Err(p) = catch_unwind(AssertUnwindSafe(|| play(sc, threshold, &mut res))) {
        res.valid = false;
        res.metrics = None;
        res.error = format!("abgebrochen: {}", panic_text(p.as_ref()));
    }
    res
}

/// Der Lauf selbst; `res` trägt Ticks, Ergebnis und Fehler (Go: der Rumpf von `runOne`).
fn play(sc: &Scenario, threshold: i64, res: &mut RunResult) {
    let shared = match shared() {
        Ok(s) => s,
        Err(e) => return res.error = e.to_string(),
    };
    let mut isl = match new_island(sc) {
        Ok(i) => i,
        Err(e) => return res.error = e.to_string(),
    };
    // Go ruft einen unbekannten Bot ohne Prüfung und stürzt ab; `Matrix::scenarios` lässt ihn nie durch.
    let Some(mut bot) = bot_by_name(&sc.bot) else {
        return res.error = format!("unbekannter Bot {:?}", sc.bot);
    };
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
    let mut c = Collector::new(&isl, threshold);
    let mut cmds = vec![PlayerCommand::default(); sc.players.max(0) as usize];
    let mut rp = Replay::new(sc);
    for tick in 1..=sc.days * MAX_TICKS_PER_DAY {
        decide_all(&isl, &bot, &mut cmds);
        rp.record(&cmds);
        step_island(&mut isl, &cmds, &shared.data.difficulty, 1.0 / TICK_HZ as f64);
        after_step(&mut isl, &sc.world, &sc.helps, tick, TICK_HZ);
        c.observe(&isl, tick);
        res.ticks = tick;
        // Kronenraub: der Lauf ist ein gültiges Ergebnis „verloren“.
        let lost = isl.over;
        if lost || isl.stages[0].cycle.day > sc.days {
            return finish(res, &isl, c.result(&isl), rp, lost);
        }
    }
    res.error = format!("Tag {} nach {} Ticks nicht beendet", sc.days, res.ticks);
}

/// Jeder Monarch jeder Stufe bekommt seine Eingabe: Profil, dann Krone aufheben (Go: die Schleife in `runOne`).
fn decide_all(isl: &Island, bot: &Bot, cmds: &mut [PlayerCommand]) {
    for w in &isl.stages {
        for p in &w.players {
            if let Some(slot) = cmds.get_mut(p.index as usize) {
                *slot = retrieve_crown(w, p, bot.decide(isl, w, p));
            }
        }
    }
}

/// Schreibt das Ergebnis eines beendeten Laufs samt Aufnahme.
fn finish(res: &mut RunResult, isl: &Island, mut metrics: Metrics, mut rp: Replay, lost: bool) {
    if lost {
        metrics.lost = "crown".into();
    }
    rp.end_hash = end_hash(isl);
    rp.castle_fall_tick = metrics.castle_fall_tick;
    res.valid = true;
    res.metrics = Some(metrics);
    res.replay = Some(rp);
}

/// Erzeugt die Insel eines Szenarios mit seinen Monarchen auf Stufe 0 (Lauf und Wiedergabe; Go: `newIsland`).
pub fn new_island(sc: &Scenario) -> Result<Island, Fehler> {
    let mut isl = create_island(sc)?;
    let grade = grade_of(&sc.grade, &sc.helps);
    if !grade.is_empty() {
        set_grade(&mut isl, &grade, true).map_err(Fehler::Sim)?;
    }
    for _ in 0..sc.players {
        add_island_player(&mut isl, 0);
    }
    apply_helps(&mut isl, &shared()?.data.difficulty, &sc.helps);
    Ok(isl)
}
