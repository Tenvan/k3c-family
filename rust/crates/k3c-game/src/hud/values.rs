//! Port von `native/k3c-game/hud/values.go`.
//!
//! Texte der Werte (Spiegel von HudScene.ts › showWorld/showCells/clockText, economyLabels.ts und citizenLabels.ts):
//! reine Funktionen auf einfachen Werten, gebaut nur, wenn sich ihr Schlüssel ändert (`Memo`).
//!
//! Abweichung von Go (Leitfaden § 3): `memo[K comparable]` wird `Memo<K: PartialEq + Clone>` mit Funktionszeiger als
//! Bauer; Go-`*float64` ist `Option<f64>`, `*Stock` ist `Option<&Stock>`. `Economy` kommt bis RU18 aus
//! `crate::props::pending`.

use super::texts::{Arg, name_of, tf, tr};
use crate::props::pending::Economy;
use k3c_data::hub::Stock;
use k3c_sim::state::{CycleInfo, Merchant};

/// SEP trennt Einträge einer Zeile (wie der Browser).
pub const SEP: &str = "  ·  ";

/// Memo hält einen Text, bis sich sein Schlüssel ändert: so entsteht je Bild kein neuer Text.
#[derive(Debug)]
pub struct Memo<K: PartialEq + Clone> {
    key: Option<K>,
    text: String,
}

impl<K: PartialEq + Clone> Default for Memo<K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: PartialEq + Clone> Memo<K> {
    pub fn new() -> Self {
        Self {
            key: None,
            text: String::new(),
        }
    }

    /// get liefert den Text zum Schlüssel k und baut ihn mit build nur bei einem neuen Schlüssel.
    pub fn get(&mut self, k: K, build: fn(&K) -> String) -> &str {
        if self.key.as_ref() != Some(&k) {
            self.text = build(&k);
            self.key = Some(k);
        }
        &self.text
    }
}

/// format_time ist eine Restzeit als m:ss (aufgerundet, nie negativ).
pub fn format_time(seconds: f64) -> String {
    let s = seconds.ceil().max(0.0) as i64;
    format!("{}:{:02}", s / 60, s % 60)
}

/// ClockKey sind die Werte der Uhr: Phase, Tag, Restzeit in ganzen Sekunden (Zeitraffer eingerechnet), Aggression in
/// ganzen Prozent (-1 = keine), gesichert.
#[derive(Clone, Debug, PartialEq)]
pub struct ClockKey {
    pub lang: String,
    pub phase: String,
    pub day: i64,
    pub left: i64,
    pub aggression: i64,
    pub saved: bool,
}

/// new_clock_key liest die Uhr aus dem Zyklus der Welt.
pub fn new_clock_key(lang: &str, c: &CycleInfo, speed: f64, aggression: Option<f64>, saved: bool) -> ClockKey {
    let speed = if speed <= 0.0 { 1.0 } else { speed };
    ClockKey {
        lang: lang.to_string(),
        phase: c.phase.clone(),
        day: c.day,
        left: (c.seconds_left / speed).ceil().max(0.0) as i64,
        aggression: aggression.map_or(-1, |a| a.floor() as i64),
        saved,
    }
}

/// clock_text: Tageszeit mit Restzeit, unter Tage mit Aggression, nach dem Speichern „gesichert“.
pub fn clock_text(k: &ClockKey) -> String {
    let key = match k.phase.as_str() {
        "day" => "hud.clockDay",
        "dusk" => "hud.clockDusk",
        "dawn" => "hud.clockDawn",
        _ => "hud.clockNight",
    };
    let left = format_time(k.left as f64);
    let mut s = tf(&k.lang, key, &[("day", k.day.into()), ("left", left.into())]);
    if k.aggression >= 0 {
        s += &tf(&k.lang, "hud.aggression", &[("pct", k.aggression.into())]);
    }
    if k.saved {
        s += tr(&k.lang, "hud.saved");
    }
    s
}

/// GoldKey: Gold und Maximum (das Gold ist die Lebensanzeige, kampf.md § 5).
#[derive(Clone, Debug, PartialEq)]
pub struct GoldKey {
    pub lang: String,
    pub gold: i64,
    pub max: i64,
}

/// gold_text ist der Wert der Spielerkarte: „Gold/Maximum“. Gold steht nur hier (Regel „Gold nur im HUD“, B-398).
pub fn gold_text(k: &GoldKey) -> String {
    format!("{}/{}", k.gold, k.max)
}

/// GOLD_FLASH_SECONDS: so lange hebt das HUD den Goldtopf nach einer Änderung hervor (B-398).
pub const GOLD_FLASH_SECONDS: f64 = 0.8;

/// GoldFlash ist die Hervorhebung des Goldtopfs eines Spielers (goldFlash.ts).
#[derive(Clone, Debug, Default)]
pub struct GoldFlash {
    gold: i64,
    changed_at: f64,
    seen: bool,
}

impl GoldFlash {
    /// step übernimmt den Goldstand zur Zeit now und meldet, ob der Topf hervorgehoben wird; der erste Wert zählt
    /// nicht als Änderung.
    pub fn step(&mut self, gold: i64, now: f64) -> bool {
        if !self.seen {
            self.gold = gold;
            self.changed_at = f64::NEG_INFINITY;
            self.seen = true;
        } else if self.gold != gold {
            self.gold = gold;
            self.changed_at = now;
        }
        now - self.changed_at < GOLD_FLASH_SECONDS
    }
}

/// RESOURCE_ORDER ist die feste Reihenfolge der Rohstoffe (wie `Stock`).
pub const RESOURCE_ORDER: [&str; 5] = ["wood", "stone", "copper", "iron", "crystal"];

/// res_name ist der Name eines Rohstoffs aus den Texten, sonst die ID.
pub(super) fn res_name(lang: &str, res: &str) -> String {
    name_of(lang, "res", res, res)
}

/// StockKey: Vorrat je Rohstoff, Primärrohstoff des Bioms, Lager-Maximum (0 = keins), Skill-Punkte.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StockKey {
    pub lang: String,
    pub primary: String,
    pub stock: [i64; 5],
    pub max: i64,
    pub points: i64,
}

/// shared_text ist die Vorrats-Zeile: Rohstoffe mit Vorrat oder der Primärrohstoff, mit Maximum „n/max“ bzw. „voll“,
/// dahinter die Skill-Punkte, wenn welche frei sind (HudScene.ts › showWorld, economyLabels.ts › stockTexts).
pub fn shared_text(k: &StockKey) -> String {
    let mut parts: Vec<String> = Vec::with_capacity(6);
    for (i, r) in RESOURCE_ORDER.iter().enumerate() {
        let n = k.stock[i];
        if n <= 0 && *r != k.primary {
            continue;
        }
        let name: Arg = res_name(&k.lang, r).into();
        parts.push(if k.max <= 0 {
            tf(&k.lang, "stock.amount", &[("res", name), ("n", n.into())])
        } else if n >= k.max {
            tf(
                &k.lang,
                "stock.full",
                &[("res", name), ("n", n.into()), ("max", k.max.into())],
            )
        } else {
            tf(
                &k.lang,
                "stock.ofMax",
                &[("res", name), ("n", n.into()), ("max", k.max.into())],
            )
        });
    }
    if k.points > 0 {
        parts.push(tf(&k.lang, "hud.skillPoints", &[("n", k.points.into())]));
    }
    parts.join(SEP)
}

/// stock_of liest den Vorrat in fester Reihenfolge (None = leer).
pub fn stock_of(s: Option<&Stock>) -> [i64; 5] {
    s.map_or([0; 5], |s| [s.wood, s.stone, s.copper, s.iron, s.crystal])
}

/// HubKey: Hub-Stufe, Schritt und Ausbau (`has` = es gibt eine nächste Stufe).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HubKey {
    pub lang: String,
    pub level: i64,
    pub step: i64,
    pub steps: i64,
    pub has: bool,
    pub gold: i64,
    pub paid: i64,
    pub material: [i64; 5],
    pub fighters: i64,
    pub limit: i64,
    pub merchant: bool,
    pub m_resource: String,
    pub m_leaves: i64,
    pub m_paid: i64,
    pub compact: bool,
}

/// new_hub_key liest Hub, Bürger und Händler aus dem Wirtschaftsstand und der Welt.
pub fn new_hub_key(lang: &str, e: &Economy, m: Option<&Merchant>) -> HubKey {
    let mut k = HubKey {
        lang: lang.to_string(),
        level: e.hub_level,
        fighters: e.fighters,
        limit: e.troop_limit,
        compact: true,
        ..HubKey::default()
    };
    if let Some(up) = &e.hub_upgrade {
        k.has = true;
        k.step = e.hub_step;
        k.steps = up.steps;
        k.gold = up.gold;
        k.paid = up.paid;
        k.material = stock_of(Some(&up.material));
    }
    if let Some(m) = m {
        k.merchant = true;
        k.m_resource = m.resource.clone();
        k.m_leaves = m.leaves;
        k.m_paid = m.buy_paid;
    }
    k
}

/// citizens_text ist die Zeile Hub, Truppen-Limit und Händler (kurze Form wie die Leiste des Browsers).
pub fn citizens_text(k: &HubKey) -> String {
    let mut parts: Vec<String> = Vec::with_capacity(3);
    let h = hub_text(k);
    if !h.is_empty() {
        parts.push(h);
    }
    parts.push(limit_text(&k.lang, k.fighters, k.limit, k.compact));
    if k.merchant {
        parts.push(merchant_text(&k.lang, &k.m_resource, k.m_leaves, k.m_paid, k.compact));
    }
    parts.join(SEP)
}

/// hub_text ist die Hub-Stufe mit Schritt und Kosten des laufenden Ausbauschritts; ohne Ausbau nur die Stufe, ohne
/// Stufe "".
pub fn hub_text(k: &HubKey) -> String {
    if k.level <= 0 {
        return String::new();
    }
    if !k.has {
        let key = pick(k.compact, "hub.levelShort", "hub.level");
        return tf(&k.lang, key, &[("n", k.level.into())]);
    }
    let gold = tf(
        &k.lang,
        "cost.gold",
        &[("paid", k.paid.into()), ("gold", k.gold.into())],
    );
    let mut parts = vec![gold];
    for (i, r) in RESOURCE_ORDER.iter().enumerate() {
        if k.material[i] > 0 {
            let item = [("n", k.material[i].into()), ("res", res_name(&k.lang, r).into())];
            parts.push(tf(&k.lang, "cost.item", &item));
        }
    }
    let cost: Arg = parts.join(", ").into();
    let (n, next): (Arg, Arg) = (k.level.into(), (k.level + 1).into());
    if k.steps > 0 {
        let key = pick(k.compact, "hub.upgradeStepShort", "hub.upgradeStep");
        let kv = [
            ("n", n),
            ("next", next),
            ("cost", cost),
            ("step", k.step.into()),
            ("steps", k.steps.into()),
        ];
        return tf(&k.lang, key, &kv);
    }
    let key = pick(k.compact, "hub.upgradeShort", "hub.upgrade");
    tf(&k.lang, key, &[("n", n), ("next", next), ("cost", cost)])
}

pub(super) fn pick<'a>(short: bool, a: &'a str, b: &'a str) -> &'a str {
    if short { a } else { b }
}

/// limit_text: „Kämpfer/Limit“, voll, sobald die Zahl das Limit erreicht.
pub fn limit_text(lang: &str, n: i64, limit: i64, compact: bool) -> String {
    let full = n >= limit;
    let key = pick(
        compact,
        pick(full, "limit.fullShort", "limit.short"),
        pick(full, "limit.full", "limit.text"),
    );
    tf(lang, key, &[("n", n.into()), ("max", limit.into())])
}

pub use super::values_texts::*;
