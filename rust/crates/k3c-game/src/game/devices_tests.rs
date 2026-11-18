//! Port von `native/k3c-game/game/devices_test.go`.

use super::*;
use crate::view::input::DeviceKind;

const KEYS1: DeviceID = DeviceID::new(DeviceKind::Keyboard, 0);
const PAD9: DeviceID = DeviceID::new(DeviceKind::Gamepad, 9);
const PAD3: DeviceID = DeviceID::new(DeviceKind::Gamepad, 3);
const PAD5: DeviceID = DeviceID::new(DeviceKind::Gamepad, 5);
const PAD6: DeviceID = DeviceID::new(DeviceKind::Gamepad, 6);

fn seat(player: i64, device: DeviceID) -> Seat {
    Seat {
        player,
        device,
        ..Seat::default()
    }
}

fn rb(player: i64, device: DeviceID) -> Rebind {
    Rebind { player, device }
}

/// devicesIn baut eine Eingabe; Geräte in `confirm` drücken Bestätigen.
fn devices_in(ds: &[DeviceID], confirm: &[DeviceID]) -> Input {
    let devices = ds
        .iter()
        .map(|&d| DeviceInput {
            device: d,
            actions: if confirm.contains(&d) {
                vec![Action::Confirm]
            } else {
                Vec::new()
            },
            ..DeviceInput::default()
        })
        .collect();
    Input { devices }
}

/// Zuordnungs-Regel (B-798/AC-02): Pad geht, neues Pad kommt bzw. ein freies Gerät bestätigt, zwei verwaiste Plätze,
/// kein freies Gerät; nie über den Index.
#[test]
fn rebinds_regel() {
    for (name, seats, input, fresh, want) in rebind_cases() {
        let mut got = Vec::new();
        rebinds(&mut got, &seats, &input, &fresh);
        assert_eq!(got, want, "{name}");
    }
}

type Case = (&'static str, Vec<Seat>, Input, Vec<DeviceID>, Vec<Rebind>);

/// Die Fälle der Tabelle von `TestRebinds`.
fn rebind_cases() -> Vec<Case> {
    let seats = vec![seat(0, PAD3), seat(1, PAD9)];
    vec![
        (
            "Pad da",
            seats.clone(),
            devices_in(&[KEYS1, PAD9, PAD3], &[]),
            vec![],
            vec![],
        ),
        (
            "Pad weg, nichts Neues",
            seats.clone(),
            devices_in(&[KEYS1, PAD9], &[]),
            vec![],
            vec![],
        ),
        (
            "Pad weg, neues Pad",
            seats.clone(),
            devices_in(&[KEYS1, PAD9, PAD5], &[]),
            vec![PAD5],
            vec![rb(0, PAD5)],
        ),
        (
            "Pad weg, altes freies Pad bestätigt",
            seats.clone(),
            devices_in(&[KEYS1, PAD9, PAD5], &[PAD5]),
            vec![],
            vec![rb(0, PAD5)],
        ),
        (
            "Pad weg, Tastatur bestätigt",
            seats.clone(),
            devices_in(&[KEYS1, PAD9], &[KEYS1]),
            vec![],
            vec![rb(0, KEYS1)],
        ),
        (
            "besetztes Gerät bestätigt",
            seats.clone(),
            devices_in(&[KEYS1, PAD9], &[PAD9]),
            vec![],
            vec![],
        ),
        (
            "zwei verwaist",
            vec![seat(0, PAD3), seat(1, PAD6)],
            devices_in(&[KEYS1, PAD9, PAD5], &[PAD9]),
            vec![PAD5],
            vec![rb(0, PAD9), rb(1, PAD5)],
        ),
        (
            "nur Spieler 1 verwaist",
            vec![seat(1, PAD3)],
            devices_in(&[KEYS1], &[KEYS1]),
            vec![],
            vec![rb(1, KEYS1)],
        ),
    ]
}

/// Zwei Spieler mit je eigenem Gerät: fällt ein Pad aus, bleibt das andere bei seinem Spieler; das neue Pad geht nur an
/// den verwaisten Platz (B-798/AC-02).
#[test]
fn rebinds_zwei_spieler_behalten_ihr_geraet() {
    let seats = [seat(0, PAD3), seat(1, PAD9)];
    let input = devices_in(&[PAD9, PAD5], &[]);
    let mut got = vec![rb(7, PAD6)]; // dst wird angehängt, nicht ersetzt
    rebinds(&mut got, &seats, &input, &[PAD5]);
    assert_eq!(got, vec![rb(7, PAD6), rb(0, PAD5)]);
    assert!(got.iter().all(|r| r.player != 1), "Spieler 1 behält PAD9");
}

/// Geräte-Cues: das erste Update meldet nichts, danach verbunden und getrennt mit Platz; unseated und lost im Frame.
#[test]
fn devices_observe_decorate() {
    let mut d = Devices::default();
    let seats = [seat(0, PAD3)];
    let mut cues = Vec::new();
    d.observe(&devices_in(&[KEYS1, PAD3], &[]), &seats, &mut cues);
    assert!(cues.is_empty(), "erstes Update: {cues:?}");

    d.observe(&devices_in(&[KEYS1, PAD5], &[]), &seats, &mut cues);
    let want = vec![
        Cue {
            kind: CueKind::DeviceConnected,
            player: -1,
            device: PAD5,
            ..Cue::default()
        },
        Cue {
            kind: CueKind::DeviceLost,
            player: 0,
            device: PAD3,
            ..Cue::default()
        },
    ];
    assert_eq!(cues, want);
    assert_eq!(d.fresh, vec![PAD5]);

    cues.clear();
    d.observe(&devices_in(&[KEYS1, PAD5], &[]), &seats, &mut cues);
    assert!(cues.is_empty() && d.fresh.is_empty(), "ohne Änderung: {cues:?}");

    let mut f = Frame {
        seats: vec![seat(0, PAD3), seat(1, KEYS1)],
        ..Frame::default()
    };
    d.decorate(&mut f, &devices_in(&[KEYS1, PAD9, PAD5], &[]));
    assert_eq!(f.unseated, vec![PAD9, PAD5]);
    assert!(f.seats[0].lost && !f.seats[1].lost, "Plätze {:?}", f.seats);
}

/// Go zählt Allokationen; hier: nach dem ersten Durchlauf wachsen die Puffer nicht mehr (Leitfaden § 11).
#[test]
fn devices_legen_nach_dem_ersten_bild_nichts_an() {
    let mut d = Devices::default();
    let input = devices_in(&[KEYS1, PAD9, PAD5], &[]);
    let mut f = Frame {
        seats: vec![seat(0, PAD3), seat(1, KEYS1)],
        ..Frame::default()
    };
    let mut cues = Vec::with_capacity(8);
    d.observe(&input, &f.seats, &mut cues);
    d.decorate(&mut f, &input);
    let caps = (d.known.capacity(), d.fresh.capacity(), f.unseated.capacity());
    for _ in 0..10 {
        cues.clear();
        d.observe(&input, &f.seats, &mut cues);
        d.decorate(&mut f, &input);
    }
    assert_eq!(caps, (d.known.capacity(), d.fresh.capacity(), f.unseated.capacity()));
    assert!(cues.is_empty());
}
