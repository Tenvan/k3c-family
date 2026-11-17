//! Port von `native/k3c-game/view/crown_test.go`.

use std::path::Path;

/// Die Kronen-Zustände der Engine (sim.CrownFlying, CrownGround, CrownEnemy, CrownPlayer); `k3c-sim` exportiert sie
/// erst mit RU8, bis dahin ist der Test ignoriert.
fn crown_states() -> [&'static str; 4] {
    unimplemented!("RU8: sim.CrownFlying, CrownGround, CrownEnemy, CrownPlayer")
}

// Die Kronen-Zustände stehen in docs/protocol.md (Zeile players[].crown); weicht die Engine ab, ändert sich die Doku mit.
#[test]
#[ignore = "wartet auf RU8"]
fn crown_states_match_protocol() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../docs/protocol.md");
    let Ok(raw) = std::fs::read_to_string(&path) else {
        eprintln!("Protokoll fehlt: {}", path.display());
        return;
    };
    let row = raw.split('\n').rfind(|l| l.starts_with("| `players[].crown`"));
    let row = row.expect("Zeile players[].crown fehlt im Protokoll");
    for s in crown_states() {
        assert!(
            row.contains(&format!("`{s}`")),
            "Zustand {s:?} steht nicht im Protokoll"
        );
    }
}
