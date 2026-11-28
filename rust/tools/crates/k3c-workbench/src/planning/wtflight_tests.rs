//! Go `wtflight_test.go`.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use super::planning::Data;
use super::wtdiff::WorktreeDiff;
use super::wtflight::{Flight, MAX_BASES};

fn pause() {
    thread::sleep(Duration::from_millis(2));
}

/// Viele gleichzeitige Aufrufe teilen sich wenige Läufe (ein laufender plus höchstens ein Folgelauf).
#[test]
fn flight_buendelt_gleichzeitige_aufrufe() {
    let flight = Arc::new(Flight::default());
    let runs = Arc::new(AtomicUsize::new(0));
    let (release, released) = mpsc::channel::<()>();
    let released = Arc::new(std::sync::Mutex::new(released));
    let threads: Vec<_> = (0..50)
        .map(|_| {
            let arrived = Instant::now();
            let (flight, runs, released) = (Arc::clone(&flight), Arc::clone(&runs), Arc::clone(&released));
            thread::spawn(move || {
                let diffs = flight.run(arrived, |_| {
                    runs.fetch_add(1, Ordering::SeqCst);
                    // Blockiert, bis der Sender weg ist.
                    let _ = released.lock().expect("Sperre").recv();
                    Ok(vec![WorktreeDiff {
                        branch: "b".to_owned(),
                        ..WorktreeDiff::default()
                    }])
                });
                assert_eq!(diffs.expect("Ergebnis").len(), 1);
            })
        })
        .collect();
    thread::sleep(Duration::from_millis(50));
    drop(release);
    for thread in threads {
        thread.join().expect("Thread");
    }
    let runs = runs.load(Ordering::SeqCst);
    assert!(runs <= 2, "{runs} Läufe für 50 gleichzeitige Aufrufe");
}

/// Wer nach dem Ende eines Laufs ankommt, bekommt einen frischen Lauf.
#[test]
fn flight_frischer_lauf_nach_ankunft() {
    let flight = Flight::default();
    let mut runs = 0;
    for _ in 0..3 {
        pause();
        let diffs = flight.run(Instant::now(), |_| {
            runs += 1;
            Ok(Vec::new())
        });
        diffs.expect("Lauf");
    }
    assert_eq!(runs, 3);
}

/// Der Cache der Merge-Bases bleibt zwischen den Läufen und ist begrenzt.
#[test]
fn flight_base_cache() {
    let flight = Flight::default();
    pause();
    let first = flight.run(Instant::now(), |bases| {
        bases.insert("x".to_owned(), Data::default());
        Ok(Vec::new())
    });
    first.expect("Lauf");
    {
        let mut state = flight.state.lock().expect("Sperre");
        assert!(state.bases.contains_key("x"), "Stand nach dem Lauf nicht im Cache");
        for i in 0..=MAX_BASES {
            state.bases.insert(format!("a{i}"), Data::default());
        }
    }
    pause();
    let second = flight.run(Instant::now(), |bases| {
        assert!(bases.is_empty(), "Cache nicht begrenzt: {} Einträge", bases.len());
        Ok(Vec::new())
    });
    second.expect("Lauf");
}
