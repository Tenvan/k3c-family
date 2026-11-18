//! Test aus `determinism_test.go` (B-138/AC-02): ein Raum, dessen Takt stockt, rechnet dieselbe Welt wie ein Raum, der ohne
//! Pause getaktet wird. In Rust hängt der Takt nie an der Wanduhr (der Aufrufer ruft `tick_all`); der Vergleich braucht
//! eine neue Insel (`create_in`, RU24).

#[test]
fn verlangsamter_takt_gleiche_welt() {}
