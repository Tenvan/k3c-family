//! RU22.5 (AC-03): Die größte Fließkomma-Abweichung je Golden bleibt unter der Toleranz (1e-4 relativ). Die Tabelle
//! (`cargo test -p k3c-sim --test golden_abweichung -- --nocapture`) füllt den G3-Bericht `docs/messung/rust-sim-abgleich.md`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

mod common;

use common::deviation::run_deviation;
use common::golden_run::load;
use k3c_det::golden;

fn names() -> Vec<String> {
    let dir = golden::testdata().join("golden");
    let mut v: Vec<String> = std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| {
            let n = e.unwrap().file_name().to_string_lossy().into_owned();
            n.strip_prefix("sim-")
                .and_then(|n| n.strip_suffix(".json.gz"))
                .map(str::to_string)
        })
        .collect();
    v.sort();
    v
}

#[test]
fn groesste_fliesskomma_abweichung_je_golden_unter_der_toleranz() {
    let names = names();
    assert!(names.len() >= 13, "nur {} Golden-Läufe", names.len());
    for n in names {
        let run = load(&n);
        let (dev, floats, exact) = run_deviation(&run);
        println!("sim-{n}: größte relative Abweichung {dev:e}, {floats} Fließkommawerte, {exact} bitgleich");
        assert!(dev <= 1e-4, "sim-{n}: {dev:e} über der Toleranz");
    }
}
