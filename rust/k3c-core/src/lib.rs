//! k3c-core: Simulationskern von K3C (Entscheidung 006). Die C-ABI ist die einzige Schnittstelle; Go lädt die
//! Bibliothek zur Laufzeit (`engine/core`) ohne cgo. Jede Funktion ist `extern "C"`, nimmt nur einfache Werte
//! oder (Zeiger, Länge) und panikt nie über die Grenze (Profil `panic = "abort"`).

/// ABI-Version: wird erhöht, wenn sich eine Signatur ändert; Go prüft sie beim Laden.
pub const ABI_VERSION: u32 = 1;

/// Version der ABI (für die Prüfung beim Laden).
#[unsafe(no_mangle)]
pub extern "C" fn k3c_abi_version() -> u32 {
    ABI_VERSION
}

/// Probe-Funktion: Summe zweier Zahlen (misst die reinen Aufrufkosten Go → Rust).
#[unsafe(no_mangle)]
pub extern "C" fn k3c_add(a: i64, b: i64) -> i64 {
    a.wrapping_add(b)
}

/// Probe-Funktion mit Puffer: Summe aller Bytes (prüft die Übergabe von Zeiger und Länge).
///
/// # Safety
/// `ptr` muss auf `len` lesbare Bytes zeigen (oder `len` ist 0).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn k3c_sum_bytes(ptr: *const u8, len: usize) -> u64 {
    if ptr.is_null() || len == 0 {
        return 0;
    }
    // SAFETY: Aufrufer garantiert `len` lesbare Bytes.
    let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
    bytes.iter().map(|&b| u64::from(b)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_und_add() {
        assert_eq!(k3c_abi_version(), ABI_VERSION);
        assert_eq!(k3c_add(40, 2), 42);
        assert_eq!(k3c_add(i64::MAX, 1), i64::MIN);
    }

    #[test]
    fn summe_der_bytes() {
        let data = [1u8, 2, 3, 250];
        assert_eq!(unsafe { k3c_sum_bytes(data.as_ptr(), data.len()) }, 256);
        assert_eq!(unsafe { k3c_sum_bytes(std::ptr::null(), 4) }, 0);
    }
}
