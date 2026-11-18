//! Formtabelle (Teil monarch.json) zu `values_shape.rs`; Lese-Typen in `k3c_data::monarch`.

use super::values_shape::{Field, Shape, boolean, float, int, text, texts};

const MONARCH: &[Field] = &[
    ("Base", || Shape::Struct(&[("Damage", float), ("Speed", float)])),
    ("SprintMultiplier", float),
    ("Acceleration", float),
    ("HitCoins", int),
    ("Crown", crown),
    ("Mount", mount),
    ("Attack", attack),
    ("TierPoints", || Shape::List(int)),
    ("SkillPointSources", skill_point_sources),
    ("Lines", || Shape::Map(|| Shape::Struct(&[("Name", text)]))),
    ("Skills", || Shape::Map(skill)),
    ("Presets", || Shape::Map(preset)),
];

/// Go: `monarchData`.
pub fn monarch() -> Shape {
    Shape::Struct(MONARCH)
}

const CROWN: &[Field] = &[
    ("DistanceUnits", float),
    ("HeightUnits", float),
    ("FlightSeconds", float),
    ("PickupUnits", float),
    ("GrabUnits", float),
    ("BlockSeconds", float),
    ("CoinOffsetUnits", float),
    ("CoinStepUnits", float),
];

fn crown() -> Shape {
    Shape::Struct(CROWN)
}

fn mount() -> Shape {
    Shape::Struct(&[
        ("ID", text),
        ("Sprite", text),
        ("SpeedFactor", float),
        ("SprintFactor", float),
    ])
}

fn attack() -> Shape {
    Shape::Struct(&[("Damage", float), ("Range", float), ("Cooldown", float)])
}

const SKILL_POINT_SOURCES: &[Field] = &[
    ("Hidden", int),
    ("Miniboss", int),
    ("Endboss", int),
    ("Milestone", int),
    ("ChestEvery", int),
];

fn skill_point_sources() -> Shape {
    Shape::Struct(SKILL_POINT_SOURCES)
}

const SKILL: &[Field] = &[
    ("ID", text),
    ("Line", text),
    ("Kind", text),
    ("Requires", text),
    ("Tier", int),
    ("Cooldown", float),
    ("Effect", skill_effect),
    ("Provisional", boolean),
];

fn skill() -> Shape {
    Shape::Struct(SKILL)
}

const SKILL_EFFECT: &[Field] = &[
    ("Type", text),
    ("Radius", float),
    ("Range", float),
    ("Duration", float),
    ("HP", float),
    ("Damage", float),
    ("Factor", float),
    ("PerSecond", float),
    ("Amount", float),
    ("BackDamage", float),
    ("Defense", float),
    ("Aura", float),
    ("Reduction", float),
    ("Chance", float),
    ("SpeedBonus", float),
    ("DamageBonus", float),
    ("SpellBonus", float),
    ("CooldownBonus", float),
    ("Castle", boolean),
    ("FreeHitEvery", int),
    ("Size", int),
    ("MaxGold", int),
];

fn skill_effect() -> Shape {
    Shape::Struct(SKILL_EFFECT)
}

fn preset() -> Shape {
    Shape::Struct(&[("Name", text), ("ActiveSkills", texts)])
}
