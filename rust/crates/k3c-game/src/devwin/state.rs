//! Zustandsabzug für das Fenster: höchstens 4 Hz und nur bei Änderung (RU56.5, B-1133). Die Schleife baut den
//! `State` nur, wenn die Frist abgelaufen ist; ohne Fenster wird nie gebaut.

use std::time::{Duration, Instant};

use k3c_devproto::State;

/// MIN_GAP ist der kleinste Abstand zweier `state`-Nachrichten (4 Hz).
pub const MIN_GAP: Duration = Duration::from_millis(250);

/// Throttle hält Zeit und Inhalt der letzten gesendeten Nachricht.
#[derive(Default)]
pub struct Throttle {
    last: Option<Instant>,
    sent: Option<State>,
}

impl Throttle {
    /// rejected nimmt den Zustand als nicht gesendet zurück (Warteschlange voll): er geht beim nächsten Mal erneut hinaus.
    pub fn rejected(&mut self) {
        self.sent = None;
    }

    /// next liefert den Zustand, wenn seit der letzten Nachricht `MIN_GAP` vergangen ist und er sich geändert hat;
    /// `make` läuft nur bei abgelaufener Frist.
    pub fn next(&mut self, now: Instant, make: impl FnOnce() -> State) -> Option<State> {
        if self.last.is_some_and(|t| now.saturating_duration_since(t) < MIN_GAP) {
            return None;
        }
        self.last = Some(now);
        let st = make();
        if self.sent.as_ref() == Some(&st) {
            return None;
        }
        self.sent = Some(st.clone());
        Some(st)
    }
}
