//! Tests zu `parallel.rs` (Port von `engine/balance/parallel_test.go`).

use std::cell::Cell;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::fehler::Fehler;
use crate::parallel::{Crash, ProgressFn, run_all_with, tick_progress};
use crate::run::{Matrix, Tick, run};

/// Go: `DevSeed` (`seedmode.go`, RU25.5).
const DEV_SEED: &str = "k3c-dev";

fn matrix(seeds: &[&str], players: i64) -> Matrix {
    Matrix {
        seeds: seeds.iter().map(|s| s.to_string()).collect(),
        players: vec![players],
        bots: vec!["saver".into()],
        depths: vec![0],
        days: 1,
        ..Matrix::default()
    }
}

/// Parallele Läufe liefern dasselbe wie nacheinander, in der Reihenfolge der Szenarien (B-634, BS1.2).
#[test]
fn parallel_wie_nacheinander() {
    let sc = matrix(&[DEV_SEED, "1", "2"], 2).scenarios().expect("Szenarien");
    let serial = run_all_with(&sc, 0, None, 1).expect("seriell");
    let calls = Cell::new(0);
    let progress: ProgressFn = Box::new(|_done, _total| calls.set(calls.get() + 1));
    let parallel = run_all_with(&sc, 0, Some(progress), 3).expect("parallel");
    assert_eq!(calls.get(), sc.len(), "Fortschrittsmeldungen");
    for (i, (a, b)) in serial.iter().zip(&parallel).enumerate() {
        let same = a.scenario == b.scenario
            && a.valid
            && b.valid
            && a.ticks == b.ticks
            && a.replay.as_ref().map(|r| &r.end_hash) == b.replay.as_ref().map(|r| &r.end_hash)
            && a.metrics == b.metrics;
        assert!(same, "Lauf {i} ({}): seriell und parallel verschieden", sc[i].seed);
    }
}

/// B-924: Eine Panik in `progress` (so beendet die Workbench einen abgebrochenen Lauf) kommt beim Aufrufer als `Crash`
/// an, lässt die übrigen Läufe aus und reißt den Prozess nicht mit.
#[test]
fn parallel_panik_kommt_beim_aufrufer_an() {
    let sc = matrix(&[DEV_SEED, "1", "2", "3"], 1).scenarios().expect("Szenarien");
    let calls = Cell::new(0);
    let progress: ProgressFn = Box::new(|_done, _total| {
        calls.set(calls.get() + 1);
        panic!("abbruch");
    });
    let got = run_all_with(&sc, 0, Some(progress), 2);
    let Err(Crash { value, .. }) = got else {
        panic!("keine Panik: {:?}", got.map(|r| r.len()));
    };
    assert_eq!(value, "abbruch");
    assert!(
        calls.get() < sc.len(),
        "{} Meldungen: die Läufe nach der Panik liefen weiter",
        calls.get()
    );
}

/// Go: `tickProgress` hängt `tick` vor `progress`; ohne `tick` bleibt `progress`, ohne beide ergibt es nichts.
#[test]
fn tick_progress_haengt_tick_vor_progress() {
    assert!(tick_progress(None, None).is_none());
    let n = Arc::new(AtomicUsize::new(0));
    let tick = Tick({
        let n = Arc::clone(&n);
        Arc::new(move || {
            n.fetch_add(1, Ordering::SeqCst);
        })
    });
    let (ticks_seen, seen) = (Cell::new(0), Cell::new((0, 0)));
    let progress: ProgressFn = Box::new(|done, total| {
        ticks_seen.set(n.load(Ordering::SeqCst)); // der Tick lief schon
        seen.set((done, total));
    });
    let mut both = tick_progress(Some(tick.clone()), Some(progress)).expect("beide");
    both(2, 5);
    assert_eq!((ticks_seen.get(), seen.get()), (1, (2, 5)));
    let mut only = tick_progress(Some(tick), None).expect("nur tick");
    only(1, 1);
    assert_eq!(n.load(Ordering::SeqCst), 2);
}

/// Eine Panik im `tick` der Matrix beendet `run` mit `Fehler::Abbruch` (Go: `Run` löst `Crash` erneut aus).
#[test]
fn panik_im_tick_beendet_run() {
    let m = Matrix {
        tick: Some(Tick(Arc::new(|| panic!("tick-abbruch")))),
        ..matrix(&["1"], 1)
    };
    match run(&m) {
        Err(Fehler::Abbruch(c)) => assert_eq!(c.value, "tick-abbruch"),
        other => panic!("Abbruch erwartet: {:?}", other.map(|r| r.results.len())),
    }
}
