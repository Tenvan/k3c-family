use super::*;

#[test]
fn set_schaltet_nur_den_genannten_effekt() {
    assert!(on("haze") && on("stars"));
    set("haze", false);
    assert!(!on("haze") && on("stars"));
    set("haze", true);
    assert!(on("haze"));
    set("gibt-es-nicht", false);
    assert!(on("gibt-es-nicht"));
    assert!(EFFECTS.len() <= 32);
}
