//! Spielstand Version 7 (Port von `engine/sim/save_economy.go`): die Wirtschaft. Je Hub die Rüstungsstufe (`armorLevel`),
//! je Truppe Kämpfer-Art (Krieger, Elite) und Beruf mit Arbeitsplatz (`kind@x`), je Platz die Herstellung (`craft`: Elite,
//! Rüstung, Schwerter, Angebote) und die Plantage. Fehlt ein Feld, gilt der Startzustand. Aufträge der Bauern, Ausrüstung am
//! Boden (`World::drops`) und von Gegnern getragene (`Enemy::carried`) sind flüchtig wie Münzen am Boden und gehen beim
//! Laden verloren.

use serde::{Deserialize, Serialize};

use super::fehler::Fehler;
use super::links::{is_fighter, is_soldier, make_fighter, plot_x, profession_of, spawn_vagrant, values};
use super::omit::{is_zero_f64, is_zero_i64};
use super::save::{HubSave, TroopSave, key};
use super::{Plot, ResourceNode, Site, World, WorldValues};

/// Go: `CraftSave`, der Herstellungs- und Plantage-Zustand eines gebauten Platzes (Zeiten sind Spielzeit wie `World::time`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CraftSave {
    /// `Site::craft_done`: Elite-Bogenschütze bzw. Rüstungsstufe fertig um.
    #[serde(rename = "done", skip_serializing_if = "is_zero_f64")]
    pub done: f64,
    /// `Site::offer_paid`.
    #[serde(rename = "offerPaid", skip_serializing_if = "Vec::is_empty")]
    pub offer_paid: Vec<i64>,
    /// `Site::offer_done`: Elite-Krieger.
    #[serde(rename = "offerDone", skip_serializing_if = "is_zero_f64")]
    pub offer_done: f64,
    /// Hämmer im Regal der Werkstatt.
    #[serde(rename = "hammers", skip_serializing_if = "is_zero_i64")]
    pub hammers: i64,
    /// Schwerter im Regal.
    #[serde(rename = "swords", skip_serializing_if = "is_zero_i64")]
    pub swords: i64,
    /// Bezahltes Gold für das nächste Schwert.
    #[serde(rename = "swordPaidGold", skip_serializing_if = "is_zero_i64")]
    pub sword_paid_gold: i64,
    #[serde(rename = "swordDone", skip_serializing_if = "is_zero_f64")]
    pub sword_done: f64,
    /// Plantage einer Baumschule, je Platz.
    #[serde(rename = "plots", skip_serializing_if = "Vec::is_empty")]
    pub plots: Vec<PlotSave>,
}

/// Go: `PlotSave`, ein Platz der Plantage: ein gewachsener Baum oder die Zeit bis zum nächsten.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PlotSave {
    #[serde(rename = "tree", skip_serializing_if = "super::omit::is_false")]
    pub tree: bool,
    #[serde(rename = "growIn", skip_serializing_if = "is_zero_f64")]
    pub grow_in: f64,
}

/// Go: `craftSave`: schreibt den Zustand von `s`; `None`, wenn nichts davon gesetzt ist. Der Baum eines Platzes ist seine
/// Knoten-ID; er zählt, solange der Knoten in `w.nodes` steht und nicht `gone` ist.
pub fn craft_save(w: &World, s: &Site) -> Option<CraftSave> {
    let mut c = CraftSave {
        done: s.craft_done,
        offer_paid: s.offer_paid.clone(),
        offer_done: s.offer_done,
        swords: s.swords,
        sword_paid_gold: s.sword_paid_gold,
        sword_done: s.sword_done,
        hammers: s.hammers,
        ..Default::default()
    };
    if s.state == "built" {
        for p in &s.plots {
            let tree = p.tree.is_some_and(|id| w.node(id).is_some_and(|n| !n.gone));
            c.plots.push(PlotSave {
                tree,
                grow_in: p.grow_in,
            });
        }
    }
    let empty = c.done == 0.0
        && !c.offer_paid.iter().any(|n| *n != 0)
        && c.offer_done == 0.0
        && c.swords == 0
        && c.sword_paid_gold == 0
        && c.sword_done == 0.0
        && c.hammers == 0
        && c.plots.is_empty();
    if empty { None } else { Some(c) }
}

/// Go: `applyCraft`: legt die Herstellung auf den gebauten Platz `s` (die Plantage in `apply_plots`). `v` ist der
/// Wertesatz der Welt (Go liest ihn über `w.values()`).
pub fn apply_craft(v: &WorldValues, s: &mut Site, c: Option<&CraftSave>) {
    let Some(c) = c else {
        return;
    };
    s.craft_done = c.done;
    s.offer_done = c.offer_done;
    s.swords = c.swords;
    s.sword_paid_gold = c.sword_paid_gold;
    s.sword_done = c.sword_done;
    s.hammers = c.hammers;
    let offers = v.buildings.get(&s.kind).map_or(0, |b| b.offers.len());
    if c.offer_paid.len() <= offers {
        s.offer_paid = c.offer_paid.clone();
    }
}

/// Go: `siteByKey`: der Bauplatz `kind@x` (`key`) der Welt, als Index in `w.sites`.
pub fn site_by_key(w: &World, k: &str) -> Option<usize> {
    w.sites.iter().position(|s| key(&s.kind, s.x) == k)
}

/// Go: `applyPlots`: stellt die Plantage gebauter Baumschulen wieder her; eine zerstörte Baumschule (nicht built) bekommt
/// keine Bäume.
pub fn apply_plots(w: &mut World, h: &HubSave) {
    for saved in &h.sites {
        let Some(si) = site_by_key(w, &key(&saved.kind, saved.x)) else {
            continue;
        };
        let v = values(w);
        let Some(p) = v.buildings.get(&w.sites[si].kind).map(|b| &b.plantation) else {
            continue;
        };
        let Some(craft) = saved.craft.as_ref() else {
            continue;
        };
        if w.sites[si].state != "built" || craft.plots.len() as i64 != p.slots {
            continue;
        }
        w.sites[si].plots = vec![Plot::default(); p.slots as usize];
        for (i, ps) in craft.plots.iter().enumerate() {
            w.sites[si].plots[i].grow_in = ps.grow_in;
            if ps.tree {
                let id = w.new_id();
                let x = plot_x(&w.sites[si], p, i);
                w.nodes.push(ResourceNode {
                    id,
                    kind: "tree".into(),
                    x,
                    marked: true,
                    plant: true,
                    ..Default::default()
                });
                w.sites[si].plots[i].tree = Some(id);
            }
        }
    }
}

/// Go: `troopsSave`: schreibt alle Truppen außer den Landstreichern (die kommen von allein aus den Camps).
pub fn troops_save(w: &World) -> Vec<TroopSave> {
    let mut out = Vec::new();
    for t in &w.troops {
        if t.is_wild() {
            continue;
        }
        let mut ts = TroopSave {
            kind: t.data_kind().to_string(), // Bürger im Dienst als `peasant`: Go-Format bleibt (FG1.1)
            x: t.x,
            anchor_x: t.anchor_x,
            profession: t.profession.clone(),
            leader: t.leader,
            hammer: t.hammer,
            ..Default::default()
        };
        if let Some(s) = w.site(t.work_site).filter(|_| t.work_site != 0) {
            ts.work_site = key(&s.kind, s.x);
        }
        out.push(ts);
    }
    out
}

/// Go: `applyTroops`: ersetzt die Truppen des Hubs durch die gespeicherten (Rüstung und Plätze müssen schon gesetzt sein);
/// die Art ist geprüft (`validate_economy`).
pub fn apply_troops(w: &mut World, h: &HubSave) {
    for t in &h.troops {
        let hub_x = w.hub_x;
        let ti = spawn_vagrant(w, hub_x, t.x);
        if t.serving() {
            let hp = values(w).troops.get("peasant").map_or(0.0, |d| d.hp);
            let work_site = site_by_key(w, &t.work_site).map(|si| w.sites[si].id);
            let troop = &mut w.troops[ti];
            troop.serve();
            troop.hp = hp;
            troop.max_hp = hp;
            troop.anchor_x = hub_x;
            troop.profession = t.profession.clone();
            troop.hammer = t.hammer;
            if let Some(id) = work_site {
                troop.work_site = id;
            }
            continue;
        }
        make_fighter(w, ti, &t.kind);
        w.troops[ti].anchor_x = t.anchor_x;
        if t.leader && is_soldier(&w.troops[ti]) {
            let troop = &mut w.troops[ti];
            troop.leader = true;
            troop.group = troop.id;
        }
    }
    w.combat.shields = h.shields;
}

/// Go: `validateEconomy`: prüft einen Hub: bekannte Truppenart (nie stillschweigend Landstreicher), bekannter Beruf,
/// Rüstungsstufe im Bereich der Daten, geprüft gegen den Wertesatz `v` der Welt.
pub fn validate_economy(v: &WorldValues, h: &HubSave) -> Result<(), Fehler> {
    let err = |s: String| Err(Fehler::Spielstand(s));
    for t in &h.troops {
        if !t.serving() && !is_fighter(&v.troops, &t.kind) {
            return err(format!("spielstand: Truppenart {:?} unbekannt", t.kind));
        }
        let known = profession_of(&v.troops, &t.profession).is_some();
        if !t.profession.is_empty() && (!known || !t.serving()) {
            return err(format!(
                "spielstand: Beruf {:?} unbekannt oder nicht für {}",
                t.profession, t.kind
            ));
        }
    }
    let armor = v.buildings.get("armory").map_or(0, |b| b.armor.len() as i64);
    if h.armor_level < 0 || h.armor_level > armor {
        return err(format!(
            "spielstand: Rüstungsstufe {} außerhalb 0..{armor}",
            h.armor_level
        ));
    }
    Ok(())
}
