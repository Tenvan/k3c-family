use std::collections::BTreeSet;

use super::*;

#[test]
fn next_timescale_and_debug() {
    for (cur, next) in [(1, 2), (2, 4), (4, 8), (8, 1), (3, 2)] {
        assert_eq!(next_timescale(cur), next, "NextTimescale({cur})");
    }
    assert!(
        next_debug(DEBUG_OFF) == DEBUG_GEOMETRY
            && next_debug(DEBUG_GEOMETRY) == DEBUG_VALUES
            && next_debug(DEBUG_VALUES) == DEBUG_OFF,
        "NextDebug schaltet nicht aus → Geometrie → Werte → aus"
    );
}

/// Jeder Befehl hat genau eine Taste, nur F-Tasten (keine Spieltasten), F für Vollbild entfällt.
#[test]
fn dev_keys_only_function_keys() {
    let f_keys = [
        Key::F1,
        Key::F2,
        Key::F3,
        Key::F5,
        Key::F6,
        Key::F7,
        Key::F8,
        Key::F9,
        Key::F10,
        Key::F11,
        Key::F12,
    ];
    let mut cmds = BTreeSet::new();
    for k in DEV_KEYS {
        assert!(
            f_keys.contains(&k.key) && k.cmd != DevCmd::None && cmds.insert(k.cmd),
            "{k:?}"
        );
    }
    assert_eq!(dev_key_of(DevCmd::Fullscreen), Some(Key::F11), "DevKeyOf");
    assert_eq!(dev_key_of(DevCmd::None), None, "DevKeyOf");
}
