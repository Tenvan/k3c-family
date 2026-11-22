//! Tests des Spieltest-Modus (B-1220, AC-01): Flag, Marker-Zeile, Drosselung, 2 Spieler, Fehlerzustand, ohne Flag nichts.

use std::path::{Path, PathBuf};
use std::rc::Rc;

use k3c_platform::{InputState, Key, PadButton};
use k3c_sim::state::World;

use super::flag::extract;
use super::marker::{FrameTimes, Marker, RateLimit};
use super::sink::FILE;
use super::{FLASH_TICKS, Playtest};
use crate::view::frame::{Frame, RouteIsland, Seat, Stage};
use crate::view::input::{DeviceID, DeviceKind};

pub(super) fn strs(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| (*s).to_string()).collect()
}

/// Eigener Ordner je Test unter `.work/` im Repo (kein Temp), vorher leer.
pub(super) fn dir(name: &str) -> PathBuf {
    let d = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../.work/playtest-tests")
        .join(name);
    let _ = std::fs::remove_dir_all(&d);
    d
}

pub(super) fn lines(d: &Path) -> Vec<serde_json::Value> {
    std::fs::read_to_string(d.join(FILE))
        .unwrap_or_default()
        .lines()
        .map(|l| serde_json::from_str(l).expect("gültiges JSON"))
        .collect()
}

/// Spiel mit zwei Spielern: Spieler 1 auf der Tastatur, Spieler 2 auf Pad 7; Spieler 2 steht in Stufe 2 in der Nacht.
#[allow(clippy::field_reassign_with_default)]
pub(super) fn frame() -> Frame {
    let mut w = World::default();
    w.cycle.phase = "night".to_string();
    w.cycle.day = 4;
    let mut f = Frame::default();
    f.seats = vec![
        Seat {
            player: 0,
            device: DeviceID::new(DeviceKind::Keyboard, 0),
            stage: 0,
            ..Seat::default()
        },
        Seat {
            player: 1,
            device: DeviceID::new(DeviceKind::Gamepad, 7),
            stage: 2,
            ..Seat::default()
        },
    ];
    f.stages = vec![Stage {
        index: 2,
        world: Some(Rc::new(w)),
        ..Stage::default()
    }];
    f.campaign.island = "insel-1".to_string();
    f.route = vec![RouteIsland {
        id: "insel-1".to_string(),
        etappe: 1,
        world: "wald".to_string(),
        ..RouteIsland::default()
    }];
    f
}

pub(super) fn lt_rt(input: &mut InputState, pad: u32, down: bool) {
    let p = input.pad_mut(pad);
    p.set_button(PadButton::LT, down);
    p.set_button(PadButton::RT, down);
}

#[test]
fn flag_wird_herausgeloest() {
    let (d, rest) = extract(&strs(&["--saves", "x", "--playtest", "ordner", "--w", "10"])).unwrap();
    assert_eq!(d.map(|o| o.dir), Some(PathBuf::from("ordner")));
    assert_eq!(rest, strs(&["--saves", "x", "--w", "10"]));
    let (d, _) = extract(&strs(&["--playtest=a b"])).unwrap();
    assert_eq!(d.map(|o| o.dir), Some(PathBuf::from("a b")));
    assert!(extract(&strs(&["--playtest"])).is_err());
}

#[test]
fn ohne_flag_bleibt_alles_wie_es_ist() {
    let a = strs(&["--saves", "x"]);
    let (d, rest) = extract(&a).unwrap();
    assert_eq!(d, None);
    assert_eq!(rest, a);
}

#[test]
fn zeile_hat_alle_felder_und_ist_json() {
    let d = dir("felder");
    let mut pt = Playtest::open(&d);
    pt.frame_done(1_000, 41.5);
    let mut input = InputState::new();
    lt_rt(&mut input, 7, true);
    let ms = pt.step(&input, Some(&frame()), 2_000);
    assert_eq!(ms.len(), 1);
    let l = lines(&d);
    assert_eq!(l.len(), 1);
    let v = &l[0];
    assert_eq!(v["ts"], 2_000);
    assert_eq!(v["kind"], "pad");
    assert_eq!(v["player"], 2);
    assert_eq!(v["world"], "wald");
    assert_eq!(v["island"], 1);
    assert_eq!(v["stage"], 2);
    assert_eq!(v["night"], true);
    assert_eq!(v["day"], 4);
    assert_eq!(v["worst_frame_ms"], 41.5);
}

#[test]
fn f9_setzt_marker_fuer_den_tastatur_spieler() {
    let d = dir("f9");
    let mut pt = Playtest::open(&d);
    let mut input = InputState::new();
    input.keys.press(Key::Insert);
    pt.step(&input, Some(&frame()), 5_000);
    let l = lines(&d);
    assert_eq!(l[0]["player"], 1);
    assert_eq!(l[0]["night"], false);
}

#[test]
fn zwei_spieler_getrennt_und_drosselung_je_spieler() {
    let d = dir("zwei");
    let mut pt = Playtest::open(&d);
    let f = frame();
    let mut input = InputState::new();
    input.keys.press(Key::Insert);
    lt_rt(&mut input, 7, true);
    assert_eq!(
        pt.step(&input, Some(&f), 10_000).len(),
        2,
        "beide Spieler im selben Takt"
    );
    // loslassen, nach 300 ms erneut: beide gedrosselt
    input = InputState::new();
    pt.step(&input, Some(&f), 10_100);
    input.keys.press(Key::Insert);
    lt_rt(&mut input, 7, true);
    assert!(pt.step(&input, Some(&f), 10_300).is_empty());
    // nur Spieler 2 nach über einer Sekunde
    input = InputState::new();
    pt.step(&input, Some(&f), 10_900);
    lt_rt(&mut input, 7, true);
    assert_eq!(pt.step(&input, Some(&f), 11_100).len(), 1);
    let players: Vec<i64> = lines(&d).iter().map(|v| v["player"].as_i64().unwrap_or(0)).collect();
    assert_eq!(players, vec![1, 2, 2]);
}

#[test]
fn gehaltene_taste_gibt_einen_marker() {
    let d = dir("halten");
    let mut pt = Playtest::open(&d);
    let mut input = InputState::new();
    lt_rt(&mut input, 7, true);
    for i in 0..5 {
        pt.step(&input, Some(&frame()), 1_000 + i * 2_000);
    }
    assert_eq!(lines(&d).len(), 1);
}

#[test]
fn rueckmeldung_nur_in_der_ansicht_des_spielers() {
    let mut pt = Playtest::open(&dir("flash"));
    let mut input = InputState::new();
    lt_rt(&mut input, 7, true);
    pt.step(&input, Some(&frame()), 1_000);
    assert!(pt.marked(1) && !pt.marked(0));
    for _ in 0..FLASH_TICKS {
        pt.step(&InputState::new(), Some(&frame()), 1_000);
    }
    assert!(!pt.marked(1));
}

#[test]
fn nicht_beschreibbarer_ordner_ist_fehlerzustand_und_spiel_laeuft() {
    let d = dir("fehler");
    std::fs::create_dir_all(d.parent().unwrap()).unwrap();
    std::fs::write(&d, b"ich bin eine datei").unwrap(); // ein Ordner kann dort nicht entstehen
    let mut pt = Playtest::open(&d);
    assert!(pt.failed());
    let mut input = InputState::new();
    lt_rt(&mut input, 7, true);
    assert_eq!(
        pt.step(&input, Some(&frame()), 1_000).len(),
        1,
        "kein Absturz, Marker zählt weiter"
    );
    let _ = std::fs::remove_file(&d);
}

#[test]
fn marker_ohne_spiel_im_menue() {
    let d = dir("menue");
    let mut pt = Playtest::open(&d);
    let mut input = InputState::new();
    lt_rt(&mut input, 3, true);
    pt.step(&input, None, 1_000);
    let l = lines(&d);
    assert_eq!(l[0]["player"], 1);
    assert_eq!(l[0]["stage"], 0);
}

#[test]
fn cheat_tasten_setzen_keinen_marker_eingabe_bleibt_unveraendert() {
    // View + Y und F9 bleiben das Cheat-Menü: kein Marker, die Eingabe geht unverändert an das Spiel
    let d = dir("cheats");
    let mut pt = Playtest::open(&d);
    let mut input = InputState::new();
    input.keys.press(Key::F9);
    let p = input.pad_mut(7);
    p.set_button(PadButton::View, true);
    p.set_button(PadButton::Y, true);
    assert!(pt.step(&input, Some(&frame()), 1_000).is_empty());
    assert!(lines(&d).is_empty());
    assert!(input.held(Key::F9) && input.pad(7).is_some_and(|p| p.held(PadButton::Y)));
}

#[test]
fn drosselung_und_schlechteste_zeit() {
    let mut r = RateLimit::default();
    assert!(r.allow(1, 0) && !r.allow(1, 999) && r.allow(1, 1_000) && r.allow(2, 10));
    let mut t = FrameTimes::default();
    t.push(0, 90.0);
    t.push(4_000, 20.0);
    assert_eq!(t.worst(4_000), 90.0);
    assert_eq!(t.worst(6_000), 20.0, "nach 5 s fällt der alte Ausreißer heraus");
    assert_eq!(FrameTimes::default().worst(1), 0.0);
}

#[test]
fn zeile_ist_eine_zeile() {
    let m = Marker {
        ts: 1,
        player: 1,
        ctx: Default::default(),
        worst_frame_ms: 16.666,
        ..Marker::default()
    };
    let l = m.to_line();
    assert!(!l.contains('\n'));
    assert!(l.contains("16.67"));
}

/// Die Workbench startet die Aufnahme erst, wenn die Datei `ready` steht (Unix-ms des ersten richtigen Bilds).
#[test]
fn ready_datei_nennt_den_zeitpunkt() {
    let d = dir("ready");
    let pt = Playtest::open(&d);
    assert!(!d.join("ready").exists());
    pt.announce_ready(1_795_359_345_287);
    assert_eq!(std::fs::read_to_string(d.join("ready")).unwrap(), "1795359345287");
}
