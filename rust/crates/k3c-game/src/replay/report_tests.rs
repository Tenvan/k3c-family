use super::*;

/// Alle JSON-Namen des Go-`Report` (`native/k3c-game/game/measure.go`), ohne das optionale `bilder`.
const GO_FIELDS: &[&str] = &[
    "szene",
    "spieler",
    "ebitengine",
    "backend",
    "fenster",
    "vsync",
    "sheets",
    "textur_mib_rgba",
    "rig_mib_rgba",
    "messdauer_s",
    "frames",
    "fps_mittel",
    "fps_1pct_low",
    "frametime_ms",
    "rechenzeit_ms",
    "takt_ms",
    "takt_max_ms",
    "takte",
    "takt_ende",
    "phase",
    "gegner",
    "go_heap_mib",
    "go_sys_mib",
    "bilder_ohne_takt",
    "takt_abstand_ms",
    "gc_laeufe",
    "gc_pause_ms",
];

#[test]
fn bericht_hat_alle_felder_des_go_berichts_in_reihenfolge() {
    let v = serde_json::to_value(Report::default()).unwrap();
    let keys: Vec<&str> = v.as_object().unwrap().keys().map(String::as_str).collect();
    let mut sorted_go = GO_FIELDS.to_vec();
    let mut sorted_keys = keys.clone();
    sorted_go.sort_unstable();
    sorted_keys.sort_unstable();
    assert_eq!(sorted_keys, sorted_go);
    let json = serde_json::to_string(&Report::default()).unwrap();
    let pos: Vec<usize> = GO_FIELDS
        .iter()
        .map(|f| json.find(&format!("\"{f}\":")).unwrap())
        .collect();
    assert!(pos.windows(2).all(|w| w[0] < w[1]), "Reihenfolge wie Go");
    for f in ["p50", "p95", "p99", "max"] {
        assert!(v["frametime_ms"].get(f).is_some() && v["takt_abstand_ms"].get(f).is_some());
    }
}

#[test]
fn kennwerte() {
    // wie Go `percentile`: sorted[round((n-1)*p)] = sorted[round(1,5)] = sorted[2] (RU39.1)
    assert_eq!(percentile(&[1.0, 2.0, 3.0, 4.0], 0.5), 3.0);
    assert_eq!(percentile(&[1.0, 2.0, 3.0, 4.0, 5.0], 0.99), 5.0);
    assert_eq!(percentile(&[], 0.5), 0.0);
    assert_eq!(stats(vec![3.0, 1.0, 2.0]).max, 3.0);
}
