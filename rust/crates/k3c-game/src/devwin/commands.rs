//! Befehle des Fensters nach Bereich weiterleiten und mit `ack` beantworten (RU56.5, B-1133). Jeder Befehl wird
//! beantwortet: `ok:true` oder `ok:false` mit deutschem Grund; nichts wird an der Sim vorbei erzwungen.

use k3c_devproto::{DevCommand, Msg};

use super::{Host, eco, foes, players, view, world};
use crate::view::devkeys::DevAction;

/// Outcome ist das Ergebnis eines Bereichs: `None` heißt "nicht mein Befehl".
pub type Outcome = Option<Result<(), String>>;

impl From<DevCommand> for DevAction {
    fn from(c: DevCommand) -> Self {
        DevAction {
            action: c.action,
            amount: c.amount,
            resource: c.resource,
            factor: c.factor,
            phase: c.phase,
            grade: c.grade,
            on: c.on,
            level: c.level,
            boss: c.boss,
            event: c.event,
            kind: c.kind,
            unit: c.unit,
        }
    }
}

/// handle führt eine Nachricht des Fensters aus und liefert das `ack` (andere Nachrichten: `None`).
pub fn handle(host: &mut dyn Host, msg: Msg) -> Option<Msg> {
    let (id, res) = match msg {
        Msg::Dev { id, player, action } => (id, dev(host, player, &action)),
        Msg::Ui { id, key, value } => (id, ui(host, &key, value)),
        Msg::Goto {
            id,
            world,
            island,
            stage,
        } => (id, host.goto(&world, island, stage)),
        Msg::Focus { id } => (id, host.focus()),
        _ => return None,
    };
    Some(match res {
        Ok(()) => Msg::Ack {
            id,
            ok: true,
            err: None,
        },
        Err(e) => Msg::Ack {
            id,
            ok: false,
            err: Some(e),
        },
    })
}

fn dev(host: &mut dyn Host, player: i64, c: &DevCommand) -> Result<(), String> {
    let n = host.player_count();
    if player < 0 || player >= n {
        return Err(format!("Spieler {} gibt es nicht (Spieler: {n})", player + 1));
    }
    world::dev(host, player, c)
        .or_else(|| eco::dev(host, player, c))
        .or_else(|| players::dev(host, player, c))
        .or_else(|| foes::dev(host, player, c))
        // Kein Bereich prüft die Aktion: die Session entscheidet (unbekannte Aktionen lehnt der Raum ab).
        .unwrap_or_else(|| run_dev(host, player, c))
}

fn ui(host: &mut dyn Host, key: &str, value: Option<i64>) -> Result<(), String> {
    world::ui(host, key, value)
        .or_else(|| view::ui(host, key, value))
        .unwrap_or_else(|| Err(format!("Unbekannte Taste „{key}“")))
}

/// run_dev führt die Aktion über die Dev-Rolle der Session aus (derselbe Pfad wie das F9-Menü).
pub fn run_dev(host: &mut dyn Host, player: i64, c: &DevCommand) -> Result<(), String> {
    let a = DevAction::from(c.clone());
    match host.dev_session() {
        Some(ds) => ds.dev(player, &a),
        None => Err("Keine Entwickler-Rolle (kein laufendes Spiel)".to_string()),
    }
}
