//! Hilfen für `omitempty` aus Go (`skip_serializing_if`): Nullwerte fehlen in der Ausgabe wie in Go.

pub fn is_false(v: &bool) -> bool {
    !*v
}

pub fn is_zero_i64(v: &i64) -> bool {
    *v == 0
}

/// Go prüft `== 0`; auch `-0.0` gilt als leer.
pub fn is_zero_f64(v: &f64) -> bool {
    *v == 0.0
}
