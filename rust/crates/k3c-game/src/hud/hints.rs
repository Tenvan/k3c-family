//! Port von `native/k3c-game/hud/hints.go`.
//!
//! Hinweise über der Welt (Spiegel von src/scenes/hints/actionHints.ts, B-319, W11.4, und guideHints.ts, S6.3, B-148):
//! je Spieler höchstens ein Aktions-Hinweis über seinem Monarchen (Ausbilden, Tauschen, Bau des Endbosses; Tasten-
//! Aktionen stehen im Pause-Menü › Tasten, B-741, B-807) und der nächste ungesehene Hinweis der geführten ersten Nacht.
//!
//! Abweichung von Go (Leitfaden § 3): `sim.AtLair` und `sim.SitePayable` kommen über die Sim-Naht `props::SimView`
//! (`action_hint` und `offers_price` nehmen `&dyn SimView`); Go-`[...]`-Tabellen sind Funktionen mit `match`.

use super::data::building_info;
use super::texts::{tf, tr};
use super::values::res_name;
use super::values_texts::profession_text;
use crate::props::simview::SimView;
use k3c_sim::state::action_view::{trade_at, train_at};
use k3c_sim::state::{Player, Site, World};

/// Höhen über dem Boden in Units (Browser: Welt-Pixel / 32): Aktions-Hinweis über dem Monarchen, Hinweis über einem
/// Objekt bzw. über dem Aktions-Hinweis.
pub const HINT_ABOVE_UNITS: f64 = 280.0 / 32.0;
pub const GUIDE_OBJECT_UNITS: f64 = 160.0 / 32.0;
pub const GUIDE_PLAYER_UNITS: f64 = 370.0 / 32.0;
/// viewRules.ts › PRICE_TAG_RANGE
pub const PRICE_TAG_RANGE_UNITS: f64 = 6.0;
pub const HINT_TOP_FREE_REF_PIXEL: f64 = 110.0;

/// HintKind ist die Art des Aktions-Hinweises.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HintKind {
    #[default]
    None,
    Train,
    Buy,
    Sell,
    Lair,
}

/// HintKey ist der gewählte Aktions-Hinweis mit seinem Namen (Beruf bzw. Rohstoff).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HintKey {
    pub lang: String,
    pub kind: HintKind,
    pub name: String,
}

/// choose_hint wählt den Hinweis (playerHint): gefallen keiner; Ausbilden und Tauschen (Gewicht wie Bezahlen) auch am
/// Preisschild; der Bau des Endbosses (Gewicht 1) nur abseits eines Preisschilds.
pub fn choose_hint(
    down: bool,
    train: &str,
    side: &str,
    resource: &str,
    at_tag: bool,
    at_lair: bool,
) -> (HintKind, String) {
    if down {
        return (HintKind::None, String::new());
    }
    if !train.is_empty() {
        return (HintKind::Train, train.to_string());
    }
    if side == "sell" {
        return (HintKind::Sell, resource.to_string());
    }
    if !side.is_empty() {
        return (HintKind::Buy, resource.to_string());
    }
    if at_lair && !at_tag {
        return (HintKind::Lair, String::new());
    }
    (HintKind::None, String::new())
}

/// action_hint liest die Aktionen des Monarchen p an seinem Ort (`train_at`, `trade_at`, `SimView::at_lair` wie die
/// Aktionsliste des Servers, engine/net/actions.go).
pub fn action_hint(lang: &str, w: &World, p: &Player, sv: &dyn SimView) -> HintKey {
    if p.crown.is_some() {
        return HintKey {
            lang: lang.to_string(),
            ..Default::default()
        };
    }
    let (side, res) = trade_at(w, p);
    let (kind, name) = choose_hint(
        false,
        &train_at(w, p),
        &side,
        &res,
        at_price_tag(w, p.x, sv),
        sv.at_lair(w, p),
    );
    HintKey {
        lang: lang.to_string(),
        kind,
        name,
    }
}

/// hint_text ist der Text des Aktions-Hinweises ohne Tastennamen ("" = keiner).
pub fn hint_text(k: &HintKey) -> String {
    let lang = k.lang.as_str();
    match k.kind {
        HintKind::Train => tf(
            lang,
            "hint.train",
            &[("name", profession_text(lang, &k.name, false).into())],
        ),
        HintKind::Buy => tf(lang, "hint.buy", &[("name", res_name(lang, &k.name).into())]),
        HintKind::Sell => tf(lang, "hint.sell", &[("name", res_name(lang, &k.name).into())]),
        HintKind::Lair => tr(lang, "hint.lair").to_string(),
        HintKind::None => String::new(),
    }
}

/// at_price_tag: Steht bei x ein Preisschild in Reichweite (unbezahlter, bezahlbarer Bauplatz oder Werkstatt mit freiem
/// Bogen-Regal, priceTagRules.ts › offersPrice)?
pub fn at_price_tag(w: &World, x: f64, sv: &dyn SimView) -> bool {
    w.sites
        .iter()
        .any(|s| (s.x - x).abs() < PRICE_TAG_RANGE_UNITS && offers_price(w, s, sv))
}

pub fn offers_price(w: &World, s: &Site, sv: &dyn SimView) -> bool {
    if s.state == "unpaid" {
        return sv.site_payable(w, s);
    }
    let info = building_info();
    let bow_rack = info.get("bowyer").map_or(0, |b| b.bow_rack);
    let hammer_rack = info.get("workshop").map_or(0, |b| b.hammer_rack);
    s.state == "built"
        && ((s.kind == "bowyer" && s.bows < bow_rack) || (s.kind == "workshop" && s.hammers < hammer_rack))
}

/// GuideId ist ein Hinweis der geführten ersten Nacht; am Bauplatz steht keiner mehr (🧑, 2026-11-05).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GuideId {
    #[default]
    None = 0,
    Coin = 1,
    Dusk = 2,
    Recruit = 3,
}

/// GUIDE_ORDER ist die Reihenfolge, wenn mehrere passen.
pub const GUIDE_ORDER: [GuideId; 3] = [GuideId::Coin, GuideId::Dusk, GuideId::Recruit];

/// guide_keys sind die Text-Schlüssel der Hinweise (`None`: keiner).
pub fn guide_keys(id: GuideId) -> &'static str {
    match id {
        GuideId::None => "",
        GuideId::Coin => "guide.coin",
        GuideId::Dusk => "guide.dusk",
        GuideId::Recruit => "guide.recruit",
    }
}

/// GuideIcon ist das Bild vor dem Hinweistext (guideHints.ts › guideImage, B-811): Münze, Mond oder keins. hud zeichnet
/// beide aus eigenen Formen (Scheibe, Sichel), ohne Asset.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GuideIcon {
    #[default]
    None,
    Coin,
    Moon,
}

/// guide_image ist das Bild eines Hinweises; ready false (keine Vorlagen zum Zeichnen) = keins, der Text steht allein.
pub fn guide_image(id: GuideId, ready: bool) -> GuideIcon {
    if !ready {
        return GuideIcon::None;
    }
    match id {
        GuideId::Coin => GuideIcon::Coin,
        GuideId::Dusk => GuideIcon::Moon,
        _ => GuideIcon::None,
    }
}

/// GuideHint ist ein gezeigter Hinweis: Ort in Units und das Objekt, an dem die Handlung erkannt wird (-1 = keins).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GuideHint {
    pub id: GuideId,
    pub x: f64,
    pub target: i64,
}

/// GuideSeen merkt die gesehenen Hinweise (Bit je GuideId); gilt für alle Spieler dieses Geräts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GuideSeen(pub u8);

impl GuideSeen {
    pub fn has(self, id: GuideId) -> bool {
        self.0 & (1 << id as u8) != 0
    }
}

/// next_guide ist der nächste ungesehene Hinweis für den Monarchen bei x; gefallen keiner.
pub fn next_guide(w: &World, x: f64, down: bool, seen: GuideSeen) -> GuideHint {
    if down {
        return GuideHint::default();
    }
    for id in GUIDE_ORDER {
        if seen.has(id) {
            continue;
        }
        let h = guide_candidate(id, w, x);
        if h.id != GuideId::None {
            return h;
        }
    }
    GuideHint::default()
}

pub fn guide_candidate(id: GuideId, w: &World, x: f64) -> GuideHint {
    match id {
        GuideId::Coin => nearest_coin(w, x),
        GuideId::Dusk => {
            if w.cycle.phase == "dusk" {
                return GuideHint {
                    id: GuideId::Dusk,
                    x,
                    target: -1,
                };
            }
            GuideHint::default()
        }
        GuideId::Recruit => nearest_vagrant(w, x),
        GuideId::None => GuideHint::default(),
    }
}

/// nearest_coin ist die nächste Münze in Reichweite des Preisschilds.
pub fn nearest_coin(w: &World, x: f64) -> GuideHint {
    let mut best = GuideHint {
        target: -1,
        ..Default::default()
    };
    for c in &w.coins {
        if (c.x - x).abs() < PRICE_TAG_RANGE_UNITS && (best.id == GuideId::None || (c.x - x).abs() < (best.x - x).abs())
        {
            best = GuideHint {
                id: GuideId::Coin,
                x: c.x,
                target: c.id,
            };
        }
    }
    best
}

/// nearest_vagrant ist der nächste Landstreicher in Reichweite des Preisschilds.
pub fn nearest_vagrant(w: &World, x: f64) -> GuideHint {
    let mut best = GuideHint {
        target: -1,
        ..Default::default()
    };
    for t in &w.troops {
        if t.kind == "vagrant"
            && (t.x - x).abs() < PRICE_TAG_RANGE_UNITS
            && (best.id == GuideId::None || (t.x - x).abs() < (best.x - x).abs())
        {
            best = GuideHint {
                id: GuideId::Recruit,
                x: t.x,
                target: t.id,
            };
        }
    }
    best
}

/// guide_done: Handlung erledigt (Münze aufgehoben, Nacht da, Landstreicher ist Bauer).
pub fn guide_done(h: &GuideHint, w: &World) -> bool {
    match h.id {
        GuideId::Coin => !w.coins.iter().any(|c| c.id == h.target),
        GuideId::Dusk => w.cycle.phase == "night",
        GuideId::Recruit => w.troops.iter().any(|t| t.id == h.target && t.kind != "vagrant"),
        GuideId::None => false,
    }
}

/// step_guide: Ist der zuletzt gezeigte Hinweis erledigt, gilt er als gesehen; dann kommt der nächste.
pub fn step_guide(prev: GuideHint, w: &World, x: f64, down: bool, seen: &mut GuideSeen) -> GuideHint {
    if prev.id != GuideId::None && !seen.has(prev.id) && guide_done(&prev, w) {
        seen.0 |= 1 << prev.id as u8;
    }
    next_guide(w, x, down, *seen)
}

/// guide_height ist die Höhe des Hinweises über dem Boden in Units.
pub fn guide_height(h: &GuideHint) -> f64 {
    if h.target < 0 {
        return GUIDE_PLAYER_UNITS;
    }
    GUIDE_OBJECT_UNITS
}
