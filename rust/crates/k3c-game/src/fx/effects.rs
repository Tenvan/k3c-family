//! Port von `native/k3c-game/fx/effects.go`.
//!
//! Ereignis → Effekt (Spiegel von src/scenes/effects/effects.ts, B-164, GR5.1): reine Zuordnung, liest nur das
//! Ereignis, ändert keinen Spielzustand. Dazu die Schadenszahl je Treffer (B-799 › Beispiele: „Monarch trifft einen
//! Gegner → Schadenszahl steigt auf“; der Browser zeigt nur den Treffer-Blitz).

use std::collections::BTreeMap;

use k3c_sim::state::{Event, World};

use super::texts::{cargo_full_text, no_target_text};

/// `Kind` ist die Art eines Effekts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Hit,
    Kill,
    Coin,
    CoinGive,
    Build,
    Built,
    Down,
    Swing,
    NoTarget,
    Skill,
    Damage,
    CoinLoss,
}

/// `Look` ist das Aussehen eines Effekts: Ring plus Partikel in Farbe, Größe (Welt-px), Dauer (s), Zahl und Bewegung.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Look {
    pub color: u32,
    pub radius: f64,
    pub duration: f64,
    pub particles: i64,
    pub motion: Motion,
}

/// `Motion` ist die Bewegung der Partikel: burst im Kreis, rise steigt auf, fall fällt herab, wave am Boden zur Seite.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    Burst,
    Rise,
    Fall,
    Wave,
}

/// `look` baut ein Aussehen (Farbe, Radius, Dauer, Partikelzahl, Bewegung).
pub const fn look(color: u32, radius: f64, duration: f64, particles: i64, motion: Motion) -> Look {
    Look {
        color,
        radius,
        duration,
        particles,
        motion,
    }
}

/// `Config`: Aussehen und Grenzen eines Effekts; `flash` = Blitz-Effekt (Grenze 3 je Sekunde).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Config {
    pub look: Look,
    pub flash: bool,
}

const fn cfg(look: Look, flash: bool) -> Config {
    Config { look, flash }
}

/// `CONFIG`: Aussehen und Grenzen aller Effekte (EFFECT_CONFIG), in der Reihenfolge von `Kind`; Zugriff über `config`.
pub const CONFIG: [Config; 12] = [
    cfg(look(0xffffff, 22.0, 0.16, 0, Motion::Burst), true),  // Hit
    cfg(look(0xff5533, 34.0, 0.38, 6, Motion::Burst), false), // Kill
    cfg(look(0xffd23c, 10.0, 0.3, 3, Motion::Burst), false),  // Coin
    cfg(look(0xffe680, 12.0, 0.3, 3, Motion::Burst), false),  // CoinGive
    cfg(look(0xc8b078, 26.0, 0.3, 4, Motion::Burst), false),  // Build
    cfg(look(0x7be07b, 44.0, 0.5, 8, Motion::Burst), false),  // Built
    cfg(look(0xb02030, 40.0, 0.6, 8, Motion::Burst), false),  // Down
    cfg(look(0xdddddd, 18.0, 0.14, 0, Motion::Burst), false), // Swing
    cfg(look(0xffffff, 0.0, 0.7, 0, Motion::Burst), false),   // NoTarget
    cfg(look(0xffffff, 30.0, 0.45, 4, Motion::Burst), false), // Skill
    cfg(look(0xffffff, 0.0, 0.8, 0, Motion::Burst), false),   // Damage
    // Münzen springen aus dem Geldsack, wenn ein Treffer den Monarchen Gold kostet (kampf.md § 5); gezeichnet als
    // Münz-Sprite (paint.go), die Farbe gilt ohne Grafik.
    cfg(look(0xffd23c, 14.0, 0.7, 4, Motion::Rise), false), // CoinLoss
];

/// `config` liefert Aussehen und Grenzen der Art k.
pub fn config(k: Kind) -> Config {
    CONFIG[k as usize]
}

// Grenzen (effects.ts): höchstens 3 Blitze je Sekunde, höchstens 48 lebende Teile je Stufe.
pub const MAX_FLASHES_PER_SECOND: i64 = 3;
pub const FLASH_MIN_GAP: f64 = 1.0 / MAX_FLASHES_PER_SECOND as f64;
pub const MAX_LIVE_PIECES: i64 = 48;

// Höhen über dem Boden in Welt-px (Browser): Treffer und Münzen, Bau, „kein Ziel“, Skill-Ring.
pub const BODY_PX: f64 = 40.0;
pub const BUILD_PX: f64 = 60.0;
pub const NO_TARGET_PX: f64 = 90.0;
pub const RING_PX: f64 = 50.0;
pub const UNIT_PX: f64 = 32.0;

/// `Effect` ist ein laufender Effekt einer Stufe.
#[derive(Debug, Clone, PartialEq)]
pub struct Effect {
    pub kind: Kind,
    /// x in Units
    pub x: f64,
    /// y Höhe über dem Boden in Welt-px
    pub y: f64,
    pub look: Look,
    /// noTarget und Schadenszahl
    pub text: String,
    /// Schadenszahl: Treffer an einem Monarchen (rot)
    pub target: bool,
    pub start: f64,
    /// Schlag: ID des ausholenden Spielers, sonst -1
    pub by: i64,
}

impl Effect {
    /// `new` ist ein Effekt der Art k ohne Text, Start 0 und `by` = -1.
    pub fn new(kind: Kind, x: f64, y: f64) -> Effect {
        Effect {
            kind,
            x,
            y,
            look: config(kind).look,
            text: String::new(),
            target: false,
            start: 0.0,
            by: -1,
        }
    }

    /// `none` ist der leere Effekt, den Go bei `ok == false` liefert.
    pub fn none() -> Effect {
        Effect::new(Kind::Hit, 0.0, 0.0)
    }

    /// `pieces` ist die Zahl der Teile (Ring plus Partikel; Text = 1).
    pub fn pieces(&self) -> i64 {
        if !self.text.is_empty() {
            return 1;
        }
        1 + self.look.particles
    }
}

/// `flash_allowed`: Blitz erlaubt, wenn der letzte mindestens `FLASH_MIN_GAP` Sekunden zurückliegt (last < 0: noch
/// keiner).
pub fn flash_allowed(now: f64, last: f64) -> bool {
    last < 0.0 || now - last >= FLASH_MIN_GAP - 1e-9
}

/// `PlayerX` liefert den Ort des Monarchen mit Index i (zweiter Wert false, wenn es ihn nicht gibt).
pub type PlayerX<'a> = dyn Fn(i64) -> (f64, bool) + 'a;

/// `effect_for` ordnet einem Ereignis genau einen Effekt zu; unbekannte Typen und Ereignisse ohne Ort: ok false.
/// `spots` merkt den Ort des letzten Baufortschritts je Bauart (Rückfall für built ohne x).
pub fn effect_for(lang: &str, d: &Event, typ: &str, px: &PlayerX, spots: &mut BTreeMap<String, f64>) -> (Effect, bool) {
    let at = |k: Kind, x: f64, ok: bool, up: f64| (Effect::new(k, x, up), ok);
    let (mut x, mut has_x) = num_of(d, "x");
    match typ {
        "hit" => at(Kind::Hit, x, has_x, BODY_PX),
        "kill" => at(Kind::Kill, x, has_x, BODY_PX),
        "coinPickup" => at(Kind::Coin, x, has_x, BODY_PX),
        "coinGive" => at(Kind::CoinGive, x, has_x, BODY_PX),
        "buildProgress" => {
            if has_x {
                spots.insert(str_of(d, "kind"), x);
            }
            at(Kind::Build, x, has_x, BUILD_PX)
        }
        "built" => {
            if !has_x {
                (x, has_x) = match spots.get(&str_of(d, "kind")) {
                    Some(v) => (*v, true),
                    None => (0.0, false),
                };
            }
            at(Kind::Built, x, has_x, BUILD_PX)
        }
        "crownFallen" => {
            if !has_x {
                let (p, _) = num_of(d, "player");
                (x, has_x) = px(p as i64);
            }
            at(Kind::Down, x, has_x, BODY_PX)
        }
        "strike" => swing(d, x, has_x),
        "castFailed" => {
            let (mut e, ok) = at(Kind::NoTarget, x, has_x, NO_TARGET_PX);
            e.text = no_target_text(lang).to_string();
            (e, ok)
        }
        _ => (Effect::none(), false),
    }
}

/// `cargo_full_for`: Rückmeldung am Ladeplatz, dessen Einwurf das volle Schiff ablehnte (Ereignis `cargoFull`, Feld
/// `bay`); der Ort kommt aus dem Fracht-Zustand der Welt, ohne Ladeplatz kein Effekt.
pub fn cargo_full_for(lang: &str, d: &Event, w: &World) -> (Effect, bool) {
    let bay = str_of(d, "bay");
    let Some(b) = w.cargo.as_ref().and_then(|c| c.bays.iter().find(|b| b.kind == bay)) else {
        return (Effect::none(), false);
    };
    let mut e = Effect::new(Kind::NoTarget, b.x, NO_TARGET_PX);
    e.text = cargo_full_text(lang).to_string();
    (e, true)
}

/// `swing`: Nur der Monarch holt aus (Schlag mit „hit“); der Schlag eines Gegners trägt kein „hit“.
fn swing(d: &Event, x: f64, has_x: bool) -> (Effect, bool) {
    if !d.contains_key("hit") || !has_x {
        return (Effect::none(), false);
    }
    let (from, _) = num_of(d, "from");
    let mut e = Effect::new(Kind::Swing, x, BODY_PX);
    e.by = from as i64;
    (e, true)
}

/// `damage_for` ist die Schadenszahl eines Treffers: aufgerundeter Schaden über dem Ziel, rot an einem Monarchen.
pub fn damage_for(d: &Event) -> (Effect, bool) {
    let (x, has_x) = num_of(d, "x");
    let (dmg, _) = num_of(d, "damage");
    if !has_x || dmg <= 0.0 {
        return (Effect::none(), false);
    }
    let n = (dmg - 1e-9).ceil() as i64;
    let mut e = Effect::new(Kind::Damage, x, BODY_PX + 40.0);
    e.text = n.to_string();
    e.target = str_of(d, "target") == "player";
    (e, true)
}

/// `coin_loss_for` ist der Münzschauer eines Treffers an einem Monarchen (Schaden > 0); andere Ziele: ok false.
pub fn coin_loss_for(d: &Event) -> (Effect, bool) {
    let (x, has_x) = num_of(d, "x");
    let (dmg, _) = num_of(d, "damage");
    if !has_x || dmg <= 0.0 || str_of(d, "target") != "player" {
        return (Effect::none(), false);
    }
    (Effect::new(Kind::CoinLoss, x, BODY_PX), true)
}

/// `num_of` liest eine Zahl aus einem Ereignis (ganze Zahl oder `f64`); ok false, wenn sie fehlt.
pub fn num_of(d: &Event, key: &str) -> (f64, bool) {
    match d.get(key) {
        Some(serde_json::Value::Number(n)) => match n.as_f64() {
            Some(v) => (v, true),
            None => (0.0, false),
        },
        _ => (0.0, false),
    }
}

/// `str_of` liest einen Text aus einem Ereignis ("" ohne Text).
pub fn str_of(d: &Event, key: &str) -> String {
    match d.get(key) {
        Some(serde_json::Value::String(s)) => s.clone(),
        _ => String::new(),
    }
}
