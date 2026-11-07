//! C-ABI für `rng` (Go ruft über `engine/core`).

use crate::rng::{Rng, hash_seed};

/// FNV-1a-Hash eines UTF-8-Seeds.
///
/// # Safety
/// `seed` muss auf `seed_len` gültige UTF-8-Bytes zeigen.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn k3c_rng_hash(seed: *const u8, seed_len: usize) -> u32 {
    hash_seed(&unsafe { seed_str(seed, seed_len) })
}

/// Schreibt die ersten `n` Zahlen von `Rng::new(seed).next()` nach `out`.
///
/// # Safety
/// `seed` zeigt auf `seed_len` UTF-8-Bytes, `out` auf Platz für `n` f64.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn k3c_rng_next_n(seed: *const u8, seed_len: usize, out: *mut f64, n: usize) {
    if out.is_null() {
        return;
    }
    let mut r = Rng::new(&unsafe { seed_str(seed, seed_len) });
    for i in 0..n {
        unsafe { *out.add(i) = r.next() };
    }
}

/// Seed als Text (ungültiges UTF-8 wird ersetzt; Go liefert immer gültiges).
///
/// # Safety
/// `ptr` zeigt auf `len` lesbare Bytes (oder `len` ist 0).
pub(crate) unsafe fn seed_str(ptr: *const u8, len: usize) -> String {
    if ptr.is_null() || len == 0 {
        return String::new();
    }
    String::from_utf8_lossy(unsafe { std::slice::from_raw_parts(ptr, len) }).into_owned()
}
