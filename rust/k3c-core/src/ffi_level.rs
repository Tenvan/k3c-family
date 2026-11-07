//! C-ABI des Level-Generators (Go ruft über `engine/core/level.go`).
//!
//! `k3c_level_generate(input, input_len, seed, seed_len, opts, opts_len, out, out_cap) -> i64`
//!
//! - `input`: JSON `{"biome": <level.Biome>, "rules": {…}}` (`core.LevelInput`), `seed`: UTF-8,
//!   `opts`: JSON wie `level.Options` (`planned`, `chunkCount`, optional `withSim`), leer = Nullwert.
//! - Rückgabe ≥ 0: so viele Bytes Layout-JSON stehen in `out` (wie `json.Marshal(level.Layout)`).
//! - [`ERR_INPUT`] (−1): ungültige Eingabe (JSON, Feld unbekannt, Seed kein UTF-8);
//!   [`ERR_GENERATE`] (−2): der Generator lehnt ab (Fehler von `GenerateWith`). In beiden Fällen steht der Fehlertext
//!   in `out`, auf `out_cap` gekürzt und mit einem 0-Byte abgeschlossen, wenn Platz ist.
//! - ≤ −3: `out` ist zu klein; der Betrag ist die benötigte Größe (ein Layout ist nie kürzer als 3 Bytes). Der Aufrufer
//!   ruft mit einem größeren Puffer erneut auf (der Generator ist deterministisch).
//!
//! Ein Panic läuft nie über die Grenze: Fehler sind `Result`, das Profil `panic = "abort"` beendet sonst den Prozess.

use crate::level::{Error, generate_json};

/// Ungültige Eingabe; Fehlertext im Puffer.
pub const ERR_INPUT: i64 = -1;
/// Der Generator lehnt ab; Fehlertext im Puffer.
pub const ERR_GENERATE: i64 = -2;

/// Bytes hinter (Zeiger, Länge); leer bei Nullzeiger oder Länge 0.
///
/// # Safety
/// `ptr` zeigt auf `len` lesbare Bytes, die bis zum Ende des Aufrufs gültig bleiben.
unsafe fn bytes<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    if ptr.is_null() || len == 0 {
        return &[];
    }
    // SAFETY: Aufrufer garantiert `len` lesbare Bytes.
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

/// Erzeugt ein Level; Vertrag siehe Modul-Kommentar.
///
/// # Safety
/// Jeder Zeiger zeigt auf so viele gültige Bytes wie seine Länge angibt (`out` beschreibbar, `out_cap` Bytes) und
/// bleibt bis zur Rückkehr gültig; die Bereiche überlappen nicht.
#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn k3c_level_generate(
    input: *const u8,
    input_len: usize,
    seed: *const u8,
    seed_len: usize,
    opts: *const u8,
    opts_len: usize,
    out: *mut u8,
    out_cap: usize,
) -> i64 {
    let (input, opts) = unsafe { (bytes(input, input_len), bytes(opts, opts_len)) };
    let result = match std::str::from_utf8(unsafe { bytes(seed, seed_len) }) {
        Ok(seed) => generate_json(input, seed, opts),
        Err(e) => Err(Error::Input(format!("Seed ist kein UTF-8: {e}"))),
    };
    let out: &mut [u8] = if out.is_null() || out_cap == 0 {
        &mut []
    } else {
        // SAFETY: Aufrufer garantiert `out_cap` beschreibbare Bytes.
        unsafe { std::slice::from_raw_parts_mut(out, out_cap) }
    };
    match result {
        Ok(json) => write_layout(json.as_bytes(), out),
        Err(Error::Input(msg)) => write_error(ERR_INPUT, &msg, out),
        Err(Error::Generate(msg)) => write_error(ERR_GENERATE, &msg, out),
    }
}

fn write_layout(json: &[u8], out: &mut [u8]) -> i64 {
    if json.len() > out.len() {
        return -(json.len().max(3) as i64);
    }
    out[..json.len()].copy_from_slice(json);
    json.len() as i64
}

/// Fehlertext an einer Zeichengrenze gekürzt, mit 0-Byte, wenn Platz ist.
fn write_error(code: i64, msg: &str, out: &mut [u8]) -> i64 {
    let mut n = msg.len().min(out.len().saturating_sub(1));
    while !msg.is_char_boundary(n) {
        n -= 1;
    }
    out[..n].copy_from_slice(&msg.as_bytes()[..n]);
    if n < out.len() {
        out[n] = 0;
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call(input: &[u8], seed: &[u8], opts: &[u8], cap: usize) -> (i64, Vec<u8>) {
        let mut out = vec![0u8; cap];
        let n = unsafe {
            k3c_level_generate(
                input.as_ptr(),
                input.len(),
                seed.as_ptr(),
                seed.len(),
                opts.as_ptr(),
                opts.len(),
                out.as_mut_ptr(),
                out.len(),
            )
        };
        (n, out)
    }

    #[test]
    fn fehler_und_zu_kleiner_puffer() {
        let (n, out) = call(b"{", b"x", b"", 64);
        assert_eq!(n, ERR_INPUT);
        assert!(out.starts_with("Biom/Regeln".as_bytes()) && out.contains(&0));
        let (n, _) = call(b"{}", &[0xff], b"", 64);
        assert_eq!(n, ERR_INPUT);
        let (n, _) = call(b"{", b"x", b"", 0);
        assert_eq!(n, ERR_INPUT, "Fehler auch ohne Puffer");
    }

    #[test]
    fn puffer_zu_klein_nennt_groesse() {
        let biome = r#"{"id":"t","lengthUnits":{"min":300,"max":300},"chunkWidthUnits":50,"hubWidthUnits":100,
            "portals":{"count":0},"chunkWeights":{"a":1}}"#;
        let input = format!(r#"{{"biome":{biome},"rules":{{}}}}"#);
        let (need, _) = call(input.as_bytes(), b"s", b"", 8);
        assert!(need <= -3, "{need}");
        let (n, out) = call(input.as_bytes(), b"s", b"", (-need) as usize);
        assert_eq!(n, -need);
        assert!(out.starts_with(br#"{"seed":"s","biomeId":"t""#));
    }
}
