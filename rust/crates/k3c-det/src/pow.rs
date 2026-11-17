//! `pow` wie Gos `math.Pow` auf amd64 (`pow.go`, reines Go; `haveArchPow` ist dort falsch).

use crate::bits::{frexp, inf, ldexp, modf, nan};
use crate::exp::exp;
use crate::log::log;

fn is_odd_int(x: f64) -> bool {
    if x.abs() >= (1u64 << 53) as f64 {
        // Ab 2**53 ist jede Zahl gerade; zu große Werte würden beim Umwandeln überlaufen.
        return false;
    }
    let (xi, xf) = modf(x);
    xf == 0.0 && (xi as i64) & 1 == 1
}

/// `pow` liefert x**y, bitgleich zu Gos `math.Pow` auf amd64 mit FMA (siehe `exp`).
pub fn pow(x: f64, y: f64) -> f64 {
    if let Some(r) = special(x, y) {
        return r;
    }
    let (mut yi, mut yf) = modf(y.abs());
    if yf != 0.0 && x < 0.0 {
        return nan();
    }
    if yi >= 9_223_372_036_854_775_808.0 {
        // yi ist eine große gerade ganze Zahl, die nicht in i64 passt.
        if x == -1.0 {
            return 1.0;
        }
        if (x.abs() < 1.0) == (y > 0.0) {
            return 0.0;
        }
        return inf(1);
    }

    // ans = a1 × 2**ae (= 1 für jetzt).
    let mut a1 = 1.0;
    let mut ae: i64 = 0;

    // ans *= x**yf
    if yf != 0.0 {
        if yf > 0.5 {
            yf -= 1.0;
            yi += 1.0;
        }
        a1 = exp(yf * log(x));
    }

    // ans *= x**yi durch wiederholtes Quadrieren, Exponent von x getrennt geführt.
    let (mut x1, mut xe) = frexp(x);
    let mut i = yi as i64;
    while i != 0 {
        #[allow(clippy::manual_range_contains)] // Form wie in Go
        let out_of_range = xe < -(1 << 12) || (1 << 12) < xe;
        if out_of_range {
            // Überlauf oder Unterlauf des Exponenten absehbar; ldexp regelt das.
            ae += xe;
            break;
        }
        if i & 1 == 1 {
            a1 *= x1;
            ae += xe;
        }
        x1 *= x1;
        xe <<= 1;
        if x1 < 0.5 {
            x1 += x1;
            xe -= 1;
        }
        i >>= 1;
    }

    // ans = a1 × 2**ae; bei y < 0 Kehrwert.
    if y < 0.0 {
        a1 = 1.0 / a1;
        ae = -ae;
    }
    ldexp(a1, ae)
}

/// `special` sind die Sonderfälle am Anfang von `pow` (der `switch` in Go), in derselben Reihenfolge.
fn special(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 || x == 1.0 {
        return Some(1.0);
    }
    if y == 1.0 {
        return Some(x);
    }
    if x.is_nan() || y.is_nan() {
        return Some(nan());
    }
    if x == 0.0 {
        if y < 0.0 {
            if x.is_sign_negative() && is_odd_int(y) {
                return Some(inf(-1));
            }
            return Some(inf(1));
        }
        if y > 0.0 {
            if x.is_sign_negative() && is_odd_int(y) {
                return Some(x);
            }
            return Some(0.0);
        }
        return None;
    }
    if y.is_infinite() {
        if x == -1.0 {
            return Some(1.0);
        }
        if (x.abs() < 1.0) == (y == f64::INFINITY) {
            return Some(0.0);
        }
        return Some(inf(1));
    }
    if x.is_infinite() {
        if x == f64::NEG_INFINITY {
            return Some(pow(1.0 / x, -y)); // Pow(-0, -y)
        }
        if y < 0.0 {
            return Some(0.0);
        }
        if y > 0.0 {
            return Some(inf(1));
        }
        return None;
    }
    if y == 0.5 {
        return Some(x.sqrt());
    }
    if y == -0.5 {
        return Some(1.0 / x.sqrt());
    }
    None
}
