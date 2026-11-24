//! Port von `native/k3c-game/props/listen.go`. Zustand je Update (Listener): was die Simulation über Plätze und Burg
//! nur abgeleitet liefert (bezahlbar, roden, sichtbar, Hub-Ausbau, Gefahr), die stehenden Preisschilder und die Truhen,
//! die sich gerade öffnen. Einmal je Frame gerechnet, `Draw` liest nur. Puffer werden wiederverwendet.
//!
//! Go hängt `Listen`, `infoFor` und `info` an den `drawer`; Rust legt dazu `Infos` an (der Drawer aus RU16.5 hält es).
//! Die Go-Zeiger `[]*stageInfo` (`infos`, `spare`) werden Indizes: `spare` hält jede je angelegte Stufe, `infos`
//! nennt die Stufen des laufenden Frames als Indizes in `spare`.

use k3c_sim::state::World;

use super::pending::CombatSpot;
use super::plan_combat::SPOTS_EVERY;
use super::simview::SimView;
use super::sites::{Candidate, nearest_tags, offers_hub_upgrade, offers_price, shelf_of};
use crate::view::frame::Frame;

/// Dauer der offenen Truhe: steht `CHEST_HOLD` Sekunden, blendet bis `CHEST_GONE` aus.
pub const CHEST_HOLD: f64 = 0.3;
pub const CHEST_GONE: f64 = 0.8;

/// `SiteFlags` ist der abgeleitete Zustand eines Platzes (Index wie `World::sites`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SiteFlags {
    pub visible: bool,
    pub payable: bool,
    pub clearing: bool,
    /// Gold bis zum vollen Preis, wenn der Platz Münzen nimmt (`sim.SiteGoldLeft`)
    pub gold_left: i64,
}

/// `HubState` ist der Hub-Ausbau einer Stufe (`sim.EconomyOf`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HubState {
    pub has: bool,
    pub gold: i64,
    pub paid: i64,
    pub steps: i64,
    pub state: String,
    pub material: [i64; 5],
}

/// `StageInfo` ist der abgeleitete Zustand einer Stufe.
#[derive(Debug, Clone, Default)]
pub struct StageInfo {
    pub index: i64,
    pub danger: bool,
    pub hub_level: i64,
    pub hub_step: i64,
    pub hub: HubState,
    pub stock: [i64; 5],
    pub flags: Vec<SiteFlags>,
    /// Nummern der Plätze bzw. der Burg mit stehendem Preisschild
    pub tags: Vec<i64>,
    pub cands: Vec<Candidate>,
    pub players: Vec<f64>,
    /// Truhen des Frames (Nummer, Ort), für das Öffnen
    pub chests: Vec<Candidate>,
    /// Truhen des vorigen Frames
    pub prev: Vec<Candidate>,
    pub openings: Vec<Opening>,
    /// Zahlstellen der Kampfvariante, alle `SPOTS_EVERY` Sekunden neu gelesen
    pub spots: Vec<CombatSpot>,
    /// `Frame::clock` der letzten Lesung
    pub spots_at: f64,
    pub spots_set: bool,
    /// Ort des markierten Flaschenpost-Ziels in dieser Stufe (GS4.7); `None` = keins hier
    pub bottle_goal: Option<f64>,
}

/// `Opening` ist eine aufgehobene Truhe: kurz offen, dann ausgeblendet (worldRenderer.ts › openChest).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Opening {
    pub x: f64,
    /// `Frame::clock` beim Aufheben
    pub start: f64,
}

/// `Infos` hält den Zustand aller Stufen (Go: `drawer.infos` und `drawer.spare`).
#[derive(Debug, Default)]
pub struct Infos {
    /// Stufen des laufenden Frames als Indizes in `spare`, in der Reihenfolge des Frames
    pub infos: Vec<usize>,
    /// alle je angelegten Stufen
    pub spare: Vec<StageInfo>,
    next: Vec<usize>,
}

impl Infos {
    /// `listen` rechnet den Zustand aller Stufen des Frames (Go: `drawer.Listen`, `view.Listener`).
    pub fn listen(&mut self, f: &Frame, view: &dyn SimView) {
        self.next.clear();
        for st in &f.stages {
            let Some(w) = st.world.as_deref() else {
                continue;
            };
            let goal = crate::view::bottle::goal_in(f, st.index);
            let i = self.info_for(st.index);
            let info = &mut self.spare[i];
            info.index = st.index;
            info.update(w, f.clock, view);
            info.bottle_goal = goal;
            self.next.push(i);
        }
        std::mem::swap(&mut self.infos, &mut self.next);
    }

    /// `info_for` liefert (als Index in `spare`) den Zustand der Stufe `i` aus dem letzten Frame oder einen neuen.
    /// Kommt eine Stufe nach einer Lücke wieder (aus `spare`), beginnt der Truhen-Vergleich neu: Truhen, die
    /// inzwischen verschwunden sind, öffnen sich nicht.
    pub fn info_for(&mut self, i: i64) -> usize {
        if let Some(&s) = self.infos.iter().find(|&&s| self.spare[s].index == i) {
            return s;
        }
        if let Some(s) = self.spare.iter().position(|s| s.index == i) {
            self.spare[s].chests.clear();
            self.spare[s].openings.clear();
            return s;
        }
        self.spare.push(StageInfo {
            index: i,
            ..Default::default()
        });
        self.spare.len() - 1
    }

    /// `info` liefert den Zustand der Stufe `i` des laufenden Frames (`None`, wenn `listen` sie nicht kennt).
    pub fn info(&self, i: i64) -> Option<&StageInfo> {
        self.infos.iter().map(|&s| &self.spare[s]).find(|s| s.index == i)
    }
}

impl StageInfo {
    /// `update` liest die Welt `w` der Stufe.
    pub fn update(&mut self, w: &World, clock: f64, view: &dyn SimView) {
        let e = view.economy_of(w);
        self.danger = e.danger;
        self.hub_level = w.hub_level.max(1);
        self.hub_step = w.hub_step.max(1);
        self.hub = HubState::default();
        if let Some(up) = &e.hub_upgrade {
            let m = &up.material;
            self.hub = HubState {
                has: true,
                gold: up.gold,
                paid: up.paid,
                steps: up.steps,
                state: up.state.clone(),
                material: [m.wood, m.stone, m.copper, m.iron, m.crystal],
            };
        }
        let st = &w.stock;
        self.stock = [st.wood, st.stone, st.copper, st.iron, st.crystal];
        self.flags.clear();
        for site in &w.sites {
            let (left, payable) = view.site_gold_left(w, site);
            self.flags.push(SiteFlags {
                visible: view.site_visible(w, site),
                payable,
                clearing: !payable && view.site_needs_clearing(w, site),
                gold_left: left,
            });
        }
        self.price_tags(w);
        if clock < self.spots_at || clock - self.spots_at >= SPOTS_EVERY || !self.spots_set {
            self.spots_set = true;
            self.spots.clear();
            self.spots.extend(view.combat_spots(w));
            self.spots_at = clock;
        }
        self.chests_of(w, clock);
    }

    /// `price_tags`: je lebendem Monarchen das nächste Preisschild (Plätze mit `offers_price`, Burg mit offenem Ausbau).
    pub fn price_tags(&mut self, w: &World) {
        self.cands.clear();
        self.players.clear();
        self.tags.clear();
        for (i, site) in w.sites.iter().enumerate() {
            if self.flags[i].visible && offers_price(&site.kind, &site.state, self.flags[i].payable, shelf_of(site)) {
                self.cands.push(Candidate { id: site.id, x: site.x });
            }
        }
        if offers_hub_upgrade(self.hub.has, &self.hub.state, self.hub.gold, self.hub.paid) {
            self.cands.push(Candidate {
                id: w.castle.id,
                x: w.castle.x,
            });
        }
        for p in &w.players {
            if p.crown.is_none() {
                self.players.push(p.x);
            }
        }
        nearest_tags(&mut self.tags, &self.cands, &self.players);
    }

    /// `chests_of` merkt die Truhen und legt für jede verschwundene eine offene an; abgelaufene fallen weg.
    pub fn chests_of(&mut self, w: &World, clock: f64) {
        std::mem::swap(&mut self.prev, &mut self.chests);
        self.chests.clear();
        for p in &w.pickups {
            if p.kind == "chest" {
                self.chests.push(Candidate { id: p.id, x: p.x });
            }
        }
        let out = std::mem::take(&mut self.openings);
        self.openings = gone(out, &self.prev, &self.chests, clock);
    }

    /// `tagged` meldet, ob das Preisschild von `id` steht.
    pub fn tagged(&self, id: i64) -> bool {
        self.tags.contains(&id)
    }

    /// `flag` liefert den Zustand des Platzes `i` (leer, wenn `listen` die Welt nicht kannte).
    pub fn flag(&self, i: usize) -> SiteFlags {
        self.flags.get(i).copied().unwrap_or(SiteFlags {
            visible: true,
            ..Default::default()
        })
    }
}

/// `info_tagged` ist `StageInfo::tagged` mit fehlendem Zustand (Go: Methode auf `nil`, `false`).
pub fn info_tagged(info: Option<&StageInfo>, id: i64) -> bool {
    info.is_some_and(|s| s.tagged(id))
}

/// `info_flag` ist `StageInfo::flag` mit fehlendem Zustand (Go: Methode auf `nil`, nur sichtbar).
pub fn info_flag(info: Option<&StageInfo>, i: usize) -> SiteFlags {
    info.map_or(
        SiteFlags {
            visible: true,
            ..Default::default()
        },
        |s| s.flag(i),
    )
}

/// `gone` hängt für jede Truhe aus `prev`, die in `now` fehlt, ein Öffnen an und entfernt abgelaufene (rein, getestet).
pub fn gone(mut out: Vec<Opening>, prev: &[Candidate], now: &[Candidate], clock: f64) -> Vec<Opening> {
    out.retain(|o| clock - o.start < CHEST_GONE && clock >= o.start);
    for c in prev {
        if !has_candidate(now, c.id) {
            out.push(Opening { x: c.x, start: clock });
        }
    }
    out
}

pub fn has_candidate(cs: &[Candidate], id: i64) -> bool {
    cs.iter().any(|c| c.id == id)
}

/// `open_alpha` ist die Deckkraft einer offenen Truhe zur Zeit `clock`.
pub fn open_alpha(o: &Opening, clock: f64) -> f32 {
    let t = clock - o.start;
    if t < CHEST_HOLD {
        return 1.0;
    }
    if t >= CHEST_GONE {
        return 0.0;
    }
    (1.0 - (t - CHEST_HOLD) / (CHEST_GONE - CHEST_HOLD)) as f32
}
