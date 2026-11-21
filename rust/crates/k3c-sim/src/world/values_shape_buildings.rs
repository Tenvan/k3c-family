//! Formtabelle (Teil buildings.json und economy.json) zu `values_shape.rs`; Lese-Typen in `k3c_data::buildings` und
//! `k3c_data::economy`.

use super::values_shape::{Field, Shape, cost, float, int, int_pair, level_data, text, texts};

// buildings.json

const BUILDING: &[Field] = &[
    ("HP", float),
    ("BuildSeconds", float),
    ("Cost", cost),
    ("CostByDepth", || Shape::List(cost)),
    ("ArcherSlots", int),
    ("RangeBonus", int),
    ("BowRack", int),
    ("SwordRack", int),
    ("HammerRack", int),
    ("HammerCost", cost),
    ("ArmorRack", int),
    ("ArmorCost", cost),
    ("Offers", || Shape::List(offer)),
    ("CraftSeconds", || Shape::Map(float)),
    ("Levels", || Shape::List(level_data)),
    ("Plantation", plantation),
    ("TroopLimit", || Shape::Struct(&[("Base", int), ("PerBuilding", int)])),
    ("Vagrants", vagrants),
    ("Heal", || {
        Shape::Struct(&[("HPPerSecond", float), ("RadiusUnits", float)])
    }),
    ("Spell", spell),
    ("Armor", || Shape::List(armor_stage)),
];

/// Go: `BuildingData`.
pub fn buildings() -> Shape {
    Shape::Struct(BUILDING)
}

fn offer() -> Shape {
    Shape::Struct(&[("Kind", text), ("DX", float)])
}

fn plantation() -> Shape {
    Shape::Struct(&[("Slots", int), ("GrowSeconds", float), ("StripUnits", float)])
}

fn vagrants() -> Shape {
    Shape::Struct(&[("PerDawn", int), ("Max", int), ("WanderUnits", float)])
}

fn spell() -> Shape {
    Shape::Struct(&[
        ("FromLevel", int),
        ("Damage", float),
        ("RadiusUnits", float),
        ("RangeUnits", float),
        ("IntervalSeconds", float),
    ])
}

fn armor_stage() -> Shape {
    Shape::Struct(&[("HubLevel", int), ("HPBonus", float), ("Cost", cost)])
}

// economy.json

const ECONOMY: &[Field] = &[
    ("Purse", || Shape::Struct(&[("StartGold", int), ("MaxGold", int)])),
    ("PayRangeUnits", float),
    ("PayIntervalSeconds", float),
    ("PickupRangeUnits", float),
    ("DropPickupDelaySeconds", float),
    ("DawnGoldPerPlayer", int),
    ("DawnGoldPerDepth", int),
    ("ChestGold", int_pair),
    ("ChestFountain", chest_fountain),
    ("TreeGold", int_pair),
    ("EnemyGoldFactor", float),
    ("EnemyResourceDrop", || {
        Shape::Struct(&[("Chance", float), ("Amount", int)])
    }),
    ("Gatherables", || Shape::Map(gatherable)),
    ("Veins", || Shape::Map(vein)),
    ("Storage", || Shape::Struct(&[("BasePerHub", int), ("PerStorage", int)])),
    ("RecruitCamp", recruit_camp),
    ("Merchant", merchant),
    ("Combat", combat),
];

/// Go: `economyData`.
pub fn economy() -> Shape {
    Shape::Struct(ECONOMY)
}

fn chest_fountain() -> Shape {
    Shape::Struct(&[
        ("FlightSeconds", float),
        ("SpreadUnits", float),
        ("StaggerSeconds", float),
    ])
}

const GATHERABLE: &[Field] = &[
    ("Resource", text),
    ("Amount", int),
    ("WorkSeconds", float),
    ("MarkCost", int),
];

/// Go: `Gatherable`.
pub fn gatherable() -> Shape {
    Shape::Struct(GATHERABLE)
}

/// Go: `VeinData` bettet `Gatherable` ein; Gos `t.Fields()` kennt das eingebettete Feld nur unter seinem Typnamen, die
/// Felder von `Gatherable` (`amount` …) sind darum kein Feld von `VeinData` (wie in Go, siehe `values_shape_tests.rs`).
fn vein() -> Shape {
    Shape::Struct(&[("Gatherable", gatherable), ("MaxWorkers", int)])
}

fn recruit_camp() -> Shape {
    Shape::Struct(&[("MaxVagrants", int), ("RespawnSeconds", float), ("WanderUnits", float)])
}

const MERCHANT: &[Field] = &[
    ("HP", float),
    ("EveryDays", int),
    ("EveryDaysWithTavern", int),
    ("StayDays", int),
    ("Material", int),
    ("Gold", int),
    ("Materials", texts),
];

fn merchant() -> Shape {
    Shape::Struct(MERCHANT)
}

const COMBAT: &[Field] = &[
    ("ShieldCost", int),
    ("TriggerCost", int),
    ("BannerCost", int),
    ("TroopSoldiers", int),
    ("TroopArchers", int),
    ("EngageUnits", float),
    ("ShieldOffsetUnits", float),
    ("GateOutUnits", float),
    ("SpotClearUnits", float),
    ("ShieldMinHubLevel", int),
    ("ShieldStackMax", int),
    ("BehindUnits", float),
    ("StandRangeFactor", float),
    ("FollowUnits", float),
    ("ShieldArriveUnits", float),
];

fn combat() -> Shape {
    Shape::Struct(COMBAT)
}
