//! Tests von `run.rs` (RU30.6): Exit-Code und Meldungen wie `fatal` in `main.go`, ungültige Schalter vor jedem Fenster.

use super::*;

fn strs(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| (*s).to_string()).collect()
}

#[test]
fn fehler_ist_exit_1_ende_ist_exit_0() {
    assert_eq!(finish(Ok(()), false), 0);
    assert_eq!(finish(Ok(()), true), 0);
    assert_eq!(finish(Err("kaputt".to_string()), false), 1);
}

#[test]
fn ungueltige_schalter_enden_vor_dem_fenster() {
    let (res, quiet) = start(&strs(&["--gibt-es-nicht"]));
    let msg = res.err().unwrap_or_default();
    assert!(msg.starts_with("🚫 ungültige Schalter"), "Meldung: {msg}");
    assert!(!quiet);
}

#[test]
fn assets_endet_ohne_fenster_und_ohne_endmeldung() {
    let (res, quiet) = start(&strs(&["--assets"]));
    assert!(res.is_ok(), "{res:?}");
    assert!(quiet, "--assets meldet das Ende nicht (Go: return vor der Meldung)");
}

#[test]
fn wegwerf_ordner_der_messung_liegt_neben_den_spielstaenden() {
    let saves = format!(
        "{}/../../../.work/run-measure-{}",
        env!("CARGO_MANIFEST_DIR"),
        std::process::id()
    );
    let args = launch::parse(&strs(&[
        "--saves",
        &saves,
        "--measure",
        "1",
        "--grafik",
        "gibt-es-nicht",
    ]))
    .unwrap();
    // Die Grafik fehlt: das Fenster würde die Meldung zeigen, `launch::run` öffnet es hier nicht, darum prüft der Test
    // nur die Ordner-Logik von `shot_saves` und `remove_temp`.
    let tmp = shot_saves(&args).unwrap();
    assert!(tmp.starts_with(&saves) && tmp.is_dir(), "{tmp:?}");
    remove_temp(&tmp);
    assert!(!tmp.exists(), "Wegwerf-Ordner entfernt");
    let _ = std::fs::remove_dir_all(&saves);
}
