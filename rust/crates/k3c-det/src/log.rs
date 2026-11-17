//! `log` wie Gos `math.Log` auf amd64: Assembler `log_amd64.s` (nicht `log.go`).
//!
//! Unterschiede zu `log.go`, wie der Assembler rechnet: denormale Zahlen werden nicht normalisiert, ein negatives
//! NaN liefert Gos NaN, und der Vergleich mit √2/2 schließt Gleichheit ein (`CMPSD …, 5`, nicht kleiner).
#![allow(clippy::excessive_precision, clippy::approx_constant)] // Konstanten wörtlich aus der Go-Quelle

use crate::bits::{SIGN_MASK, UVINF, inf, nan};

const HSQRT2: f64 = 7.07106781186547524401e-01; // sqrt(2)/2
const LN2HI: f64 = 6.93147180369123816490e-01; // 0x3fe62e42fee00000
const LN2LO: f64 = 1.90821492927058770002e-10; // 0x3dea39ef35793c76
const L1: f64 = 6.666666666666735130e-01; // 0x3FE5555555555593
const L2: f64 = 3.999999999940941908e-01; // 0x3FD999999997FA04
const L3: f64 = 2.857142874366239149e-01; // 0x3FD2492494229359
const L4: f64 = 2.222219843214978396e-01; // 0x3FCC71C51D8E78AF
const L5: f64 = 1.818357216161805012e-01; // 0x3FC7466496CB03DE
const L6: f64 = 1.531383769920937332e-01; // 0x3FC39A09D078C69F
const L7: f64 = 1.479819860511658591e-01; // 0x3FC2F112DF3E5244

/// `log` liefert den natürlichen Logarithmus, bitgleich zu Gos `math.Log` auf amd64.
pub fn log(x: f64) -> f64 {
    let bx = x.to_bits();
    if bx & !SIGN_MASK == 0 {
        return inf(-1); // ±0
    }
    if bx & SIGN_MASK != 0 {
        return nan(); // negativ, auch -Inf und negatives NaN
    }
    if UVINF <= bx {
        return x; // +Inf oder NaN
    }
    // f1, ki := math.Frexp(x); k := float64(ki), ohne Normalisieren
    let f1 = f64::from_bits((bx & 0x000F_FFFF_FFFF_FFFF) | 0.5_f64.to_bits());
    let ki = ((bx >> 52) & 0x7FF) as i32 - 0x3FE;
    let mut k = f64::from(ki);
    // if f1 <= √2/2 { k -= 1; f1 *= 2 }
    let adj = if f1 <= HSQRT2 { 1.0 } else { 0.0 };
    k -= adj;
    let f1 = f1 * (adj + 1.0);
    let f = f1 - 1.0;
    let s = f / (2.0 + f);
    let s2 = s * s;
    let s4 = s2 * s2;
    let t1 = s2 * (L1 + s4 * (L3 + s4 * (L5 + s4 * L7)));
    let t2 = s4 * (L2 + s4 * (L4 + s4 * L6));
    let r = t1 + t2;
    let hfsq = 0.5 * f * f;
    k * LN2HI - ((hfsq - (s * (hfsq + r) + k * LN2LO)) - f)
}
