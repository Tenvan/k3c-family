//! Tests der Spielstände der Session (Go: `saves_test.go`, RU50.1): Liste, Laden, Löschen, Weiter, kaputte und
//! unbekannte Stände, nicht beschreibbarer Ordner, derselbe Name nach dem Löschen; dazu ein Spielstand im Go-Format.

use super::session::Session;
use super::testutil::{TWO_DEVICES, TempDir, has_cue, step_n, test_session, unwritable_session};
use crate::game::sim::Sim as _;
use crate::view::frame::CueKind;
use crate::view::input::DeviceID;
use crate::view::session::{NewGame, Session as _};

fn game(name: &str, devices: &[DeviceID]) -> NewGame {
    NewGame {
        name: name.into(),
        devices: devices.to_vec(),
        ..NewGame::default()
    }
}

/// Der Go-Spielstand `testdata/saves/v9/familie.json` (wird nur kopiert, nie geändert).
fn go_save() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../testdata/saves/v9/familie.json")
}

/// Legt das Spiel spiel-1 an, taktet 60 Mal, speichert und schließt es.
fn saved_game(s: &mut Session) {
    s.new_game(NewGame {
        devices: TWO_DEVICES.to_vec(),
        ..NewGame::default()
    })
    .expect("neues Spiel");
    assert_eq!(s.name, "spiel-1", "freier Name");
    step_n(s, 60);
    s.save().expect("speichern");
    assert!(has_cue(&s.step(), CueKind::Saved), "kein Saved nach save");
    s.close().expect("beenden");
    assert!(!s.active());
}

/// Legt das Spiel „alt“ an, taktet 5 Mal und schließt es.
fn old_game(s: &mut Session) {
    s.new_game(game("alt", &TWO_DEVICES[..1])).expect("neues Spiel");
    step_n(s, 5);
    s.close().expect("beenden");
}

fn read(dir: &TempDir, name: &str) -> Vec<u8> {
    std::fs::read(dir.path().join(format!("{name}.json"))).expect("Spielstand lesen")
}

// AC-02 (Go: TestSaveListLoadDelete): Speichern, Liste, Laden und Löschen gegen einen Ordner. Speichern ohne Takt nach dem
// Laden schreibt denselben Spielstand (Bytes der Datei); der Welt-Vergleich mit dem Raum direkt steht in
// `load_determinism_like_room`.
#[test]
fn save_list_load_delete() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    saved_game(&mut s);
    let want = read(&dir, "spiel-1");
    let list = s.list().expect("Liste");
    assert!(
        list.len() == 1 && list[0].name == "spiel-1" && list[0].loadable && list[0].saved_at.is_some(),
        "Liste {list:?}"
    );
    s.load("spiel-1", &[]).expect("laden");
    s.save().expect("speichern");
    assert_eq!(read(&dir, "spiel-1"), want, "Spielstand nach Laden und Speichern");
    assert_eq!(s.step().seats.len(), 2, "nach Laden die 2 Geräte des letzten Spiels");
    check_delete(&mut s);
}

// Der Stand des laufenden Spiels bleibt, nach dem Schließen ist er löschbar und danach weg.
fn check_delete(s: &mut Session) {
    assert!(s.delete("spiel-1").is_err(), "Spielstand des laufenden Spiels gelöscht");
    s.close().expect("beenden");
    s.delete("spiel-1").expect("löschen");
    assert!(s.list().expect("Liste").is_empty(), "Liste nach Löschen");
    let err = s.saves_delete("spiel-1").expect_err("fehlender Stand");
    assert_eq!(err.msg, "Spielstand „spiel-1“ gibt es nicht");
    assert!(err.cause.is_some(), "Ursache des Stores bleibt");
    assert!(s.load("spiel-1", &[]).is_err(), "gelöschter Stand geladen");
}

// Ungültige Namen lehnt das Löschen ab, ohne Pfade außerhalb des Ordners zu berühren; ein gelöschter Stand verschwindet aus
// der Liste (er liegt in den Sicherungen, `Saves::discard`).
#[test]
fn delete_rejects_invalid_names() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    for bad in ["../x", "Groß", "", "a/b"] {
        assert!(s.delete(bad).is_err(), "Name {bad:?} angenommen");
    }
    old_game(&mut s);
    s.delete("alt").expect("löschen");
    assert!(!dir.path().join("alt.json").exists());
    assert!(s.list().expect("Liste").is_empty());
}

// Einen Namen gibt es nur einmal; ungültige Namen lehnt `new_game` ab.
#[test]
fn new_duplicate_and_invalid() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    old_game(&mut s);
    assert!(
        s.new_game(game("alt", &TWO_DEVICES[..1])).is_err(),
        "zweites Spiel mit demselben Namen"
    );
    assert!(
        s.new_game(game("Ungültig!", &TWO_DEVICES[..1])).is_err(),
        "ungültiger Name angenommen"
    );
}

// Weiter lädt den zuletzt gespeicherten ladbaren Stand, ohne Geräte mit denen des letzten Spiels.
#[test]
fn continue_game() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    assert!(s.continue_game(&[]).is_err(), "Weiter ohne Spielstand");
    old_game(&mut s);
    s.continue_game(&[]).expect("weiter");
    assert_eq!(s.name, "alt");
    let f = s.step();
    assert!(
        f.seats.len() == 1 && f.seats[0].device == TWO_DEVICES[0],
        "Weiter ohne Geräte: {:?}",
        f.seats
    );
    // Mit Geräten gelten diese, nicht die des letzten Spiels (Vertrag NG1.4).
    s.continue_game(&TWO_DEVICES[1..]).expect("weiter mit Pad");
    let f = s.step();
    assert!(
        f.seats.len() == 1 && f.seats[0].device == TWO_DEVICES[1],
        "Weiter mit Pad: {:?}",
        f.seats
    );
    s.load("alt", &TWO_DEVICES[..1]).expect("laden");
    let f = s.step();
    assert!(
        f.seats.len() == 1 && f.seats[0].device == TWO_DEVICES[0],
        "Laden mit Tastatur: {:?}",
        f.seats
    );
}

// Kaputter Stand und unbekannte Version stehen als nicht ladbar in der Liste; Laden meldet einen Fehler, kein Absturz.
#[test]
fn broken_saves() {
    let dir = TempDir::new();
    std::fs::write(dir.path().join("kaputt.json"), "{nicht json").expect("schreiben");
    let future = r#"{"campaignId":"x","version":99,"savedAt":"2026-01-01T00:00:00.000Z"}"#;
    std::fs::write(dir.path().join("zukunft.json"), future).expect("schreiben");
    let mut s = test_session(&dir);
    let list = s.list().expect("Liste");
    assert_eq!(list.len(), 2, "Liste {list:?}");
    for e in &list {
        assert!(!e.loadable && !e.problem.is_empty(), "{} ladbar: {e:?}", e.name);
        assert!(s.load(&e.name, &[]).is_err() && !s.active(), "{} geladen", e.name);
    }
    assert!(s.continue_game(&[]).is_err(), "Weiter mit nur kaputten Ständen");
}

// Ist der Ordner nicht beschreibbar, meldet Speichern SaveFailed und das Spiel läuft weiter.
#[test]
fn save_dir_not_writable() {
    let dir = TempDir::new();
    let mut s = unwritable_session(&dir);
    s.new_game(game("fest", &TWO_DEVICES[..1])).expect("neues Spiel");
    step_n(&mut s, 3);
    assert!(s.save().is_err(), "save ohne Fehler");
    let f = step_n(&mut s, 1);
    assert!(has_cue(&f, CueKind::SaveFailed) && s.active(), "Cues {:?}", f.cues);
    assert_eq!(step_n(&mut s, 1).tick, f.tick + 1, "Spiel steht");
}

// B-916: Der Stresstest-Ablauf (laufendes Spiel schließen, Stand löschen, mit demselben Namen neu anlegen) geht, der
// andere Spielstand bleibt in der Liste.
#[test]
fn new_after_close_and_discard_same_name() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    old_game(&mut s);
    s.new_game(game("stresstest", &TWO_DEVICES[..1])).expect("neues Spiel");
    s.close().expect("beenden");
    s.delete("stresstest").expect("löschen");
    s.new_game(game("stresstest", &TWO_DEVICES[..1]))
        .expect("neu mit demselben Namen");
    let list = s.list().expect("Liste");
    assert!(list.iter().any(|e| e.name == "alt"), "Spielstand alt fehlt: {list:?}");
}

// AC-02: Ein Spielstand im Format des Go-Spiels (`testdata/saves/v9`, unverändert kopiert) steht ladbar in der Liste und
// lässt sich laden; sein Name ist der Dateiname, seine Welt die gespeicherte.
#[test]
fn go_format_save_loads() {
    let dir = TempDir::new();
    std::fs::copy(go_save(), dir.path().join("familie.json")).expect("kopieren");
    let mut s = test_session(&dir);
    let list = s.list().expect("Liste");
    assert!(
        list.len() == 1 && list[0].name == "familie" && list[0].loadable && list[0].world == "standard",
        "Liste {list:?}"
    );
    assert!(list[0].saved_at.is_some() && list[0].day == 1, "Kopf {:?}", list[0]);
    s.continue_game(&[]).expect("weiter");
    assert!(s.active() && s.name == "familie");
    assert_eq!(s.world(), "standard");
    assert!(!s.step().stages.is_empty(), "Stufen aus dem Go-Spielstand");
}

// Die Liste ist nach Zeit absteigend sortiert, nicht nach Name.
#[test]
fn list_newest_first() {
    let dir = TempDir::new();
    let raw = std::fs::read_to_string(go_save()).expect("lesen");
    let newer = raw.replacen("2026-10-13T20:00:00Z", "2026-11-01T08:30:00Z", 1);
    std::fs::write(dir.path().join("a-alt.json"), raw).expect("schreiben");
    std::fs::write(dir.path().join("z-neu.json"), newer).expect("schreiben");
    let mut s = test_session(&dir);
    let names: Vec<String> = s.list().expect("Liste").into_iter().map(|e| e.name).collect();
    assert_eq!(names, ["z-neu", "a-alt"]);
}
