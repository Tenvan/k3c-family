//! Tests zu `archer.rs` (Go hat dafür keine eigene Testdatei).

use super::archer::is_on_tower;
use crate::enemies::testutil::{bare_world, built_site};
use crate::state::Troop;

#[test]
fn is_on_tower_nur_am_eigenen_turm() {
    let mut w = bare_world();
    let s = built_site(&mut w, "tower", 40.0);
    let id = w.sites[s].id;
    let mut t = Troop {
        x: 40.1,
        ..Troop::default()
    };
    assert!(!is_on_tower(&w, &t), "ohne Turm-ID");
    t.tower_id = Some(id);
    assert!(is_on_tower(&w, &t));
    t.x = 40.5;
    assert!(!is_on_tower(&w, &t), "zu weit vom Turm");
    t.tower_id = Some(id + 100);
    t.x = 40.0;
    assert!(!is_on_tower(&w, &t), "Turm gibt es nicht");
}
