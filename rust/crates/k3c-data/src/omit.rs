//! Hilfen für `omitempty` aus Go (`skip_serializing_if`): Nullwerte fehlen in der Ausgabe wie in Go.

pub(crate) fn is_false(v: &bool) -> bool {
    !*v
}

pub(crate) fn is_zero_i64(v: &i64) -> bool {
    *v == 0
}

#[allow(dead_code)] // omitempty auf float64; RU3.4 hat noch kein solches Feld
pub(crate) fn is_zero_f64(v: &f64) -> bool {
    *v == 0.0
}
