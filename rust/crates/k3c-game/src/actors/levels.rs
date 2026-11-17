//! Port von `native/k3c-game/actors/levels.go`.
//!
//! Sheets je Level (view.AssetSource): nur die requisite-Figuren, die in der Welt vorkommen können, statt aller
//! (B-797-Review: der Kristallherz-Boss allein sind 6 MiB). Quelle sind die Daten: Gegner des Bioms (Portal, Nacht),
//! alle Gegner der Tiefe (enemies.json › depth: Elite, Händler-Überfall), Bosse der Tiefe (bosses.json, wie
//! engine/sim/boss_data.go nach World.Biome.Depth) und ihre Beschwörungen. Dev-Spawns beliebiger Arten deckt das nicht.

use std::collections::BTreeMap;

use k3c_sim::state::World as SimWorld;

use super::catalog::{BossEntry, Catalog};

type ByDepth = BTreeMap<i64, Vec<String>>;

/// byDepth ordnet die Gegner-Arten (Rohdaten aus enemies.json, Schlüssel ohne depth wie "notes" fallen weg) und die
/// Bosse samt Beschwörungen ihrer Tiefe zu.
pub fn by_depth(enemies: &BTreeMap<String, serde_json::Value>, bosses: &[BossEntry]) -> (ByDepth, ByDepth) {
    let (mut kinds, mut ids): (ByDepth, ByDepth) = Default::default();
    for (kind, raw) in enemies {
        if let Some(d) = raw
            .as_object()
            .and_then(|o| o.get("depth"))
            .and_then(serde_json::Value::as_i64)
        {
            kinds.entry(d).or_default().push(kind.clone());
        }
    }
    for b in bosses {
        ids.entry(b.depth).or_default().push(b.id.clone());
        let abilities = std::iter::once(&b.ability).chain(b.phases.iter().map(|p| &p.ability));
        for a in abilities.flatten() {
            if !a.enemy.is_empty() {
                kinds.entry(b.depth).or_default().push(a.enemy.clone());
            }
        }
    }
    (kinds, ids)
}

impl Catalog {
    /// LevelSheets nennt die Sheet-Assets der Figuren, die in der Welt w vorkommen können, sortiert und je einmal.
    /// Go: ohne Welt (`nil`) alle (`sheet_assets`); in Rust gibt es immer eine Welt, der Aufrufer nimmt sonst
    /// `sheet_assets`.
    pub fn level_sheets(&self, w: &SimWorld) -> Vec<String> {
        let depth = w.biome.depth;
        let mut kinds: Vec<&String> = Vec::new();
        kinds.extend(&w.biome.enemies.portal);
        kinds.extend(&w.biome.enemies.night);
        if let Some(k) = self.depth_kinds.get(&depth) {
            kinds.extend(k);
        }
        let mut out: Vec<String> = Vec::new();
        for k in kinds {
            if let Some(l) = self.enemies.get(k)
                && l.sheet
            {
                out.push(l.asset.clone());
            }
        }
        if let Some(ids) = self.depth_bosses.get(&depth) {
            for id in ids {
                if let Some(l) = self.bosses.get(id)
                    && l.sheet
                {
                    out.push(l.asset.clone());
                }
            }
        }
        for l in self.troops.values() {
            if l.sheet {
                out.push(l.asset.clone());
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }
}

#[cfg(test)]
#[path = "levels_tests.rs"]
mod levels_tests;
