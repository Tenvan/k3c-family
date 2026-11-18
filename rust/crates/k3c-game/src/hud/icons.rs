//! Port von `native/k3c-game/hud/icons.go`: Auswahl der Icons des HUD (Rohstoffe, Münze, Geldsack, Tasten, Seekarte),
//! die gemeldeten Assets und die Teile der Vorrats-Anzeige.

use super::Hud;
use super::glyphs::{CAP_NARROW, CAP_WIDE};
use super::purse::PURSE_ASSET;
use super::texts::tf;
use super::values::{RESOURCE_ORDER, StockKey};
use crate::art::SeaMap;
use crate::view::roles::AssetSource;
use k3c_sim::state::World;

/// ICON_ANIM ist die Animation der Icons.
pub const ICON_ANIM: &str = "idle";

/// GOLD_ICON ist das Münz-Icon (die Spielerkarte zeigt seit B-919 den Geldsack, purse.go).
pub const GOLD_ICON: &str = "icons/icon-muenze";

/// RESOURCE_ICONS ordnet jedem Rohstoff (`RESOURCE_ORDER`) sein Icon zu.
pub const RESOURCE_ICONS: [(&str, &str); 5] = [
    ("wood", "icons/icon-holz"),
    ("stone", "icons/icon-stein"),
    ("copper", "icons/icon-kupfer"),
    ("iron", "icons/icon-eisen"),
    ("crystal", "icons/icon-kristall"),
];

/// resource_icon ist das Icon eines Rohstoffs ("" = keins).
pub fn resource_icon(res: &str) -> &'static str {
    RESOURCE_ICONS.iter().find(|(r, _)| *r == res).map_or("", |(_, a)| a)
}

/// HUD_ASSETS sind alle Icons des HUD, sortiert.
pub const HUD_ASSETS: [&str; 7] = [
    PURSE_ASSET,
    "icons/icon-eisen",
    "icons/icon-holz",
    "icons/icon-kristall",
    "icons/icon-kupfer",
    GOLD_ICON,
    "icons/icon-stein",
];

/// KEY_ASSETS sind die Tasten-Icons (B-911), sortiert: die zwei Kappen und je Xbox-Taste mit Bild (`PAD_GLYPHS`) eins.
pub const KEY_ASSETS: [&str; 13] = [
    CAP_NARROW,
    CAP_WIDE,
    "icons/xbox-a",
    "icons/xbox-down",
    "icons/xbox-lb",
    "icons/xbox-lt",
    "icons/xbox-menu",
    "icons/xbox-rb",
    "icons/xbox-rs",
    "icons/xbox-rt",
    "icons/xbox-up",
    "icons/xbox-x",
    "icons/xbox-y",
];

/// first_unusable ist das erste Asset, das `usable` nicht laden kann ("" = alle ladbar).
pub fn first_unusable(assets: &[&str], usable: &dyn Fn(&str) -> bool) -> String {
    assets
        .iter()
        .find(|a| !usable(a))
        .map_or(String::new(), |a| (*a).to_string())
}

impl Hud {
    /// Go: `(*hud).Assets`: meldet die Icons des HUD für jedes Level (`AssetSource`), mit ladbaren Tasten-Icons und
    /// ladbarer Seekarte auch deren Assets.
    pub fn hud_assets(&self) -> Vec<String> {
        match &self.assets {
            Some(a) => a.clone(),
            None => HUD_ASSETS.iter().map(|a| (*a).to_string()).collect(),
        }
    }

    /// use_key_icons nimmt die Tasten-Icons, wenn `usable` alle laden kann (weiche Assets wie die Seekarte); sonst malt
    /// das HUD die Glyphen selbst (drawGlyph).
    pub fn use_key_icons(&mut self, usable: &dyn Fn(&str) -> bool) {
        let a = first_unusable(&KEY_ASSETS, usable);
        if !a.is_empty() {
            k3c_platform::log::info(
                "🚫 Tasten ohne Papierschnitt-Icons, Asset nicht ladbar",
                &[("asset", &a)],
            );
            return;
        }
        self.key_icons = true;
        self.list_assets();
    }

    /// use_sea_map nimmt die Assets der Seekarte m, wenn `usable` alle laden kann (Library.Usable: ein fehlendes Bild
    /// darf nie ein Level verhindern); sonst bleibt die Karte aus Formen (B-964/R5).
    pub fn use_sea_map(&mut self, m: Option<SeaMap>, usable: &dyn Fn(&str) -> bool) {
        let Some(m) = m else {
            return;
        };
        let assets = m.assets();
        let refs: Vec<&str> = assets.iter().map(String::as_str).collect();
        let a = first_unusable(&refs, usable);
        if !a.is_empty() {
            k3c_platform::log::info("🚫 Seekarte ohne Diorama, Asset nicht ladbar", &[("asset", &a)]);
            return;
        }
        self.sea_map = Some(m);
        self.list_assets();
    }

    /// list_assets stellt die gemeldeten Assets zusammen: Icons, Tasten-Icons und Seekarte, sortiert.
    pub fn list_assets(&mut self) {
        let mut a: Vec<String> = HUD_ASSETS.iter().map(|s| (*s).to_string()).collect();
        if self.key_icons {
            a.extend(KEY_ASSETS.iter().map(|s| (*s).to_string()));
        }
        if let Some(m) = &self.sea_map {
            a.extend(m.assets());
        }
        a.sort_unstable();
        self.assets = Some(a);
    }
}

impl AssetSource for Hud {
    fn assets(&self, _level: &str, _w: &World) -> Vec<String> {
        self.hud_assets()
    }
}

/// StockPart ist ein Teil der Vorrats-Anzeige: Rohstoff (Icon; "" = nur Text) und Zahl bzw. Text.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StockPart {
    pub res: String,
    pub text: String,
    pub full: bool,
}

/// STOCK_SLOTS ist die Zahl der Teile (fünf Rohstoffe und die Skill-Punkte).
pub const STOCK_SLOTS: usize = RESOURCE_ORDER.len() + 1;

/// stock_parts schreibt die Teile der Vorrats-Anzeige in dst (geleert): wie `shared_text` die Rohstoffe mit Vorrat oder
/// den Primärrohstoff, je „n“ bzw. „n/max“, dahinter die freien Skill-Punkte als Text.
pub fn stock_parts(dst: &mut Vec<StockPart>, k: &StockKey) {
    dst.clear();
    for (i, r) in RESOURCE_ORDER.iter().enumerate() {
        let n = k.stock[i];
        if n <= 0 && *r != k.primary {
            continue;
        }
        let mut p = StockPart {
            res: (*r).to_string(),
            text: n.to_string(),
            full: false,
        };
        if k.max > 0 {
            p.text.push('/');
            p.text.push_str(&k.max.to_string());
            p.full = n >= k.max;
        }
        dst.push(p);
    }
    if k.points > 0 {
        dst.push(StockPart {
            res: String::new(),
            text: tf(&k.lang, "hud.skillPoints", &[("n", k.points.into())]),
            full: false,
        });
    }
}
