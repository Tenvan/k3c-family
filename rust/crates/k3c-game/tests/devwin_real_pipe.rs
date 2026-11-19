//! Spielseite gegen den echten Pipe-Server des Fensters (nur Windows, RU56.5): Das Spiel schreibt trotz wartendem
//! Lese-Thread (der Herzschlag des Servers beendet das Lesen, sonst serialisiert das synchrone Handle) und der
//! `dev`-Befehl kommt als `ack` zurück. Dev-Abhängigkeit nur für diesen Test.
#![cfg(windows)]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode

use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use k3c_devproto::{DevCommand, GameInfo, Msg};
use k3c_devwin::win32::pipe::{self, Sink};
use k3c_game::devwin::mock::MockHost;
use k3c_game::devwin::{DevWin, open_pipe};

struct ChanSink(Mutex<mpsc::Sender<Msg>>);

impl Sink for ChanSink {
    fn msg(&self, m: Msg) -> bool {
        self.0.lock().unwrap().send(m).is_ok()
    }
    fn lost(&self) -> bool {
        true
    }
}

fn wait_for(rx: &Receiver<Msg>, dw: &mut DevWin, host: &mut MockHost, want: impl Fn(&Msg) -> bool) -> Msg {
    let end = Instant::now() + Duration::from_secs(5);
    while Instant::now() < end {
        dw.update(host, Instant::now());
        if let Ok(m) = rx.recv_timeout(Duration::from_millis(10))
            && want(&m)
        {
            return m;
        }
    }
    panic!("Nachricht blieb aus");
}

#[test]
fn befehl_ueber_die_echte_pipe() {
    let name = format!("k3c-devwin-game-test-{}", std::process::id());
    let (tx, rx) = mpsc::channel();
    let to_game = pipe::start(&name, Arc::new(ChanSink(Mutex::new(tx)))).unwrap();
    let mut host = MockHost::new(1);
    let mut dw = DevWin::default();
    dw.start(
        open_pipe(format!(r"\\.\pipe\{name}")),
        GameInfo {
            name: "k3c".into(),
            version: "test".into(),
        },
    );
    let m = wait_for(&rx, &mut dw, &mut host, |m| matches!(m, Msg::Hello { .. }));
    assert!(matches!(m, Msg::Hello { game: Some(_), .. }));
    wait_for(&rx, &mut dw, &mut host, |m| matches!(m, Msg::Catalog(_)));
    // Das Lesen wartet jetzt; der Befehl muss trotzdem herauskommen und das ack zurück.
    std::thread::sleep(Duration::from_millis(300));
    to_game
        .send(Msg::Dev {
            id: 5,
            player: 0,
            action: DevCommand {
                action: "night".into(),
                ..DevCommand::default()
            },
        })
        .unwrap();
    let ack = wait_for(&rx, &mut dw, &mut host, |m| matches!(m, Msg::Ack { .. }));
    assert_eq!(
        ack,
        Msg::Ack {
            id: 5,
            ok: true,
            err: None
        }
    );
    assert_eq!(host.log().len(), 1);
    assert_eq!(host.log()[0].1.action, "night");
}
