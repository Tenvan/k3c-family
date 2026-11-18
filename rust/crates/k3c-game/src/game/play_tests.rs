//! Tests von `play.rs`: `TestPlayOverlayConsumes`, `TestPlayHomePauses`, `TestPlayRebindsLostPad` aus `play_test.go` und
//! `TestOverlayHidesConsumedPlayer` aus `devtable_test.go`.

use super::play_testutil::{SkillMenu, play_game, press};
use super::stack::Mode;
use super::testutil::{Probe, keys1, pad};
use crate::view::frame::CueKind;
use crate::view::input::{Action, Command, DeviceID, Input};

/// Der letzte Befehl, den die Simulation für Spieler `p` bekam.
fn last_cmd(probe: &Probe, p: i64) -> Command {
    let cmds = probe.cmds.borrow();
    cmds.iter()
        .rev()
        .find(|(q, _)| *q == p)
        .map_or_else(|| unreachable!("kein Befehl für Spieler {p}"), |(_, c)| *c)
}

fn devices_in(ids: &[DeviceID]) -> Input {
    Input {
        devices: ids.iter().map(|&d| press(d, Command::default(), &[])).collect(),
    }
}

/// Ein offenes Overlay nimmt die Eingabe seines Spielers (leerer Befehl, Menu öffnet keine Pause), der andere spielt
/// weiter; das Overlay lernt über die Session, seine Cues kommen in den Frame.
#[test]
fn play_overlay_consumes() {
    let (menu, _) = SkillMenu::new();
    let (mut g, probe) = play_game(vec![Box::new(menu)]);
    let run = Command {
        move_: 1.0,
        ..Command::default()
    };
    let input = |a0: &[Action]| Input {
        devices: vec![press(keys1(), run, a0), press(pad(2), run, &[])],
    };
    g.play(&input(&[Action::View]));
    assert_eq!(last_cmd(&probe, 0), Command::default(), "Befehle mit offenem Menü");
    assert_eq!(last_cmd(&probe, 1), run);
    g.play(&input(&[Action::Confirm, Action::Menu]));
    assert_eq!(g.mode, Mode::Play);
    assert!(!probe.paused.get(), "Menu im Overlay pausiert");
    assert_eq!(*probe.learned.borrow(), ["schild"]);
    assert_eq!(g.cues.len(), 1);
    assert!(probe.joins.borrow().is_empty());
    g.play(&input(&[Action::View]));
    assert_eq!(last_cmd(&probe, 0), run, "nach dem Schließen");
}

/// `Action::Home` öffnet im Spiel das Pause-Menü wie `Action::Menu`.
#[test]
fn play_home_pauses() {
    let (mut g, probe) = play_game(Vec::new());
    let input = Input {
        devices: vec![press(pad(2), Command::default(), &[Action::Home])],
    };
    g.play(&input);
    assert_eq!(g.mode, Mode::Menu);
    assert!(probe.paused.get());
    assert_eq!(g.stack.len(), 1);
}

/// Verliert Spieler 2 sein Pad, übernimmt ein neues Pad den Platz (Neuzuordnung statt Beitritt); ein weiteres freies
/// Gerät mit Bestätigen tritt bei.
#[test]
fn play_rebinds_lost_pad() {
    let (mut g, probe) = play_game(Vec::new());
    let (pad9, pad5) = (pad(8), pad(4));
    let seats = g.frame.seats.clone();
    g.devs
        .observe(&devices_in(&[keys1(), pad9, pad(2)]), &seats, &mut g.cues);
    g.cues.clear();
    let input = Input {
        devices: vec![
            press(keys1(), Command::default(), &[]),
            press(pad9, Command::default(), &[Action::Confirm]),
            press(pad5, Command::default(), &[Action::Confirm]),
        ],
    };
    g.devs.observe(&input, &seats, &mut g.cues);
    g.play(&input);
    assert_eq!(*probe.rebinds.borrow(), [(1, pad9)], "Neuzuordnung");
    assert_eq!(*probe.joins.borrow(), [pad5], "Beitritte");
    let kinds: Vec<CueKind> = g.cues.iter().map(|c| c.kind).collect();
    assert_eq!(kinds, [CueKind::DeviceConnected, CueKind::DeviceLost]);
}

/// Ein Spieler, den ein früheres Overlay verbraucht (Cheat-Menü), fehlt in der Eingabe der späteren (Skill-Menü); der
/// andere Spieler bleibt sichtbar.
#[test]
fn overlay_hides_consumed_player() {
    let (first, first_open) = SkillMenu::new();
    let (second, second_open) = SkillMenu::new();
    let (mut g, _) = play_game(vec![Box::new(first), Box::new(second)]);
    let view = |d: DeviceID| Input {
        devices: vec![press(d, Command::default(), &[Action::View])],
    };
    g.play(&view(keys1()));
    assert!(
        first_open.get()[0] && !second_open.get()[0],
        "erstes {:?}, zweites {:?}",
        first_open.get(),
        second_open.get()
    );
    g.play(&view(pad(2)));
    assert!(
        first_open.get()[1] && !second_open.get()[1],
        "Spieler 2: erstes {:?}, zweites {:?}",
        first_open.get(),
        second_open.get()
    );
}
