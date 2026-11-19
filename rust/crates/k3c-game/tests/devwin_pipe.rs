//! Spielseite des Cheat-Fensters mit einem Kanal-Paar statt Pipe (RU56.5, B-1133, AC-05): `hello` und `catalog` gehen
//! hinaus, ein `dev`-Befehl erreicht `DevSession::dev` mit denselben Feldern wie der F9-Eintrag und wird mit `ack`
//! beantwortet; `state` höchstens 4 Hz und bei voller Warteschlange verworfen, `ack` nie; getrennte Verbindung und
//! defekte Zeile ohne Panic; ohne Fenster nichts.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::too_many_lines)] // Testcode

use std::collections::VecDeque;
use std::io::{self, Read, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use k3c_devproto::{Catalog, DevCommand, GameInfo, Msg, State, decode_line, encode_line};
use k3c_game::devtools::cheatlist::cheat_groups;
use k3c_game::devwin::mock::MockHost;
use k3c_game::devwin::state::MIN_GAP;
use k3c_game::devwin::{DevWin, Event, Link, Opener, RETRY, STATE_QUEUE, Streams};

struct ChanRead {
    rx: Receiver<Vec<u8>>,
    buf: Vec<u8>,
    pos: usize,
}

impl Read for ChanRead {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if self.pos >= self.buf.len() {
            match self.rx.recv() {
                Ok(v) => (self.buf, self.pos) = (v, 0),
                Err(_) => return Ok(0),
            }
        }
        let n = out.len().min(self.buf.len() - self.pos);
        out[..n].copy_from_slice(&self.buf[self.pos..self.pos + n]);
        self.pos += n;
        Ok(n)
    }
}

/// Schreiben wartet, bis `gate` ein Zeichen liefert (nur wenn gesetzt): so bleibt die Warteschlange voll.
struct ChanWrite {
    tx: Sender<Vec<u8>>,
    gate: Option<Receiver<()>>,
}

impl Write for ChanWrite {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        if let Some(g) = &self.gate {
            let _ = g.recv();
        }
        self.tx
            .send(b.to_vec())
            .map_err(|_| io::Error::from(io::ErrorKind::BrokenPipe))?;
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Peer ist die Gegenseite (das Fenster).
struct Peer {
    to_game: Sender<Vec<u8>>,
    from_game: Receiver<Vec<u8>>,
    acc: Vec<u8>,
}

fn pair(gate: Option<Receiver<()>>) -> (Streams, Peer) {
    let (to_game, game_rx) = mpsc::channel();
    let (game_tx, from_game) = mpsc::channel();
    let streams: Streams = (
        Box::new(ChanRead {
            rx: game_rx,
            buf: Vec::new(),
            pos: 0,
        }),
        Box::new(ChanWrite { tx: game_tx, gate }),
    );
    (
        streams,
        Peer {
            to_game,
            from_game,
            acc: Vec::new(),
        },
    )
}

impl Peer {
    fn send(&self, m: &Msg) {
        self.to_game.send(encode_line(m).unwrap().into_bytes()).unwrap();
    }

    /// Nächste Nachricht des Spiels (wartet höchstens `wait`).
    fn recv(&mut self, wait: Duration) -> Option<Msg> {
        let end = Instant::now() + wait;
        loop {
            if let Some(pos) = self.acc.iter().position(|b| *b == b'\n') {
                let line: Vec<u8> = self.acc.drain(..=pos).collect();
                return Some(decode_line(&String::from_utf8_lossy(&line)).unwrap());
            }
            let left = end.checked_duration_since(Instant::now())?;
            match self.from_game.recv_timeout(left) {
                Ok(v) => self.acc.extend(v),
                Err(_) => return None,
            }
        }
    }

    fn drain(&mut self, wait: Duration) -> Vec<Msg> {
        let mut v = Vec::new();
        while let Some(m) = self.recv(wait) {
            v.push(m);
        }
        v
    }
}

fn opener(queue: Vec<Streams>) -> (Opener, Arc<Mutex<i64>>) {
    let q = Arc::new(Mutex::new(VecDeque::from(queue)));
    let calls = Arc::new(Mutex::new(0));
    let c = Arc::clone(&calls);
    let o: Opener = Box::new(move || {
        *c.lock().unwrap() += 1;
        q.lock()
            .unwrap()
            .pop_front()
            .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
    });
    (o, calls)
}

fn game() -> GameInfo {
    GameInfo {
        name: "k3c".into(),
        version: "test".into(),
    }
}

const WAIT: Duration = Duration::from_secs(5);

/// Ruft `update` im 5-ms-Takt, bis `done` wahr ist.
fn pump(dw: &mut DevWin, host: &mut MockHost, mut done: impl FnMut(&MockHost) -> bool) {
    let end = Instant::now() + WAIT;
    while Instant::now() < end {
        dw.update(host, Instant::now());
        if done(host) {
            return;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    panic!("Frist abgelaufen");
}

#[test]
fn hello_catalog_dev_ack() {
    let (streams, mut peer) = pair(None);
    let (op, _) = opener(vec![streams]);
    let mut host = MockHost::new(2);
    host.catalog = Catalog {
        phases: vec!["night".into()],
        ..Catalog::default()
    };
    let mut dw = DevWin::default();
    dw.start(op, game());
    dw.update(&mut host, Instant::now());
    assert!(dw.connected());
    let Some(Msg::Hello { v, game }) = peer.recv(WAIT) else {
        panic!("hello fehlt")
    };
    assert_eq!(v, 1);
    assert_eq!(game.unwrap().name, "k3c");
    let Some(Msg::Catalog(c)) = peer.recv(WAIT) else {
        panic!("catalog fehlt")
    };
    assert_eq!(c.phases, ["night"]);

    // derselbe Eintrag wie im F9-Menü: Gruppe Kampf, "Boss: nächste Phase" bzw. Gold +100 aus Vorrat
    let entry = cheat_groups()
        .iter()
        .flat_map(|g| g.entries.iter())
        .find(|e| e.act.action == "gold" && e.act.amount == 100)
        .unwrap();
    peer.send(&Msg::Dev {
        id: 7,
        player: 1,
        action: DevCommand {
            action: "gold".into(),
            amount: 100,
            ..DevCommand::default()
        },
    });
    pump(&mut dw, &mut host, |h| !h.log().is_empty());
    assert_eq!(host.log(), [(1, entry.act.clone())]);
    let acks: Vec<_> = std::iter::from_fn(|| peer.recv(Duration::from_millis(500)))
        .filter(|m| matches!(m, Msg::Ack { .. }))
        .collect();
    assert_eq!(
        acks,
        [Msg::Ack {
            id: 7,
            ok: true,
            err: None
        }]
    );
}

#[test]
fn falscher_spieler_und_abgelehnter_befehl_geben_ack_false() {
    let (streams, mut peer) = pair(None);
    let (op, _) = opener(vec![streams]);
    let mut host = MockHost::new(1);
    let mut dw = DevWin::default();
    dw.start(op, game());
    dw.update(&mut host, Instant::now());
    let dev = |id, player, action: &str| Msg::Dev {
        id,
        player,
        action: DevCommand {
            action: action.into(),
            ..DevCommand::default()
        },
    };
    peer.send(&dev(1, 5, "night"));
    host.session.as_mut().unwrap().reject = Some("Cheat „x“ geht gerade nicht".into());
    peer.send(&dev(2, 0, "x"));
    peer.send(&Msg::Focus { id: 3 });
    pump(&mut dw, &mut host, |h| h.focused == 1);
    let acks: Vec<_> = peer
        .drain(Duration::from_millis(500))
        .into_iter()
        .filter_map(|m| match m {
            Msg::Ack { id, ok, err } => Some((id, ok, err.is_some())),
            _ => None,
        })
        .collect();
    assert_eq!(acks, [(1, false, true), (2, false, true), (3, true, false)]);
    assert_eq!(host.log().len(), 1, "Spieler 6 erreicht die Session nicht");
}

#[test]
fn state_hoechstens_4_hz_und_nur_bei_aenderung() {
    let (streams, mut peer) = pair(None);
    let (op, _) = opener(vec![streams]);
    let mut host = MockHost::new(1);
    let mut dw = DevWin::default();
    dw.start(op, game());
    let t0 = Instant::now();
    dw.update(&mut host, t0);
    // eine Sekunde im 50-ms-Takt, der Zustand ändert sich jedes Mal
    for k in 0..=20u32 {
        host.state.stage = i64::from(k);
        dw.update(&mut host, t0 + Duration::from_millis(50) * k);
    }
    let states = peer
        .drain(Duration::from_millis(300))
        .into_iter()
        .filter(|m| matches!(m, Msg::State(_)))
        .count();
    assert!((4..=5).contains(&states), "{states} state in 1 s");
    // unverändert: keine weitere Nachricht, und gebaut wird erst nach Ablauf der Frist
    let calls = host.state_calls.get();
    dw.update(&mut host, t0 + Duration::from_millis(1000) + MIN_GAP);
    dw.update(
        &mut host,
        t0 + Duration::from_millis(1000) + MIN_GAP + Duration::from_millis(10),
    );
    assert_eq!(host.state_calls.get(), calls + 1, "nur nach Ablauf der Frist gebaut");
    let more = peer
        .drain(Duration::from_millis(200))
        .into_iter()
        .filter(|m| matches!(m, Msg::State(_)))
        .count();
    assert!(more <= 1);
}

#[test]
fn state_faellt_bei_voller_warteschlange_weg_ack_nie() {
    let (gate_tx, gate_rx) = mpsc::channel();
    let (streams, mut peer) = pair(Some(gate_rx));
    let link = Link::connect(streams).unwrap();
    // der Schreib-Thread hängt im ersten Schreiben; alles Weitere bleibt in der Warteschlange
    link.send_ctl(Msg::Bye);
    std::thread::sleep(Duration::from_millis(100));
    let st = Msg::State(State::default());
    let accepted = (0..STATE_QUEUE * 3).filter(|_| link.send_state(st.clone())).count();
    assert_eq!(accepted, STATE_QUEUE, "darüber wird verworfen");
    for id in 0..100 {
        link.send_ctl(Msg::Ack {
            id,
            ok: true,
            err: None,
        });
    }
    assert_eq!(link.pending(), STATE_QUEUE + 100, "ack geht nie verloren");
    assert!(!link.send_state(st), "weiter voll");
    for _ in 0..(STATE_QUEUE + 102) {
        let _ = gate_tx.send(());
    }
    let acks = peer
        .drain(Duration::from_millis(500))
        .into_iter()
        .filter(|m| matches!(m, Msg::Ack { .. }))
        .count();
    assert_eq!(acks, 100);
}

#[test]
fn getrennt_wiederverbinden_und_defekte_zeile() {
    let (s1, peer1) = pair(None);
    let (s2, mut peer2) = pair(None);
    let (s3, peer3) = pair(None);
    let (op, calls) = opener(vec![s1, s2, s3]);
    let mut host = MockHost::new(1);
    let mut dw = DevWin::default();
    dw.start(op, game());
    let t0 = Instant::now();
    dw.update(&mut host, t0);
    assert!(dw.connected());
    drop(peer1); // Pipe weg
    // Verbindung gilt als getrennt, das Spiel läuft weiter; erst nach RETRY gibt es einen neuen Versuch
    let mut now = t0;
    let end = Instant::now() + WAIT;
    while dw.connected() && Instant::now() < end {
        dw.update(&mut host, now);
        std::thread::sleep(Duration::from_millis(5));
    }
    assert!(!dw.connected(), "Trennung erkannt");
    let n = *calls.lock().unwrap();
    dw.update(&mut host, now + Duration::from_millis(500));
    assert_eq!(*calls.lock().unwrap(), n, "keine Versuche vor Ablauf von RETRY");
    now += RETRY + Duration::from_millis(1);
    dw.update(&mut host, now);
    assert!(dw.connected(), "Wiederverbindung nach RETRY");
    assert!(matches!(peer2.recv(WAIT), Some(Msg::Hello { .. })));

    // defekte Zeile: Verbindung getrennt, kein Panic
    peer2.to_game.send(b"{kaputt\n".to_vec()).unwrap();
    let end = Instant::now() + WAIT;
    while dw.connected() && Instant::now() < end {
        dw.update(&mut host, now);
        std::thread::sleep(Duration::from_millis(5));
    }
    assert!(!dw.connected(), "defekte Zeile trennt");
    now += RETRY + Duration::from_millis(1);
    dw.update(&mut host, now);
    assert!(dw.connected(), "und die nächste Verbindung steht");
    drop(peer3);
}

#[test]
fn zu_lange_zeile_ohne_ende_trennt() {
    let (streams, peer) = pair(None);
    let mut link = Link::connect(streams).unwrap();
    peer.to_game.send(vec![b'x'; 200 * 1024]).unwrap();
    let end = Instant::now() + WAIT;
    while Instant::now() < end {
        if link.poll() == Some(Event::Lost) {
            return;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    panic!("keine Trennung");
}

#[test]
fn ohne_fenster_nichts() {
    let (op, calls) = opener(Vec::new());
    drop(op);
    let mut host = MockHost::new(1);
    let mut dw = DevWin::default();
    for _ in 0..100 {
        dw.update(&mut host, Instant::now());
    }
    assert!(!dw.is_on() && !dw.connected());
    assert_eq!(host.state_calls.get(), 0);
    assert_eq!(*calls.lock().unwrap(), 0, "nie versucht zu verbinden");
    // gewollt, aber ohne Fenster: nur Versuche im Abstand FAST_RETRY, kein Thread
    let (op, calls) = opener(Vec::new());
    dw.start(op, game());
    let t0 = Instant::now();
    for k in 0..100 {
        dw.update(&mut host, t0 + Duration::from_millis(10) * k);
    }
    let n = *calls.lock().unwrap();
    assert!((3..=5).contains(&n), "{n} Versuche in 1 s (alle 250 ms nach dem Start)");
    assert!(!dw.connected());
    dw.stop();
    assert!(!dw.is_on());
}
