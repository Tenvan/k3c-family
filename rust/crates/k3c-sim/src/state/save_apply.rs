//! Spielstand anwenden (Port von `engine/sim/save.go`, `applyHub` und Hilfen): legt einen gespeicherten Hub auf eine frisch
//! erzeugte Stufe.

use std::collections::BTreeSet;

use super::links::{apply_guards, apply_portals, hub_steps, values};
use super::node_mark::apply_marked_by;
use super::save::{HubSave, SiteSave, key};
use super::save_economy::{apply_craft, apply_plots, apply_troops};
use super::{Site, World};

/// Go: `applyHub`: legt einen gespeicherten Hub auf eine frisch erzeugte Stufe.
pub fn apply_hub(w: &mut World, h: &HubSave) {
    w.castle.hp = h.castle_hp;
    w.stock = h.stock.clone();
    w.wave = h.wave;
    if w.aggression.is_some() && h.aggression.is_some() {
        w.aggression = h.aggression;
    }
    w.armor_level = h.armor_level;
    for saved in &h.sites {
        apply_site(w, saved);
    }
    apply_hub_level(w, h);
    // Start-Truppen durch die gespeicherten ersetzen
    w.troops.retain(|t| t.kind == "vagrant");
    apply_troops(w, h);
    let gone = set(&h.nodes_gone);
    let marked = set(&h.nodes_marked);
    let taken = set(&h.pickups_taken);
    w.nodes.retain_mut(|n| {
        let k = key(&n.kind, n.x);
        if gone.contains(&k) {
            return false;
        }
        n.marked = n.marked || marked.contains(&k);
        apply_marked_by(n, &h.nodes_marked_by);
        true
    });
    w.pickups.retain(|p| !taken.contains(&key(&p.kind, p.x)));
    apply_guards(w, h);
    apply_portals(w, h);
    apply_plots(w, h); // nach den Knoten: die Bäume der Plantage kommen dazu
}

/// Go: `applySite`: legt einen gespeicherten Bauplatz auf den Platz gleicher Art und Lage; gibt es ihn in dieser Version
/// nicht mehr, passiert nichts.
pub fn apply_site(w: &mut World, saved: &SiteSave) {
    let saved_key = key(&saved.kind, saved.x);
    let Some(si) = w.sites.iter().position(|s| key(&s.kind, s.x) == saved_key) else {
        return;
    };
    let state = match saved.state.as_str() {
        st @ ("waitingWorker" | "waitingMaterial" | "built" | crate::hermit::BUILDING_STATE) => st.to_string(),
        _ => "unpaid".to_string(),
    };
    let v = values(w);
    let s = &mut w.sites[si];
    s.state = state.clone();
    s.paid_gold = saved.paid_gold;
    s.build_progress = saved.build_progress;
    s.hp = saved.hp;
    s.bows = saved.bows;
    s.bow_paid_gold = saved.bow_paid_gold;
    s.worker_id = None;
    let levels = v
        .buildings
        .get(&s.kind)
        .map(|b| b.levels.as_slice())
        .unwrap_or_default();
    if state == "built" && saved.level > 1 && saved.level <= levels.len() as i64 {
        s.level = saved.level;
        s.max_hp = levels[(saved.level - 1) as usize].hp;
    }
    if state == "built" {
        apply_upgrade(s, saved);
        apply_craft(&v, s, saved.craft.as_ref());
    }
}

/// Go: `applyHubLevel`: übernimmt Hub-Stufe, Ausbauschritt (1 bis Schrittzahl der Stufe) und laufenden Hub-Ausbau.
pub fn apply_hub_level(w: &mut World, h: &HubSave) {
    w.hub_level = h.hub_level.max(1);
    w.hub_step = h.hub_step.max(1).min(hub_steps(w).max(1));
    if let (Some(u), Some(hub_site)) = (h.hub_upgrade.as_ref(), w.hub_site.as_mut()) {
        apply_upgrade(hub_site, u);
    }
}

/// Go: `applyUpgrade`: übernimmt einen laufenden Ausbau eines gebauten Platzes.
pub fn apply_upgrade(s: &mut Site, saved: &SiteSave) {
    if saved.upgrade == "waitingMaterial" || saved.upgrade == "waitingWorker" {
        s.upgrade = saved.upgrade.clone();
        s.build_progress = saved.build_progress;
    }
    s.upgrade_paid = saved.upgrade_paid;
}

/// Go: `set`: die Schlüssel als Menge.
pub fn set(items: &[String]) -> BTreeSet<String> {
    items.iter().cloned().collect()
}
