//! Bosskampf (Port von `engine/balance/bot_boss.go`, BS11.3, B-642): Im Insel-Lauf schlagen die Bots jeden Gegner in
//! Reichweite und gehen auf den Boss los. Der Miniboss kommt mit seiner Welle zum Hub (die Truppen und Monarchen dort
//! kämpfen); den Endboss weckt nur, wer sich dem Bau der tiefsten Stufe nähert (boss_endboss): Am hellen Tag geht ein
//! gesunder Monarch zum Bau und schlägt auf den Boss ein. Fällt sein Gold unter `RETREAT_GOLD` (der Geldsack ist die
//! Lebensanzeige, B-930), zieht er sich zur Burg zurück. Ist der Endboss besiegt (`Island::endboss_defeated`), bleibt er
//! bei seinem Profil.

use k3c_sim::island::Island;
use k3c_sim::state::{Enemy, Player, PlayerCommand, World};

use crate::bots::{Bot, economy_view, go_to};

/// Darunter zieht sich der Bot zur Burg zurück.
const RETREAT_GOLD: i64 = 3;
/// Ab so viel Gold geht er zum Bau des Endbosses.
const FIGHT_GOLD: i64 = 10;
/// So nah am Hub gilt ein Boss als Verteidigungsfall, weiter weg geht der Bot nur mit Heer hin.
const BOSS_HUB_RANGE: f64 = 40.0;
/// So viele Kämpfer braucht er, bevor er den Endboss am Bau weckt (B-651: 8 Bögen starben in Minuten an dessen
/// Beschwörung).
pub const ENDBOSS_WANT: i64 = 16;
/// Etwas unter monarch.json › attack.range (1,5): der Schlag trifft sicher.
const STRIKE_RANGE: f64 = 1.4;

/// Go: `withBossFight`: der Boss-Dekorator um `inner`; `deepest` ist die tiefste Tiefe der Insel.
pub fn boss_decide(inner: &Bot, deepest: i64, isl: &Island, w: &World, p: &Player) -> PlayerCommand {
    let mut cmd = inner.decide(isl, w, p);
    if p.gold < RETREAT_GOLD {
        cmd = go_to(p, w.hub_x, false);
    } else if let Some(x) = fight_x(w, p, deepest, isl.endboss_defeated) {
        cmd = go_to(p, x, false);
    }
    cmd.attack = enemy_within(w, p.x, STRIKE_RANGE);
    cmd
}

/// Der Ort, zu dem der Bot zum Kampf läuft: ein lebender Boss der Stufe, sonst der Bau des Endbosses der tiefsten Stufe,
/// solange der Endboss nicht besiegt ist und es hell ist.
pub fn fight_x(w: &World, p: &Player, deepest: i64, won: bool) -> Option<f64> {
    // Ein Boss weit vom Hub (Endboss am Bau, B-645): allein dorthin zu laufen kostet das Leben und das Gold
    // (Kristallhöhle, Tag 34: der Monarch starb im Minutentakt am Bau). Ohne Heer verteidigt er den Hub; der Boss kommt
    // mit der Welle zu ihm.
    if let Some(boss) = nearest_boss(w, p)
        && w.biome.depth == deepest
        && ((boss.x - w.hub_x).abs() <= BOSS_HUB_RANGE || economy_view(w).fighters >= ENDBOSS_WANT)
    {
        return Some(boss.x - 0.3f64.copysign(boss.x - p.x));
    }
    // Erst die Armee: Seit dem Begleitschutz (unit_escort) kämpfen die Truppen am Boss mit, also baut der Bot sie vorher
    // auf.
    if w.biome.depth != deepest
        || won
        || w.cycle.phase != "day"
        || p.gold < FIGHT_GOLD
        || economy_view(w).fighters < ENDBOSS_WANT
    {
        return None;
    }
    w.level.entities.iter().find(|e| e.kind == "lair").map(|e| e.x)
}

/// Der nächste lebende Boss der Stufe (`None`: keiner); bei gleichem Abstand gilt der frühere.
pub fn nearest_boss<'a>(w: &'a World, p: &Player) -> Option<&'a Enemy> {
    let mut boss: Option<&Enemy> = None;
    for e in &w.enemies {
        if e.boss && e.hp > 0.0 && boss.is_none_or(|b| (e.x - p.x).abs() < (b.x - p.x).abs()) {
            boss = Some(e);
        }
    }
    boss
}

/// Ein lebender Gegner liegt höchstens `dist` von `x` entfernt.
pub fn enemy_within(w: &World, x: f64, dist: f64) -> bool {
    w.enemies.iter().any(|e| e.hp > 0.0 && (e.x - x).abs() <= dist)
}
