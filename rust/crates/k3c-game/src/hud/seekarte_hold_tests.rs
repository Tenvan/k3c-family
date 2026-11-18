//! Port von `native/k3c-game/hud/seekarte_hold_test.go` (die Sperre durch gehaltenes A, B-1006). Der Test
//! `TestSeekarteSperreOhneAllokationen` entfällt (`testing.AllocsPerRun`, Frame-Budget misst RU39).

use super::Hud;
use super::testutil::{HudSession, hud, open_switch, pay_input, sea_world, set_world};
use crate::view::frame::Frame;
use crate::view::input::Action::{Confirm, SkillMenu};
use crate::view::input::Input;
use crate::view::roles::Overlay;
use crate::view::session::Fehler;
use k3c_sim::state::{Crown, World};

/// held_sea: Seekarte offen, beide am Steg, Spieler 1 hat eben bestätigt und hält A.
fn held_sea(err: Option<Fehler>) -> (World, Frame, Hud, HudSession) {
    let (mut w, mut f) = sea_world();
    w.island_switch = Some(open_switch("b"));
    set_world(&mut f, &w);
    let mut h = hud();
    h.listen(&f);
    let mut s = HudSession {
        choose_err: err,
        ..HudSession::default()
    };
    let st = h.update(&pay_input(true, true, &[Confirm], &[]), &f, &mut s);
    assert!(
        st.consumed[0] && !st.consumed[1],
        "Spieler 1 bestätigt (verbraucht), Spieler 2 hält A und zahlt"
    );
    (w, f, h, s)
}

/// B-1006/AC-01: Nach dem Bestätigen bleibt die Eingabe verbraucht, solange A gehalten ist; ein Update ohne A beendet
/// die Sperre, erneutes Halten zahlt wieder. Gilt auch bei abgelehnter Wahl.
#[test]
fn held_a_does_not_pay() {
    for err in [None, Some(Fehler::Session("abgelehnt".to_string()))] {
        let (_, f, mut h, mut s) = held_sea(err.clone());
        for i in 0..3 {
            let st = h.update(&pay_input(true, false, &[], &[]), &f, &mut s);
            assert!(st.consumed[0], "err={err:?}: gehaltenes A zahlt im Folgetakt {}", i + 1);
        }
        let st = h.update(&pay_input(false, false, &[], &[]), &f, &mut s);
        assert!(!st.consumed[0], "err={err:?}: A losgelassen, Sperre endet");
        let st = h.update(&pay_input(true, false, &[], &[]), &f, &mut s);
        assert!(!st.consumed[0], "err={err:?}: erneut gehaltenes A zahlt wie überall");
        assert_eq!(s.picks.len(), 1, "genau eine Wahl: {:?}", s.picks);
    }
}

/// B-1006/AC-02: Die Sperre gilt je Spieler; der Mitspieler am selben Steg zahlt und wählt unabhängig weiter.
#[test]
fn hold_is_per_player() {
    let (_, f, mut h, mut s) = held_sea(None);
    let st = h.update(&pay_input(true, true, &[], &[]), &f, &mut s);
    assert!(
        st.consumed[0] && !st.consumed[1],
        "Sperre von Spieler 1 trifft Spieler 2 nicht: {:?}",
        st.consumed
    );
    let st = h.update(&pay_input(true, true, &[], &[Confirm]), &f, &mut s);
    assert!(
        st.consumed[0] && st.consumed[1],
        "Spieler 2 wählt unabhängig: {:?}",
        st.consumed
    );
    let st = h.update(&pay_input(false, true, &[], &[]), &f, &mut s);
    assert!(
        !st.consumed[0] && st.consumed[1],
        "Spieler 1 lässt los, Spieler 2 hält weiter: {:?}",
        st.consumed
    );
    assert_eq!(s.picks, ["0:b", "1:b"], "Wahlen");
}

/// Die Sperre endet auch beim Verlassen des Stegs, bei Kronenverlust, getrenntem Gerät und geschlossener Karte.
#[test]
fn hold_ends() {
    type Change = fn(&mut World, &mut Input);
    let cases: [(&str, Change); 4] = [
        ("Steg verlassen", |w, _| w.players[0].x = 80.0),
        ("Krone verloren", |w, _| w.players[0].crown = Some(Crown::default())),
        ("Gerät getrennt", |_, i| {
            i.devices.remove(0);
        }),
        ("Karte zu", |w, _| w.island_switch = None),
    ];
    for (name, change) in cases {
        let (mut w, mut f, mut h, mut s) = held_sea(None);
        let mut inp = pay_input(true, false, &[], &[]);
        change(&mut w, &mut inp);
        set_world(&mut f, &w);
        h.listen(&f);
        let st = h.update(&inp, &f, &mut s);
        assert!(!st.consumed[0] && !h.seats[0].sea.hold, "{name}: Sperre endet");
    }
}

/// Ein offenes Skill-Menü führt die Sperre weiter: gehalten bleibt sie, im Menü losgelassen endet sie.
#[test]
fn hold_in_skill_menu() {
    let (_, f, mut h, mut s) = held_sea(None);
    let st = h.update(&pay_input(true, false, &[SkillMenu], &[]), &f, &mut s);
    assert!(
        st.consumed[0] && h.seats[0].sea.hold,
        "Skill-Menü offen, A gehalten: Sperre bleibt"
    );
    h.update(&pay_input(false, false, &[], &[]), &f, &mut s);
    assert!(!h.seats[0].sea.hold, "im Skill-Menü losgelassen: Sperre endet");
}
