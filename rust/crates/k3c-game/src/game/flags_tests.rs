//! Tests für `flags.rs` (neu, kein Go-Gegenstück: Go nutzt das Paket `flag`).

use super::*;

const SPECS: &[Spec] = &[
    spec("flow", Kind::Bool, "false"),
    spec("name", Kind::Str, "std"),
    spec("count", Kind::Int, "8"),
    spec("ratio", Kind::Float, "0"),
];

fn args(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

fn parsed(v: &[&str]) -> Result<FlagSet, String> {
    let mut fs = FlagSet::new(SPECS);
    fs.parse(&args(v))?;
    Ok(fs)
}

#[test]
fn standardwerte() {
    let fs = parsed(&[]).unwrap();
    assert!(!fs.bool("flow"));
    assert_eq!((fs.str("name"), fs.int("count"), fs.float("ratio")), ("std", 8, 0.0));
    assert!(fs.args().is_empty());
}

#[test]
fn ein_und_zwei_striche_gleich_und_leerzeichen() {
    let fs = parsed(&["-name", "a", "--count=5", "--ratio", "-1.5", "-flow"]).unwrap();
    assert_eq!((fs.str("name"), fs.int("count"), fs.float("ratio")), ("a", 5, -1.5));
    assert!(fs.bool("flow"));
}

#[test]
fn bool_mit_wert_und_wert_mit_minus() {
    assert!(!parsed(&["--flow=false"]).unwrap().bool("flow"));
    assert!(parsed(&["--flow=true"]).unwrap().bool("flow"));
    assert_eq!(parsed(&["--count", "-3"]).unwrap().int("count"), -3);
    assert!(parsed(&["--flow=vielleicht"]).is_err());
}

#[test]
fn letzter_wert_gewinnt_und_zahlenformen() {
    assert_eq!(parsed(&["--count", "1", "--count", "2"]).unwrap().int("count"), 2);
    assert_eq!(parsed(&["--count=0x10"]).unwrap().int("count"), 16);
    assert_eq!(parsed(&["--count=010"]).unwrap().int("count"), 8);
    assert_eq!(parsed(&["--count=+7"]).unwrap().int("count"), 7);
}

#[test]
fn fehler() {
    for bad in [
        &["--unbekannt"][..],
        &["--count"],
        &["--count", "x"],
        &["--count=1.5"],
        &["---flow"],
        &["--=x"],
    ] {
        assert!(parsed(bad).is_err(), "angenommen: {bad:?}");
    }
}

#[test]
fn rest_und_doppelstrich() {
    let fs = parsed(&["--count", "2", "ueber", "--flow"]).unwrap();
    assert_eq!(fs.args(), &args(&["ueber", "--flow"])[..]);
    let fs = parsed(&["--", "--flow"]).unwrap();
    assert!(!fs.bool("flow"));
    assert_eq!(fs.args(), &args(&["--flow"])[..]);
    assert_eq!(parsed(&["-"]).unwrap().args(), &args(&["-"])[..]);
}

#[test]
fn set_default_ersetzt_standard() {
    let mut fs = FlagSet::new(SPECS);
    fs.set_default("name", "x");
    fs.parse(&[]).unwrap();
    assert_eq!(fs.str("name"), "x");
}
