//! Zeitsammlung: fester Takt, Rest als `alpha` für die Interpolation.

/// Takt der Simulation in Sekunden (30 Hz).
pub const STEP: f64 = 1.0 / 30.0;
/// Höchstens so viele Schritte je Bild; mehr Rückstand wird verworfen (kein Aufholen im Kreis).
pub const MAX_STEPS: u32 = 4;
/// Rundungsspielraum, damit 2 × 1/60 s sicher einen vollen Takt ergeben.
const EPS: f64 = 1e-9;

/// Sammelt Bildzeit und liefert Takt-Schritte.
#[derive(Default, Debug, Clone)]
pub struct Clock {
    acc: f64,
}

impl Clock {
    pub fn new() -> Self {
        Self::default()
    }

    /// Nimmt `dt` Sekunden auf und liefert `(Schritte, alpha)`; `alpha` liegt in [0, 1).
    pub fn advance(&mut self, dt: f64) -> (u32, f64) {
        if dt.is_finite() && dt > 0.0 {
            self.acc += dt;
        }
        let mut n = 0;
        while self.acc + EPS >= STEP {
            self.acc = (self.acc - STEP).max(0.0);
            n += 1;
            if n == MAX_STEPS {
                self.acc = 0.0;
                break;
            }
        }
        (n, (self.acc / STEP).clamp(0.0, 1.0 - f64::EPSILON))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sechzig_hz_gibt_abwechselnd_null_und_ein_takt() {
        let mut c = Clock::new();
        let mut total = 0;
        let mut last = 1;
        for i in 0..600 {
            let (n, a) = c.advance(1.0 / 60.0);
            assert!(n <= 1, "Bild {i}: {n} Schritte");
            assert!((0.0..1.0).contains(&a), "alpha {a}");
            if i > 0 {
                assert_ne!(n, last, "Bild {i}: kein Wechsel");
            }
            last = n;
            total += n;
        }
        assert_eq!(total, 300);
    }

    #[test]
    fn rueckstand_wird_auf_vier_schritte_begrenzt() {
        let mut c = Clock::new();
        let (n, a) = c.advance(1.0);
        assert_eq!(n, MAX_STEPS);
        assert!((0.0..1.0).contains(&a));
        assert_eq!(c.advance(0.0).0, 0, "Rest wurde verworfen");
    }

    #[test]
    fn ungueltige_zeit_zaehlt_nicht() {
        let mut c = Clock::new();
        assert_eq!(c.advance(f64::NAN).0, 0);
        assert_eq!(c.advance(-1.0).0, 0);
    }
}
