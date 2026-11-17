//! RU3/AC-04: Laden von Platte und eingebettet; Fehlertext einer fehlenden Datei wie in Go.

use std::collections::BTreeSet;

use k3c_det::golden;

use crate::source::EMBEDDED;
use crate::testutil::data_dir;
use crate::{Source, load_all};

/// Eingebettet sind genau die Dateien von `//go:embed *.json biomes/*.json worlds/standard.json`.
#[test]
fn eingebettet_wie_go_embed() {
    let dir = data_dir();
    let mut want = BTreeSet::new();
    for sub in ["", "biomes/"] {
        for e in std::fs::read_dir(dir.join(sub)).expect("data/") {
            let name = e.expect("Eintrag").file_name().into_string().expect("Name");
            if name.ends_with(".json") {
                want.insert(format!("{sub}{name}"));
            }
        }
    }
    want.insert("worlds/standard.json".to_string());
    let got: BTreeSet<String> = EMBEDDED.iter().map(|(n, _)| n.to_string()).collect();
    assert_eq!(got, want);
}

/// Jede eingebettete Datei ist gleich der auf der Platte; beide Quellen laden zum selben Baum.
#[test]
fn platte_und_eingebettet_gleich() {
    let disk = Source::Dir(data_dir());
    for (name, text) in EMBEDDED {
        let from_disk = disk.read(name).expect("Platte");
        assert_eq!(
            golden::diff(
                name,
                &serde_json::from_str(&from_disk).expect("json"),
                &serde_json::from_str(text).expect("json")
            ),
            "",
            "{name}: eingebettet veraltet (cargo build neu)"
        );
    }
    let a = load_all(&disk).expect("von Platte");
    let b = load_all(&Source::Embedded).expect("eingebettet");
    assert_eq!(a, b);
    assert_eq!(a.biomes.len(), 5);
    assert!(
        a.biomes.windows(2).all(|w| w[0].depth <= w[1].depth),
        "Biome nach Tiefe"
    );
}

#[test]
fn fehlende_datei_wie_go() {
    let e = Source::Embedded.read("fehlt.json").expect_err("fehlt");
    assert_eq!(e.to_string(), "data/fehlt.json: open fehlt.json: file does not exist");
    let e = Source::Dir(data_dir()).read("fehlt.json").expect_err("fehlt");
    #[cfg(windows)]
    assert_eq!(
        e.to_string(),
        "data/fehlt.json: open fehlt.json: The system cannot find the file specified."
    );
    #[cfg(not(windows))]
    assert_eq!(
        e.to_string(),
        "data/fehlt.json: open fehlt.json: no such file or directory"
    );
}

#[test]
fn quelle_waehlen_wie_go() {
    assert_eq!(Source::choose(None), Source::Embedded);
    assert_eq!(
        Source::choose(Some(&data_dir().join("gibt-es-nicht"))),
        Source::Embedded
    );
    assert_eq!(Source::choose(Some(&data_dir())), Source::Dir(data_dir()));
}
