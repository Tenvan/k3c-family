//! Tests aus `dev_grade_test.go` (B-080/AC-02, K4.2): Die Dev-Aktion `grade` wechselt den Grad des Raums; er gilt ab der
//! nächsten Welle, nicht für die laufende.
use super::DevAction;
use super::dev_tests::{act, act_slot, dev, dev_room};
use super::testutil::rm;
use crate::fehler::Fehler;

fn grade(g: &str) -> DevAction {
    DevAction {
        grade: g.into(),
        ..act("grade")
    }
}

#[test]
fn dev_grade_ab_naechster_welle() {
    let (mut m, code, _i) = dev_room();
    assert_eq!(dev(&mut m, &code, &act_slot("wave", 0, 0)), Ok(()));
    let n1 = rm(&mut m, &code).isl.stages[0].spawn_queue.len();
    assert_eq!(dev(&mut m, &code, &grade("ultra")), Ok(()));
    let r = rm(&mut m, &code);
    assert!(
        r.isl.options.grade == "ultra" && r.isl.stages[0].spawn_queue.len() == n1,
        "Grad {:?}, Warteschlange {} statt {n1}",
        r.isl.options.grade,
        r.isl.stages[0].spawn_queue.len()
    );
    assert_eq!(dev(&mut m, &code, &act_slot("wave", 0, 0)), Ok(()));
    let n2 = rm(&mut m, &code).isl.stages[0].spawn_queue.len();
    assert!(n2 > n1, "neue Welle nicht größer: {n2} nach {n1}");
}

#[test]
fn dev_grade_ungueltig_oder_verboten() {
    let (mut m, code, _i) = dev_room();
    assert_eq!(
        dev(&mut m, &code, &grade("gibtsnicht")),
        Err(Fehler::BadRequest),
        "unbekannter Grad"
    );
    m.dev = false;
    assert_eq!(
        dev(&mut m, &code, &grade("hard")),
        Err(Fehler::Forbidden),
        "ohne Dev-Mode"
    );
    assert_ne!(rm(&mut m, &code).isl.options.grade, "hard", "Grad geändert");
}
