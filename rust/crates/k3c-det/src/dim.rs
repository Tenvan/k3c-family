//! `fmax` und `fmin` wie Gos `math.Max`/`math.Min` auf amd64 (`dim_amd64.s`, gleiche Regeln wie `dim.go`):
//! ±Inf gewinnt vor NaN, NaN gewinnt sonst, `+0` schlägt `-0` bei `fmax`, `-0` schlägt `+0` bei `fmin`.
//! Auch für Gos eingebautes `max`/`min` auf Gleitkommazahlen (Leitfaden § 5).

use crate::bits::{SIGN_MASK, UVINF, UVNEGINF, inf, nan};

/// `fmax` liefert die größere Zahl, bitgleich zu Gos `math.Max`.
pub fn fmax(x: f64, y: f64) -> f64 {
    let (bx, by) = (x.to_bits(), y.to_bits());
    if bx == UVINF || by == UVINF {
        return inf(1);
    }
    let (ax, ay) = (bx & !SIGN_MASK, by & !SIGN_MASK);
    if ax > UVINF || ay > UVINF {
        return nan();
    }
    if ax | ay == 0 {
        // beide 0: +0 gewinnt
        if bx == SIGN_MASK {
            return y;
        }
        return x;
    }
    // MAXSD
    if x > y { x } else { y }
}

/// `fmin` liefert die kleinere Zahl, bitgleich zu Gos `math.Min`.
pub fn fmin(x: f64, y: f64) -> f64 {
    let (bx, by) = (x.to_bits(), y.to_bits());
    if bx == UVNEGINF || by == UVNEGINF {
        return inf(-1);
    }
    let (ax, ay) = (bx & !SIGN_MASK, by & !SIGN_MASK);
    if ax > UVINF || ay > UVINF {
        return nan();
    }
    if ax | ay == 0 {
        // beide 0: -0 gewinnt
        if bx == SIGN_MASK {
            return x;
        }
        return y;
    }
    // MINSD
    if x < y { x } else { y }
}
