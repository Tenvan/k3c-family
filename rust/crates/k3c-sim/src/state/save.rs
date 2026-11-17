//! Spielstand (Port von `engine/sim/save.go`). RU8.4 liefert nur `key`, das die Knoten-Markierung braucht; RU8.5 ergänzt
//! den Rest.

/// Go: `key` ist `kind@x` mit x wie `x.toFixed(2)` in JS: Gleichstände (nur bei x = n/8) runden weg von 0, Go `FormatFloat`
/// würde zur geraden Ziffer runden.
pub fn key(kind: &str, x: f64) -> String {
    let mut x = x;
    let t = x * 8.0;
    if t == t.trunc() && t % 2.0 != 0.0 {
        x += 0.001_f64.copysign(x);
    }
    format!("{kind}@{x:.2}")
}
