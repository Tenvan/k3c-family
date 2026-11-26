//! Port von `workkit/console/console_test.go`.

use std::io::Write;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::*;

#[test]
fn ring_haelt_die_neuesten_zeilen() {
    let s = Store::new(3, None);
    for i in 1..=5 {
        s.add("a", "stdout", &i.to_string());
    }
    let lines = s.tail("a", 0).expect("Quelle a");
    assert!(
        lines.len() == 3 && lines[0].text == "3" && lines[2].seq == 5,
        "Ring: {lines:?}"
    );
    let tail = s.tail("a", 2).expect("Quelle a");
    assert!(tail.len() == 2 && tail[0].text == "4", "Tail: {tail:?}");
    assert!(s.tail("fehlt", 1).is_none(), "unbekannte Quelle gefunden");
}

#[test]
fn reset_behaelt_die_folge() {
    let s = Store::new(10, None);
    s.add("a", "stdout", "alt");
    s.reset("a");
    s.reset("neu");
    s.add("a", "stdout", "neu");
    let lines = s.tail("a", 0).unwrap_or_default();
    assert!(lines.len() == 1 && lines[0].seq == 2, "nach Reset: {lines:?}");
    assert_eq!(s.sources().join(","), "a,neu", "Quellen");
}

#[test]
fn benachrichtigung_blockiert_nie() {
    let (block_tx, block_rx) = mpsc::channel::<()>();
    let block_rx = Mutex::new(block_rx);
    let s = Arc::new(Store::new(
        10,
        Some(Box::new(move |_l| {
            // Wartet, bis der Test den Kanal schließt (wie `<-block` in Go).
            let _ = block_rx.lock().map(|rx| rx.recv());
        })),
    ));
    let (done_tx, done_rx) = mpsc::channel();
    let writer = Arc::clone(&s);
    std::thread::spawn(move || {
        for _ in 0..NOTIFY_QUEUE * 4 {
            writer.add("a", "stdout", "x");
        }
        let _ = done_tx.send(());
    });
    assert!(
        done_rx.recv_timeout(Duration::from_secs(5)).is_ok(),
        "add blockiert auf einen langsamen Empfänger"
    );
    drop(block_tx);
    s.close();
}

#[test]
fn line_writer() {
    let got = Arc::new(Mutex::new(Vec::<String>::new()));
    let sink = Arc::clone(&got);
    let mut w = LineWriter::new(move |line| sink.lock().unwrap().push(line));
    let long = "x".repeat(100_000);
    let mid = format!("te\n{long}\n");
    let parts: [&[u8]; 5] = [b"ers", b"te\r\nzwei", mid.as_bytes(), b"kaputt \xff", b""];
    for part in parts {
        w.write_all(part).unwrap();
    }
    w.flush_line();
    let got = got.lock().unwrap();
    assert!(
        got.len() == 4 && got[0] == "erste" && got[1] == "zweite" && got[2] == long && got[3] == "kaputt \u{FFFD}",
        "Zeilen: {}, erste {:?}, zweite {:?}, letzte {:?}",
        got.len(),
        got.first(),
        got.get(1),
        got.last()
    );
}

#[test]
fn clean_fasst_ungueltige_folgen_zusammen_wie_go() {
    assert_eq!(writer::clean(b"a\xff\xfe\xfdb\r\r"), "a\u{FFFD}b");
    assert_eq!(writer::clean(b"\xc3\xa4\xff"), "ä\u{FFFD}");
}
