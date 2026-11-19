//! Round-Trip aller Nachrichten und Fehlerfälle (RU56.1, AC-01).
#![allow(clippy::unwrap_used, clippy::too_many_lines)] // Testcode (RU42: nur Nicht-Test-Code ist panic-frei)

use k3c_devproto::*;

fn rt(m: Msg) {
    let line = encode_line(&m).unwrap();
    assert!(line.ends_with('\n') && line.matches('\n').count() == 1);
    assert_eq!(decode_line(&line).unwrap(), m);
}

#[test]
fn alle_nachrichten_round_trip() {
    rt(Msg::Hello {
        v: PROTO_VERSION,
        game: None,
    });
    let game = GameInfo {
        name: "k3c".into(),
        version: "0.1".into(),
    };
    rt(Msg::Hello {
        v: PROTO_VERSION,
        game: Some(game),
    });
    let action = DevCommand {
        action: "gold".into(),
        amount: 50,
        on: Some(true),
        boss: "b".into(),
        ..Default::default()
    };
    rt(Msg::Dev {
        id: 7,
        player: 1,
        action,
    });
    rt(Msg::Ui {
        id: 2,
        key: "speed".into(),
        value: Some(4),
    });
    rt(Msg::Ui {
        id: 3,
        key: "pause".into(),
        value: None,
    });
    rt(Msg::Goto {
        id: 4,
        world: "w1".into(),
        island: 1,
        stage: 2,
    });
    rt(Msg::Focus { id: 5 });
    let island = IslandInfo {
        name: "I".into(),
        stages: 3,
    };
    let cat = Catalog {
        worlds: vec![WorldInfo {
            id: "w".into(),
            name: "Welt".into(),
            islands: vec![island],
        }],
        bosses: vec![Entry {
            id: "b".into(),
            label: "Boss".into(),
        }],
        phases: vec!["day".into(), "night".into()],
        timescales: vec![1, 2, 4],
        ..Default::default()
    };
    rt(Msg::Catalog(cat));
    let mut st = State {
        world: "w".into(),
        timescale: 4,
        halted: true,
        ticks_per_s: 60.5,
        ..Default::default()
    };
    st.players.push(PlayerState {
        gold: 9,
        stock: [("holz".to_string(), 3)].into(),
    });
    rt(Msg::State(st));
    rt(Msg::Ack {
        id: 7,
        ok: false,
        err: Some("kein Boss".into()),
    });
    rt(Msg::Ack {
        id: 8,
        ok: true,
        err: None,
    });
    rt(Msg::Bye);
}

#[test]
fn hello_zeile_ist_wie_spezifiziert() {
    let line = encode_line(&Msg::Hello { v: 1, game: None }).unwrap();
    assert_eq!(line, "{\"t\":\"hello\",\"v\":1}\n");
}

#[test]
fn unbekanntes_feld_wird_ignoriert() {
    let m = decode_line(r#"{"t":"ack","id":1,"ok":true,"neu":[1,2]}"#).unwrap();
    assert_eq!(
        m,
        Msg::Ack {
            id: 1,
            ok: true,
            err: None
        }
    );
}

#[test]
fn unbekannte_art_ist_unknown() {
    assert_eq!(decode_line(r#"{"t":"zukunft","x":1}"#).unwrap(), Msg::Unknown);
}

#[test]
fn crlf_wird_akzeptiert() {
    assert_eq!(decode_line("{\"t\":\"bye\"}\r\n").unwrap(), Msg::Bye);
}

#[test]
fn zu_lange_zeile_wird_abgelehnt() {
    let long = "x".repeat(MAX_LINE);
    let line = format!("{{\"t\":\"ack\",\"id\":1,\"ok\":false,\"err\":\"{long}\"}}");
    assert!(matches!(decode_line(&line), Err(ProtoError::TooLong(_))));
    let m = Msg::Ack {
        id: 1,
        ok: false,
        err: Some(long),
    };
    assert!(matches!(encode_line(&m), Err(ProtoError::TooLong(_))));
}

#[test]
fn fremde_version_ist_fehler() {
    let r = decode_line(r#"{"t":"hello","v":2}"#);
    assert!(matches!(r, Err(ProtoError::Version(2))));
}

#[test]
fn muell_ist_fehler_ohne_panic() {
    for s in [
        "",
        "kein json",
        "[]",
        "{}",
        "{\"t\":5}",
        "{\"t\":\"dev\"}",
        "\u{0}",
        "{\"t\":\"hello\"}",
    ] {
        let _ = decode_line(s);
    }
    assert!(matches!(decode_line("kein json"), Err(ProtoError::Json(_))));
    assert!(decode_line(r#"{"t":"dev","id":"x"}"#).is_err());
}
