//! `k3c-game sim save` (RU40.2): legt den Spielstand „voll ausgebaut“ an, den „Im Spiel ansehen“ der Workbench braucht.
//! Schreibt `voll-<seed>` in den Ordner der Eingabe wie `room.Manager.FullSave` im Dev-Modus (`k3c-room`, `full_save`);
//! die Ausgabe ist die Zeile `result` mit dem Namen. Der Seed muss `^[a-z0-9-]{1,27}$` entsprechen, sonst Exit 2.

use std::sync::Arc;

use k3c_data::{Source, load_game};
use k3c_room::fehler::Fehler;
use k3c_room::room::{Manager, NullLog};
use k3c_room::store::{Saves, Stamp};

use super::RunError;
use super::input_world::SaveInput;

/// Gos `fullSeed`: 1 bis 27 Zeichen aus `a-z`, `0-9` und `-`.
pub fn valid_seed(seed: &str) -> bool {
    (1..=27).contains(&seed.len())
        && seed
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

fn wall_clock() -> Stamp {
    let d = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    Stamp::new(i64::try_from(d.as_secs()).unwrap_or(i64::MAX), d.subsec_nanos())
}

/// Legt `voll-<seed>` im Ordner `saves` an und liefert den Namen; `now` bestimmt `savedAt`.
pub fn run_with(
    src: &Source,
    input: &SaveInput,
    now: impl Fn() -> Stamp + Clone + 'static,
) -> Result<String, RunError> {
    if !valid_seed(&input.seed) {
        return Err(RunError::Usage(format!(
            "seed muss ^[a-z0-9-]{{1,27}}$ entsprechen, war {:?}",
            input.seed
        )));
    }
    if input.saves.is_empty() {
        return Err(RunError::Usage("saves (Ordner der Spielstände) fehlt".into()));
    }
    let data = load_game(src).map_err(|e| RunError::Failed(format!("Spieldaten nicht lesbar: {e}")))?;
    let clock = now.clone();
    let store = Saves::new(&input.saves, clock);
    let mut m = Manager::new(store, NullLog, now, "save", Arc::new(data));
    m.dev = true;
    m.full_save(&input.seed).map_err(|e| match e {
        Fehler::BadRequest => RunError::Usage(format!("seed ungültig: {:?}", input.seed)),
        other => RunError::Failed(format!("Spielstand nicht angelegt: {other}")),
    })
}

/// Einstieg von `sim save` mit der Wanduhr.
pub fn run(src: &Source, input: &SaveInput) -> Result<String, RunError> {
    run_with(src, input, wall_clock)
}
