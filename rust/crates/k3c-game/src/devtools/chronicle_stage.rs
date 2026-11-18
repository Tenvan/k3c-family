//! Port von `native/k3c-game/devtools/chronicle_stage.go`: Wechsel und Zusammenfassungen der Chronik je Stufe, Spieler und
//! Takt (`chronicle.rs`).

use k3c_sim::state::{Event as SimEvent, World};
use serde_json::Value;

use super::chronicle::{Chronicle, changed, changed_str};
use crate::view::MAX_PLAYERS;
use crate::view::frame::{Frame, Stage};

/// NightStats ist der Kampf einer Nacht in einer Stufe.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NightStats {
    pub day: i64,
    pub spawned: i64,
    pub killed: i64,
    /// eigene Verluste (Monarch am Boden, Bau zerstört, Truppen weniger)
    pub losses: i64,
    /// Truppen bei Nachtbeginn
    pub troops: i64,
    pub dmg_enemy: f64,
    pub dmg_own: f64,
}

impl NightStats {
    /// add zählt ein Kampf-Ereignis der Nacht.
    pub fn add(&mut self, typ: &str, e: &SimEvent) {
        match typ {
            "wave" => self.spawned += trunc(num(e.get("count"))),
            "kill" => self.killed += 1,
            "crownFallen" | "destroyed" => self.losses += 1,
            "hit" => {
                if e.get("target").and_then(Value::as_str) == Some("enemy") {
                    self.dmg_enemy += num(e.get("damage"));
                } else {
                    self.dmg_own += num(e.get("damage"));
                }
            }
            _ => {}
        }
    }
}

impl Chronicle {
    /// stage_changes loggt Tageszeit und Welle beim Wechsel und wertet die Ereignisse der Stufe aus.
    pub(super) fn stage_changes(&mut self, f: &Frame, st: &Stage) {
        let Some(w) = st.world.as_deref() else {
            return;
        };
        if changed_str(&mut self.run.phase, st.index, &w.cycle.phase) {
            self.log.info(
                "🚦 Tageszeit",
                &[
                    ("stufe", st.index.to_string()),
                    ("tag", w.cycle.day.to_string()),
                    ("phase", w.cycle.phase.clone()),
                ],
            );
        }
        if changed(&mut self.run.wave, st.index, Value::from(w.wave)) {
            self.log.info(
                "🚦 Welle",
                &[
                    ("stufe", st.index.to_string()),
                    ("welle", w.wave.to_string()),
                    ("gegner", w.enemies.len().to_string()),
                ],
            );
        }
        for e in f.events_of(st.index) {
            self.event(st.index, w, &e.data);
        }
    }

    /// event wertet ein Ereignis aus: Bau fertig und Burgfall sofort, Kampf in die Nacht der Stufe.
    fn event(&mut self, stage: i64, w: &World, e: &SimEvent) {
        let typ = e.get("type").and_then(Value::as_str).unwrap_or("");
        match typ {
            "built" => self.log.info(
                "🚦 Bau fertig",
                &[
                    ("stufe", stage.to_string()),
                    ("art", show(e.get("kind"))),
                    ("x", show(e.get("x"))),
                ],
            ),
            "castleFallen" => self.log.info(
                "🚦 Burgfall",
                &[("stufe", stage.to_string()), ("tag", w.cycle.day.to_string())],
            ),
            "night" => {
                let troops = i64::try_from(w.troops.len()).unwrap_or(i64::MAX);
                let n = NightStats {
                    day: w.cycle.day,
                    troops,
                    ..NightStats::default()
                };
                self.run.nights.insert(stage, n);
            }
            "dawn" => self.end_night(stage, w),
            _ => {
                if let Some(n) = self.run.nights.get_mut(&stage) {
                    n.add(typ, e);
                }
            }
        }
    }

    /// end_night loggt die Kampf-Zusammenfassung der Nacht der Stufe und vergisst sie.
    fn end_night(&mut self, stage: i64, w: &World) {
        let Some(n) = self.run.nights.remove(&stage) else {
            return;
        };
        let now_troops = i64::try_from(w.troops.len()).unwrap_or(i64::MAX);
        let lost = n.losses + (n.troops - now_troops).max(0);
        self.log.info(
            "🏹 Kampf der Nacht",
            &[
                ("stufe", stage.to_string()),
                ("tag", n.day.to_string()),
                ("gespawnt", n.spawned.to_string()),
                ("getötet", n.killed.to_string()),
                ("verluste", lost.to_string()),
                ("schaden_an_gegnern", round1(n.dmg_enemy).to_string()),
                ("schaden_eigene_seite", round1(n.dmg_own).to_string()),
            ],
        );
    }

    /// seats loggt Stufenwechsel je Spieler, zählt Einnahmen und Ausgaben (Gold-Änderungen) und fasst sie je Tag zusammen.
    pub(super) fn seats(&mut self, f: &Frame) {
        for s in &f.seats {
            let Some(p) = slot(s.player) else {
                continue;
            };
            let r = &mut self.run;
            if r.stage[p] >= 0 && r.stage[p] != s.stage {
                self.log.info(
                    "🚦 Stufenwechsel",
                    &[
                        ("spieler", (p + 1).to_string()),
                        ("von", r.stage[p].to_string()),
                        ("nach", s.stage.to_string()),
                    ],
                );
            }
            r.stage[p] = s.stage;
            let Some(m) = f.monarch(s.player) else {
                continue;
            };
            let d = m.gold - r.gold[p];
            if r.seen[p] && d > 0 {
                r.income[p] += d;
            } else if r.seen[p] && d < 0 {
                r.spending[p] -= d;
            }
            r.gold[p] = m.gold;
            r.seen[p] = true;
        }
        let day = f.stages.first().and_then(|st| st.world.as_deref()).map(|w| w.cycle.day);
        if let Some(day) = day.filter(|d| *d > self.run.day) {
            self.economy(f);
            self.run.day = day;
        }
    }

    /// economy loggt Einnahmen und Ausgaben je Spieler des abgelaufenen Tags und setzt sie zurück.
    fn economy(&mut self, f: &Frame) {
        for s in &f.seats {
            let Some(p) = slot(s.player) else {
                continue;
            };
            let r = &mut self.run;
            self.log.info(
                "🪙 Wirtschaft des Tags",
                &[
                    ("tag", r.day.to_string()),
                    ("spieler", (p + 1).to_string()),
                    ("einnahmen", r.income[p].to_string()),
                    ("ausgaben", r.spending[p].to_string()),
                    ("gold", r.gold[p].to_string()),
                ],
            );
            r.income[p] = 0;
            r.spending[p] = 0;
        }
    }

    /// budget loggt einen Takt über Budget mit der Objektanzahl, höchstens alle `slowLogEverySeconds`; die Zahl der
    /// langsamen Takte seither steht dabei.
    pub(super) fn budget(&mut self, f: &Frame) {
        if f.step_ms <= self.vals.step_budget_ms {
            return;
        }
        self.run.slow_n += 1;
        let every = ticks_of(self.vals.slow_log_every_seconds * 30.0);
        if self.run.slow_at != 0 && f.tick < self.run.slow_at + every {
            return;
        }
        let (mut troops, mut enemies, mut coins) = (0, 0, 0);
        for w in f.stages.iter().filter_map(|st| st.world.as_deref()) {
            troops += w.troops.len();
            enemies += w.enemies.len();
            coins += w.coins.len();
        }
        self.log.warn(
            "🐢 Takt über Budget",
            &[
                ("ms", round1(f.step_ms).to_string()),
                ("budget_ms", self.vals.step_budget_ms.to_string()),
                ("langsame_takte", self.run.slow_n.to_string()),
                ("takt", f.tick.to_string()),
                ("truppen", troops.to_string()),
                ("gegner", enemies.to_string()),
                ("muenzen", coins.to_string()),
            ],
        );
        self.run.slow_at = f.tick.max(1);
        self.run.slow_n = 0;
    }
}

/// slot liefert den Spieler-Slot, wenn er in `0..MAX_PLAYERS` liegt.
fn slot(player: i64) -> Option<usize> {
    usize::try_from(player).ok().filter(|p| *p < MAX_PLAYERS)
}

/// num liest eine Zahl aus einem Ereignis-Feld (0 sonst).
pub fn num(v: Option<&Value>) -> f64 {
    v.and_then(Value::as_f64).unwrap_or(0.0)
}

/// trunc schneidet wie Gos `int(float64)` ab.
#[allow(clippy::cast_possible_truncation)]
fn trunc(v: f64) -> i64 {
    v as i64
}

/// ticks_of schneidet wie Gos `uint64(float64)` ab (negativ: 0).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn ticks_of(v: f64) -> u64 {
    v as u64
}

/// round1 rundet auf eine Nachkommastelle wie Go (`float64(int(v*10+0.5)) / 10`), nicht mit `round`.
#[allow(clippy::cast_precision_loss)]
pub fn round1(v: f64) -> f64 {
    trunc(v * 10.0 + 0.5) as f64 / 10.0
}

/// show schreibt ein Ereignis-Feld wie Gos `%v` (Text ohne Anführungszeichen, ganze Zahlen ohne Nachkomma, fehlend:
/// `<nil>`).
fn show(v: Option<&Value>) -> String {
    match v {
        None | Some(Value::Null) => "<nil>".to_string(),
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => match (n.as_i64(), n.as_f64()) {
            (Some(i), _) => i.to_string(),
            (None, Some(x)) if x.fract() == 0.0 && x.abs() < 1e21 => trunc(x).to_string(),
            (None, Some(x)) => x.to_string(),
            _ => n.to_string(),
        },
        Some(other) => other.to_string(),
    }
}
