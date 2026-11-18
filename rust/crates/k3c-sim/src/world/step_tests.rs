//! Tests ohne Go-Gegenstück zu `step` (AC-01): Die Reihenfolge der Teilschritte ist die von Go (`Step`, world.go). Go hat
//! keinen eigenen Test dafür; die Abdeckung liegt in `rules_test.go`, `sim_test.go` und den Golden (RU22). Der Test liest
//! `step.rs` und prüft, dass jeder Teilschritt in dieser Reihenfolge aufgerufen wird, auch wenn er noch ein Platzhalter ist.

/// Go: die Aufrufe in `Step`, Zeile für Zeile (snake_case).
const GO_ORDER: [&str; 26] = [
    "step_cycle",
    "spawn_guards",
    "step_spawns",
    "step_players",
    "step_crowns",
    "step_passives",
    "step_camps",
    "step_sites",
    "step_plantations",
    "step_combat",
    "step_troops",
    "step_portals",
    "step_healing",
    "step_spell_towers",
    "step_endboss",
    "step_enemies",
    "step_lava",
    "step_hazards",
    "step_storms",
    "step_projectiles",
    "remove_dead_enemies",
    "guarded_leader", // Bürger sterben nicht: Verlust-Kaskade
    "lose_layer",
    "castle_fallen",
    "step_travel",
    "cap_events",
];

#[test]
fn step_ruft_die_teilschritte_in_der_reihenfolge_von_go() {
    let src = include_str!("step.rs");
    let body = &src[src.find("pub fn step(").expect("step")..];
    let mut at = 0;
    for name in GO_ORDER.iter() {
        let Some(i) = body[at..].find(&format!("{name}(")) else {
            panic!("{name} fehlt oder steht nicht an seiner Stelle in step.rs");
        };
        at += i + name.len();
    }
}

#[test]
fn step_kennt_keinen_teilschritt_zweimal() {
    let src = include_str!("step.rs");
    let body = &src[src.find("pub fn step(").expect("step")..];
    for name in GO_ORDER.iter() {
        let calls = body.matches(&format!("{name}(")).count();
        assert_eq!(calls, 1, "{name} wird {calls}-mal gerufen");
    }
}
