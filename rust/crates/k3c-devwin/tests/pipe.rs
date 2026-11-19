//! Pipe-Server mit echtem Client über `std::fs` (nur Windows): `hello` und `state` kommen an, das Modell zeigt
//! "verbunden", nach dem Trennen "getrennt", ein neuer Client verbindet wieder (RU56.4, AC-04).
#![cfg(windows)]
#![allow(
    clippy::large_enum_variant,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_lines
)] // Testcode

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use k3c_devproto::{Msg, PROTO_VERSION, State, decode_line, encode_line};
use k3c_devwin::state::{Conn, Model};
use k3c_devwin::win32::pipe::{self, Sink};

#[derive(Debug)]
enum Ev {
    Msg(Msg),
    Lost,
}

struct ChanSink(Mutex<Sender<Ev>>);

impl Sink for ChanSink {
    fn msg(&self, m: Msg) -> bool {
        self.0.lock().unwrap().send(Ev::Msg(m)).is_ok()
    }
    fn lost(&self) -> bool {
        self.0.lock().unwrap().send(Ev::Lost).is_ok()
    }
}

fn connect(name: &str) -> File {
    let path = format!(r"\\.\pipe\{name}");
    for _ in 0..100 {
        if let Ok(f) = OpenOptions::new().read(true).write(true).open(&path) {
            return f;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    panic!("Pipe {path} nicht erreichbar");
}

fn next(rx: &Receiver<Ev>) -> Ev {
    rx.recv_timeout(Duration::from_secs(5)).expect("Ereignis der Pipe")
}

fn feed(model: &mut Model, ev: Ev) {
    match ev {
        Ev::Msg(m) => {
            model.on_msg(m);
        }
        Ev::Lost => {
            model.on_lost();
        }
    }
}

fn send_line(f: &mut File, m: &Msg) {
    f.write_all(encode_line(m).unwrap().as_bytes()).unwrap();
}

#[test]
fn verbinden_trennen_und_wiederverbinden() {
    let name = format!("k3c-devwin-test-{}", std::process::id());
    let (tx, rx) = mpsc::channel();
    let to_game = pipe::start(&name, Arc::new(ChanSink(Mutex::new(tx)))).unwrap();
    assert!(
        pipe::start(&name, Arc::new(ChanSink(Mutex::new(mpsc::channel().0)))).is_err(),
        "zweiter Server mit demselben Namen wird abgelehnt"
    );
    let mut model = Model::default();

    // Erster Client: die Pipe begrüßt zuerst, dann hello und state vom Spiel.
    let mut client = connect(&name);
    let mut reader = BufReader::new(client.try_clone().unwrap());
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    assert!(matches!(
        decode_line(&line).unwrap(),
        Msg::Hello { v: PROTO_VERSION, .. }
    ));
    send_line(
        &mut client,
        &Msg::Hello {
            v: PROTO_VERSION,
            game: None,
        },
    );
    send_line(
        &mut client,
        &Msg::State(State {
            world: "w".into(),
            ..State::default()
        }),
    );
    feed(&mut model, next(&rx));
    assert_eq!(model.conn, Conn::Connected);
    feed(&mut model, next(&rx));
    assert_eq!(model.state.as_ref().map(|s| s.world.as_str()), Some("w"));

    // Befehl vom Fenster zum Spiel.
    to_game.send(Msg::Focus { id: 9 }).unwrap();
    line.clear();
    reader.read_line(&mut line).unwrap();
    assert_eq!(decode_line(&line).unwrap(), Msg::Focus { id: 9 });

    // Müllzeile stört nicht, die Verbindung bleibt.
    client.write_all(b"kein json\n").unwrap();
    send_line(
        &mut client,
        &Msg::Ack {
            id: 9,
            ok: true,
            err: None,
        },
    );
    feed(&mut model, next(&rx));
    assert!(model.last_ack.is_some());

    // Trennen: getrennt.
    drop(reader);
    drop(client);
    assert!(matches!(next(&rx), Ev::Lost));
    model.on_lost();
    assert_eq!(model.conn, Conn::Disconnected);
    assert!(!model.enabled());

    // Neuer Client: wieder verbunden.
    let mut again = connect(&name);
    send_line(
        &mut again,
        &Msg::Hello {
            v: PROTO_VERSION,
            game: None,
        },
    );
    feed(&mut model, next(&rx));
    assert_eq!(model.conn, Conn::Connected);
    assert!(model.enabled());
}
