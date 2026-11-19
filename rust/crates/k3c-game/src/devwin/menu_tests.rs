//! Jeder Eintrag des F9-Menüs (alle Gruppen) löst über das Fenster dieselbe `DevAction` aus (B-1134, AC-01, AC-02, AC-04).

use k3c_devproto::{DevCommand, Msg};

use super::commands::handle;
use super::mock::MockHost;
use crate::devtools::cheatlist::cheat_groups;
use crate::view::devkeys::DevAction;

fn command(a: &DevAction) -> DevCommand {
    DevCommand {
        action: a.action.clone(),
        amount: a.amount,
        resource: a.resource.clone(),
        factor: a.factor,
        phase: a.phase.clone(),
        grade: a.grade.clone(),
        on: a.on,
        level: a.level,
        boss: a.boss.clone(),
        event: a.event.clone(),
        kind: a.kind.clone(),
        unit: a.unit.clone(),
    }
}

#[test]
fn alle_f9_eintraege_erreichen_die_session_unveraendert() {
    let mut n = 0;
    for entry in cheat_groups().iter().flat_map(|g| g.entries.iter()) {
        let mut h = MockHost::new(1);
        // Schalter tragen `on` erst beim Auslösen
        let mut act = entry.act.clone();
        if !entry.toggle.is_empty() {
            act.on = Some(true);
        }
        let ack = handle(
            &mut h,
            Msg::Dev {
                id: 1,
                player: 0,
                action: command(&act),
            },
        );
        assert!(
            matches!(ack, Some(Msg::Ack { ok: true, .. })),
            "{}: {ack:?}",
            entry.label
        );
        assert_eq!(h.log(), [(0, act)], "{}", entry.label);
        n += 1;
    }
    assert!(n >= 30, "nur {n} Einträge geprüft");
}
