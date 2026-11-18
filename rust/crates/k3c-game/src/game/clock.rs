//! Port von `native/k3c-game/game/clock_other.go`: die echte Zeit in Sekunden seit dem Programmstart, nur für Messwerte
//! und die Leistungsanzeige, nie für die Sim (alle Zeit im Spiel kommt aus `Frame::clock`).
//!
//! `clock_windows.go` entfällt: Go brauchte dort einen eigenen Syscall, weil `time.Since` auf Windows je Aufruf Heap
//! allokierte; `std::time::Instant` allokiert nicht.

use std::sync::OnceLock;
use std::time::Instant;

static START: OnceLock<Instant> = OnceLock::new();

/// Echte Zeit in Sekunden seit einem festen, beliebigen Nullpunkt (dem ersten Aufruf); nur für Messwerte.
pub fn clock() -> f64 {
    START.get_or_init(Instant::now).elapsed().as_secs_f64()
}

#[cfg(test)]
mod tests {
    use super::clock;

    #[test]
    fn clock_steigt_nie() {
        let (a, b) = (clock(), clock());
        assert!(a >= 0.0 && b >= a);
    }
}
