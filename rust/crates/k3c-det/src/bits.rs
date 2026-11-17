//! Bit-Hilfen aus Gos `math` (bits.go, frexp.go, ldexp.go, modf.go), wie amd64 sie rechnet (reines Go).

pub(crate) const UVNAN: u64 = 0x7FF8_0000_0000_0001;
pub(crate) const UVINF: u64 = 0x7FF0_0000_0000_0000;
pub(crate) const UVNEGINF: u64 = 0xFFF0_0000_0000_0000;
pub(crate) const SIGN_MASK: u64 = 1 << 63;
const MASK: u64 = 0x7FF;
const SHIFT: u64 = 64 - 11 - 1;
const BIAS: i64 = 1023;

/// `nan` ist Gos `math.NaN()` (andere Bits als `f64::NAN`).
pub(crate) fn nan() -> f64 {
    f64::from_bits(UVNAN)
}

/// `inf` ist Gos `math.Inf(sign)`.
pub(crate) fn inf(sign: i64) -> f64 {
    if sign >= 0 {
        f64::from_bits(UVINF)
    } else {
        f64::from_bits(UVNEGINF)
    }
}

/// `normalize` liefert eine normale Zahl y und den Exponenten exp mit x == y × 2**exp.
fn normalize(x: f64) -> (f64, i64) {
    const SMALLEST_NORMAL: f64 = 2.2250738585072014e-308; // 2**-1022
    if x.abs() < SMALLEST_NORMAL {
        return (x * (1u64 << 52) as f64, -52);
    }
    (x, 0)
}

/// `frexp` zerlegt f in einen Bruch in [½, 1) und eine Zweierpotenz.
pub(crate) fn frexp(f: f64) -> (f64, i64) {
    if f == 0.0 || f.is_infinite() || f.is_nan() {
        return (f, 0); // -0 bleibt -0
    }
    let (f, mut exp) = normalize(f);
    let mut x = f.to_bits();
    exp += ((x >> SHIFT) & MASK) as i64 - BIAS + 1;
    x &= !(MASK << SHIFT);
    x |= ((-1 + BIAS) as u64) << SHIFT;
    (f64::from_bits(x), exp)
}

/// `ldexp` ist die Umkehrung von `frexp`: frac × 2**exp.
pub(crate) fn ldexp(frac: f64, exp: i64) -> f64 {
    if frac == 0.0 || frac.is_infinite() || frac.is_nan() {
        return frac; // -0 bleibt -0
    }
    let (frac, e) = normalize(frac);
    let mut exp = exp + e;
    let mut x = frac.to_bits();
    exp += ((x >> SHIFT) & MASK) as i64 - BIAS;
    if exp < -1075 {
        return 0.0_f64.copysign(frac); // Unterlauf
    }
    if exp > 1023 {
        // Überlauf
        if frac < 0.0 {
            return inf(-1);
        }
        return inf(1);
    }
    let mut m: f64 = 1.0;
    if exp < -1022 {
        // denormal
        exp += 53;
        m = 1.0 / (1u64 << 53) as f64; // 2**-53
    }
    x &= !(MASK << SHIFT);
    x |= ((exp + BIAS) as u64) << SHIFT;
    m * f64::from_bits(x)
}

/// `modf` liefert ganzen und gebrochenen Teil mit dem Vorzeichen von f.
pub(crate) fn modf(f: f64) -> (f64, f64) {
    let integer = f.trunc();
    let fractional = (f - integer).copysign(f);
    (integer, fractional)
}
