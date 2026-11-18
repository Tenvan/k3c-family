//! Go: `events_budget_test.go` (F4/AC-04, B-140/AC-04): Das Mittel der Ereignis-Bytes je Stufe und Tick bleibt im Budget.
//! Der Lauf (1800 Ticks, drei Stufen, vier Spieler) gehört zu den Messungen; er läuft im Abnahme-Ticket des Sprints.

use crate::island::testutil::{ISLAND_SPEED, test_data};
use crate::island::{CreateOptions, add_island_player, create_island, step_island};
use crate::state::PlayerCommand;
use crate::world::testutil::DT;

/// Go: `eventBudgetBytes`: das Budget der Feedback-Ereignisse je Tick und Client im Mittel (Beschluss Q08): 200 Byte × 30
/// Ticks/s = 6 KB/s je Client.
const EVENT_BUDGET_BYTES: f64 = 200.0;

#[test]
fn ereignis_budget_je_tick() {
    let data = test_data();
    let world = crate::island::testutil::standard_world(&data);
    let mut isl = create_island(
        &data,
        &world,
        "bench",
        &[0, 1, 2],
        ISLAND_SPEED,
        CreateOptions::default(),
    )
    .expect("Insel");
    for st in [0, 1, 2, 0] {
        add_island_player(&mut isl, st);
    }
    let cmds = [
        PlayerCommand {
            move_x: 1.0,
            ..Default::default()
        },
        PlayerCommand {
            move_x: -1.0,
            ..Default::default()
        },
        PlayerCommand {
            pay: true,
            ..Default::default()
        },
        PlayerCommand::default(),
    ];
    let mut tm = 0.0;
    while tm < 30.0 {
        step_island(&mut isl, &cmds, &data.difficulty, DT);
        tm += DT;
    }
    let (mut bytes, mut ticks, mut nights) = (0usize, 0usize, 0);
    for _ in 0..1800 {
        step_island(&mut isl, &cmds, &data.difficulty, DT);
        for w in &isl.stages {
            bytes += serde_json::to_string(&w.events).expect("JSON").len();
            ticks += 1;
        }
        if isl.stages[0].cycle.phase == "night" {
            nights += 1;
        }
    }
    assert!(nights > 0, "der Lauf erreicht keine Nacht");
    let mean = bytes as f64 / ticks as f64;
    assert!(
        mean <= EVENT_BUDGET_BYTES,
        "Ereignis-Budget überschritten: Mittel {mean:.1} Byte je Tick"
    );
}
