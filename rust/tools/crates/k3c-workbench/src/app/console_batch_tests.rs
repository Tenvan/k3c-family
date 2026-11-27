//! Port von `console_batch_test.go`.

use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use k3c_kit::console::Line;

use super::*;

/// 10 000 Zeilen in kurzer Zeit werden zu wenigen Sendungen, jede höchstens `BATCH_MAX` Zeilen, die jüngste Zeile kommt
/// immer an.
#[test]
fn buendler_begrenzt() {
    let sends = Arc::new(Mutex::new(Vec::new()));
    let (seen, (done, wait)) = (Arc::clone(&sends), channel());
    let batcher = LineBatcher::with_window(Duration::from_millis(20), move |lines: Vec<Line>| {
        seen.lock().unwrap().push((lines.len(), lines.last().unwrap().seq));
        let _ = done.send(());
    });
    for seq in 1..=10_000 {
        batcher.add(Line {
            source: "x".to_owned(),
            stream: "stdout".to_owned(),
            text: String::new(),
            seq,
        });
    }
    wait.recv_timeout(Duration::from_secs(2)).expect("nichts gesendet");
    std::thread::sleep(Duration::from_millis(60));
    let sends = sends.lock().unwrap();
    assert!(sends.len() <= 3, "{} Sendungen", sends.len());
    assert!(sends.iter().all(|(len, _)| *len <= BATCH_MAX), "{sends:?}");
    assert_eq!(sends.last().unwrap().1, 10_000);
}
