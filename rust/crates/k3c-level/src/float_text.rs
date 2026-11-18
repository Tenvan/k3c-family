//! `write_go_float`: derselbe Text wie `k3c_det::golden::go_float`, aber ohne `String` je Aufruf (B-1079).
//! Der Text ist Teil der Variantenströme (`<biom>:<seed>:variant:<gruppe>:<x>`) und muss bitgleich bleiben.

use std::fmt::{self, Write};

/// Fester Puffer auf dem Stack: `{:e}` eines `f64` hat höchstens 17 Ziffern, Punkt, `e`, Vorzeichen und drei Ziffern.
struct Stack {
    buf: [u8; 40],
    len: usize,
}

impl Write for Stack {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let end = self.len + s.len();
        self.buf
            .get_mut(self.len..end)
            .ok_or(fmt::Error)?
            .copy_from_slice(s.as_bytes());
        self.len = end;
        Ok(())
    }
}

/// Schreibt `x` wie Gos `%v` (siehe `go_float`): kürzeste Ziffern, Exponent ab 1e+06 und unter 1e-04.
pub(crate) fn write_go_float(out: &mut impl Write, x: f64) -> fmt::Result {
    if x == 0.0 {
        return out.write_str(if x.is_sign_negative() { "-0" } else { "0" });
    }
    if !x.is_finite() {
        return write!(out, "{x}");
    }
    let mut e = Stack { buf: [0; 40], len: 0 };
    write!(e, "{:e}", x.abs())?;
    let text = std::str::from_utf8(&e.buf[..e.len]).map_err(|_| fmt::Error)?;
    let (mant, exp) = text.split_once('e').ok_or(fmt::Error)?;
    let exp: i32 = exp.parse().map_err(|_| fmt::Error)?;
    if x < 0.0 {
        out.write_char('-')?;
    }
    if !(-4..6).contains(&exp) {
        let sign = if exp < 0 { '-' } else { '+' };
        return write!(out, "{mant}e{sign}{:02}", exp.abs());
    }
    let (int, frac) = mant.split_once('.').unwrap_or((mant, ""));
    let digits = int.len() + frac.len(); // `int` ist genau eine Ziffer
    if exp < 0 {
        out.write_str("0.")?;
        for _ in 0..(-exp - 1) {
            out.write_char('0')?;
        }
        out.write_str(int)?;
        return out.write_str(frac);
    }
    let dp = (exp + 1) as usize;
    out.write_str(int)?;
    if digits <= dp {
        out.write_str(frac)?;
        for _ in 0..dp - digits {
            out.write_char('0')?;
        }
        return Ok(());
    }
    out.write_str(&frac[..dp - 1])?;
    out.write_char('.')?;
    out.write_str(&frac[dp - 1..])
}

#[cfg(test)]
#[path = "float_text_tests.rs"]
mod float_text_tests;
