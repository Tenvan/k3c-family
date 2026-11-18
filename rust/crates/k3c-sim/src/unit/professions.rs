//! Berufe der Bauern (Port von `engine/sim/unit_professions.go`, docs/rules/buerger.md § 1 und 2, B-121, Q34, Q35, Q52):
//! Bergmann, Baumeister und Handwerker werden an Angebots-Zahlzielen ausgebildet, Anhängen mit festem dx am gebauten
//! Gebäude (data/buildings.json › offers). Das Gold aus troops.json › peasant.professions zahlt man per A halten
//! (Teilzahlung wie der Bogen); voll bezahlt bekommt der nächste freie Bauer zum Gebäude den Beruf, eine Umschulung
//! kostet erneut. `work_factor` wirkt auf den Abbau (Bergmann, nur seine resources), auf Bauen und Reparatur
//! (Baumeister) und je Handwerker auf die Herstellung in seinem Gebäude (höchstens `max_per_building` je Gebäude).

use std::collections::BTreeMap;

use serde_json::json;

use crate::economy::{can_afford, clear_pending, nearest, spend};
use crate::state::common::event;
use crate::state::island_ref::{IslandRef, cheap};
use crate::state::links::{emit, unit_x};
use crate::state::{World, WorldValues};
use k3c_data::buildings::Offer;
use k3c_data::troops::{ProfessionData, TroopData};

/// Go: `offerTarget` ist das Angebot Nummer `idx` (`offers` = Offers des Gebäudes im Wertesatz) am Platz `site`
/// (Index in `w.sites`).
#[derive(Debug, Clone, Copy)]
pub struct OfferTarget<'a> {
    pub site: usize,
    pub idx: usize,
    pub offers: &'a [Offer],
}

impl OfferTarget<'_> {
    /// Go: `kind`.
    pub fn kind(&self) -> &str {
        &self.offers[self.idx].kind
    }

    /// Go: `x`.
    pub fn x(&self, w: &World) -> f64 {
        w.sites[self.site].x + self.offers[self.idx].dx
    }

    /// Go: `paid`: wie viel auf das Angebot bezahlt ist.
    pub fn paid(&self, w: &World) -> i64 {
        w.sites[self.site].offer_paid.get(self.idx).copied().unwrap_or(0)
    }

    /// Go: `key`: Schlüssel der Zahlung ("offer:<Platz-ID>.<Nummer>").
    pub fn key(&self, w: &World) -> String {
        format!("offer:{}.{}", w.sites[self.site].id, self.idx)
    }
}

/// Go: `professionOf`: der Beruf `kind` des Bauern laut `troops` (Wertesatz der Welt); `None`, wenn es ihn nicht gibt.
pub fn profession_of<'a>(troops: &'a BTreeMap<String, TroopData>, kind: &str) -> Option<&'a ProfessionData> {
    troops.get("peasant")?.professions.as_ref()?.get(kind)
}

/// Die Angebote des Gebäudes `kind` (leer ohne Eintrag).
fn offers_of<'a>(v: &'a WorldValues, kind: &str) -> &'a [Offer] {
    v.buildings.get(kind).map_or(&[], |b| b.offers.as_slice())
}

/// Go: `findOffer` ist das nächste bezahlbare Berufs-Angebot in Zahl-Reichweite von `x`; `None`, wenn es keins gibt.
pub fn find_offer<'a>(w: &World, v: &'a WorldValues, isl: Option<&IslandRef>, x: f64) -> Option<OfferTarget<'a>> {
    let mut best: Option<(OfferTarget<'a>, f64)> = None;
    for (si, s) in w.sites.iter().enumerate() {
        let offers = offers_of(v, &s.kind);
        for idx in 0..offers.len() {
            let o = OfferTarget { site: si, idx, offers };
            let d = (o.x(w) - x).abs();
            if d <= v.economy.pay_range_units && offer_payable(w, v, isl, &o) && best.is_none_or(|(_, bd)| d < bd) {
                best = Some((o, d));
            }
        }
    }
    best.map(|(o, _)| o)
}

/// Go: `nearOffer`: Liegt `x` in Zahl-Reichweite eines Berufs-Angebots an einem gebauten Gebäude (auch voll oder ohne
/// Platz)?
pub fn near_offer(w: &World, v: &WorldValues, x: f64) -> bool {
    w.sites.iter().any(|s| {
        s.state == "built"
            && offers_of(v, &s.kind).iter().any(|o| {
                profession_of(&v.troops, &o.kind).is_some() && (s.x + o.dx - x).abs() <= v.economy.pay_range_units
            })
    })
}

/// Go: `offerPayable`: Das Gebäude steht, das Angebot ist ein Beruf, noch nicht voll bezahlt und (Handwerker) hat Platz.
pub fn offer_payable(w: &World, v: &WorldValues, isl: Option<&IslandRef>, o: &OfferTarget) -> bool {
    let Some(prof) = profession_of(&v.troops, o.kind()) else {
        return false;
    };
    let s = &w.sites[o.site];
    if s.state != "built" || o.paid(w) >= cheap(isl, &prof.cost).gold {
        return false;
    }
    prof.max_per_building == 0 || workers_at(w, s.id, o.kind()) < prof.max_per_building
}

/// Go: `payOffer`: Spieler `pi` zahlt eine Münze auf das Angebot.
pub fn pay_offer(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, pi: usize, o: &OfferTarget) {
    let (index, px) = (w.players[pi].index, w.players[pi].x);
    emit(
        w,
        "coinGive",
        event(json!({"player": index, "x": unit_x(px), "to": "offer"})),
    );
    let n = o.offers.len();
    let s = &mut w.sites[o.site];
    if s.offer_paid.len() < n {
        s.offer_paid.resize(n, 0);
    }
    s.offer_paid[o.idx] += 1;
    if !offer_payable(w, v, isl, o) {
        clear_pending(&mut w.players[pi]);
    }
}

/// Go: `refundOffer` zieht bis zu `amount` Münzen aus dem Angebot `ref_` ("<Platz-ID>.<Nummer>") zurück und liefert die
/// Anzahl.
pub fn refund_offer(w: &mut World, ref_: &str, amount: i64) -> i64 {
    let (id_text, idx_text) = ref_.split_once('.').unwrap_or((ref_, ""));
    let id = id_text.parse::<i64>().unwrap_or(0); // Go: `Atoi` ohne Fehlerprüfung
    let idx = idx_text.parse::<i64>().unwrap_or(0);
    let Some(s) = w.site_mut(id) else {
        return 0;
    };
    let Some(paid) = usize::try_from(idx).ok().and_then(|i| s.offer_paid.get_mut(i)) else {
        return 0;
    };
    let back = amount.min(*paid);
    *paid -= back;
    back
}

/// Go: `stepOffers` bildet für jedes voll bezahlte Angebot den nächsten freien Bauern aus; ohne freien Bauern wartet das
/// Gold.
pub fn step_offers(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>) {
    for si in 0..w.sites.len() {
        let offers = offers_of(v, &w.sites[si].kind);
        for i in 0..w.sites[si].offer_paid.len() {
            let Some(offer) = offers.get(i) else {
                continue;
            };
            let Some(prof) = profession_of(&v.troops, &offer.kind) else {
                continue;
            };
            let (sid, sx) = (w.sites[si].id, w.sites[si].x);
            if w.sites[si].state != "built" || w.sites[si].offer_paid[i] < cheap(isl, &prof.cost).gold {
                continue;
            }
            let Some(ti) = free_peasant(w, sx, &offer.kind) else {
                continue;
            };
            w.sites[si].offer_paid[i] = 0;
            let t = &mut w.troops[ti];
            t.profession.clone_from(&offer.kind);
            t.work_site = if prof.max_per_building > 0 { sid } else { 0 };
            let (kind, tx) = (t.profession.clone(), t.x);
            emit(w, "trained", event(json!({"kind": kind, "x": unit_x(tx)})));
        }
    }
}

/// Go: `freePeasant` ist der Bauer ohne Auftrag nächst an `x`, der den Beruf `kind` noch nicht hat; `None`, wenn keiner
/// frei ist.
pub fn free_peasant(w: &World, x: f64, kind: &str) -> Option<usize> {
    nearest(
        &w.troops,
        |t| t.x,
        x,
        f64::INFINITY,
        |t| t.kind == "peasant" && !t.hammer && t.job.is_none() && t.profession != kind,
    )
}

/// Go: `workersAt` zählt die Bauern mit Beruf `kind`, die an den Platz `site_id` gebunden sind.
pub fn workers_at(w: &World, site_id: i64, kind: &str) -> i64 {
    w.troops
        .iter()
        .filter(|t| t.kind == "peasant" && t.profession == kind && t.work_site == site_id)
        .count() as i64
}

/// Go: `workFactor` ist das Arbeitstempo von `w.troops[ti]` für Arbeit, die der Beruf `kind` beschleunigt (1 ohne
/// diesen Beruf).
pub fn work_factor(w: &World, v: &WorldValues, ti: usize, kind: &str) -> f64 {
    let t = &w.troops[ti];
    match profession_of(&v.troops, kind) {
        Some(prof) if t.kind == "peasant" && t.profession == kind => prof.work_factor,
        _ => 1.0,
    }
}

/// Go: `mineFactor`: Abbautempo von `w.troops[ti]` an einer Ressource, die `resource` liefert (Bergmann nur für seine
/// resources).
pub fn mine_factor(w: &World, v: &WorldValues, ti: usize, resource: &str) -> f64 {
    if profession_of(&v.troops, "miner").is_some_and(|p| p.resources.iter().any(|r| r == resource)) {
        work_factor(w, v, ti, "miner")
    } else {
        1.0
    }
}

/// Go: `craftSeconds` ist die Herstellungszeit von `product` im Gebäude `site` (Index in `w.sites`): Wert aus den Daten,
/// je Handwerker schneller.
pub fn craft_seconds(w: &World, v: &WorldValues, site: usize, product: &str) -> f64 {
    let s = &w.sites[site];
    let (factor, max) = profession_of(&v.troops, "craftsman").map_or((0.0, 0), |p| (p.work_factor, p.max_per_building));
    let n = workers_at(w, s.id, "craftsman").min(max);
    let base = v
        .buildings
        .get(&s.kind)
        .and_then(|b| b.craft_seconds.as_ref())
        .and_then(|m| m.get(product))
        .copied()
        .unwrap_or(0.0);
    base / (1.0 + (factor - 1.0) * n as f64)
}

/// Go: `bowsCrafting` ist 1, solange in der Werkstatt ein Bogen entsteht.
pub fn bows_crafting(s: &crate::state::Site) -> i64 {
    i64::from(s.craft_done > 0.0)
}

/// Go: `stepBowCraft`: Ein voll bezahlter Bogen geht in Arbeit, sobald das Material da ist und gerade keiner entsteht;
/// `craft_done` ist die Spielzeit, zu der er im Regal liegt (Q35). Das Tempo gilt ab Beginn, ein später ausgebildeter
/// Handwerker beschleunigt erst den nächsten Bogen.
pub fn step_bow_craft(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, si: usize) {
    let p = super::hammer::place_product(v, &w.sites[si]);
    if w.sites[si].craft_done > 0.0 && w.time >= w.sites[si].craft_done {
        p.product.add(&mut w.sites[si], 1);
        w.sites[si].craft_done = 0.0;
    }
    let cost = cheap(isl, &p.cost);
    if w.sites[si].craft_done == 0.0 && w.sites[si].bow_paid_gold >= cost.gold && can_afford(&w.stock, &cost) {
        spend(&mut w.stock, &cost);
        w.sites[si].bow_paid_gold = 0;
        w.sites[si].craft_done = w.time + craft_seconds(w, v, si, p.name);
    }
}
