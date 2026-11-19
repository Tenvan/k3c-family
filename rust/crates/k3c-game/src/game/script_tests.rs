//! Tests der festen Eingabe-Folge (Go: `script_test.go`, `TestCommandsAt` und `TestScriptedSource` aus `scene_test.go`).

use k3c_platform::InputState;

use super::*;
use crate::game::scene::parse_args;
use crate::view::input::DeviceKind;

fn argv(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| (*s).to_string()).collect()
}

/// Menü-Aktionen der Eingabe-Folge: genau im genannten Takt, in Reihenfolge der Folge, der gehaltene Befehl bleibt.
#[test]
fn script_actions() {
    let script = parse_script("0@0=R,0@2=a:skillmenu,0@2=a:right,1@2=a:menu,0@3=a:confirm").unwrap();
    let sc = Scene {
        players: 2,
        script,
        ..Scene::default()
    };
    let mut src = scripted(&sc);
    let raw = InputState::new();
    let want: [(usize, [&[Action]; 2]); 2] = [
        (2, [&[Action::SkillMenu, Action::Right], &[Action::Menu]]),
        (3, [&[Action::Confirm], &[]]),
    ];
    for tick in 0..5 {
        let input = src.poll(&raw);
        for p in 0..2 {
            let exp: &[Action] = want.iter().find(|(t, _)| *t == tick).map_or(&[], |(_, a)| a[p]);
            assert_eq!(input.devices[p].actions, exp, "Takt {tick}, Spieler {p}");
        }
        assert_eq!(input.devices[0].command.move_, 1.0, "Takt {tick}: Befehl verloren");
    }
    for bad in ["0@1=a:", "0@1=a:sprung", "0@1=a:Down"] {
        assert!(parse_script(bad).is_err(), "angenommen: {bad}");
    }
}

/// --menu: main, saves und lobby ohne Spiel (Aufnahme ohne Standard-Seed), pause mit Spiel (Seed k3c), options mit
/// Szene über der Pause.
#[test]
fn parse_args_menu() {
    let cases: [(&[&str], &str, bool); 5] = [
        (&["--shots", "o", "--menu", "main"], "", false),
        (&["--shots", "o", "--menu", "saves", "--seed", "wald"], "wald", false),
        (&["--shots", "o", "--menu", "pause"], "k3c", true),
        (&["--menu", "options"], "", false),
        (&["--shots", "o", "--menu", "options", "--seed", "x"], "x", true),
    ];
    for (v, seed, in_game) in cases {
        let a = parse_args(&argv(v), "s").unwrap_or_else(|e| panic!("{v:?}: {e}"));
        assert!(
            a.scene.seed == seed && a.scene.in_game() == in_game && (a.shots.is_empty() || a.scene.script.is_empty()),
            "{v:?}: Seed {:?}, im Spiel {}, Folge {:?}",
            a.scene.seed,
            a.scene.in_game(),
            a.scene.script
        );
    }
    assert!(
        parse_args(&argv(&["--menu", "credits"]), "s").is_err(),
        "unbekanntes Menü angenommen"
    );
}

/// Die feste Folge hält je Spieler den letzten Eintrag bis zum Takt.
#[test]
fn commands_at_haelt_letzten_eintrag() {
    let script = parse_script("0@0=R,1@0=LSPA2,0@10=-,0@10=RL,1@5=A").unwrap();
    let c = commands_at(&script, 0);
    assert!(
        c[0].move_ == 1.0 && c[1].move_ == -1.0 && c[1].sprint && c[1].pay && c[1].skill == 2,
        "Takt 0: {c:?}"
    );
    let c = commands_at(&script, 7);
    let attack = Command {
        attack: true,
        ..Command::default()
    };
    assert!(c[0].move_ == 1.0 && c[1] == attack, "Takt 7: {c:?}");
    let c = commands_at(&script, 10);
    assert_eq!(c[0], Command::default(), "Takt 10, späterer Eintrag gewinnt");
    assert_eq!(commands_at(&[], 3), [Command::default(); MAX_PLAYERS], "ohne Folge");
}

/// Die Quelle der festen Folge liefert je Poll einen Takt, gleich für gleiche Szenen.
#[test]
fn scripted_source() {
    let sc = Scene {
        players: 2,
        script: vec![Hold {
            tick: 2,
            player: 1,
            cmd: Command {
                move_: -1.0,
                ..Command::default()
            },
            press: false,
            action: Action::Up,
        }],
        ..Scene::default()
    };
    let (mut a, mut b) = (scripted(&sc), scripted(&sc));
    let raw = InputState::new();
    for tick in 0..4 {
        let (ia, ib) = (a.poll(&raw), b.poll(&raw));
        assert!(
            ia.devices.len() == 2 && ia.devices[1].command == ib.devices[1].command,
            "Takt {tick}"
        );
        assert_eq!(
            ia.devices[1].command.move_ == -1.0,
            tick >= 2,
            "Takt {tick}: Spieler 2 läuft"
        );
        assert_eq!(ia.devices[1].device, DeviceID::new(DeviceKind::Gamepad, 0), "Gerät");
    }
}
