//! Port von `notify_test.go`.

use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use super::*;
use crate::simrun::registry::report_dir;
use crate::simrun::run::{CancelToken, DONE, RUNNING, lock};
use crate::simrun::test_helpers::{registry, temp_root, wait_for};

/// B-662/AC-04: Änderungen melden sich gedrosselt (höchstens etwa 2 je Sekunde je Lauf), die letzte kommt immer an; ein
/// fertiger Lauf meldet geänderte Berichte.
#[test]
fn lauf_meldet_aenderungen_gedrosselt() {
    let (_dir, root) = temp_root();
    let g = registry();
    let runs: Arc<Mutex<Vec<Info>>> = Arc::default();
    let (reports_tx, reports_rx) = channel();
    let (seen, own) = (Arc::clone(&runs), root.clone());
    g.set_notify(Notify {
        run: Some(Arc::new(move |r, info| {
            if r == own {
                lock(&seen).push(info);
            }
        })),
        reports: Some(Arc::new(move |r| {
            let _ = reports_tx.send(r.to_path_buf());
        })),
    });
    let release = CancelToken::new();
    let (counted_tx, counted_rx) = channel();
    let (weak, gate) = (Arc::downgrade(&g), release.clone());
    let runner: crate::simrun::Runner = Arc::new(move |_, run| {
        let Some(g) = weak.upgrade() else { return };
        // eine Sekunde lang alle paar Millisekunden Fortschritt
        let mut n = 0;
        let end = Instant::now() + Duration::from_secs(1);
        while Instant::now() < end {
            n += 1;
            g.set_progress(&run, n, 0);
            thread::sleep(Duration::from_millis(5));
        }
        let _ = counted_tx.send(n);
        gate.wait_timeout(Duration::from_secs(5));
        g.finish(&run, DONE, "Pass", Vec::new(), &report_dir(&run));
    });
    let run = g.start(&root, "Repo-Wurzel", "x", None, runner, true).expect("start");
    let updates = counted_rx.recv_timeout(Duration::from_secs(5)).expect("gezählt");
    thread::sleep(Duration::from_secs(1)); // die letzte Meldung kommt höchstens 0,5 s nach der letzten Änderung
    {
        let seen = lock(&runs);
        let n = seen.len();
        assert!(
            (2..=5).contains(&n),
            "{n} Meldungen für {updates} Änderungen in etwa 1 s, erwartet etwa 2 je Sekunde"
        );
        let last = seen.last().expect("Meldung");
        assert!(
            last.done == updates && last.state == RUNNING,
            "letzte Meldung {last:?}, erwartet Fortschritt {updates}"
        );
    }
    release.cancel();
    let from = reports_rx
        .recv_timeout(Duration::from_secs(5))
        .expect("keine Meldung „Berichte geändert“");
    assert_eq!(from, root);
    wait_for(&g, &root, run.id(), |i| !i.report.is_empty());
    let deadline = Instant::now() + Duration::from_secs(5);
    while lock(&runs).last().is_none_or(|i| i.state != DONE) {
        assert!(Instant::now() < deadline, "Ende von {} nicht gemeldet", run.id());
        thread::sleep(Duration::from_millis(20));
    }
}

/// Auch ein entfernter wartender Lauf meldet sich.
#[test]
fn entfernter_wartender_lauf_meldet_sich() {
    let (_dir, root) = temp_root();
    let g = registry();
    let release = CancelToken::new();
    let gate = release.clone();
    let idle: crate::simrun::Runner = Arc::new(move |_, _| {
        gate.wait_timeout(Duration::from_secs(5));
    });
    for _ in 0..2 {
        g.start(&root, "", "x", None, Arc::clone(&idle), true).expect("start");
    }
    let (tx, rx) = channel();
    g.set_notify(Notify {
        run: Some(Arc::new(move |_, info| {
            let _ = tx.send(info.id);
        })),
        reports: None,
    });
    let waiting = g.start(&root, "", "x", None, idle, true).expect("start");
    assert_eq!(rx.recv_timeout(Duration::from_secs(5)).expect("angelegt"), waiting.id());
    g.stop(&root, waiting.id()).expect("stop");
    assert_eq!(rx.recv_timeout(Duration::from_secs(5)).expect("entfernt"), waiting.id());
    release.cancel();
}
