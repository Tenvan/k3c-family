use k3c_devproto::Msg;

use super::super::commands::handle;
use super::super::mock::MockHost;
use crate::view::devkeys::DevCmd;

fn ui(host: &mut MockHost, key: &str, value: Option<i64>) -> bool {
    let msg = Msg::Ui {
        id: 1,
        key: key.into(),
        value,
    };
    match handle(host, msg) {
        Some(Msg::Ack { ok, .. }) => ok,
        other => panic!("kein ack: {other:?}"),
    }
}

#[test]
fn tasten_wie_die_f_tasten() {
    let mut h = MockHost::new(1);
    for (key, cmd) in [
        ("river", DevCmd::River),
        ("help", DevCmd::Help),
        ("fullscreen", DevCmd::Fullscreen),
        ("shot", DevCmd::Shot),
        ("stats", DevCmd::Stats),
        ("debug", DevCmd::Debug),
    ] {
        h.cmds.clear();
        assert!(ui(&mut h, key, None), "{key}");
        assert_eq!(h.cmds, [cmd], "{key}");
    }
}

#[test]
fn debug_seite_und_leistung_mit_wert() {
    let mut h = MockHost::new(1);
    assert!(ui(&mut h, "debug", Some(2)));
    assert_eq!(h.state.debug_page, 2);
    assert_eq!(h.cmds.len(), 2, "aus -> Geometrie -> Werte");
    h.cmds.clear();
    assert!(ui(&mut h, "debug", Some(2)));
    assert!(h.cmds.is_empty(), "schon auf der Seite");
    assert!(ui(&mut h, "debug", Some(0)));
    assert_eq!(h.state.debug_page, 0);
    assert!(!ui(&mut h, "debug", Some(5)));
    h.cmds.clear();
    assert!(ui(&mut h, "stats", Some(1)));
    assert!(h.state.perf);
    assert!(ui(&mut h, "stats", Some(1)));
    assert_eq!(h.cmds, [DevCmd::Stats], "unverändert gelassen");
    assert!(ui(&mut h, "stats", Some(0)));
    assert!(!h.state.perf);
}

#[test]
fn unbekannte_taste_ack_false() {
    let mut h = MockHost::new(1);
    assert!(!ui(&mut h, "flieg", None));
}

#[test]
fn umgebungseffekt_schalten() {
    use crate::world::fx;
    let mut h = MockHost::new(1);
    assert!(ui(&mut h, "fx:haze", Some(0)));
    assert!(!fx::on("haze"));
    assert!(ui(&mut h, "fx:haze", Some(1)));
    assert!(fx::on("haze"));
    assert!(ui(&mut h, "fx:haze", None), "ohne Wert: um");
    assert!(!fx::on("haze"));
    assert!(ui(&mut h, "fx:haze", Some(1)), "wieder an für die anderen Tests");
    assert!(!ui(&mut h, "fx:gibtsnicht", Some(0)));
    assert!(h.cmds.is_empty(), "kein F-Tasten-Befehl");
}
