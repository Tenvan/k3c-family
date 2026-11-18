//! Hilfen der Tests dieser Gruppe (Go: `strikeCmd`, `mustLearn` aus `monarch_test.go`, `eventsOf` aus `events_test.go`;
//! `must_learn`, `add_pool_points` und `add_island_pool` stehen in `island/testutil.rs`).

use crate::enemies::testutil::DT;
use crate::state::{Event, PlayerCommand, World};
use crate::world::step;

/// Go: `strikeCmd`: ein Schlag (Taste X).
pub fn strike_cmd() -> PlayerCommand {
    PlayerCommand {
        attack: true,
        ..PlayerCommand::default()
    }
}

/// Go: `eventsOf`: tickt `steps` Mal mit `commands` und liefert alle Ereignisse der Art `typ`.
pub fn events_of(w: &mut World, typ: &str, steps: usize, commands: &[PlayerCommand]) -> Vec<Event> {
    let mut found = Vec::new();
    for _ in 0..steps {
        step(w, commands, None, DT);
        found.extend(
            w.events
                .iter()
                .filter(|e| e.get("type").and_then(|t| t.as_str()) == Some(typ))
                .cloned(),
        );
    }
    found
}
