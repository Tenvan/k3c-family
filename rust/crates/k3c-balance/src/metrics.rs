//! Kennzahlen je Lauf (Port von `engine/balance/metrics.go`, B-099 › Anforderungen), alle aus Zustand und Ereignissen der
//! Simulation. Ticks zählen ab 1 (Tick n = nach dem n-ten Schritt). `Option` ist `null`, solange es den Zeitpunkt im Lauf
//! nicht gab.

use k3c_data::hub::Stock;
use k3c_sim::island::Island;
use k3c_sim::state::{Event, World};
use serde::{Deserialize, Serialize};

use crate::bots::economy_view;
use crate::metrics_island::{BossMetrics, StageMetrics};
use crate::ship_need::ShipNeed;

/// Die Kennzahlen eines Laufs; feste Feldreihenfolge, damit das JSON byte-gleich bleibt.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Metrics {
    /// Erster Tick, an dem das Gold aller Monarchen zusammen mindestens die Schwelle erreicht.
    pub gold_threshold_tick: Option<i64>,
    /// Erstes Ereignis `built` mit kind `wall`.
    pub first_wall_tick: Option<i64>,
    /// Erstes Ereignis `armed` (ein Bauer holt einen Bogen aus der Werkstatt); Start-Bogenschützen zählen nicht.
    pub first_bow_tick: Option<i64>,
    /// Erstes `castleFallen`.
    pub castle_fall_tick: Option<i64>,
    /// Ursache des verlorenen Laufs: `crown` (Kronenraub, kampf.md § 6).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub lost: String,
    /// Erstes Ereignis `built` mit kind `bowyer`, dem Bogner (WZ28.4, B-951).
    pub first_workshop_tick: Option<i64>,
    /// Erstes Ereignis `built` mit kind `tower`; `hub_level2_tick`/`hub_level3_tick`: erstes `upgraded` mit kind `castle`
    /// auf Stufe 2 bzw. 3 (BAL6, B-347).
    pub first_tower_tick: Option<i64>,
    pub hub_level2_tick: Option<i64>,
    pub hub_level3_tick: Option<i64>,
    /// Tiefste Stufe, auf der am Ende ein Monarch steht.
    pub reached_depth: i64,
    pub days: Vec<DayMetrics>,
    pub waves: Vec<WaveMetrics>,
    pub dawns: Vec<DawnMetrics>,
    // Nur ein Insel-Lauf mit Welt füllt die Felder ab hier (Go: eingebetteter `IslandMetrics`, im JSON flach, BS11).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stages: Vec<StageMetrics>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bosses: Vec<BossMetrics>,
    /// Ereignis `bossSpawned` (Miniboss mit der Welle, Endboss am Bau); zeigt, ob ein Boss überhaupt kam.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub boss_spawns: Vec<BossMetrics>,
    /// Tag des Siegs über den Endboss der Insel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endboss_day: Option<i64>,
    /// Tick dazu.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endboss_tick: Option<i64>,
    /// Ereignis `victory` (nur auf der letzten Insel der Welt).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub victory_tick: Option<i64>,
    /// Je Schiff-Abschnitt der Tag, ab dem er bezahlbar gewesen wäre (ship_need.go, BS3.3); nur Insel-Lauf.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ship: Vec<ShipNeed>,
}

/// Stand zu Tagesbeginn (BAL6, B-347): Tag 1 ist der Start (Tick 0), jeder weitere das Ereignis `dawn` der Stufe des
/// Szenarios, auch der Morgen nach dem letzten Tag. Gold je Monarch in der Reihenfolge der Spieler, Vorrat der Insel,
/// Hub-Stufe und Kämpfer (Truppen außer Landstreichern und Bauern) der Stufe.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DawnMetrics {
    pub day: i64,
    pub tick: i64,
    pub gold: Vec<i64>,
    pub stock: Stock,
    pub hub_level: i64,
    pub fighters: i64,
}

/// Stand zu Dämmerungsbeginn (Ereignis `dusk`) und Gold-Fluss eines Tags. Der Fluss ist die Änderung des Golds aller
/// Monarchen je Tick: Zunahme = Einnahmen, Abnahme = Ausgaben (auch gestohlenes und fallen gelassenes Gold). Ein Tag reicht
/// vom Morgen bis zum nächsten Morgen, Nacht n gehört also zu Tag n (WZ28.4):
///   - `night_income`/`night_expense`: der Teil von `income`/`expense` in der Phase `night` (Tag-Anteil = Differenz)
///   - `stock_in`/`stock_out`: Zu- und Abgang des Insel-Vorrats je Rohstoff (Änderung je Tick)
///   - `castle_fallen`: In diesem Tag (samt Nacht) fiel die Burg
///   - `peasant_ticks`/`peasant_idle_ticks`: Bauern-Ticks und davon ohne Auftrag und ohne Arbeitsplatz (Wartezeit)
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DayMetrics {
    pub day: i64,
    pub dusk_tick: Option<i64>,
    pub gold_at_dusk: Option<i64>,
    pub stock_at_dusk: Option<Stock>,
    pub income: i64,
    pub expense: i64,
    pub night_income: i64,
    pub night_expense: i64,
    pub stock_in: Stock,
    pub stock_out: Stock,
    pub castle_fallen: bool,
    pub peasant_ticks: i64,
    pub peasant_idle_ticks: i64,
}

/// Eine Welle vom Ereignis `wave` bis zum nächsten `dawn` (oder zur nächsten Welle). Verluste sind Truppen (ohne
/// Landstreicher), die zu Wellenbeginn lebten und am Ende fehlen; überlebt = kein `castleFallen` in der Welle.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct WaveMetrics {
    pub wave: i64,
    pub start_tick: i64,
    /// `null`: Lauf endete in der Welle.
    pub end_tick: Option<i64>,
    pub enemies: i64,
    pub troops_at_start: i64,
    pub troops_lost: i64,
    pub buildings_destroyed: i64,
    pub players_down: i64,
    pub survived: bool,
}

/// Sammelt die Kennzahlen Tick für Tick (Go: `collector`). Die Insel hält der Aufrufer (Go hält sie im Sammler).
pub struct Collector {
    threshold: i64,
    /// Gold aller Monarchen nach dem letzten Tick.
    gold: i64,
    /// Insel-Vorrat nach dem letzten Tick.
    stock: Stock,
    pub(crate) m: Metrics,
    wave: Option<WaveMetrics>,
    /// Truppen-IDs zu Beginn der laufenden Welle (sortiert).
    wave_ids: Vec<i64>,
}

impl Collector {
    /// Go: `newCollector(isl, threshold)`.
    pub fn new(isl: &Island, threshold: i64) -> Self {
        let mut c = Collector {
            threshold,
            gold: total_gold(isl),
            stock: isl.stock.clone(),
            m: Metrics::default(),
            wave: None,
            wave_ids: Vec::new(),
        };
        c.dawn(isl, &isl.stages[0], 0);
        c.init_stages(isl);
        c
    }

    /// Go: `observe(tick)`: liest den Zustand nach Tick `tick`.
    pub fn observe(&mut self, isl: &Island, tick: i64) {
        let night = isl.stages[0].cycle.phase == "night";
        let gold = total_gold(isl);
        let d = gold - self.gold;
        let n = night as i64;
        let day = self.day(isl.stages[0].cycle.day);
        if d > 0 {
            day.income += d;
            day.night_income += d * n;
        } else {
            day.expense -= d;
            day.night_expense -= d * n;
        }
        self.gold = gold;
        self.observe_stock(isl);
        self.observe_peasants(isl);
        if self.threshold > 0 && gold >= self.threshold && self.m.gold_threshold_tick.is_none() {
            self.m.gold_threshold_tick = Some(tick);
        }
        for w in &isl.stages {
            for ev in &w.events {
                self.event(isl, w, ev, tick);
                self.island_event(isl, w, ev, tick);
            }
        }
        self.observe_island(isl, tick);
    }

    fn event(&mut self, isl: &Island, w: &World, ev: &Event, tick: i64) {
        let kind = ev.get("kind").and_then(|v| v.as_str()).unwrap_or("");
        match ev.get("type").and_then(|v| v.as_str()).unwrap_or("") {
            "built" => match kind {
                "wall" => first(&mut self.m.first_wall_tick, tick),
                "bowyer" => first(&mut self.m.first_workshop_tick, tick),
                "tower" => first(&mut self.m.first_tower_tick, tick),
                _ => {}
            },
            "upgraded" => self.upgraded(ev, tick),
            "armed" => first(&mut self.m.first_bow_tick, tick),
            "castleFallen" => {
                first(&mut self.m.castle_fall_tick, tick);
                self.day(w.cycle.day).castle_fallen = true;
                if let Some(wave) = self.wave.as_mut() {
                    wave.survived = false;
                }
            }
            "dusk" => {
                let gold = self.gold;
                let d = self.day(w.cycle.day);
                (d.dusk_tick, d.gold_at_dusk, d.stock_at_dusk) = (Some(tick), Some(gold), Some(isl.stock.clone()));
            }
            "wave" => {
                self.close_wave(w, tick);
                self.open_wave(w, ev, tick);
            }
            "dawn" => {
                self.close_wave(w, tick);
                if std::ptr::eq(w, &isl.stages[0]) {
                    self.dawn(isl, w, tick);
                }
            }
            "destroyed" => self.count_in_wave(|wm| wm.buildings_destroyed += 1),
            "crownFallen" => self.count_in_wave(|wm| wm.players_down += 1),
            _ => {}
        }
    }

    /// Merkt den ersten Ausbau des Hubs (Burg) auf Stufe 2 und 3.
    fn upgraded(&mut self, ev: &Event, tick: i64) {
        if ev.get("kind").and_then(|v| v.as_str()) != Some("castle") {
            return;
        }
        match ev.get("level").and_then(|v| v.as_i64()) {
            Some(2) => first(&mut self.m.hub_level2_tick, tick),
            Some(3) => first(&mut self.m.hub_level3_tick, tick),
            _ => {}
        }
    }

    /// Legt den Stand zu Tagesbeginn ab (Tag aus dem Zyklus der Stufe).
    fn dawn(&mut self, isl: &Island, w: &World, tick: i64) {
        let e = economy_view(w);
        self.m.dawns.push(DawnMetrics {
            day: w.cycle.day,
            tick,
            gold: isl.players().iter().map(|p| p.gold).collect(),
            stock: isl.stock.clone(),
            hub_level: e.hub_level,
            fighters: e.fighters,
        });
    }

    fn open_wave(&mut self, w: &World, ev: &Event, tick: i64) {
        let int = |k: &str| ev.get(k).and_then(|v| v.as_i64()).unwrap_or(0);
        self.wave_ids = fighter_ids(w);
        self.wave = Some(WaveMetrics {
            wave: int("wave"),
            start_tick: tick,
            enemies: int("count"),
            troops_at_start: self.wave_ids.len() as i64,
            survived: true,
            ..WaveMetrics::default()
        });
    }

    fn close_wave(&mut self, w: &World, tick: i64) {
        if let Some(wave) = self.wave.as_mut() {
            wave.end_tick = Some(tick);
            self.finish_wave(w);
        }
    }

    /// Zählt die Verluste und legt die Welle ab.
    fn finish_wave(&mut self, w: &World) {
        let Some(mut wave) = self.wave.take() else { return };
        let alive = fighter_ids(w);
        wave.troops_lost = self
            .wave_ids
            .iter()
            .filter(|id| alive.binary_search(id).is_err())
            .count() as i64;
        self.m.waves.push(wave);
        self.wave_ids.clear();
    }

    fn count_in_wave(&mut self, f: impl FnOnce(&mut WaveMetrics)) {
        if let Some(wave) = self.wave.as_mut() {
            f(wave);
        }
    }

    /// Liefert die Zeile des Tags; fehlende Tage davor werden angelegt.
    pub(crate) fn day(&mut self, n: i64) -> &mut DayMetrics {
        let n = n.max(1) as usize;
        while self.m.days.len() < n {
            let day = self.m.days.len() as i64 + 1;
            self.m.days.push(DayMetrics {
                day,
                ..DayMetrics::default()
            });
        }
        &mut self.m.days[n - 1]
    }

    /// Go: `result()`: schließt eine offene Welle (ohne `end_tick`) und setzt die erreichte Tiefe.
    pub fn result(&mut self, isl: &Island) -> Metrics {
        if self.wave.is_some() {
            self.finish_wave(&isl.stages[0]);
        }
        for w in isl.stages.iter().filter(|w| !w.players.is_empty()) {
            self.m.reached_depth = self.m.reached_depth.max(w.biome.depth);
        }
        self.ship_need(isl);
        self.m.clone()
    }

    /// Verbucht die Änderung des Insel-Vorrats je Rohstoff als Zu- oder Abgang des Tags.
    fn observe_stock(&mut self, isl: &Island) {
        let now = isl.stock.clone();
        let before = std::mem::replace(&mut self.stock, now.clone());
        let day = self.day(isl.stages[0].cycle.day);
        flow(&mut day.stock_in.wood, &mut day.stock_out.wood, before.wood, now.wood);
        flow(
            &mut day.stock_in.stone,
            &mut day.stock_out.stone,
            before.stone,
            now.stone,
        );
        flow(
            &mut day.stock_in.copper,
            &mut day.stock_out.copper,
            before.copper,
            now.copper,
        );
        flow(&mut day.stock_in.iron, &mut day.stock_out.iron, before.iron, now.iron);
        flow(
            &mut day.stock_in.crystal,
            &mut day.stock_out.crystal,
            before.crystal,
            now.crystal,
        );
    }

    /// Zählt Bauern-Ticks; wartend ist ein Bauer ohne Auftrag und ohne Arbeitsplatz (Handwerker an der Werkstatt arbeiten
    /// ohne Auftrag).
    fn observe_peasants(&mut self, isl: &Island) {
        let (mut ticks, mut idle) = (0, 0);
        for t in isl.stages.iter().flat_map(|w| &w.troops).filter(|t| t.serving()) {
            ticks += 1;
            idle += (t.job.is_none() && t.work_site == 0) as i64;
        }
        let day = self.day(isl.stages[0].cycle.day);
        day.peasant_ticks += ticks;
        day.peasant_idle_ticks += idle;
    }
}

fn total_gold(isl: &Island) -> i64 {
    isl.players().iter().map(|p| p.gold).sum()
}

fn flow(inn: &mut i64, out: &mut i64, before: i64, after: i64) {
    let d = after - before;
    if d > 0 {
        *inn += d;
    } else {
        *out -= d;
    }
}

/// Sortierte IDs aller Truppen außer Landstreichern.
fn fighter_ids(w: &World) -> Vec<i64> {
    let mut ids: Vec<i64> = w.troops.iter().filter(|t| !t.is_wild()).map(|t| t.id).collect();
    ids.sort_unstable();
    ids
}

pub(crate) fn first(p: &mut Option<i64>, tick: i64) {
    p.get_or_insert(tick);
}
