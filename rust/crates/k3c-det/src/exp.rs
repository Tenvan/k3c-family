//! `exp` wie Gos `math.Exp` auf amd64: Assembler `exp_amd64.s`, FMA-Pfad.
//!
//! Go wählt auf amd64 zur Laufzeit (`useFMA = AVX && FMA`): mit FMA die Befehle `VFNMADD231SD`/`VFMADD213SD`,
//! sonst einen SSE-Pfad, der bei etwa 9 % der Eingaben in der letzten Stelle abweicht. Alle Golden stammen von
//! Rechnern mit FMA; hier gilt deshalb immer der FMA-Pfad. `mul_add` rundet wie ein FMA-Befehl genau einmal,
//! auch ohne FMA in der CPU. Das ist die einzige erlaubte Stelle für `mul_add` (Leitfaden § 5).
#![allow(clippy::excessive_precision, clippy::approx_constant)] // Konstanten wörtlich aus der Go-Quelle

use crate::bits::{UVINF, UVNEGINF};

const LOG2E: f64 = 1.4426950408889634073599246810018920; // 1/LN2
const LN2U: f64 = 0.69314718055966295651160180568695068359375; // obere Hälfte LN2
const LN2L: f64 = 0.28235290563031577122588448175013436025525412068e-12; // untere Hälfte LN2
const OVERFLOW: f64 = 7.09782712893384e+02;

// exprodata aus exp_amd64.s, gleiche Reihenfolge.
const E0: f64 = 0.5;
const E1: f64 = 1.0;
const E2: f64 = 2.0;
const E3: f64 = 1.6666666666666666667e-1;
const E4: f64 = 4.1666666666666666667e-2;
const E5: f64 = 8.3333333333333333333e-3;
const E6: f64 = 1.3888888888888888889e-3;
const E7: f64 = 1.9841269841269841270e-4;
const E8: f64 = 2.4801587301587301587e-5;

/// `exp` liefert e**x, bitgleich zu Gos `math.Exp` auf amd64 mit FMA.
pub fn exp(x: f64) -> f64 {
    let bx = x.to_bits();
    // nicht endlich: -Inf → 0, NaN und +Inf → x
    if UVINF <= bx & !(1 << 63) {
        if bx == UVNEGINF {
            return 0.0;
        }
        return x;
    }
    if x > OVERFLOW {
        return f64::from_bits(UVINF);
    }
    // CVTSD2SL rundet zur nächsten geraden Zahl; außerhalb von i32 liefert er i32::MIN (wie `as` hier).
    let k = (LOG2E * x).round_ties_even() as i32;
    let kf = f64::from(k);
    let mut x0 = (-kf).mul_add(LN2U, x);
    x0 = (-kf).mul_add(LN2L, x0);
    // Argument verkleinern
    x0 *= 0.0625;
    // Taylor-Reihe
    let mut x1 = E8;
    for c in [E7, E6, E5, E4, E3, E0, E1] {
        x1 = x1.mul_add(x0, c);
    }
    x0 *= x1;
    for _ in 0..3 {
        x1 = x0 + E2;
        x0 *= x1;
    }
    x1 = x0 + E2;
    x0 = x0.mul_add(x1, E1);
    scale(x0, k)
}

/// `scale` ist der Schluss von exp_amd64.s: fr × 2**k mit Überlauf, denormalen Zahlen und Unterlauf.
fn scale(x0: f64, k: i32) -> f64 {
    let b = k + 0x3FF; // Bias
    if b <= 0 {
        // denormal
        if b < -52 {
            return 0.0;
        }
        let x0 = x0 * f64::from_bits(((b + 0x3FE) as u64) << 52);
        return x0 * f64::from_bits(1u64 << 52);
    }
    if b >= 0x7FF {
        return f64::from_bits(UVINF);
    }
    x0 * f64::from_bits((b as u64) << 52)
}
