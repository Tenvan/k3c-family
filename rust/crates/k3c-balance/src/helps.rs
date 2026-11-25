//! Hilfen (Port von `engine/balance/helps.go`, BS3.4): Der Stufen-Test fullgame_test und die Balancing-Seite spielen
//! dieselbe Insel mit demselben Bot; was der Test an Dev-Hilfen einschaltet, kann die Seite im Feld „Hilfen“ ebenfalls
//! wählen (vorher unterschieden sich beide genau darin). Leer = keine Hilfen (Grad normal, nichts geschenkt: der offene
//! Meilenstein der Balance). Hilfen gelten nur mit World.
//!
//! Go reicht `Scenario` durch; `Scenario` kommt mit RU25.4 (`run.rs`), die Funktionen nehmen hier die Felder `grade`,
//! `helps` und `world` des Szenarios einzeln.

use k3c_data::difficulty::DifficultyFile;
use k3c_sim::dev::{dev_build_all, dev_fill_stock, dev_set_cheap_build};
use k3c_sim::island::Island;

use crate::bot_learn::bot_learn;
use crate::fehler::Fehler;

/// Grad dev (falls keiner gesetzt), Dev-Baukosten, voller Insel-Vorrat.
pub const HELPS_AIDS: &str = "hilfen";
/// Zusätzlich alle Stufen voll ausgebaut (Dev-Bau Stufe 5).
pub const HELPS_FULL: &str = "voll";
/// Zusätzlich schwacher Endboss (300 HP) und jeden Tick geheilte Monarchen (B-647 Weg D).
pub const HELPS_BOSS: &str = "boss";

/// Die gültigen Werte von `Scenario.helps` (ohne den leeren).
pub fn helps_names() -> Vec<&'static str> {
    vec![HELPS_AIDS, HELPS_FULL, HELPS_BOSS]
}

/// Prüft Wert und Zusammenhang mit der Welt (Go: `checkHelps`).
pub fn check_helps(helps: &str, world: &str) -> Result<(), Fehler> {
    match helps {
        "" => Ok(()),
        HELPS_AIDS | HELPS_FULL | HELPS_BOSS => {
            if world.is_empty() {
                return Err(Fehler::Hilfen(format!(
                    "hilfen {helps:?} brauchen eine Welt (ganze Insel)"
                )));
            }
            Ok(())
        }
        _ => Err(Fehler::Hilfen(format!(
            "unbekannte hilfen {helps:?} (bekannt: [{}])",
            helps_names().join(" ")
        ))),
    }
}

/// Der Grad des Laufs: gesetzt wie angegeben, mit Hilfen sonst `dev` (Go: `gradeOf`).
pub fn grade_of(grade: &str, helps: &str) -> String {
    if grade.is_empty() && !helps.is_empty() {
        return "dev".to_string();
    }
    grade.to_string()
}

/// Schaltet die Dev-Hilfen des Szenarios auf der frischen Insel ein (Go: `applyHelps`); `difficulty` ist die Tabelle,
/// die Go als Paket-Variable hält (`dev_fill_stock` braucht sie).
pub fn apply_helps(isl: &mut Island, difficulty: &DifficultyFile, helps: &str) {
    if helps.is_empty() {
        return;
    }
    dev_set_cheap_build(isl, true);
    dev_fill_stock(isl, difficulty);
    if helps == HELPS_FULL || helps == HELPS_BOSS {
        dev_build_all(isl, 5);
    }
}

/// Läuft nach jedem Tick einer Insel-Welt: einmal je Sekunde lernt der Bot Skills (wie ein Mensch im Menü, `bot_learn`),
/// bei Hilfen „boss“ wird der Endboss geschwächt und werden die Monarchen geheilt (Go: `afterStep`).
pub fn after_step(isl: &mut Island, world: &str, helps: &str, tick: i64, tick_hz: i64) {
    if world.is_empty() {
        return;
    }
    if tick % tick_hz == 0 {
        bot_learn(isl);
    }
    if helps == HELPS_BOSS {
        weaken_endboss(isl);
        heal_monarchs(isl);
    }
}

/// Setzt den erschienenen Endboss auf 300 HP: Ohne das trifft ihn nur der Monarch (10 Schaden), er steht am Bau
/// außerhalb der Reichweite der Truppen (B-647). Prüft so den Rest der Kette: Sieg, Belohnung, Insel-Zustand.
pub fn weaken_endboss(isl: &mut Island) {
    for w in &mut isl.stages {
        for e in &mut w.enemies {
            if e.boss && e.max_hp > 300.0 && !mini_boss_kind(&e.kind) {
                e.hp = 300.0;
                e.max_hp = 300.0;
            }
        }
    }
}

/// Miniboss-IDs bleiben unberührt (nur der Endboss wird abgeschwächt).
fn mini_boss_kind(kind: &str) -> bool {
    matches!(
        kind,
        "goblinLeader" | "trollKing" | "ratQueen" | "shardTitan" | "lavaGolem"
    )
}

/// Setzt gefallene Kronen wieder auf (Dev-Hilfe ohne Skills, B-647 Weg D; Monarchen haben keine HP, B-930).
pub fn heal_monarchs(isl: &mut Island) {
    for w in &mut isl.stages {
        for p in &mut w.players {
            // Trägt ein Gegner die Krone, bleibt sie dort.
            if p.crown.as_ref().is_some_and(|c| c.state != "enemy") {
                p.crown = None;
            }
        }
    }
}
