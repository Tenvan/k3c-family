//! Handwerker und Hammer (Port von `engine/sim/unit_hammer.go`, docs/rules/buerger.md § 1, B-950, B-951): Nur ein Bauer
//! mit Hammer baut, baut aus und repariert. Seit ECO1.3 (native Regeln) ist der „Hammer“ der Werkzeuggürtel (`belt_cost`).
//!   - Hammer (`Troop::hammer`): Die Werkstatt (Handwerkerbedarf) stellt sie her (Preis buildings.json › workshop.hammerCost,
//!     Regal hammerRack); ein Bauer ohne Hammer holt sie dort (`fetchHammer`) oder vom Boden (Drop `hammer`), mit demselben
//!     Gegner-Abstand wie Waffen (`weapon_to_fetch`, B-929).
//!   - Ein Handwerker holt weder Waffe noch zweiten Hammer; fällt er (Verlust), bleibt der Hammer als Drop am Boden.
//!   - Start: hub.json › startTroops.craftsman Handwerker zusätzlich zu den Bauern (je Welt übersteuerbar, Standard 1).
//!   - Bögen stellt der Bogner her (Gebäude `bowyer`, Regal `bowRack`); Werkstatt und Bogner teilen den Mechanismus
//!     „Produkt am Platz“ (`Site::bow_paid_gold`, `Site::craft_done`, building_upgrades.go).

use k3c_data::buildings::Cost;

use crate::state::{ItemDrop, Site, World, WorldValues};

/// Go: `hammerKind` ist die Art des Hammers als Drop und als Ausrüstung.
pub const HAMMER_KIND: &str = "hammer";

/// Welches Regal ein Hersteller führt (Go: der Zeiger `have` auf `Site.Hammers` oder `Site.Bows`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Product {
    Hammer,
    Bow,
}

impl Product {
    /// Go: `*p.have`: wie viele liegen im Regal.
    pub fn have(self, s: &Site) -> i64 {
        match self {
            Self::Hammer => s.hammers,
            Self::Bow => s.bows,
        }
    }

    /// Go: `*p.have += n`.
    pub fn add(self, s: &mut Site, n: i64) {
        match self {
            Self::Hammer => s.hammers += n,
            Self::Bow => s.bows += n,
        }
    }
}

/// Go: `placeProductInfo` beschreibt das Produkt am Platz eines Herstellers: Name in `craftSeconds`, Preis, Regalgröße
/// und welches Regal zählt (Hämmer der Werkstatt, Bögen des Bogners).
#[derive(Debug, Clone, PartialEq)]
pub struct PlaceProduct {
    pub name: &'static str,
    pub cost: Cost,
    pub rack: i64,
    pub product: Product,
}

/// ECO1.3 (B-1193): Der Preis des Werkzeuggürtels, den die Werkstatt statt des Einzelhammers herstellt: 30 Holz, 10 Stein,
/// 3 Gold. Native Regel (`WorldValues::native_rules`), die Zahlen stehen als Werte der Regel hier, weil `buildings.json`
/// mit Go geteilt ist (Go-Golden). Der Gürtel enthält Hammer, Spitzhacke und Axt; er ist das, was `Troop::hammer` und das
/// Regal `Site::hammers` (Art `hammer`) zählen, die Namen bleiben wegen der Go-Parität.
pub fn belt_cost() -> Cost {
    Cost {
        wood: 30,
        stone: 10,
        gold: 3,
        ..Cost::default()
    }
}

/// Go: `placeProduct`.
pub fn place_product(v: &WorldValues, s: &Site) -> PlaceProduct {
    let fallback = Default::default();
    let b = v.buildings.get(s.kind.as_str()).unwrap_or(&fallback);
    if s.kind == "workshop" {
        return PlaceProduct {
            name: HAMMER_KIND,
            cost: if v.native_rules {
                belt_cost()
            } else {
                b.hammer_cost.clone()
            },
            rack: b.hammer_rack,
            product: Product::Hammer,
        };
    }
    let cost = v.troops.get("archer").map(|t| t.cost.clone()).unwrap_or_default();
    PlaceProduct {
        name: "bow",
        cost,
        rack: b.bow_rack,
        product: Product::Bow,
    }
}

/// Go: `spawnVagrant` mit bereits geholtem Wertesatz: legt einen Landstreicher an, gibt seinen Index zurück.
pub fn spawn_vagrant_with(w: &mut World, v: &WorldValues, camp_x: f64, x: f64) -> usize {
    let hp = v.troops.get("vagrant").map_or(0.0, |t| t.hp);
    let id = w.new_id();
    w.troops.push(crate::state::Troop {
        id,
        kind: "vagrant".into(),
        x,
        hp,
        max_hp: hp,
        anchor_x: camp_x,
        target_x: x,
        ..Default::default()
    });
    w.troops.len() - 1
}

/// Go: `dropHammer` lässt den Hammer von `w.troops[ti]` am Boden liegen (nichts, wenn die Truppe keinen trägt).
pub fn drop_hammer(w: &mut World, ti: usize) {
    if !w.troops[ti].hammer {
        return;
    }
    w.troops[ti].hammer = false;
    let (id, x) = (w.new_id(), w.troops[ti].x);
    w.drops.push(ItemDrop {
        id,
        kind: HAMMER_KIND.into(),
        x,
        work_site: 0,
    });
}

/// Go: `spawnStartPeasants` stellt die Start-Bauern hin: `startTroops.craftsman` Handwerker mit Hammer, dazu
/// `startTroops.peasant` Bauern ohne (die Waffen holen).
pub fn spawn_start_peasants(w: &mut World, v: &WorldValues, hub_x: f64) {
    let st = &v.hub.start_troops;
    let hp = v.troops.get("peasant").map_or(0.0, |t| t.hp);
    let craftsmen = st.craftsman.max(1); // jedes Spiel startet mit mindestens einem Handwerker
    for i in 0..craftsmen + st.peasant {
        let ti = spawn_vagrant_with(w, v, hub_x, hub_x + 2.0 + i as f64);
        let t = &mut w.troops[ti];
        t.kind = "peasant".into();
        t.hp = hp;
        t.max_hp = hp;
        t.hammer = i < craftsmen;
    }
}

/// Go: `spareHammer`: Gibt es keinen Handwerker und keinen Hammer (Boden, Regal), legt die Burg zum Tagesbeginn einen
/// Hammer ab (hub.json › `castleSpareHammer`), damit der letzte Hammer nie endgültig verloren ist.
pub fn spare_hammer(w: &mut World, v: &WorldValues) {
    if !v.hub.castle_spare_hammer {
        return;
    }
    if w.troops.iter().any(|t| t.hammer)
        || w.drops.iter().any(|d| d.kind == HAMMER_KIND)
        || w.sites.iter().any(|s| s.hammers > 0)
    {
        return;
    }
    let (id, x) = (w.new_id(), w.castle.x);
    w.drops.push(ItemDrop {
        id,
        kind: HAMMER_KIND.into(),
        x,
        work_site: 0,
    });
}
