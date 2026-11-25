//! Hilfen für die Tests des Crates: kleine Welten ohne Lauf.

use k3c_data::{Data, Source, load_all};
use k3c_level::rules::Rules;
use k3c_sim::island::{
    CreateOptions, Island, add_island_player, create_island, create_island_in, load_world, standard_world,
};
use k3c_sim::state::{CycleInfo, Player, Site, Troop, World};

/// Die eingebetteten Spiel-Daten (Go: die Paket-Variablen).
fn data() -> Data {
    load_all(&Source::Embedded).expect("eingebettete Daten")
}

/// Die Regeln des Level-Generators aus den eingebetteten Daten.
fn rules(data: &Data) -> Rules {
    let hub = Source::Embedded.read("hub.json").expect("hub.json");
    Rules::from_data(data, &hub).expect("Regeln")
}

/// Go: `sim.CreateIsland(seed, depths, 1, CreateOptions{})`: die erste Insel der Standard-Welt, ohne Spieler.
pub fn create(seed: &str, depths: &[i64]) -> Island {
    let data = data();
    let world = standard_world(&data, &rules(&data)).expect("Standard-Welt");
    create_island(&data, &world, seed, depths, 1.0, CreateOptions::default()).expect("Insel")
}

/// Go: `newIsland(Scenario{Seed, Players, World: "standard"})`: die erste Insel der Welt `standard` mit ihren Monarchen auf
/// Stufe 0 (Lauf und Wiedergabe; ohne Grad und Hilfen).
pub fn scenario_island(seed: &str, players: usize) -> Island {
    let data = data();
    let world = load_world(&Source::Embedded, "standard", &data, &rules(&data)).expect("Welt standard");
    let first = world.islands.first().expect("erste Insel");
    let opts = first.create_options(&data.biomes);
    let mut isl = create_island_in(&data, &world, seed, &first.depths, 1.0, opts).expect("Insel");
    for _ in 0..players {
        add_island_player(&mut isl, 0);
    }
    isl
}

/// Eine leere Stufe am Tag 3 mit der Burg bei x = 100.
pub fn world() -> World {
    World {
        hub_x: 100.0,
        cycle: CycleInfo {
            phase: "day".into(),
            day: 3,
            ..CycleInfo::default()
        },
        ..World::default()
    }
}

/// Eine leere Insel.
pub fn island() -> Island {
    Island::default()
}

/// Ein Spieler mit Index, Ort und Gold.
pub fn player(index: i64, x: f64, gold: i64) -> Player {
    Player {
        index,
        x,
        gold,
        ..Player::default()
    }
}

/// Ein Bauplatz.
#[allow(dead_code)]
pub fn site(kind: &str, x: f64, state: &str) -> Site {
    Site {
        kind: kind.into(),
        x,
        state: state.into(),
        ..Site::default()
    }
}

/// Ein Landstreicher.
pub fn vagrant(x: f64) -> Troop {
    Troop {
        kind: "citizen".into(),
        wild: true,
        x,
        ..Troop::default()
    }
}
